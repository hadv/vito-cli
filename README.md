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

### TUI Mode (Text User Interface)

Vito CLI now includes a modern TUI interface for interactive Safe wallet management:

```bash
# Start TUI mode
vito tui

# Start TUI with a specific Safe address
vito --safe 0x1234567890123456789012345678901234567890 tui
```

#### TUI Features

- **Dashboard View**: Overview of Safe wallet status, Trezor connection, and pending transactions
- **Interactive Transaction List**: Browse and select transactions with keyboard navigation
- **Transaction Details**: Detailed view of selected transactions
- **Real-time Updates**: Automatic refresh of transaction data
- **Keyboard Shortcuts**:
  - `d` - Dashboard view
  - `t` - Transaction list
  - `r` - Refresh data
  - `h` or `F1` - Help
  - `↑/↓` - Navigate lists
  - `Enter` - View details
  - `Backspace/←` - Go back
  - `q` or `Esc` - Quit

### Shell Mode

For users who prefer a command-line interface:

```bash
# Start interactive shell mode
vito shell

# Start shell with a specific Safe address
vito --safe 0x1234567890123456789012345678901234567890 shell
```

## Project Structure

- `src/main.rs` - Main entry point for the CLI application
- `Cargo.toml` - Project configuration and dependencies

## Dependencies

### Core Dependencies
- [clap](https://github.com/clap-rs/clap) - Command Line Argument Parser for Rust
- [anyhow](https://github.com/dtolnay/anyhow) - Flexible error handling with context
- [tokio](https://github.com/tokio-rs/tokio) - Asynchronous runtime for Rust
- [ethers](https://github.com/gakonst/ethers-rs) - Ethereum library for Rust

### TUI Dependencies
- [ratatui](https://github.com/ratatui-org/ratatui) - Modern terminal user interface library
- [crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation

### Hardware Wallet Integration
- [trezor-client](https://github.com/trezor/trezor-firmware) - Trezor hardware wallet integration
- [rustyline](https://github.com/kkawakam/rustyline) - Readline implementation for interactive shell

## Development

To contribute to this project, you'll need Rust installed on your system. Follow these steps to set up the development environment:

1. Install Rust: https://www.rust-lang.org/tools/install
2. Clone the repository
3. Run `cargo build` to compile the project
4. Run `cargo test` to run the tests

## License

This project is licensed under the MIT License - see the LICENSE file for details. 
