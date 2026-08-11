use colored::Colorize;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Color, Table};
use std::collections::HashSet;

use crate::types::{DataType, DataValue, RecordSet};

#[derive(Debug)]
pub struct ColumnStats {
    pub name: String,
    pub index: usize,
    pub data_type: DataType,
    pub total_count: usize,
    pub null_count: usize,
    pub unique_count: usize,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub mean: Option<f64>,
    pub median: Option<f64>,
    pub sparkline: String,
}

pub struct Profiler;

impl Profiler {
    pub fn profile(rs: &RecordSet) -> Vec<ColumnStats> {
        let total_rows = rs.rows.len();
        let mut stats = Vec::new();

        for (col_idx, header) in rs.headers.iter().enumerate() {
            let mut null_cnt = 0;
            let mut unique_set = HashSet::new();
            let mut numbers = Vec::new();
            let mut type_counts = [0usize; 5]; // Integer, Float, Boolean, String, Null

            for row in &rs.rows {
                let val = &row[col_idx];
                unique_set.insert(val.to_string_repr());

                match val {
                    DataValue::Null => {
                        null_cnt += 1;
                        type_counts[4] += 1;
                    }
                    DataValue::Integer(i) => {
                        numbers.push(*i as f64);
                        type_counts[0] += 1;
                    }
                    DataValue::Float(f) => {
                        numbers.push(*f);
                        type_counts[1] += 1;
                    }
                    DataValue::Boolean(_) => {
                        type_counts[2] += 1;
                    }
                    DataValue::String(_) => {
                        type_counts[3] += 1;
                    }
                }
            }

            // Determine predominant datatype
            let predominant_type = if null_cnt == total_rows {
                DataType::Null
            } else {
                let max_idx = type_counts[..4]
                    .iter()
                    .enumerate()
                    .max_by_key(|&(_, count)| count)
                    .map(|(idx, _)| idx)
                    .unwrap_or(3);

                match max_idx {
                    0 => DataType::Integer,
                    1 => DataType::Float,
                    2 => DataType::Boolean,
                    _ => DataType::String,
                }
            };

            let (min, max, mean, median, sparkline) = if !numbers.is_empty() {
                numbers.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                let min_val = numbers[0];
                let max_val = numbers[numbers.len() - 1];
                let sum: f64 = numbers.iter().sum();
                let mean_val = sum / numbers.len() as f64;

                let median_val = if numbers.len() % 2 == 0 {
                    (numbers[numbers.len() / 2 - 1] + numbers[numbers.len() / 2]) / 2.0
                } else {
                    numbers[numbers.len() / 2]
                };

                let spark = Self::generate_sparkline(&numbers, 10);
                (
                    Some(min_val),
                    Some(max_val),
                    Some(mean_val),
                    Some(median_val),
                    spark,
                )
            } else {
                (None, None, None, None, "".to_string())
            };

            stats.push(ColumnStats {
                name: header.clone(),
                index: col_idx,
                data_type: predominant_type,
                total_count: total_rows,
                null_count: null_cnt,
                unique_count: unique_set.len(),
                min,
                max,
                mean,
                median,
                sparkline,
            });
        }

        stats
    }

    fn generate_sparkline(sorted_vals: &[f64], num_buckets: usize) -> String {
        if sorted_vals.is_empty() {
            return "".to_string();
        }

        let min = sorted_vals[0];
        let max = sorted_vals[sorted_vals.len() - 1];
        if (max - min).abs() < 1e-9 {
            return "██████████".to_string();
        }

        let mut counts = vec![0usize; num_buckets];
        let bucket_width = (max - min) / (num_buckets as f64);

        for &v in sorted_vals {
            let mut bucket = ((v - min) / bucket_width).floor() as usize;
            if bucket >= num_buckets {
                bucket = num_buckets - 1;
            }
            counts[bucket] += 1;
        }

        let max_count = counts.iter().copied().max().unwrap_or(1);
        let spark_chars = [' ', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

        counts
            .iter()
            .map(|&c| {
                if c == 0 {
                    ' '
                } else {
                    let idx = ((c as f64 / max_count as f64) * 7.0).round() as usize;
                    spark_chars[idx.min(7)]
                }
            })
            .collect()
    }

    pub fn print_summary(rs: &RecordSet, no_color: bool) {
        let stats = Self::profile(rs);

        println!("\n{}", "=== DATASET PROFILING SUMMARY ===".bold());
        println!(
            "Total Rows: {} | Total Columns: {} | Delimiter: '{}'\n",
            rs.row_count().to_string().cyan(),
            rs.col_count().to_string().cyan(),
            rs.delimiter
        );

        let mut table = Table::new();
        table.load_preset(UTF8_FULL);
        table.set_header(vec![
            "Col #",
            "Name",
            "Type",
            "Nulls (%)",
            "Unique",
            "Min",
            "Max",
            "Mean",
            "Median",
            "Distribution Sparkline",
        ]);

        for s in &stats {
            let null_pct = if s.total_count > 0 {
                (s.null_count as f64 / s.total_count as f64) * 100.0
            } else {
                0.0
            };

            let null_str = format!("{} ({:.1}%)", s.null_count, null_pct);
            let min_str = s.min.map_or("-".to_string(), |v| format!("{:.2}", v));
            let max_str = s.max.map_or("-".to_string(), |v| format!("{:.2}", v));
            let mean_str = s.mean.map_or("-".to_string(), |v| format!("{:.2}", v));
            let med_str = s.median.map_or("-".to_string(), |v| format!("{:.2}", v));

            let type_cell = if no_color {
                Cell::new(s.data_type.to_string())
            } else {
                match s.data_type {
                    DataType::Integer | DataType::Float => {
                        Cell::new(s.data_type.to_string()).fg(Color::Cyan)
                    }
                    DataType::Boolean => Cell::new(s.data_type.to_string()).fg(Color::Magenta),
                    DataType::String => Cell::new(s.data_type.to_string()).fg(Color::Green),
                    DataType::Null => Cell::new(s.data_type.to_string()).fg(Color::DarkGrey),
                }
            };

            table.add_row(vec![
                Cell::new(s.index),
                Cell::new(&s.name),
                type_cell,
                Cell::new(null_str),
                Cell::new(s.unique_count),
                Cell::new(min_str),
                Cell::new(max_str),
                Cell::new(mean_str),
                Cell::new(med_str),
                Cell::new(&s.sparkline).fg(Color::Yellow),
            ]);
        }

        println!("{}\n", table);
    }
}
