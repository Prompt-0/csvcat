use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Row, Table, Wrap},
    Frame,
};

use crate::profiler::Profiler;
use crate::tui::app::App;
use crate::types::DataType;

pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header title & status
            Constraint::Min(5),    // Table Grid
            Constraint::Length(3), // Footer / Search bar
        ])
        .split(f.size());

    // 1. Header Render
    let header_text = vec![Line::from(vec![
        Span::styled(
            " csvcat ",
            Style::default()
                .bg(Color::Cyan)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!(
            " | Rows: {} | Cols: {} | Selected: ({}, {})",
            app.row_count(),
            app.col_count(),
            app.selected_row + 1,
            app.selected_col + 1
        )),
    ])];
    let header_block = Paragraph::new(header_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Interactive CSV/TSV Explorer "),
    );
    f.render_widget(header_block, chunks[0]);

    // 2. Table Grid Render
    let header_cells: Vec<String> = app
        .rs
        .headers
        .iter()
        .enumerate()
        .map(|(i, h)| format!("{}: {}", i + 1, h))
        .collect();

    let table_header = Row::new(header_cells)
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .bottom_margin(1);

    let rows: Vec<Row> = app
        .rs
        .rows
        .iter()
        .enumerate()
        .map(|(r_idx, r_values)| {
            let cells: Vec<Span> = r_values
                .iter()
                .enumerate()
                .map(|(c_idx, val)| {
                    let text = val.to_string_repr();
                    let is_selected = r_idx == app.selected_row && c_idx == app.selected_col;
                    let is_match = app.search_matches.contains(&(r_idx, c_idx));

                    let mut style = match val.data_type() {
                        DataType::Integer | DataType::Float => Style::default().fg(Color::Cyan),
                        DataType::Boolean => Style::default().fg(Color::Magenta),
                        DataType::String => Style::default().fg(Color::Green),
                        DataType::Null => Style::default().fg(Color::DarkGray),
                    };

                    if is_match {
                        style = style.bg(Color::Rgb(80, 80, 0)).add_modifier(Modifier::UNDERLINED);
                    }

                    if is_selected {
                        style = style
                            .bg(Color::Blue)
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD);
                    }

                    Span::styled(text, style)
                })
                .collect();

            Row::new(cells)
        })
        .collect();

    let widths: Vec<Constraint> = (0..app.col_count())
        .map(|_| Constraint::Percentage((100 / app.col_count().max(1)) as u16))
        .collect();

    let table = Table::new(rows, widths)
        .header(table_header)
        .block(Block::default().borders(Borders::ALL).title(" Table Data "))
        .highlight_symbol(">> ");

    f.render_widget(table, chunks[1]);

    // 3. Footer / Search Bar Render
    let footer_content = if app.search_active {
        Line::from(vec![
            Span::styled(
                " SEARCH: ",
                Style::default().bg(Color::Yellow).fg(Color::Black),
            ),
            Span::raw(&app.search_query),
            Span::styled("█", Style::default().fg(Color::Yellow)),
            Span::raw(format!(" (Matches: {})", app.search_matches.len())),
        ])
    } else {
        Line::from(vec![Span::styled(
            " [q] Quit  [hjkl/Arrows] Scroll  [/] Search  [n/N] Match  [Enter] Detail  [s] Stats ",
            Style::default().fg(Color::DarkGray),
        )])
    };

    let footer_block = Paragraph::new(footer_content)
        .block(Block::default().borders(Borders::ALL).title(" Status "));
    f.render_widget(footer_block, chunks[2]);

    // 4. Detail Modal View
    if app.show_detail {
        let area = centered_rect(70, 50, f.size());
        f.render_widget(Clear, area);

        let cell_text = app
            .rs
            .raw_rows
            .get(app.selected_row)
            .and_then(|r| r.get(app.selected_col))
            .map(|s| s.as_str())
            .unwrap_or("");

        let col_name = app
            .rs
            .headers
            .get(app.selected_col)
            .map(|s| s.as_str())
            .unwrap_or("");

        let modal_text = vec![
            Line::from(vec![
                Span::styled("Column: ", Style::default().fg(Color::Yellow)),
                Span::raw(col_name),
            ]),
            Line::from(vec![
                Span::styled("Row: ", Style::default().fg(Color::Yellow)),
                Span::raw((app.selected_row + 1).to_string()),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "Cell Content:",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(cell_text),
        ];

        let modal_block = Paragraph::new(modal_text)
            .wrap(Wrap { trim: false })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Cell Inspector (Esc/Enter to Close) ")
                    .style(Style::default().bg(Color::Rgb(20, 20, 30))),
            );

        f.render_widget(modal_block, area);
    }

    // 5. Stats Overlay Modal
    if app.show_stats {
        let area = centered_rect(85, 75, f.size());
        f.render_widget(Clear, area);

        let stats = Profiler::profile(&app.rs);
        let mut lines = vec![Line::from(Span::styled(
            "=== Dataset Statistics Overlay ===",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))];

        for s in &stats {
            let spark = format!(
                "Col {}: {:12} | Type: {:7} | Nulls: {:3} | Min: {:6} | Max: {:6} | {}",
                s.index + 1,
                s.name,
                s.data_type.to_string(),
                s.null_count,
                s.min.map_or("-".to_string(), |v| format!("{:.1}", v)),
                s.max.map_or("-".to_string(), |v| format!("{:.1}", v)),
                s.sparkline
            );
            lines.push(Line::from(spark));
        }

        let stats_block = Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Quick Stats Overlay (Press 's' to Close) ")
                    .style(Style::default().bg(Color::Rgb(15, 25, 35))),
            );

        f.render_widget(stats_block, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
