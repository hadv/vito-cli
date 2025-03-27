// Configuration constants for the application
use std::collections::HashMap;
use lazy_static::lazy_static;

// Network IDs
pub const MAINNET: u64 = 1;
pub const GOERLI: u64 = 5;
pub const SEPOLIA: u64 = 11155111;
pub const POLYGON: u64 = 137;
pub const ARBITRUM: u64 = 42161;
pub const OPTIMISM: u64 = 10;
pub const BASE: u64 = 8453;
pub const GNOSIS: u64 = 100;

// Default RPC endpoint for Ethereum mainnet
// Note: In production, you should use your own API key
pub const DEFAULT_MAINNET_RPC: &str = "https://eth-mainnet.g.alchemy.com/v2/demo";

lazy_static! {
    // Map of network IDs to Safe transaction pool addresses
    pub static ref SAFE_TX_POOL_ADDRESSES: HashMap<u64, &'static str> = {
        let mut m = HashMap::new();
        m.insert(MAINNET, "0x6b8e1f0D2c34A0AeaD9A25B6966f7C0CAD653E5c");
        m.insert(GOERLI, "0x3A4fA54b8AaB5E2E2DBD0a41f41f629e4e71e2E7");
        m.insert(SEPOLIA, "0xa2ad21dc93B362570D0159b9E3A2fE5D8ecA0424");
        m.insert(POLYGON, "0xA3B9Ff95a78e04845a82ee5F75595E7bDaB8723D");
        m.insert(ARBITRUM, "0x7c4A2Db70E5f39BA5Db11B8A942f02A8D3B3aA1B");
        m.insert(OPTIMISM, "0x6E4d941A6fAD76B3d26E0c5447B4f5A7EfcA8ab8");
        m.insert(BASE, "0x2d340e22C5A33c1Ea01DAC41E331b7FE4c033C3b");
        m.insert(GNOSIS, "0x8d0C7BC9c4c588534dC1BF96d3ee9A4bCcBf28C7");
        m
    };
}

// Default fallback address if network is not recognized
pub const DEFAULT_SAFE_TX_POOL_ADDRESS: &str = "0x6b8e1f0D2c34A0AeaD9A25B6966f7C0CAD653E5c";

// Get network name from chain ID for display purposes
pub fn get_network_name(chain_id: u64) -> &'static str {
    match chain_id {
        MAINNET => "Ethereum Mainnet",
        GOERLI => "Goerli Testnet",
        SEPOLIA => "Sepolia Testnet",
        POLYGON => "Polygon",
        ARBITRUM => "Arbitrum",
        OPTIMISM => "Optimism",
        BASE => "Base",
        GNOSIS => "Gnosis Chain",
        _ => "Unknown Network"
    }
}

// Get Safe transaction pool address for a network
pub fn get_safe_tx_pool_address(network_id: u64) -> &'static str {
    SAFE_TX_POOL_ADDRESSES.get(&network_id).unwrap_or(&DEFAULT_SAFE_TX_POOL_ADDRESS)
} 