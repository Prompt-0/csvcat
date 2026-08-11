use csvcat::reader::CsvReader;

#[test]
fn test_delimiter_detection() {
    let csv_data = b"name,age,salary\nAlice,30,70000\nBob,25,50000\n";
    assert_eq!(CsvReader::detect_delimiter(csv_data), ',');

    let tsv_data = b"name\tage\tsalary\nAlice\t30\t70000\nBob\t25\t50000\n";
    assert_eq!(CsvReader::detect_delimiter(tsv_data), '\t');

    let pipe_data = b"name|age|salary\nAlice|30|70000\nBob|25|50000\n";
    assert_eq!(CsvReader::detect_delimiter(pipe_data), '|');

    let semi_data = b"name;age;salary\nAlice;30;70000\nBob;25;50000\n";
    assert_eq!(CsvReader::detect_delimiter(semi_data), ';');
}
