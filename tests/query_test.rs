use csvcat::cli::{BorderStyle, Cli};
use csvcat::query::QueryEngine;
use csvcat::types::RecordSet;

fn create_sample_rs() -> RecordSet {
    let headers = vec![
        "id".to_string(),
        "name".to_string(),
        "age".to_string(),
        "salary".to_string(),
        "status".to_string(),
    ];
    let raw_rows = vec![
        vec![
            "1".to_string(),
            "Alice".to_string(),
            "30".to_string(),
            "70000".to_string(),
            "active".to_string(),
        ],
        vec![
            "2".to_string(),
            "Bob".to_string(),
            "25".to_string(),
            "50000".to_string(),
            "inactive".to_string(),
        ],
        vec![
            "3".to_string(),
            "Charlie".to_string(),
            "35".to_string(),
            "90000".to_string(),
            "active".to_string(),
        ],
        vec![
            "4".to_string(),
            "Diana".to_string(),
            "28".to_string(),
            "60000".to_string(),
            "active".to_string(),
        ],
    ];

    RecordSet::new(headers, raw_rows, ',')
}

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
fn test_where_filtering() {
    let rs = create_sample_rs();
    let mut cli = create_default_cli();
    cli.where_clause = Some(vec!["salary > 55000".to_string()]);

    let res = QueryEngine::process(&rs, &cli).unwrap();
    assert_eq!(res.row_count(), 3);
    assert_eq!(res.raw_rows[0][1], "Alice");
    assert_eq!(res.raw_rows[1][1], "Charlie");
    assert_eq!(res.raw_rows[2][1], "Diana");
}

#[test]
fn test_regex_filtering() {
    let rs = create_sample_rs();
    let mut cli = create_default_cli();
    cli.filter = Some("Charlie|Diana".to_string());

    let res = QueryEngine::process(&rs, &cli).unwrap();
    assert_eq!(res.row_count(), 2);
    assert_eq!(res.raw_rows[0][1], "Charlie");
    assert_eq!(res.raw_rows[1][1], "Diana");
}

#[test]
fn test_sorting_desc() {
    let rs = create_sample_rs();
    let mut cli = create_default_cli();
    cli.sort = Some("salary:desc".to_string());

    let res = QueryEngine::process(&rs, &cli).unwrap();
    assert_eq!(res.row_count(), 4);
    assert_eq!(res.raw_rows[0][1], "Charlie"); // 90000
    assert_eq!(res.raw_rows[1][1], "Alice"); // 70000
    assert_eq!(res.raw_rows[2][1], "Diana"); // 60000
    assert_eq!(res.raw_rows[3][1], "Bob"); // 50000
}

#[test]
fn test_column_selection() {
    let rs = create_sample_rs();
    let mut cli = create_default_cli();
    cli.cols = Some(vec!["name".to_string(), "salary".to_string()]);

    let res = QueryEngine::process(&rs, &cli).unwrap();
    assert_eq!(res.col_count(), 2);
    assert_eq!(res.headers, vec!["name", "salary"]);
    assert_eq!(res.raw_rows[0], vec!["Alice", "70000"]);
}

#[test]
fn test_row_slicing() {
    let rs = create_sample_rs();
    let mut cli = create_default_cli();
    cli.offset = 1;
    cli.head = Some(2);

    let res = QueryEngine::process(&rs, &cli).unwrap();
    assert_eq!(res.row_count(), 2);
    assert_eq!(res.raw_rows[0][1], "Bob");
    assert_eq!(res.raw_rows[1][1], "Charlie");
}
