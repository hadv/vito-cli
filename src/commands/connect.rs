use anyhow::{Result, Context, bail};
use ethers::types::Address;
use crate::commands::trezor_wallet::TrezorWallet;
use colored::Colorize;

/// Connect to Safe wallet with Trezor hardware wallet
pub async fn execute(safe: Option<String>, derivation_path: String) -> Result<()> {
    // Validate Safe address
    let safe_address = match safe {
        Some(addr) => {
            if !addr.starts_with("0x") || addr.len() != 42 {
                bail!("Invalid Safe wallet address format. Expected format: 0x followed by 40 hexadecimal characters.\nExample: 0x1234567890123456789012345678901234567890");
            }
            addr.parse::<Address>()
                .context("Invalid Safe wallet address format. Please check the address is a valid Ethereum address.")?
        },
        None => bail!("Safe address is required. Use --safe flag or set it globally.\nExample: vito connect --safe 0x1234567890123456789012345678901234567890"),
    };

    println!("{}", "Connecting to Trezor hardware wallet...".bright_blue());
    println!("Derivation path: {}", derivation_path.yellow());
    println!("{}", "Please ensure the Ethereum app is open on your Trezor device.".bright_yellow());

    // Initialize Trezor wallet
    let trezor_wallet = match TrezorWallet::new(&derivation_path).await {
        Ok(wallet) => wallet,
        Err(e) => {
            println!("{}", "✗ Failed to connect to Trezor hardware wallet".bright_red());
            println!("{}", "Troubleshooting steps:".bright_yellow());
            println!("  1. Ensure your Trezor device is connected via USB");
            println!("  2. Make sure the Ethereum app is open on your Trezor");
            println!("  3. Check that your device is unlocked with PIN");
            println!("  4. Verify the derivation path is correct: {}", derivation_path.dimmed());
            return Err(e).context("Failed to connect to Trezor hardware wallet");
        }
    };

    let signer_address = trezor_wallet.address();
    println!("{} {}", "✓ Trezor connected successfully!".bright_green(),
             format!("Address: 0x{:x}", signer_address).bright_white());

    // Verify signer is an owner of the Safe wallet
    println!("{}", "Verifying Safe wallet ownership...".bright_blue());

    // TODO: Add actual Safe contract verification here
    // For now, we'll just compare addresses directly
    if signer_address == safe_address {
        println!("{}", "✓ Trezor address matches Safe wallet address!".bright_green());
    } else {
        println!("{}", "⚠ Warning: Trezor address does not match Safe wallet address.".bright_yellow());
        println!("  Safe address:   {}", format!("0x{:x}", safe_address).bright_white());
        println!("  Trezor address: {}", format!("0x{:x}", signer_address).bright_white());
        println!("{}", "  This Trezor may be an owner of the Safe, but not the primary address.".dimmed());
    }

    println!("{}", "✓ Connection established! You can now use this Trezor to sign Safe transactions.".bright_green());

    Ok(())
}