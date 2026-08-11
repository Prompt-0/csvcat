use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataValue {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

impl DataValue {
    pub fn parse(input: &str) -> Self {
        let trimmed = input.trim();
        if trimmed.is_empty()
            || trimmed.eq_ignore_ascii_case("null")
            || trimmed.eq_ignore_ascii_case("none")
            || trimmed.eq_ignore_ascii_case("n/a")
            || trimmed.eq_ignore_ascii_case("nan")
        {
            return DataValue::Null;
        }

        if trimmed.eq_ignore_ascii_case("true") {
            return DataValue::Boolean(true);
        }
        if trimmed.eq_ignore_ascii_case("false") {
            return DataValue::Boolean(false);
        }

        if let Ok(i) = trimmed.parse::<i64>() {
            return DataValue::Integer(i);
        }

        if let Ok(f) = trimmed.parse::<f64>() {
            if f.is_finite() {
                return DataValue::Float(f);
            }
        }

        DataValue::String(input.to_string())
    }

    pub fn data_type(&self) -> DataType {
        match self {
            DataValue::Null => DataType::Null,
            DataValue::Boolean(_) => DataType::Boolean,
            DataValue::Integer(_) => DataType::Integer,
            DataValue::Float(_) => DataType::Float,
            DataValue::String(_) => DataType::String,
        }
    }

    pub fn to_string_repr(&self) -> String {
        match self {
            DataValue::Null => "null".to_string(),
            DataValue::Boolean(b) => b.to_string(),
            DataValue::Integer(i) => i.to_string(),
            DataValue::Float(f) => {
                if f.fract() == 0.0 {
                    format!("{:.1}", f)
                } else {
                    f.to_string()
                }
            }
            DataValue::String(s) => s.clone(),
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            DataValue::Integer(i) => Some(*i as f64),
            DataValue::Float(f) => Some(*f),
            _ => None,
        }
    }
}

impl fmt::Display for DataValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_repr())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataType {
    Integer,
    Float,
    Boolean,
    String,
    Null,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            DataType::Integer => "Integer",
            DataType::Float => "Float",
            DataType::Boolean => "Boolean",
            DataType::String => "String",
            DataType::Null => "Null",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Clone)]
pub struct RecordSet {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<DataValue>>,
    pub raw_rows: Vec<Vec<String>>,
    pub delimiter: char,
}

impl RecordSet {
    pub fn new(headers: Vec<String>, raw_rows: Vec<Vec<String>>, delimiter: char) -> Self {
        let rows = raw_rows
            .iter()
            .map(|row| row.iter().map(|cell| DataValue::parse(cell)).collect())
            .collect();

        RecordSet {
            headers,
            rows,
            raw_rows,
            delimiter,
        }
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn col_count(&self) -> usize {
        self.headers.len()
    }
}
