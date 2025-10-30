use litesvm::LiteSVM;
use solana_pubkey::Pubkey;
use solana_transaction::Transaction;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::responses::{AccountInfo, ForkInfo, TransactionResult};

/// Represents a Solana blockchain fork with metadata
pub struct Fork {
    pub id: String,
    pub svm: LiteSVM,
    pub created_at: u64,
    pub slot: u64,
    pub transaction_count: u64,
}

impl Fork {
    /// Create a new fork with the given ID
    pub fn new(id: String) -> Self {
        let svm = LiteSVM::new();
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            id,
            svm,
            created_at,
            slot: 0,
            transaction_count: 0,
        }
    }

    /// Get fork information summary
    pub fn get_info(&self) -> ForkInfo {
        ForkInfo {
            fork_id: self.id.clone(),
            status: "active".to_string(),
            slot: self.slot,
            created_at: self.created_at,
            uptime_seconds: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                - self.created_at,
            transaction_count: self.transaction_count,
        }
    }

    /// Set account balance using liteSVM's airdrop
    pub fn set_balance(&mut self, address: &Pubkey, lamports: u64) -> Result<(), String> {
        self.svm
            .airdrop(address, lamports)
            .map(|_| ()) // Discard the TransactionMetadata, return ()
            .map_err(|e| format!("Failed to set balance: {:?}", e))
    }

    /// Get account balance
    pub fn get_balance(&self, address: &Pubkey) -> u64 {
        self.svm
            .get_account(address)
            .map(|acc| acc.lamports)
            .unwrap_or(0)
    }

    /// Get detailed account information
    pub fn get_account_info(&self, address: &Pubkey) -> Option<AccountInfo> {
        self.svm.get_account(address).map(|acc| AccountInfo {
            address: address.to_string(),
            lamports: acc.lamports,
            owner: acc.owner.to_string(),
            executable: acc.executable,
            rent_epoch: acc.rent_epoch,
            data_length: acc.data.len(),
        })
    }

    /// Send a transaction to the fork
    pub fn send_transaction(&mut self, transaction: Transaction) -> Result<TransactionResult, String> {
        // Get signature before sending
        let signature = transaction.signatures[0].to_string();

        // Process the transaction
        let result = self.svm.send_transaction(transaction);

        // Increment transaction count
        self.transaction_count += 1;

        match result {
            Ok(_) => Ok(TransactionResult {
                success: true,
                signature,
                error: None,
            }),
            Err(e) => Ok(TransactionResult {
                success: false,
                signature,
                error: Some(format!("{:?}", e)),
            }),
        }
    }
}