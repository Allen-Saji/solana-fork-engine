use serde::{Deserialize, Serialize};

/// Response after creating a fork
#[derive(Serialize)]
pub struct CreateForkResponse {
    pub success: bool,
    pub message: String,
    pub fork_id: String,
    pub rpc_url: String,
}

/// Fork information summary
#[derive(Serialize)]
pub struct ForkInfo {
    pub fork_id: String,
    pub status: String,
    pub slot: u64,
    pub created_at: u64,
    pub uptime_seconds: u64,
    pub transaction_count: u64,
}

/// Response after setting balance
#[derive(Serialize)]
pub struct SetBalanceResponse {
    pub success: bool,
    pub message: String,
    pub address: String,
    pub new_balance: u64,
}

/// Response with account balance
#[derive(Serialize)]
pub struct GetBalanceResponse {
    pub address: String,
    pub lamports: u64,
    pub sol: f64,
}

/// Detailed account information
#[derive(Serialize)]
pub struct AccountInfo {
    pub address: String,
    pub lamports: u64,
    pub owner: String,
    pub executable: bool,
    pub rent_epoch: u64,
    pub data_length: usize,
}

/// Response after airdrop
#[derive(Serialize)]
pub struct AirdropResponse {
    pub success: bool,
    pub message: String,
    pub address: String,
    pub amount_sol: f64,
    pub amount_lamports: u64,
}

/// Response after sending a transaction
#[derive(Serialize)]
pub struct SendTransactionResponse {
    pub success: bool,
    pub signature: String,
    pub error: Option<String>,
}

/// Response after SOL transfer
#[derive(Serialize)]
pub struct TransferResponse {
    pub success: bool,
    pub signature: String,
    pub from: String,
    pub to: String,
    pub amount_sol: f64,
    pub amount_lamports: u64,
    pub error: Option<String>,
}

/// Transaction execution result
#[derive(Serialize)]
pub struct TransactionResult {
    pub success: bool,
    pub signature: String,
    pub error: Option<String>,
}

// ============================================
// MAINNET FORKING RESPONSES
// ============================================

/// Response for mainnet fork creation with loaded accounts
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMainnetForkResponse {
    pub fork_id: String,
    pub user_id: String,
    pub created_at: String,
    pub expires_at: String,
    pub mainnet_slot: u64,           // ← NEW
    pub mainnet_blockhash: String,   // ← NEW
    pub accounts_loaded: usize,
    pub loaded_addresses: Vec<String>,
}

/// Response for loading accounts
#[derive(Serialize)]
pub struct LoadAccountsResponse {
    pub success: bool,
    pub accounts_loaded: usize,
    pub loaded_addresses: Vec<String>,
}