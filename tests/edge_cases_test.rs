use csvcat::cli::{BorderStyle, Cli};
use csvcat::query::QueryEngine;
use csvcat::reader::CsvReader;
use csvcat::render::Exporter;
use csvcat::types::{DataValue, RecordSet};
use std::fs::File;
use std::io::Write;
use flate2::write::GzEncoder;
use flate2::Compression;

fn create_default_cli() -> Cli {
    Cli {
        file: None,
        delimiter: None,
        border: BorderStyle::Rounded,
        stats: false,
        interactive: false,
        cols: None,
        where_clause: None,
        filter: None,
        sort: None,
        head: None,
        tail: None,
        offset: 0,
        export: None,
        wrap: false,
        max_col_width: None,
        row_numbers: false,
        no_color: false,
        no_page: false,
    }
}

#[test]
fn test_data_value_parsing() {
    assert_eq!(DataValue::parse("123"), DataValue::Integer(123));
    assert_eq!(DataValue::parse("-456"), DataValue::Integer(-456));
    assert_eq!(DataValue::parse("3.14159"), DataValue::Float(3.14159));
    assert_eq!(DataValue::parse("true"), DataValue::Boolean(true));
    assert_eq!(DataValue::parse("FALSE"), DataValue::Boolean(false));
    assert_eq!(DataValue::parse(""), DataValue::Null);
    assert_eq!(DataValue::parse("NULL"), DataValue::Null);
    assert_eq!(DataValue::parse("N/A"), DataValue::Null);
    assert_eq!(DataValue::parse("hello world"), DataValue::String("hello world".to_string()));
}

#[test]
fn test_gzip_file_reading() {
    let tmp_dir = std::env::temp_dir();
    let gz_path = tmp_dir.join("test_data.csv.gz");

    {
        let file = File::create(&gz_path).unwrap();
        let mut encoder = GzEncoder::new(file, Compression::default());
        encoder.write_all(b"item,qty,price\nApple,10,1.5\nBanana,20,0.8\n").unwrap();
        encoder.finish().unwrap();
    }

    let rs = CsvReader::read(Some(&gz_path), None).unwrap();
    assert_eq!(rs.row_count(), 2);
    assert_eq!(rs.col_count(), 3);
    assert_eq!(rs.headers, vec!["item", "qty", "price"]);
    assert_eq!(rs.raw_rows[0], vec!["Apple", "10", "1.5"]);
    assert_eq!(rs.raw_rows[1], vec!["Banana", "20", "0.8"]);

    let _ = std::fs::remove_file(gz_path);
}

#[test]
fn test_invalid_column_error() {
    let headers = vec!["id".to_string(), "name".to_string()];
    let raw_rows = vec![vec!["1".to_string(), "Alice".to_string()]];
    let rs = RecordSet::new(headers, raw_rows, ',');

    let mut cli = create_default_cli();
    cli.cols = Some(vec!["non_existent_col".to_string()]);

    let res = QueryEngine::process(&rs, &cli);
    assert!(res.is_err());
    assert!(res.unwrap_err().to_string().contains("not found in headers"));
}

#[test]
fn test_contains_operator() {
    let headers = vec!["email".to_string()];
    let raw_rows = vec![
        vec!["alice@gmail.com".to_string()],
        vec!["bob@yahoo.com".to_string()],
        vec!["charlie@gmail.com".to_string()],
    ];
    let rs = RecordSet::new(headers, raw_rows, ',');

    let mut cli = create_default_cli();
    cli.where_clause = Some(vec!["email contains gmail".to_string()]);

    let res = QueryEngine::process(&rs, &cli).unwrap();
    assert_eq!(res.row_count(), 2);
    assert_eq!(res.raw_rows[0][0], "alice@gmail.com");
    assert_eq!(res.raw_rows[1][0], "charlie@gmail.com");
}

#[test]
fn test_csv_export() {
    let headers = vec!["col1".to_string(), "col2".to_string()];
    let raw_rows = vec![vec!["val1".to_string(), "val2".to_string()]];
    let rs = RecordSet::new(headers, raw_rows, ',');

    let csv_out = Exporter::to_csv(&rs).unwrap();
    assert!(csv_out.contains("col1,col2"));
    assert!(csv_out.contains("val1,val2"));
}
