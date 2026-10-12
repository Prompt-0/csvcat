use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "csvcat",
    author = "Ritesh",
    version = "0.1.0",
    about = "Modern Terminal CSV/TSV Table Viewer, Data Profiler, and Interactive TUI",
    long_about = "csvcat formats, filters, profiles, and browses CSV/TSV data directly in your terminal with rich ANSI coloring, auto-delimiter detection, gzip support, and interactive TUI navigation."
)]
pub struct Cli {
    /// Input file path (reads from STDIN if omitted or if '-')
    #[arg(value_name = "FILE")]
    pub file: Option<PathBuf>,

    /// Specific delimiter char (e.g. ',', '\t', ';', '|'). Auto-detected if omitted.
    #[arg(short = 'd', long = "delimiter")]
    pub delimiter: Option<char>,

    /// Border style for table rendering
    #[arg(short = 'b', long = "border", value_enum, default_value_t = BorderStyle::Rounded)]
    pub border: BorderStyle,

    /// Show data profiling summary & ASCII sparklines instead of full table
    #[arg(short = 's', long = "stats")]
    pub stats: bool,

    /// Launch interactive fullscreen TUI viewer
    #[arg(short = 'i', long = "interactive")]
    pub interactive: bool,

    /// Select specific columns by name or index (comma-separated, e.g. "id,name,salary" or "0,1,3")
    #[arg(short = 'c', long = "cols", value_delimiter = ',')]
    pub cols: Option<Vec<String>>,

    /// Filter rows by condition (e.g. "salary > 50000", "age <= 30", "status == active")
    #[arg(short = 'w', long = "where")]
    pub where_clause: Option<Vec<String>>,

    /// Filter rows by regex matching across any cell
    #[arg(short = 'f', long = "filter")]
    pub filter: Option<String>,

    /// Sort rows by column specification (e.g. "salary:desc,age:asc")
    #[arg(short = 'S', long = "sort")]
    pub sort: Option<String>,

    /// Limit output to first N data rows
    #[arg(short = 'n', long = "head")]
    pub head: Option<usize>,

    /// Limit output to last N data rows
    #[arg(long = "tail")]
    pub tail: Option<usize>,

    /// Skip initial N data rows
    #[arg(long = "offset", default_value_t = 0)]
    pub offset: usize,

    /// Export format (markdown, json, jsonl, csv)
    #[arg(short = 'e', long = "export", value_enum)]
    pub export: Option<ExportFormat>,

    /// Wrap cell text instead of truncating
    #[arg(short = 'W', long = "wrap")]
    pub wrap: bool,

    /// Max width per column in characters (default: dynamic calculated from terminal)
    #[arg(long = "max-col-width")]
    pub max_col_width: Option<usize>,

    /// Show row index numbers column
    #[arg(short = 'N', long = "row-numbers")]
    pub row_numbers: bool,

    /// Disable color formatting
    #[arg(long = "no-color")]
    pub no_color: bool,

    /// Disable terminal auto-paging
    #[arg(long = "no-page")]
    pub no_page: bool,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderStyle {
    Rounded,
    Box,
    Ascii,
    Grid,
    None,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Markdown,
    Json,
    Jsonl,
    Csv,
}
