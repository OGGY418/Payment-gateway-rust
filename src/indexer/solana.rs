use solana_client::{
    rpc_client::RpcClient,
    rpc_config::RpcTransactionConfig,
};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::Signature,
};
use solana_transaction_status::{
    EncodedConfirmedTransactionWithStatusMeta,
    UiTransactionEncoding,
};
use std::str::FromStr;
use std::time::Duration;

/// Solana RPC Client wrapper
/// Concept: Connects to Solana blockchain and fetches transaction data
pub struct SolanaIndexer {
    client: RpcClient,
    wallet_address: Pubkey,
}

impl SolanaIndexer {
    /// Create new Solana indexer
    /// Concept: Initialize connection to devnet/mainnet
    pub fn new(rpc_url: &str, wallet_address: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = RpcClient::new_with_timeout_and_commitment(
            rpc_url.to_string(),
            Duration::from_secs(30),
            CommitmentConfig::confirmed(),
        );

        let wallet_pubkey = Pubkey::from_str(wallet_address)?;

        println!("✅ Solana indexer connected to: {}", rpc_url);
        println!("👀 Watching wallet: {}", wallet_address);

        Ok(SolanaIndexer {
            client,
            wallet_address: wallet_pubkey,
        })
    }

    /// Get recent transaction signatures for our wallet
    /// Concept: Fetch list of transactions involving our wallet
    pub fn get_recent_signatures(
        &self,
        limit: usize,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let signatures = self.client.get_signatures_for_address(&self.wallet_address)?;

        let sig_strings: Vec<String> = signatures
            .iter()
            .take(limit)
            .map(|sig| sig.signature.clone())
            .collect();

        Ok(sig_strings)
    }

    /// Get transaction details by signature
    /// Concept: Fetch full transaction data including memos
    pub fn get_transaction(
        &self,
        signature_str: &str,
    ) -> Result<Option<EncodedConfirmedTransactionWithStatusMeta>, Box<dyn std::error::Error>> {
        let signature = Signature::from_str(signature_str)?;

        let config = RpcTransactionConfig {
            encoding: Some(UiTransactionEncoding::Json),
            commitment: Some(CommitmentConfig::confirmed()),
            max_supported_transaction_version: Some(0),
        };

        let transaction = self.client.get_transaction_with_config(&signature, config)?;

        Ok(Some(transaction))
    }

    /// Get wallet balance
    /// Concept: Check how much SOL the wallet has
    pub fn get_balance(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let balance = self.client.get_balance(&self.wallet_address)?;
        Ok(balance)
    }
}
