use csvcat::profiler::Profiler;
use csvcat::types::{DataType, RecordSet};

#[test]
fn test_profiler_statistics() {
    let headers = vec!["val".to_string()];
    let raw_rows = vec![
        vec!["10".to_string()],
        vec!["20".to_string()],
        vec!["30".to_string()],
        vec!["40".to_string()],
        vec!["50".to_string()],
    ];

    let rs = RecordSet::new(headers, raw_rows, ',');
    let stats = Profiler::profile(&rs);

    assert_eq!(stats.len(), 1);
    let col = &stats[0];

    assert_eq!(col.data_type, DataType::Integer);
    assert_eq!(col.min, Some(10.0));
    assert_eq!(col.max, Some(50.0));
    assert_eq!(col.mean, Some(30.0));
    assert_eq!(col.median, Some(30.0));
    assert_eq!(col.null_count, 0);
    assert_eq!(col.unique_count, 5);
    assert!(!col.sparkline.is_empty());
}
