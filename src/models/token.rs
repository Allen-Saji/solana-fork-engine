use serde::{Deserialize, Serialize};

// Create Token Mint Request
#[derive(Deserialize)]
pub struct CreateTokenRequest {
    pub user_id: String,
    pub payer_keypair: String,  // base58 encoded keypair
    pub decimals: u8,
}

#[derive(Serialize)]
pub struct CreateTokenResponse {
    pub mint_address: String,
    pub signature: String,
}

// Create Token Account Request
#[derive(Deserialize)]
pub struct CreateTokenAccountRequest {
    pub user_id: String,
    pub payer_keypair: String,  // base58 encoded keypair
    pub mint_address: String,
    pub owner_address: String,
}

#[derive(Serialize)]
pub struct CreateTokenAccountResponse {
    pub token_account: String,
    pub signature: String,
}

// Mint Tokens Request
#[derive(Deserialize)]
pub struct MintTokensRequest {
    pub user_id: String,
    pub mint_authority_keypair: String,  // base58 encoded keypair
    pub mint_address: String,
    pub destination_account: String,
    pub amount: u64,
}

#[derive(Serialize)]
pub struct MintTokensResponse {
    pub signature: String,
    pub new_balance: u64,
}

// Transfer Tokens Request
#[derive(Deserialize)]
pub struct TransferTokensRequest {
    pub user_id: String,
    pub from_keypair: String,  // base58 encoded keypair (owner of source account)
    pub source_account: String,
    pub destination_account: String,
    pub amount: u64,
}

#[derive(Serialize)]
pub struct TransferTokensResponse {
    pub signature: String,
    pub source_balance: u64,
    pub destination_balance: u64,
}

// Get Token Balance Request
#[derive(Deserialize)]
pub struct GetTokenBalanceRequest {
    pub user_id: String,
    pub token_account: String,
}

#[derive(Serialize)]
pub struct GetTokenBalanceResponse {
    pub token_account: String,
    pub balance: u64,
    pub mint: String,
    pub owner: String,
}