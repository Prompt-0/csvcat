use crate::types::RecordSet;

pub struct App {
    pub rs: RecordSet,
    pub selected_row: usize,
    pub selected_col: usize,
    pub search_query: String,
    pub search_active: bool,
    pub search_matches: Vec<(usize, usize)>,
    pub match_idx: usize,
    pub show_detail: bool,
    pub show_stats: bool,
    pub should_quit: bool,
}

impl App {
    pub fn new(rs: RecordSet) -> Self {
        App {
            rs,
            selected_row: 0,
            selected_col: 0,
            search_query: String::new(),
            search_active: false,
            search_matches: Vec::new(),
            match_idx: 0,
            show_detail: false,
            show_stats: false,
            should_quit: false,
        }
    }

    pub fn row_count(&self) -> usize {
        self.rs.rows.len()
    }

    pub fn col_count(&self) -> usize {
        self.rs.headers.len()
    }

    pub fn move_up(&mut self) {
        if self.selected_row > 0 {
            self.selected_row -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected_row + 1 < self.row_count() {
            self.selected_row += 1;
        }
    }

    pub fn move_left(&mut self) {
        if self.selected_col > 0 {
            self.selected_col -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.selected_col + 1 < self.col_count() {
            self.selected_col += 1;
        }
    }

    pub fn page_up(&mut self, page_size: usize) {
        self.selected_row = self.selected_row.saturating_sub(page_size);
    }

    pub fn page_down(&mut self, page_size: usize) {
        let max_r = self.row_count().saturating_sub(1);
        self.selected_row = (self.selected_row + page_size).min(max_r);
    }

    pub fn move_to_top(&mut self) {
        self.selected_row = 0;
    }

    pub fn move_to_bottom(&mut self) {
        if self.row_count() > 0 {
            self.selected_row = self.row_count() - 1;
        }
    }

    pub fn update_search(&mut self) {
        self.search_matches.clear();
        self.match_idx = 0;

        if self.search_query.is_empty() {
            return;
        }

        let query_lower = self.search_query.to_lowercase();
        for (r_idx, row) in self.rs.raw_rows.iter().enumerate() {
            for (c_idx, cell) in row.iter().enumerate() {
                if cell.to_lowercase().contains(&query_lower) {
                    self.search_matches.push((r_idx, c_idx));
                }
            }
        }

        if !self.search_matches.is_empty() {
            self.selected_row = self.search_matches[0].0;
            self.selected_col = self.search_matches[0].1;
        }
    }

    pub fn next_match(&mut self) {
        if self.search_matches.is_empty() {
            return;
        }
        self.match_idx = (self.match_idx + 1) % self.search_matches.len();
        let (r, c) = self.search_matches[self.match_idx];
        self.selected_row = r;
        self.selected_col = c;
    }

    pub fn prev_match(&mut self) {
        if self.search_matches.is_empty() {
            return;
        }
        if self.match_idx == 0 {
            self.match_idx = self.search_matches.len() - 1;
        } else {
            self.match_idx -= 1;
        }
        let (r, c) = self.search_matches[self.match_idx];
        self.selected_row = r;
        self.selected_col = c;
    }
}
