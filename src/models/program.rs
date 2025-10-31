use serde::{Deserialize, Serialize};

// ============================================
// PROGRAM DEPLOYMENT REQUESTS
// ============================================

/// Deploy a compiled program (.so file) to the fork
#[derive(Deserialize)]
pub struct DeployProgramRequest {
    pub user_id: String,
    pub program_keypair: String,  // base58 encoded keypair for the program
    pub program_data: String,      // base64 encoded .so file
    pub payer_keypair: String,     // base58 encoded keypair for paying rent
}

/// Invoke a program instruction
#[derive(Deserialize)]
pub struct InvokeProgramRequest {
    pub user_id: String,
    pub program_id: String,
    pub instruction_data: String,  // base64 encoded instruction data
    pub accounts: Vec<AccountMetaData>,
    pub signers: Vec<String>,      // base58 encoded keypairs that need to sign
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AccountMetaData {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

/// Load a program from mainnet
#[derive(Deserialize)]
pub struct LoadProgramRequest {
    pub user_id: String,
    pub program_id: String,
    pub rpc_endpoint: Option<String>,
}

/// Get program account info
#[derive(Deserialize)]
pub struct GetProgramRequest {
    pub user_id: String,
    pub program_id: String,
}

/// Upgrade a program
#[derive(Deserialize)]
pub struct UpgradeProgramRequest {
    pub user_id: String,
    pub program_id: String,
    pub new_program_data: String,  // base64 encoded new .so file
    pub upgrade_authority_keypair: String,  // base58 encoded
}

// ============================================
// PROGRAM DEPLOYMENT RESPONSES
// ============================================

/// Response after deploying a program
#[derive(Serialize)]
pub struct DeployProgramResponse {
    pub program_id: String,
    pub signature: String,
    pub success: bool,
    pub deployed_size: usize,
}

/// Response after invoking a program
#[derive(Serialize)]
pub struct InvokeProgramResponse {
    pub signature: String,
    pub success: bool,
    pub logs: Vec<String>,
    pub error: Option<String>,
}

/// Response after loading a program from mainnet
#[derive(Serialize)]
pub struct LoadProgramResponse {
    pub program_id: String,
    pub success: bool,
    pub program_size: usize,
    pub is_executable: bool,
}

/// Program account information
#[derive(Serialize)]
pub struct ProgramInfo {
    pub program_id: String,
    pub executable: bool,
    pub owner: String,
    pub data_size: usize,
    pub lamports: u64,
}