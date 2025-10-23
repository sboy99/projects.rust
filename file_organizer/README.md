# 📁 File Organizer

A powerful and efficient Rust command-line tool that automatically organizes files by extension or modification date. Perfect for cleaning up messy directories and maintaining a well-structured file system.

## ✨ Features

- **🔄 Dual Sorting Modes**: Organize files by extension or modification date
- **🔍 Dry Run Mode**: Preview changes before actually moving files
- **📂 Recursive Processing**: Automatically processes all subdirectories
- **⚡ High Performance**: Built with Rust for blazing-fast file operations
- **🛡️ Safe Operations**: Uses atomic file operations to prevent data loss
- **📊 Smart Date Grouping**: Groups files by year and month for easy navigation

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ (2024 edition)
- Cargo package manager

### Installation

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd file_organizer
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

3. **Run the executable:**
   ```bash
   ./target/release/file_organizer --help
   ```

## 📖 Usage

### Basic Commands

```bash
# Organize files by extension (default)
file_organizer

# Organize files by modification date
file_organizer --by date

# Organize files in a specific directory
file_organizer --path /path/to/directory

# Preview changes without moving files (dry run)
file_organizer --dry-run
```

### Command Line Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--path` | `-p` | Directory to organize | `.` (current directory) |
| `--by` | `-b` | Sorting mode: `extension` or `date` | `extension` |
| `--dry-run` | | Preview changes without moving files | `false` |

### Examples

#### Organize by Extension
```bash
# Organize current directory by file extension
file_organizer

# Result: Files will be moved to sorted/ directory:
# sorted/
# ├── pdf/
# │   ├── document1.pdf
# │   └── document2.pdf
# ├── jpg/
# │   ├── photo1.jpg
# │   └── photo2.jpg
# └── txt/
#     ├── notes.txt
#     └── readme.txt
```

#### Organize by Date
```bash
# Organize by modification date
file_organizer --by date

# Result: Files will be organized by year and month:
# sorted/
# ├── 2024/January/
# │   ├── file1.txt
# │   └── file2.jpg
# ├── 2024/February/
# │   └── file3.pdf
# └── 2023/December/
#     └── file4.docx
```

#### Dry Run Preview
```bash
# See what would happen without actually moving files
file_organizer --dry-run --by date

# Output:
# Would move "document.pdf" -> "sorted/2024/January/"
# Would move "photo.jpg" -> "sorted/2024/February/"
# Would move "notes.txt" -> "sorted/2024/January/"
```

## 🏗️ Project Structure

```
file_organizer/
├── src/
│   ├── main.rs          # Application entry point
│   ├── cli.rs           # Command-line interface and argument parsing
│   ├── organizer.rs     # Core file organization logic
│   ├── sorter.rs        # File sorting strategies (extension/date)
│   └── utils.rs         # Utility functions
├── Cargo.toml           # Project dependencies and metadata
└── README.md            # This documentation
```

## 🔧 Development

### Dependencies

- **clap**: Modern command-line argument parsing
- **anyhow**: Error handling and propagation
- **chrono**: Date and time manipulation
- **walkdir**: Recursive directory traversal

### Building from Source

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with specific arguments
cargo run -- --path /tmp/test --dry-run
```

### Code Organization

- **`main.rs`**: Entry point that parses CLI arguments and executes the organizer
- **`cli.rs`**: Defines command-line interface using Clap derive macros
- **`organizer.rs`**: Core logic for walking directories and moving files
- **`sorter.rs`**: Implements different sorting strategies (extension vs date)
- **`utils.rs`**: Currently empty, reserved for future utility functions

## 🛡️ Safety Features

- **Atomic Operations**: Uses `fs::rename()` for safe file moving
- **Directory Creation**: Automatically creates destination directories
- **Error Handling**: Comprehensive error handling with `anyhow`
- **Dry Run Mode**: Always test before making changes

## 🎯 Use Cases

- **Desktop Cleanup**: Organize your Downloads folder
- **Project Management**: Sort project files by type or date
- **Media Organization**: Group photos and videos by date
- **Document Management**: Organize PDFs, Word docs, and spreadsheets
- **Development**: Clean up temporary files and organize code assets

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is open source and available under the [MIT License](LICENSE).

## 🐛 Troubleshooting

### Common Issues

**Permission Denied:**
```bash
# Make sure you have write permissions to the target directory
chmod +w /path/to/directory
```

**Files Not Moving:**
- Check if files are in use by other applications
- Ensure you have sufficient disk space
- Verify the destination directory is writable

**Unexpected Behavior:**
- Use `--dry-run` to preview changes first
- Check file permissions and ownership
- Ensure the source directory exists and is readable

## 🔮 Future Enhancements

- [ ] Custom sorting rules via configuration file
- [ ] File filtering (exclude certain file types)
- [ ] Backup creation before organizing
- [ ] Progress bars for large operations
- [ ] Undo functionality
- [ ] GUI interface
- [ ] Parallel processing for better performance

---

**Made with ❤️ in Rust** 🦀
