# csvcat

[![Crate](https://img.shields.io/badge/rust-2021%20edition-orange?style=for-the-badge&logo=rust)](https://github.com/Prompt-0/csvcat)
[![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)](LICENSE)
[![TUI](https://img.shields.io/badge/tui-ratatui%200.26-purple?style=for-the-badge)](https://github.com/Prompt-0/csvcat)

A modern, high-performance terminal CSV/TSV table viewer, data profiler, and interactive TUI built with Rust. Designed for data engineers, systems programmers, and terminal enthusiasts who need instant, ergonomic insights into tabular data without spinning up heavy notebooks or GUI tools.

---

## ✨ Features

- **Automatic Delimiter Detection**: Sniffs comma, tab, semicolon, and pipe delimiters automatically.
- **Interactive TUI Mode (`-i` / `--interactive`)**: Fullscreen Ratatui terminal browser with smooth scrolling, column sorting, and live regex search.
- **Data Profiling & ASCII Sparklines (`-s` / `--stats`)**: Computes column types, null ratios, distinct counts, min, max, mean, standard deviation, and quartile distribution sparklines.
- **SQL-like Where Filtering (`-w` / `--where`)**: Filter rows with intuitive comparison expressions (e.g. `--where "salary > 50000"` or `--where "status == active"`).
- **Column Projection (`-c` / `--cols`)**: Select and reorder columns by name or index.
- **Transparent Gzip Streaming**: Seamlessly reads `.csv.gz` and `.tsv.gz` archives on the fly.
- **Multiple Border Styles**: Render tables with `rounded`, `modern`, `markdown`, `ascii`, or `compact` borders.
- **Pipe-Friendly**: Full STDIN support (`cat data.csv | csvcat`).

---

## 🚀 Installation & Build

```bash
# Clone the repository
git clone https://github.com/Prompt-0/csvcat.git
cd csvcat

# Build release binary
cargo build --release

# Install locally
cargo install --path .
```

---

## 📖 Usage Examples

### 1. View Table with Auto-Formatting
```bash
csvcat data.csv
```

### 2. Launch Interactive Fullscreen TUI
```bash
csvcat -i large_dataset.csv
```

### 3. Profile Data & Distribution Stats
```bash
csvcat --stats metrics.csv
```

### 4. Filter Rows & Project Columns
```bash
csvcat customers.csv --cols id,name,revenue --where "revenue > 10000" --sort "revenue:desc" --head 20
```

### 5. Stream Gzipped Data from STDIN
```bash
curl -sL https://example.com/data.csv.gz | csvcat -
```

---

## ⌨️ Interactive TUI Keybindings

| Key | Action |
| :--- | :--- |
| `↑` / `k` | Scroll up one row |
| `↓` / `j` | Scroll down one row |
| `←` / `h` | Scroll left |
| `→` / `l` | Scroll right |
| `Page Up` / `Page Down` | Scroll half-page |
| `g` / `G` | Jump to top / bottom |
| `/` | Open live search filter |
| `s` | Cycle column sort |
| `q` / `Esc` | Exit viewer |

---

## 🧪 Testing & Verification

```bash
# Run test suite
cargo test

# Run linter
cargo clippy -- -D warnings
```

---

## 📄 License
Licensed under the MIT License.
