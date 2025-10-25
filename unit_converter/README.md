# 🔄 Unit Converter

A fast, lightweight command-line unit converter written in Rust. Convert between different units of temperature, length, and weight with ease.

## ✨ Features

- **🌡️ Temperature Conversion**: Celsius, Fahrenheit, and Kelvin
- **📏 Length Conversion**: Millimeters, centimeters, and meters
- **⚖️ Weight Conversion**: Kilograms, grams, and milligrams
- **🖥️ Interactive Mode**: User-friendly interactive interface
- **⚡ Fast & Lightweight**: Built with Rust for optimal performance
- **🎯 Simple CLI**: Easy-to-use command-line interface

## 🚀 Installation

### Prerequisites

- Rust 1.70+ (2024 edition)
- Cargo (comes with Rust)

### Building from Source

1. Clone the repository:
```bash
git clone <repository-url>
cd unit_converter
```

2. Build the project:
```bash
cargo build --release
```

3. Run the executable:
```bash
./target/release/unit_converter --help
```

### Development

For development builds:
```bash
cargo build
cargo run -- --help
```

## 📖 Usage

### Command Line Interface

The unit converter supports three main categories: temperature, length, and weight.

#### Temperature Conversion

Convert between Celsius (c), Fahrenheit (f), and Kelvin (k):

```bash
# Convert 25°C to Fahrenheit
./target/release/unit_converter temperature --from c --to f --value 25

# Convert 100°F to Celsius
./target/release/unit_converter temperature --from f --to c --value 100

# Convert 0°C to Kelvin
./target/release/unit_converter temperature --from c --to k --value 0
```

#### Length Conversion

Convert between millimeters (mm), centimeters (cm), and meters (m):

```bash
# Convert 1000mm to meters
./target/release/unit_converter length --from mm --to m --value 1000

# Convert 1.5m to centimeters
./target/release/unit_converter length --from m --to cm --value 1.5

# Convert 50cm to millimeters
./target/release/unit_converter length --from cm --to mm --value 50
```

#### Weight Conversion

Convert between kilograms (kg), grams (g), and milligrams (mg):

```bash
# Convert 2.5kg to grams
./target/release/unit_converter weight --from kg --to g --value 2.5

# Convert 1500g to kilograms
./target/release/unit_converter weight --from g --to kg --value 1500

# Convert 500mg to grams
./target/release/unit_converter weight --from mg --to g --value 500
```

### Interactive Mode

For a more user-friendly experience, use the interactive mode:

```bash
./target/release/unit_converter interactive
```

This will guide you through:
1. Selecting a category (Temperature, Length, or Weight)
2. Entering the source unit
3. Entering the target unit
4. Entering the value to convert

## 🎯 Supported Units

### Temperature
- **c** - Celsius
- **f** - Fahrenheit  
- **k** - Kelvin

### Length
- **mm** - Millimeters
- **cm** - Centimeters
- **m** - Meters

### Weight
- **kg** - Kilograms
- **g** - Grams
- **mg** - Milligrams

## 🔧 Examples

### Quick Conversions

```bash
# Temperature examples
./target/release/unit_converter temperature --from c --to f --value 0    # 0°C = 32°F
./target/release/unit_converter temperature --from f --to c --value 32    # 32°F = 0°C
./target/release/unit_converter temperature --from c --to k --value 25    # 25°C = 298.15K

# Length examples
./target/release/unit_converter length --from m --to cm --value 1.75     # 1.75m = 175cm
./target/release/unit_converter length --from mm --to m --value 2500      # 2500mm = 2.5m

# Weight examples
./target/release/unit_converter weight --from kg --to g --value 0.5       # 0.5kg = 500g
./target/release/unit_converter weight --from g --to mg --value 2.5       # 2.5g = 2500mg
```

### Interactive Session Example

```bash
$ ./target/release/unit_converter interactive

? Select a category › Temperature
? From unit › c
? To unit › f
? Value › 25
25 c = 77 f
```

## 🏗️ Project Structure

```
unit_converter/
├── src/
│   ├── main.rs              # Entry point
│   ├── cli.rs               # CLI argument parsing
│   ├── interactive.rs       # Interactive mode
│   └── converters/
│       ├── mod.rs          # Module exports
│       ├── temperature.rs  # Temperature conversion logic
│       ├── length.rs       # Length conversion logic
│       └── weight.rs       # Weight conversion logic
├── Cargo.toml               # Project dependencies
└── README.md               # This file
```

## 🛠️ Development

### Dependencies

- **clap**: Command-line argument parsing
- **inquire**: Interactive prompts
- **anyhow**: Error handling

### Adding New Units

To add support for new units or categories:

1. Create a new converter module in `src/converters/`
2. Implement the conversion logic
3. Export the function in `src/converters/mod.rs`
4. Add a new command variant in `src/cli.rs`
5. Update the interactive mode in `src/interactive.rs`

### Running Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is open source and available under the [MIT License](LICENSE).

## 🐛 Issues

Found a bug or have a feature request? Please open an issue on GitHub!

## 🚀 Future Enhancements

- [ ] Add more unit categories (area, volume, time)
- [ ] Support for imperial units (feet, inches, pounds)
- [ ] Batch conversion mode
- [ ] Configuration file support
- [ ] Unit aliases (e.g., "celsius" for "c")
- [ ] History of conversions
- [ ] JSON output format

---

**Happy Converting! 🎉**
