use colored::Colorize;
use comfy_table::presets::{ASCII_FULL, NOTHING, UTF8_FULL, UTF8_FULL_CONDENSED, UTF8_NO_BORDERS};
use comfy_table::{Cell, Color, ContentArrangement, Table};
use terminal_size::{terminal_size, Width};

use crate::cli::{BorderStyle, Cli};
use crate::types::{DataType, RecordSet};

pub struct TableRenderer;

impl TableRenderer {
    pub fn render(rs: &RecordSet, cli: &Cli) -> String {
        let mut table = Table::new();

        // 1. Configure Borders
        match cli.border {
            BorderStyle::Rounded => {
                table.load_preset(UTF8_FULL);
            }
            BorderStyle::Box => {
                table.load_preset(UTF8_FULL_CONDENSED);
            }
            BorderStyle::Ascii => {
                table.load_preset(ASCII_FULL);
            }
            BorderStyle::Grid => {
                table.load_preset(UTF8_NO_BORDERS);
            }
            BorderStyle::None => {
                table.load_preset(NOTHING);
            }
        }

        // 2. Terminal Width & Wrapping Setup
        if let Some((Width(term_w), _)) = terminal_size() {
            table.set_width(term_w);
            if cli.wrap {
                table.set_content_arrangement(ContentArrangement::Dynamic);
            } else {
                table.set_content_arrangement(ContentArrangement::DynamicFullWidth);
            }
        } else {
            table.set_content_arrangement(ContentArrangement::Dynamic);
        }

        // 3. Build Header Row
        let mut header_cells = Vec::new();
        if cli.row_numbers {
            header_cells.push(Cell::new("#").fg(Color::Yellow));
        }
        for h in &rs.headers {
            if cli.no_color {
                header_cells.push(Cell::new(h));
            } else {
                header_cells.push(Cell::new(h).fg(Color::Blue));
            }
        }
        table.set_header(header_cells);

        // 4. Add Data Rows
        for (row_idx, row) in rs.rows.iter().enumerate() {
            let mut cells = Vec::new();
            if cli.row_numbers {
                cells.push(Cell::new(row_idx + 1).fg(Color::DarkGrey));
            }

            for cell_val in row {
                let cell_str = cell_val.to_string_repr();
                let mut cell = Cell::new(&cell_str);

                if !cli.no_color {
                    match cell_val.data_type() {
                        DataType::Integer | DataType::Float => {
                            cell = cell.fg(Color::Cyan);
                        }
                        DataType::Boolean => {
                            cell = cell.fg(Color::Magenta);
                        }
                        DataType::String => {
                            cell = cell.fg(Color::Green);
                        }
                        DataType::Null => {
                            cell = cell.fg(Color::DarkGrey);
                        }
                    }
                }

                cells.push(cell);
            }
            table.add_row(cells);
        }

        let mut output = table.to_string();

        // Footer info
        let footer = format!(
            "\nShowing {} rows, {} columns (Delimiter: '{}')",
            rs.row_count().to_string().cyan(),
            rs.col_count().to_string().cyan(),
            rs.delimiter
        );
        output.push_str(&footer);

        output
    }
}
