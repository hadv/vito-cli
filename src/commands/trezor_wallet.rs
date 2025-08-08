use anyhow::{Result, Context};
use std::{env, sync::Arc};
use ethers::{
    providers::{Provider, Http, Middleware},
    signers::{Signer, Trezor, TrezorHDPath},
    types::{Address, Signature, transaction::eip2718::TypedTransaction},
};
use crate::config::DEFAULT_MAINNET_RPC;

/// A wrapper around the Trezor hardware wallet for Ethereum signing
pub struct TrezorWallet {
    inner: Trezor,
}

impl TrezorWallet {
    /// Creates a new TrezorWallet given a derivation path (e.g. "m/44'/60'/0'/0/0").
    /// Requires `RPC_URL` or will fall back to the default mainnet RPC.
    pub async fn new(derivation_path: &str) -> Result<Self> {
        // Parse derivation path
        let derivation = TrezorHDPath::Other(derivation_path.to_string());

        // Connect to provider and get chain ID
        let rpc_url = env::var("RPC_URL").unwrap_or_else(|_| DEFAULT_MAINNET_RPC.to_string());
        let provider = Provider::<Http>::try_from(rpc_url.clone())
            .context("Failed to connect to RPC provider")?;
        let provider = Arc::new(provider);
        let chain_id = provider.get_chainid().await
            .context("Failed to fetch chain ID from network")?
            .as_u64();

        // Workaround: firmware only supports mainnet, so initialize with dummy chain ID 1 to bypass the network check
        // Ensure the Ethereum app is open on your Trezor device to enter your PIN when prompted
        let mut signer = Trezor::new(derivation.clone(), 1, None)
            .await
            .context("Failed to initialize Trezor hardware wallet. Please ensure:\n  1. Trezor device is connected via USB\n  2. Ethereum app is open on the device\n  3. Device is unlocked with PIN\n  4. No other applications are using the device")?;
        // Override to the correct chain ID after unlocking
        signer = signer.with_chain_id(chain_id);

        Ok(TrezorWallet { inner: signer })
    }

    /// Returns the Ethereum address from the Trezor device
    pub fn address(&self) -> Address {
        self.inner.address()
    }

    /// Signs an Ethereum transaction with the Trezor
    pub async fn sign_transaction(&self, tx: &TypedTransaction) -> Result<Signature> {
        self.inner
            .sign_transaction(tx)
            .await
            .context("Failed to sign transaction with Trezor")
    }

    /// Signs an arbitrary message with the Trezor
    pub async fn sign_message<S: AsRef<[u8]>>(&self, msg: S) -> Result<Signature> {
        self.inner
            .sign_message(msg)
            .await
            .context("Failed to sign message with Trezor")
    }
} 