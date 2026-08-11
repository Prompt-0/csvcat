use csvcat::render::Exporter;
use csvcat::types::RecordSet;

fn create_sample_rs() -> RecordSet {
    let headers = vec!["name".to_string(), "age".to_string()];
    let raw_rows = vec![
        vec!["Alice".to_string(), "30".to_string()],
        vec!["Bob".to_string(), "25".to_string()],
    ];

    RecordSet::new(headers, raw_rows, ',')
}

#[test]
fn test_markdown_export() {
    let rs = create_sample_rs();
    let md = Exporter::to_markdown(&rs);

    assert!(md.contains("| name | age |"));
    assert!(md.contains("| --- | --- |"));
    assert!(md.contains("| Alice | 30 |"));
    assert!(md.contains("| Bob | 25 |"));
}

#[test]
fn test_json_export() {
    let rs = create_sample_rs();
    let json_str = Exporter::to_json(&rs).unwrap();

    assert!(json_str.contains("\"name\": \"Alice\""));
    assert!(json_str.contains("\"age\": 30"));
}

#[test]
fn test_jsonl_export() {
    let rs = create_sample_rs();
    let jsonl_str = Exporter::to_jsonl(&rs).unwrap();

    let lines: Vec<&str> = jsonl_str.lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(lines[0].contains("\"name\":\"Alice\""));
    assert!(lines[1].contains("\"name\":\"Bob\""));
}
