## mini_http

Minimalist command-line HTTP client built with Rust, `reqwest`, and `clap`. It focuses on a straightforward developer experience for exploring APIs without leaving your terminal. Requests are executed with async `tokio`, responses are prettified when JSON, and the CLI keeps flags intentionally simple.

### Features
- GET, POST, PATCH, and DELETE verbs behind a single binary
- JSON body and header helpers using lightweight `key: value` syntax
- Colorized output: prettified JSON in green, fallback plain text in yellow
- Extensible `HttpClient` wrapper around `reqwest::Client`
- Pragmatic error handling powered by `anyhow`

### Getting Started

1. **Install Rust** (if needed)  
   Requires Rust 1.80+ with `cargo`. Visit <https://rustup.rs>.

2. **Clone & build**
   ```bash
   git clone https://github.com/<your-org>/mini_http.git
   cd mini_http
   cargo build
   ```

3. **Run the CLI**
   ```bash
   cargo run -- <command> [flags]
   ```
   The double dash `--` tells `cargo` to stop parsing flags; everything after belongs to `mini_http`.

### Usage

The binary exposes subcommands for each HTTP verb. All commands require a `url` and accept an optional `--headers` flag. POST/PATCH also accept `--body`.

```bash
cargo run -- get https://httpbin.org/get \
  --headers "Cache-Control: no-cache\nX-Demo: 123"

cargo run -- post https://httpbin.org/post \
  --headers "Content-Type: application/json" \
  --body "name: mini_http\nlang: rust"

cargo run -- patch https://httpbin.org/patch --body "status: updated"

cargo run -- delete https://httpbin.org/delete
```

#### Formatting headers and bodies
- Provide multiline strings with each line in the form `key: value`.
- Headers and body entries are parsed into maps, then serialized to JSON automatically.
- For more complex payloads, feel free to extend `utils::parse_string_to_hash_map` or pass raw JSON by editing `http::HttpClient`.

### Project Structure
- `src/main.rs` wires the CLI to `tokio`
- `src/cli.rs` defines commands via `clap`
- `src/cmds/` holds verb-specific orchestration
- `src/http.rs` wraps `reqwest::Client` with shared behavior (headers, auth hook, JSON encoding)
- `src/printer.rs` handles colored output
- `src/utils.rs` parses CLI-friendly input into JSON-friendly maps

### Development

```bash
# Run clippy + tests once you add them
cargo fmt
cargo clippy --all-targets --all-features
cargo test
```

Planned next steps (see `tasks.md`):
- Logging with `log`
- Richer error handling via `anyhow`
- Automated tests, docs, and CI/CD

### Contributing

1. Fork, branch, and code (`git checkout -b feature/my-improvement`)
2. Keep additions documented and tested
3. Open a PR describing the change and expected behavior

### License

MIT © 2024 mini_http contributors
