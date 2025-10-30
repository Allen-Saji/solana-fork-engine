use serde::Deserialize;

/// Request to create a new fork
#[derive(Deserialize)]
pub struct CreateForkRequest {
    pub user_id: Option<String>,
}

/// Request to create a fork from mainnet with specific accounts
#[derive(Deserialize)]
pub struct CreateMainnetForkRequest {
    pub user_id: Option<String>,
    pub accounts: Vec<String>, // Pubkeys to load from mainnet
    pub rpc_endpoint: Option<String>, // Optional custom RPC endpoint
}

/// Request to load a single account from mainnet into fork
#[derive(Deserialize)]
pub struct LoadAccountRequest {
    pub fork_id: Option<String>,
    pub user_id: Option<String>,
    pub address: String,
    pub rpc_endpoint: Option<String>,
}

/// Request to load multiple accounts from mainnet
#[derive(Deserialize)]
pub struct LoadAccountsRequest {
    pub fork_id: Option<String>,
    pub user_id: Option<String>,
    pub addresses: Vec<String>,
    pub rpc_endpoint: Option<String>,
}

/// Request to load all token accounts for an owner
#[derive(Deserialize)]
pub struct LoadTokenAccountsRequest {
    pub fork_id: Option<String>,
    pub user_id: Option<String>,
    pub owner: String,
    pub rpc_endpoint: Option<String>,
}

/// Request to set account balance
#[derive(Deserialize)]
pub struct SetBalanceRequest {
    pub fork_id: Option<String>,
    pub user_id: Option<String>,
    pub address: String,
    pub lamports: u64,
}

/// Request to get account balance
#[derive(Deserialize)]
pub struct GetBalanceRequest {
    pub fork_id: Option<String>,
    pub user_id: Option<String>,
    pub address: String,
}

/// Request to airdrop SOL
#[derive(Deserialize)]
pub struct AirdropRequest {
    pub fork_id: Option<String>,
    pub user_id: Option<String>,
    pub address: String,
    pub sol: f64,
}

/// Request to send a raw transaction
#[derive(Deserialize)]
pub struct SendTransactionRequest {
    pub fork_id: Option<String>,
    pub user_id: Option<String>,
    pub transaction: String, // Base64 encoded transaction
}

/// Request to transfer SOL between accounts
#[derive(Deserialize)]
pub struct TransferRequest {
    pub fork_id: Option<String>,
    pub user_id: Option<String>,
    pub from: String,
    pub to: String,
    pub amount_sol: f64,
    pub private_key: String,
}

