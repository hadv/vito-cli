use rustyline::{error::ReadlineError, DefaultEditor};
use anyhow::Result;
use std::env;
use colored::Colorize;

// Helper function to shorten an Ethereum address
fn shorten_address(address: &str) -> String {
    if address.len() >= 10 {
        let prefix = &address[0..6];  // 0x1234
        let suffix = &address[address.len() - 4..];  // abcd
        format!("{}...{}", prefix, suffix)
    } else {
        address.to_string()
    }
}

pub async fn execute(safe: Option<String>) -> Result<()> {
    println!("Welcome to Vito CLI interactive mode. Type 'help' for available commands or 'exit' to quit.");
    
    // Use the provided safe address or start with none
    let current_safe = safe;
    if let Some(ref safe) = current_safe {
        println!("Current Safe address: {}", safe);
    } else {
        println!("{}", "No Safe address set. Please restart with --safe <address> option.".red());
    }
    
    // Get RPC URL from environment
    let rpc_url = env::var("RPC_URL").ok();
    if let Some(ref rpc) = rpc_url {
        println!("Using RPC URL: {}", rpc);
    } else {
        println!("No RPC URL set. Please set the RPC_URL environment variable.");
    }
    
    let mut rl = DefaultEditor::new()?;
    loop {
        // Create prompt based on whether a safe address is set
        let prompt = match current_safe {
            Some(ref safe) => {
                let addr_part = shorten_address(safe).dimmed().to_string();
                format!("{} vito> ", addr_part)
            },
            None => "vito> ".to_string(),
        };
        
        // Display prompt and get input
        let readline = rl.readline(&prompt);
        
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                
                // Process the command
                let command = line.trim();
                
                match command {
                    "exit" | "quit" => {
                        println!("Goodbye!");
                        break;
                    },
                    "help" => {
                        display_help();
                    },
                    "" => {
                        // Do nothing for empty command
                    },
                    _ => {
                        if command.starts_with("tx ") {
                            if let Some(ref safe) = current_safe {
                                let args: Vec<&str> = command[3..].split_whitespace().collect();
                                handle_tx_command(safe.clone(), args, rpc_url.clone()).await;
                            } else {
                                println!("{}", "Error: No Safe address provided. Please restart with --safe <address> option.".red());
                            }
                        } else if command.starts_with("connect") {
                            if let Some(ref safe) = current_safe {
                                let args: Vec<&str> = command.split_whitespace().collect();
                                handle_connect_command(safe.clone(), args).await;
                            } else {
                                println!("{}", "Error: No Safe address provided. Please restart with --safe <address> option.".red());
                            }
                        } else {
                            println!("{}: {}. Type 'help' for available commands.",
                                "Unknown command".red(),
                                command.yellow());
                        }
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    
    Ok(())
}

fn display_help() {
    println!("{}", "Available commands:".bright_green());
    println!("  {} - List pending transactions", "tx".yellow());
    println!("  {} - Show specific transaction", "tx --hash <hash>".yellow());
    println!("  {} - Sign transaction with Trezor", "tx --hash <hash> --sign".yellow());
    println!("  {} - Connect to Trezor hardware wallet", "connect [--path <derivation_path>]".yellow());
    println!("  {} - Show this help message", "help".yellow());
    println!("  {} - Exit interactive mode", "exit".yellow());
}

async fn handle_tx_command(safe: String, args: Vec<&str>, rpc_url: Option<String>) {
    let mut hash: Option<String> = None;
    let mut sign = false;
    let mut derivation_path = "m/44'/60'/0'/0/0".to_string();
    let mut tx_pool: Option<String> = None;

    // Simple argument parsing
    let mut i = 0;
    while i < args.len() {
        match args[i] {
            "--hash" | "-t" => {
                if i + 1 < args.len() {
                    hash = Some(args[i + 1].to_string());
                    i += 1;
                }
            },
            "--sign" => {
                sign = true;
            },
            "--path" => {
                if i + 1 < args.len() {
                    derivation_path = args[i + 1].to_string();
                    i += 1;
                }
            },
            "--tx-pool" => {
                if i + 1 < args.len() {
                    tx_pool = Some(args[i + 1].to_string());
                    i += 1;
                }
            },
            _ => {}
        }
        i += 1;
    }

    // Execute the tx command
    if let Err(e) = crate::commands::tx::execute(
        Some(safe),
        rpc_url,
        hash,
        tx_pool,
        sign,
        derivation_path
    ).await {
        println!("{}: {}", "Error".red(), e);
    }
}

async fn handle_connect_command(safe: String, args: Vec<&str>) {
    let mut derivation_path = "m/44'/60'/0'/0/0".to_string();

    // Simple argument parsing
    let mut i = 1; // Skip "connect"
    while i < args.len() {
        match args[i] {
            "--path" => {
                if i + 1 < args.len() {
                    derivation_path = args[i + 1].to_string();
                    i += 1;
                }
            },
            _ => {}
        }
        i += 1;
    }

    // Execute the connect command
    if let Err(e) = crate::commands::connect::execute(Some(safe), derivation_path).await {
        println!("{}: {}", "Error".red(), e);
    }
}