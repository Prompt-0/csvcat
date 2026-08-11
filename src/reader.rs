use anyhow::{Context, Result};
use csv::{ReaderBuilder, StringRecord};
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;

use crate::types::RecordSet;

pub struct CsvReader;

impl CsvReader {
    pub fn read(path: Option<&Path>, explicit_delimiter: Option<char>) -> Result<RecordSet> {
        let (raw_data, _is_gzipped) = match path {
            Some(p) if p.to_string_lossy() != "-" => {
                let file = File::open(p).with_context(|| format!("Failed to open file: {:?}", p))?;
                let mut reader = BufReader::new(file);
                let mut magic = [0u8; 2];
                let bytes_read = reader.read(&mut magic).unwrap_or(0);

                let file = File::open(p)?;
                let buf_reader = BufReader::new(file);
                let mut buffer = Vec::new();

                let is_gz = bytes_read == 2 && magic[0] == 0x1f && magic[1] == 0x8b;
                if is_gz {
                    let mut gz = GzDecoder::new(buf_reader);
                    gz.read_to_end(&mut buffer)
                        .context("Failed to decompress gzipped CSV stream")?;
                } else {
                    let mut br = buf_reader;
                    br.read_to_end(&mut buffer)
                        .context("Failed to read file contents")?;
                }
                (buffer, is_gz)
            }
            _ => {
                let stdin = io::stdin();
                let mut handle = stdin.lock();
                let mut buffer = Vec::new();
                handle
                    .read_to_end(&mut buffer)
                    .context("Failed to read from STDIN")?;
                (buffer, false)
            }
        };

        let delimiter = match explicit_delimiter {
            Some(d) => d,
            None => Self::detect_delimiter(&raw_data),
        };

        let mut csv_reader = ReaderBuilder::new()
            .delimiter(delimiter as u8)
            .has_headers(true)
            .flexible(true)
            .from_reader(&raw_data[..]);

        let headers = csv_reader
            .headers()
            .context("Failed to parse CSV headers")?
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let mut raw_rows = Vec::new();
        let num_cols = headers.len();

        for result in csv_reader.records() {
            let record: StringRecord = result.context("Error reading CSV row record")?;
            let mut row: Vec<String> = record.iter().map(|s| s.to_string()).collect();
            if row.len() < num_cols {
                row.resize(num_cols, "".to_string());
            } else if row.len() > num_cols {
                row.truncate(num_cols);
            }
            raw_rows.push(row);
        }

        Ok(RecordSet::new(headers, raw_rows, delimiter))
    }

    pub fn detect_delimiter(data: &[u8]) -> char {
        let text = match std::str::from_utf8(data) {
            Ok(t) => t,
            Err(_) => return ',',
        };

        let candidate_delimiters = [',', '\t', ';', '|', ':'];
        let lines: Vec<&str> = text.lines().take(15).collect();

        if lines.is_empty() {
            return ',';
        }

        let mut best_delimiter = ',';
        let mut max_score = -1.0f64;

        for &delim in &candidate_delimiters {
            let counts: Vec<usize> = lines
                .iter()
                .map(|line| line.split(delim).count() - 1)
                .collect();

            if counts.is_empty() {
                continue;
            }

            let first_count = counts[0];
            if first_count == 0 {
                continue;
            }

            let all_same = counts.iter().all(|&c| c == first_count);
            let avg = counts.iter().sum::<usize>() as f64 / counts.len() as f64;

            let score = if all_same { avg * 10.0 } else { avg };

            if score > max_score {
                max_score = score;
                best_delimiter = delim;
            }
        }

        best_delimiter
    }
}
