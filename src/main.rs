use clap::{Parser, Subcommand};
use anyhow::Result;
use std::env;

mod commands;
mod config;

/// A CLI tool for Ethereum Safe operations
#[derive(Parser)]
#[command(
    name = "vito",
    author, 
    version, 
    about = "A powerful CLI tool for managing Safe wallet transactions", 
    long_about = "A feature-rich command-line interface tool designed to help you interact with Ethereum Safe wallets."
)]
struct Cli {
    /// Ethereum Safe wallet address (0x...)
    #[arg(short, long)]
    safe: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Connect to Safe wallet with Trezor hardware wallet
    Connect {
        /// HD derivation path for Trezor (e.g. m/44'/60'/0'/0/0)
        #[arg(long, default_value = "m/44'/60'/0'/0/0")]
        derivation_path: String,
    },
    /// Fetch transaction data from Safe transaction pool
    Tx {
        /// Transaction hash (0x...) - Optional
        #[arg(short = 't', long)]
        hash: Option<String>,

        /// Custom Safe transaction pool address (0x...) - Optional
        #[arg(long)]
        tx_pool: Option<String>,

        /// Sign the transaction with Trezor hardware wallet
        #[arg(long)]
        sign: bool,

        /// HD derivation path for Trezor when signing (e.g. m/44'/60'/0'/0/0)
        #[arg(long, default_value = "m/44'/60'/0'/0/0")]
        derivation_path: String,
    },
    
    /// Start interactive shell mode
    Shell,

    /// Start TUI (Text User Interface) mode
    Tui,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Get RPC URL from environment variables
    let rpc = env::var("RPC_URL").ok();
    
    match cli.command {
        Some(Commands::Connect { derivation_path }) => {
            commands::connect::execute(cli.safe.clone(), derivation_path).await?;
        },
        Some(Commands::Tx { hash, tx_pool, sign, derivation_path }) => {
            // Pass the safe address from the top-level CLI
            commands::tx::execute(cli.safe.clone(), rpc, hash, tx_pool, sign, derivation_path).await?;
        },
        Some(Commands::Shell) => {
            // Pass the safe address to the shell
            commands::shell::execute(cli.safe.clone()).await?;
        },
        Some(Commands::Tui) => {
            // Pass the safe address to the TUI
            commands::tui::execute(cli.safe.clone(), rpc).await?;
        },
        None => {
            // No command provided, start interactive mode by default
            commands::shell::execute(cli.safe).await?;
        }
    }
    
    Ok(())
}
