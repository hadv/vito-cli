# Vito CLI

A powerful command-line interface tool built with Rust.

## Description

Vito CLI is a feature-rich command-line interface tool designed to help you manage your projects more efficiently. This tool demonstrates best practices for building CLI applications in Rust using the `clap` library.

## Installation

### From Source

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/vito-cli.git
   cd vito-cli
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Install the binary:
   ```
   cargo install --path .
   ```

The binary will be installed as `vito` and available in your PATH.

## Usage

### Basic Commands

```
vito --help    # Display detailed help information
vito -h        # Display brief help information
vito --version # Display version information
vito -V        # Display version information (short form)
```

## Project Structure

- `src/main.rs` - Main entry point for the CLI application
- `Cargo.toml` - Project configuration and dependencies

## Dependencies

- [clap](https://github.com/clap-rs/clap) - Command Line Argument Parser for Rust
- [anyhow](https://github.com/dtolnay/anyhow) - Flexible error handling with context

## Development

To contribute to this project, you'll need Rust installed on your system. Follow these steps to set up the development environment:

1. Install Rust: https://www.rust-lang.org/tools/install
2. Clone the repository
3. Run `cargo build` to compile the project
4. Run `cargo test` to run the tests

## License

This project is licensed under the MIT License - see the LICENSE file for details. 
