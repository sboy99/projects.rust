# 🦀 Todo CLI - A Rust-Powered Task Manager

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![CLI](https://img.shields.io/badge/CLI-Command%20Line%20Interface-blue?style=for-the-badge)](https://en.wikipedia.org/wiki/Command-line_interface)
[![License](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)](LICENSE)

A blazingly fast, memory-safe command-line todo application built with Rust. Manage your tasks efficiently with a simple yet powerful CLI interface.

## ✨ Features

- 🚀 **Lightning Fast** - Built with Rust for maximum performance
- 💾 **Persistent Storage** - Your tasks are saved locally in JSON format
- 🎯 **Simple Commands** - Intuitive CLI interface for task management
- 🔢 **Auto-incrementing IDs** - Each task gets a unique identifier
- ✅ **Task Completion** - Mark tasks as completed
- 🗑️ **Task Removal** - Remove tasks you no longer need
- 📋 **Task Listing** - View all your tasks at a glance

## 🛠️ Installation

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- Cargo (comes with Rust)

### Build from Source

```bash
# Clone the repository
git clone <your-repo-url>
cd todo_list

# Build the project
cargo build --release

# The binary will be available at target/release/todo_list
```

## 🚀 Usage

### Basic Commands

```bash
# Add a new task
./target/release/todo_list add "Learn Rust programming"

# List all tasks
./target/release/todo_list list

# Mark a task as completed
./target/release/todo_list complete 1

# Remove a task
./target/release/todo_list remove 1
```

### Command Reference

| Command | Description | Example |
|---------|-------------|---------|
| `add <title>` | Add a new task with the specified title | `todo add "Buy groceries"` |
| `list` | Display all tasks with their IDs and titles | `todo list` |
| `complete <id>` | Mark a task as completed | `todo complete 1` |
| `remove <id>` | Remove a task by its ID | `todo remove 1` |

### Example Session

```bash
$ ./target/release/todo_list add "Learn Rust"
Task 'Learn Rust' added successfully!

$ ./target/release/todo_list add "Build a CLI app"
Task 'Build a CLI app' added successfully!

$ ./target/release/todo_list list
Task ID: '1', Title: Learn Rust
Task ID: '2', Title: Build a CLI app

$ ./target/release/todo_list complete 1
Task completed successfully!

$ ./target/release/todo_list list
Task ID: '1', Title: Learn Rust
Task ID: '2', Title: Build a CLI app

$ ./target/release/todo_list remove 1
Task removed successfully!

$ ./target/release/todo_list list
Task ID: '2', Title: Build a CLI app
```

## 📁 Project Structure

```
todo_list/
├── src/
│   ├── main.rs          # Application entry point
│   ├── cli.rs           # CLI argument parsing
│   ├── models/
│   │   └── task.rs      # Task data structure
│   ├── cmds/            # Command implementations
│   │   ├── add.rs       # Add task command
│   │   ├── list.rs      # List tasks command
│   │   ├── complete.rs  # Complete task command
│   │   └── remove.rs    # Remove task command
│   └── storage.rs       # File I/O operations
├── data/                # Data storage directory
│   ├── tasks.json       # Tasks storage file
│   └── id_counter.txt   # ID counter file
├── Cargo.toml           # Project dependencies
└── README.md            # This file
```

## 🔧 Dependencies

- **[clap](https://crates.io/crates/clap)** - Command-line argument parsing
- **[serde](https://crates.io/crates/serde)** - Serialization/deserialization
- **[serde_json](https://crates.io/crates/serde_json)** - JSON support
- **[anyhow](https://crates.io/crates/anyhow)** - Error handling

## 💾 Data Storage

The application stores data locally in the `data/` directory:

- `tasks.json` - Contains all tasks in JSON format
- `id_counter.txt` - Maintains the next available task ID

The data directory is created automatically when you add your first task.

## 🧪 Development

### Running in Development Mode

```bash
# Run directly with cargo
cargo run add "Test task"
cargo run list
cargo run complete 1
cargo run remove 1
```

### Building for Development

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### Running Tests

```bash
cargo test
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with ❤️ using [Rust](https://www.rust-lang.org/)
- CLI framework powered by [clap](https://crates.io/crates/clap)
- Inspired by the need for a simple, fast task management tool

---

**Happy task managing! 🎯**
