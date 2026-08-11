use anyhow::{anyhow, Result};
use regex::Regex;

use crate::cli::Cli;
use crate::types::{DataValue, RecordSet};

pub struct QueryEngine;

impl QueryEngine {
    pub fn process(rs: &RecordSet, cli: &Cli) -> Result<RecordSet> {
        let mut processed = rs.clone();

        // 1. Where Filtering
        if let Some(ref conditions) = cli.where_clause {
            for cond in conditions {
                processed = Self::filter_where(&processed, cond)?;
            }
        }

        // 2. Regex Filtering
        if let Some(ref pattern) = cli.filter {
            let re = Regex::new(pattern)?;
            let mut keep_indices = Vec::new();
            for (idx, row) in processed.raw_rows.iter().enumerate() {
                if row.iter().any(|cell| re.is_match(cell)) {
                    keep_indices.push(idx);
                }
            }
            processed.rows = keep_indices
                .iter()
                .map(|&i| processed.rows[i].clone())
                .collect();
            processed.raw_rows = keep_indices
                .iter()
                .map(|&i| processed.raw_rows[i].clone())
                .collect();
        }

        // 3. Multi-column Sorting
        if let Some(ref sort_spec) = cli.sort {
            Self::sort_recordset(&mut processed, sort_spec)?;
        }

        // 4. Column Selection
        if let Some(ref cols_spec) = cli.cols {
            processed = Self::select_columns(&processed, cols_spec)?;
        }

        // 5. Row Slicing (Offset, Head, Tail)
        let total_rows = processed.rows.len();
        let mut start = cli.offset.min(total_rows);
        let mut end = total_rows;

        if let Some(head) = cli.head {
            end = (start + head).min(total_rows);
        }

        if let Some(tail) = cli.tail {
            if tail < (end - start) {
                start = end - tail;
            }
        }

        processed.rows = processed.rows[start..end].to_vec();
        processed.raw_rows = processed.raw_rows[start..end].to_vec();

        Ok(processed)
    }

    fn filter_where(rs: &RecordSet, condition: &str) -> Result<RecordSet> {
        let (col_name, op, target_val_str) = parse_condition(condition)?;

        let col_idx = rs
            .headers
            .iter()
            .position(|h| h.eq_ignore_ascii_case(&col_name))
            .ok_or_else(|| {
                anyhow!(
                    "Column '{}' not found in headers: {:?}",
                    col_name,
                    rs.headers
                )
            })?;

        let target_val = DataValue::parse(&target_val_str);

        let mut keep_indices = Vec::new();
        for (idx, row) in rs.rows.iter().enumerate() {
            let cell = &row[col_idx];
            if evaluate_condition(cell, &op, &target_val, &target_val_str) {
                keep_indices.push(idx);
            }
        }

        Ok(RecordSet {
            headers: rs.headers.clone(),
            rows: keep_indices.iter().map(|&i| rs.rows[i].clone()).collect(),
            raw_rows: keep_indices
                .iter()
                .map(|&i| rs.raw_rows[i].clone())
                .collect(),
            delimiter: rs.delimiter,
        })
    }

    fn sort_recordset(rs: &mut RecordSet, sort_spec: &str) -> Result<()> {
        let parts: Vec<&str> = sort_spec.split(',').collect();
        let mut criteria = Vec::new();

        for part in parts {
            let subparts: Vec<&str> = part.split(':').map(|s| s.trim()).collect();
            let col_name = subparts[0];
            let desc = subparts
                .get(1)
                .map_or(false, |&s| s.eq_ignore_ascii_case("desc"));

            let col_idx = rs
                .headers
                .iter()
                .position(|h| h.eq_ignore_ascii_case(col_name))
                .ok_or_else(|| anyhow!("Sort column '{}' not found in headers", col_name))?;

            criteria.push((col_idx, desc));
        }

        let mut paired: Vec<(Vec<DataValue>, Vec<String>)> = rs
            .rows
            .iter()
            .cloned()
            .zip(rs.raw_rows.iter().cloned())
            .collect();

        paired.sort_by(|a, b| {
            for &(col_idx, desc) in &criteria {
                let val_a = &a.0[col_idx];
                let val_b = &b.0[col_idx];

                let cmp = match (val_a, val_b) {
                    (DataValue::Null, DataValue::Null) => std::cmp::Ordering::Equal,
                    (DataValue::Null, _) => std::cmp::Ordering::Greater,
                    (_, DataValue::Null) => std::cmp::Ordering::Less,
                    (DataValue::Integer(i1), DataValue::Integer(i2)) => i1.cmp(i2),
                    (DataValue::Float(f1), DataValue::Float(f2)) => {
                        f1.partial_cmp(f2).unwrap_or(std::cmp::Ordering::Equal)
                    }
                    (DataValue::Integer(i1), DataValue::Float(f2)) => (*i1 as f64)
                        .partial_cmp(f2)
                        .unwrap_or(std::cmp::Ordering::Equal),
                    (DataValue::Float(f1), DataValue::Integer(i2)) => {
                        f1.partial_cmp(&(*i2 as f64)).unwrap_or(std::cmp::Ordering::Equal)
                    }
                    (DataValue::Boolean(b1), DataValue::Boolean(b2)) => b1.cmp(b2),
                    _ => val_a.to_string_repr().cmp(&val_b.to_string_repr()),
                };

                let final_cmp = if desc { cmp.reverse() } else { cmp };
                if final_cmp != std::cmp::Ordering::Equal {
                    return final_cmp;
                }
            }
            std::cmp::Ordering::Equal
        });

        rs.rows = paired.iter().map(|p| p.0.clone()).collect();
        rs.raw_rows = paired.iter().map(|p| p.1.clone()).collect();

        Ok(())
    }

    fn select_columns(rs: &RecordSet, col_specs: &[String]) -> Result<RecordSet> {
        let mut target_indices = Vec::new();

        for spec in col_specs {
            let spec_trim = spec.trim();
            if let Ok(idx) = spec_trim.parse::<usize>() {
                if idx < rs.headers.len() {
                    target_indices.push(idx);
                } else {
                    return Err(anyhow!("Column index {} out of bounds", idx));
                }
            } else if let Some(pos) = rs
                .headers
                .iter()
                .position(|h| h.eq_ignore_ascii_case(spec_trim))
            {
                target_indices.push(pos);
            } else {
                return Err(anyhow!("Column '{}' not found in headers", spec_trim));
            }
        }

        let new_headers = target_indices
            .iter()
            .map(|&i| rs.headers[i].clone())
            .collect();
        let new_rows = rs
            .rows
            .iter()
            .map(|row| target_indices.iter().map(|&i| row[i].clone()).collect())
            .collect();
        let new_raw_rows = rs
            .raw_rows
            .iter()
            .map(|row| target_indices.iter().map(|&i| row[i].clone()).collect())
            .collect();

        Ok(RecordSet {
            headers: new_headers,
            rows: new_rows,
            raw_rows: new_raw_rows,
            delimiter: rs.delimiter,
        })
    }
}

fn parse_condition(condition: &str) -> Result<(String, String, String)> {
    let operators = ["==", "!=", ">=", "<=", ">", "<", "contains"];

    for op in &operators {
        if let Some(pos) = condition.find(op) {
            let left = condition[..pos].trim().to_string();
            let right = condition[pos + op.len()..]
                .trim()
                .trim_matches('\'')
                .trim_matches('"')
                .to_string();
            return Ok((left, op.to_string(), right));
        }
    }

    Err(anyhow!(
        "Invalid condition format: '{}'. Expected operator like ==, !=, >, <, >=, <=, contains",
        condition
    ))
}

fn evaluate_condition(cell: &DataValue, op: &str, target: &DataValue, target_str: &str) -> bool {
    match op {
        "==" => cell == target || cell.to_string_repr().eq_ignore_ascii_case(target_str),
        "!=" => cell != target && !cell.to_string_repr().eq_ignore_ascii_case(target_str),
        "contains" => cell
            .to_string_repr()
            .to_lowercase()
            .contains(&target_str.to_lowercase()),
        ">" | ">=" | "<" | "<=" => {
            if let (Some(c_num), Some(t_num)) = (cell.as_f64(), target.as_f64()) {
                match op {
                    ">" => c_num > t_num,
                    ">=" => c_num >= t_num,
                    "<" => c_num < t_num,
                    "<=" => c_num <= t_num,
                    _ => false,
                }
            } else {
                let c_str = cell.to_string_repr();
                let t_str = target_str;
                match op {
                    ">" => c_str.as_str() > t_str,
                    ">=" => c_str.as_str() >= t_str,
                    "<" => c_str.as_str() < t_str,
                    "<=" => c_str.as_str() <= t_str,
                    _ => false,
                }
            }
        }
        _ => false,
    }
}
