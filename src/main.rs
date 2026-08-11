use anyhow::Result;
use clap::Parser;

mod cli;
mod profiler;
mod query;
mod reader;
mod render;
mod tui;
mod types;

use cli::{Cli, ExportFormat};
use profiler::Profiler;
use query::QueryEngine;
use reader::CsvReader;
use render::{Exporter, TableRenderer};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let rs = CsvReader::read(cli.file.as_deref(), cli.delimiter)?;
    let processed_rs = QueryEngine::process(&rs, &cli)?;

    if cli.interactive {
        tui::run_tui(processed_rs)?;
    } else if cli.stats {
        Profiler::print_summary(&processed_rs, cli.no_color);
    } else if let Some(fmt) = cli.export {
        let output = match fmt {
            ExportFormat::Markdown => Exporter::to_markdown(&processed_rs),
            ExportFormat::Json => Exporter::to_json(&processed_rs)?,
            ExportFormat::Jsonl => Exporter::to_jsonl(&processed_rs)?,
            ExportFormat::Csv => Exporter::to_csv(&processed_rs)?,
        };
        println!("{}", output);
    } else {
        let rendered = TableRenderer::render(&processed_rs, &cli);
        println!("{}", rendered);
    }

    Ok(())
}
