use anyhow::Result;
use serde_json::{json, Map, Value};

use crate::types::RecordSet;

pub struct Exporter;

impl Exporter {
    pub fn to_markdown(rs: &RecordSet) -> String {
        let mut out = String::new();

        // Header row
        out.push_str("| ");
        out.push_str(&rs.headers.join(" | "));
        out.push_str(" |\n");

        // Separator row
        out.push_str("| ");
        let separators: Vec<&str> = rs.headers.iter().map(|_| "---").collect();
        out.push_str(&separators.join(" | "));
        out.push_str(" |\n");

        // Data rows
        for row in &rs.raw_rows {
            out.push_str("| ");
            let escaped: Vec<String> = row.iter().map(|s| s.replace('|', "\\|")).collect();
            out.push_str(&escaped.join(" | "));
            out.push_str(" |\n");
        }

        out
    }

    pub fn to_json(rs: &RecordSet) -> Result<String> {
        let mut list = Vec::new();

        for row in &rs.rows {
            let mut obj = Map::new();
            for (idx, header) in rs.headers.iter().enumerate() {
                let cell_val = &row[idx];
                let json_val = match cell_val {
                    crate::types::DataValue::Null => Value::Null,
                    crate::types::DataValue::Boolean(b) => Value::Bool(*b),
                    crate::types::DataValue::Integer(i) => json!(*i),
                    crate::types::DataValue::Float(f) => json!(*f),
                    crate::types::DataValue::String(s) => Value::String(s.clone()),
                };
                obj.insert(header.clone(), json_val);
            }
            list.push(Value::Object(obj));
        }

        Ok(serde_json::to_string_pretty(&list)?)
    }

    pub fn to_jsonl(rs: &RecordSet) -> Result<String> {
        let mut lines = Vec::new();

        for row in &rs.rows {
            let mut obj = Map::new();
            for (idx, header) in rs.headers.iter().enumerate() {
                let cell_val = &row[idx];
                let json_val = match cell_val {
                    crate::types::DataValue::Null => Value::Null,
                    crate::types::DataValue::Boolean(b) => Value::Bool(*b),
                    crate::types::DataValue::Integer(i) => json!(*i),
                    crate::types::DataValue::Float(f) => json!(*f),
                    crate::types::DataValue::String(s) => Value::String(s.clone()),
                };
                obj.insert(header.clone(), json_val);
            }
            lines.push(serde_json::to_string(&Value::Object(obj))?);
        }

        Ok(lines.join("\n"))
    }

    pub fn to_csv(rs: &RecordSet) -> Result<String> {
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(rs.delimiter as u8)
            .from_writer(Vec::new());

        wtr.write_record(&rs.headers)?;
        for row in &rs.raw_rows {
            wtr.write_record(row)?;
        }

        let bytes = wtr.into_inner()?;
        Ok(String::from_utf8(bytes)?)
    }
}
