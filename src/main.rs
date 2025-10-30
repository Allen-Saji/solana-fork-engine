use axum::{
    routing::{get, post},
    Router,
    Json,
    http::StatusCode,
    extract::State,
};
use serde::{Deserialize, Serialize};
use litesvm::LiteSVM;

// NEW: Correct imports from liteSVM example
use solana_pubkey::Pubkey;
use solana_keypair::Keypair;
use solana_signer::Signer;
use solana_transaction::Transaction;
use solana_message::Message;
use solana_system_interface::instruction::transfer;
use solana_account::Account;

use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use std::str::FromStr;

// System program ID constant
const SYSTEM_PROGRAM_ID: Pubkey = solana_pubkey::pubkey!("11111111111111111111111111111111");

// ============================================
// DATA STRUCTURES
// ============================================

/// Our Fork with metadata
struct Fork {
    id: String,
    svm: LiteSVM,
    created_at: u64,
    slot: u64,
    transaction_count: u64,
}

impl Fork {
    /// Create a new fork
    fn new(id: String) -> Self {
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
    
    /// Get fork information
    fn get_info(&self) -> ForkInfo {
        ForkInfo {
            fork_id: self.id.clone(),
            status: "active".to_string(),
            slot: self.slot,
            created_at: self.created_at,
            uptime_seconds: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() - self.created_at,
            transaction_count: self.transaction_count,
        }
    }
    
    /// Set account balance (using liteSVM's airdrop)
  fn set_balance(&mut self, address: &Pubkey, lamports: u64) -> Result<(), String> {
    // liteSVM has a built-in airdrop method
    self.svm.airdrop(address, lamports)
        .map(|_| ())  // Discard the TransactionMetadata, return ()
        .map_err(|e| format!("Failed to set balance: {:?}", e))
}
    
    /// Get account balance
    fn get_balance(&self, address: &Pubkey) -> u64 {
        self.svm.get_account(address)
            .map(|acc| acc.lamports)
            .unwrap_or(0)
    }
    
    /// Get account info
    fn get_account_info(&self, address: &Pubkey) -> Option<AccountInfo> {
        self.svm.get_account(address).map(|acc| AccountInfo {
            address: address.to_string(),
            lamports: acc.lamports,
            owner: acc.owner.to_string(),
            executable: acc.executable,
            rent_epoch: acc.rent_epoch,
            data_length: acc.data.len(),
        })
    }
    
    /// Send transaction
    fn send_transaction(&mut self, transaction: Transaction) -> Result<TransactionResult, String> {
        // Get signature before sending
        let signature = transaction.signatures[0].to_string();
        
        // Process the transaction
        let result = self.svm.send_transaction(transaction);
        
        // Increment transaction count
        self.transaction_count += 1;
        
        match result {
            Ok(_) => {
                Ok(TransactionResult {
                    success: true,
                    signature,
                    error: None,
                })
            }
            Err(e) => {
                Ok(TransactionResult {
                    success: false,
                    signature,
                    error: Some(format!("{:?}", e)),
                })
            }
        }
    }
}

// Request/Response structures

#[derive(Deserialize)]
struct CreateForkRequest {
    user_id: Option<String>,
}

#[derive(Serialize)]
struct CreateForkResponse {
    success: bool,
    message: String,
    fork_id: String,
    rpc_url: String,
}

#[derive(Serialize)]
struct ForkInfo {
    fork_id: String,
    status: String,
    slot: u64,
    created_at: u64,
    uptime_seconds: u64,
    transaction_count: u64,
}

#[derive(Deserialize)]
struct SetBalanceRequest {
    address: String,
    lamports: u64,
}

#[derive(Serialize)]
struct SetBalanceResponse {
    success: bool,
    message: String,
    address: String,
    new_balance: u64,
}

#[derive(Deserialize)]
struct GetBalanceRequest {
    address: String,
}

#[derive(Serialize)]
struct GetBalanceResponse {
    address: String,
    lamports: u64,
    sol: f64,
}

#[derive(Serialize)]
struct AccountInfo {
    address: String,
    lamports: u64,
    owner: String,
    executable: bool,
    rent_epoch: u64,
    data_length: usize,
}

#[derive(Deserialize)]
struct AirdropRequest {
    address: String,
    sol: f64,
}

#[derive(Serialize)]
struct AirdropResponse {
    success: bool,
    message: String,
    address: String,
    amount_sol: f64,
    amount_lamports: u64,
}

#[derive(Deserialize)]
struct SendTransactionRequest {
    transaction: String,
}

#[derive(Serialize)]
struct SendTransactionResponse {
    success: bool,
    signature: String,
    error: Option<String>,
}

#[derive(Deserialize)]
struct TransferRequest {
    from: String,
    to: String,
    amount_sol: f64,
    private_key: String,
}

#[derive(Serialize)]
struct TransferResponse {
    success: bool,
    signature: String,
    from: String,
    to: String,
    amount_sol: f64,
    amount_lamports: u64,
    error: Option<String>,
}

#[derive(Serialize)]
struct TransactionResult {
    success: bool,
    signature: String,
    error: Option<String>,
}

// ============================================
// APPLICATION STATE
// ============================================

#[derive(Clone)]
struct AppState {
    fork: Arc<Mutex<Option<Fork>>>,
}

// ============================================
// MAIN FUNCTION
// ============================================

#[tokio::main]
async fn main() {
    println!("🚀 Starting Solana Fork Engine...");
    
    println!("📦 Creating initial fork...");
    let fork = Fork::new("fork-001".to_string());
    println!("✅ Fork created with ID: {}", fork.id);
    
    let state = AppState {
        fork: Arc::new(Mutex::new(Some(fork))),
    };
    
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/api/v1/fork/info", get(get_fork_info))
        .route("/api/v1/forks", post(create_fork))
        .route("/api/v1/fork/balance/set", post(set_balance))
        .route("/api/v1/fork/balance/get", post(get_balance))
        .route("/api/v1/fork/account", post(get_account))
        .route("/api/v1/fork/airdrop", post(airdrop))
        .route("/api/v1/fork/transaction/send", post(send_transaction))
        .route("/api/v1/fork/transfer", post(transfer_sol))
        .with_state(state);
    
    let addr = "0.0.0.0:8899";
    println!("🌐 Server listening on http://{}", addr);
    println!("\n📝 Available endpoints:");
    println!("  GET  http://localhost:8899/");
    println!("  GET  http://localhost:8899/health");
    println!("  GET  http://localhost:8899/api/v1/fork/info");
    println!("  POST http://localhost:8899/api/v1/forks");
    println!("  POST http://localhost:8899/api/v1/fork/balance/set");
    println!("  POST http://localhost:8899/api/v1/fork/balance/get");
    println!("  POST http://localhost:8899/api/v1/fork/account");
    println!("  POST http://localhost:8899/api/v1/fork/airdrop");
    println!("  POST http://localhost:8899/api/v1/fork/transaction/send");
    println!("  POST http://localhost:8899/api/v1/fork/transfer");
    
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();
    
    axum::serve(listener, app)
        .await
        .unwrap();
}

// ============================================
// ROUTE HANDLERS
// ============================================

async fn root() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "Solana Fork Engine API",
        "version": "0.1.0",
        "endpoints": {
            "health": "GET /health",
            "fork_info": "GET /api/v1/fork/info",
            "create_fork": "POST /api/v1/forks",
            "set_balance": "POST /api/v1/fork/balance/set",
            "get_balance": "POST /api/v1/fork/balance/get",
            "get_account": "POST /api/v1/fork/account",
            "airdrop": "POST /api/v1/fork/airdrop",
            "send_transaction": "POST /api/v1/fork/transaction/send",
            "transfer": "POST /api/v1/fork/transfer"
        }
    }))
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn get_fork_info(
    State(state): State<AppState>
) -> Result<Json<ForkInfo>, StatusCode> {
    let fork_guard = state.fork.lock().unwrap();
    
    match fork_guard.as_ref() {
        Some(fork) => Ok(Json(fork.get_info())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_fork(
    State(state): State<AppState>,
    Json(payload): Json<CreateForkRequest>,
) -> Json<CreateForkResponse> {
    let user = payload.user_id.unwrap_or("anonymous".to_string());
    
    let fork_guard = state.fork.lock().unwrap();
    
    if let Some(fork) = fork_guard.as_ref() {
        Json(CreateForkResponse {
            success: true,
            message: format!("Using existing fork for user: {}", user),
            fork_id: fork.id.clone(),
            rpc_url: format!("http://localhost:8899/rpc/fork/{}", fork.id),
        })
    } else {
        Json(CreateForkResponse {
            success: false,
            message: "No fork available".to_string(),
            fork_id: "".to_string(),
            rpc_url: "".to_string(),
        })
    }
}

async fn set_balance(
    State(state): State<AppState>,
    Json(payload): Json<SetBalanceRequest>,
) -> Result<Json<SetBalanceResponse>, StatusCode> {
    let address = Pubkey::from_str(&payload.address)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let mut fork_guard = state.fork.lock().unwrap();
    
    match fork_guard.as_mut() {
        Some(fork) => {
            fork.set_balance(&address, payload.lamports)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            Ok(Json(SetBalanceResponse {
                success: true,
                message: "Balance updated successfully".to_string(),
                address: payload.address,
                new_balance: payload.lamports,
            }))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn get_balance(
    State(state): State<AppState>,
    Json(payload): Json<GetBalanceRequest>,
) -> Result<Json<GetBalanceResponse>, StatusCode> {
    let address = Pubkey::from_str(&payload.address)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let fork_guard = state.fork.lock().unwrap();
    
    match fork_guard.as_ref() {
        Some(fork) => {
            let lamports = fork.get_balance(&address);
            let sol = lamports as f64 / 1_000_000_000.0;
            
            Ok(Json(GetBalanceResponse {
                address: payload.address,
                lamports,
                sol,
            }))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn get_account(
    State(state): State<AppState>,
    Json(payload): Json<GetBalanceRequest>,
) -> Result<Json<AccountInfo>, StatusCode> {
    let address = Pubkey::from_str(&payload.address)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let fork_guard = state.fork.lock().unwrap();
    
    match fork_guard.as_ref() {
        Some(fork) => {
            fork.get_account_info(&address)
                .map(Json)
                .ok_or(StatusCode::NOT_FOUND)
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn airdrop(
    State(state): State<AppState>,
    Json(payload): Json<AirdropRequest>,
) -> Result<Json<AirdropResponse>, StatusCode> {
    let address = Pubkey::from_str(&payload.address)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let lamports = (payload.sol * 1_000_000_000.0) as u64;
    
    let mut fork_guard = state.fork.lock().unwrap();
    
    match fork_guard.as_mut() {
        Some(fork) => {
            fork.set_balance(&address, lamports)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            Ok(Json(AirdropResponse {
                success: true,
                message: format!("Airdropped {} SOL", payload.sol),
                address: payload.address,
                amount_sol: payload.sol,
                amount_lamports: lamports,
            }))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn send_transaction(
    State(state): State<AppState>,
    Json(payload): Json<SendTransactionRequest>,
) -> Result<Json<SendTransactionResponse>, StatusCode> {
    use base64::Engine;
    
    // Decode base64 transaction
    let tx_bytes = base64::engine::general_purpose::STANDARD
        .decode(&payload.transaction)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Deserialize transaction - Works with bincode 1.3
    let transaction: Transaction = bincode::deserialize(&tx_bytes)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let mut fork_guard = state.fork.lock().unwrap();
    
    match fork_guard.as_mut() {
        Some(fork) => {
            let result = fork.send_transaction(transaction)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            Ok(Json(SendTransactionResponse {
                success: result.success,
                signature: result.signature,
                error: result.error,
            }))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn transfer_sol(
    State(state): State<AppState>,
    Json(payload): Json<TransferRequest>,
) -> Result<Json<TransferResponse>, StatusCode> {
    // Parse addresses
    let from_pubkey = Pubkey::from_str(&payload.from)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let to_pubkey = Pubkey::from_str(&payload.to)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Convert SOL to lamports
    let lamports = (payload.amount_sol * 1_000_000_000.0) as u64;
    
    // Parse private key
    let keypair = parse_keypair(&payload.private_key)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Verify the from address matches the keypair
    if keypair.pubkey() != from_pubkey {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    let mut fork_guard = state.fork.lock().unwrap();
    
    match fork_guard.as_mut() {
        Some(fork) => {
            // Create transfer instruction (following liteSVM example)
            let instruction = transfer(&from_pubkey, &to_pubkey, lamports);
            
            // Get latest blockhash from fork
            let recent_blockhash = fork.svm.latest_blockhash();
            
            // Create message
            let message = Message::new(&[instruction], Some(&from_pubkey));
            
            // Create and sign transaction
            let transaction = Transaction::new(&[&keypair], message, recent_blockhash);
            
            // Send transaction
            let result = fork.send_transaction(transaction)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            Ok(Json(TransferResponse {
                success: result.success,
                signature: result.signature.clone(),
                from: payload.from,
                to: payload.to,
                amount_sol: payload.amount_sol,
                amount_lamports: lamports,
                error: result.error,
            }))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

// Helper function to parse keypair
// Helper function to parse keypair
// Helper function to parse keypair - FIXED
// Helper function to parse keypair - FIXED
// Helper function to parse keypair - FIXED
fn parse_keypair(key_str: &str) -> Result<Keypair, Box<dyn std::error::Error>> {
    // Try to parse as JSON array first (e.g., [1,2,3,...])
    if let Ok(bytes) = serde_json::from_str::<Vec<u8>>(key_str) {
        if bytes.len() == 64 {
            // Take only the first 32 bytes (secret key)
            let mut array = [0u8; 32];
            array.copy_from_slice(&bytes[0..32]);
            return Ok(Keypair::new_from_array(array));
        } else if bytes.len() == 32 {
            // If it's already 32 bytes, use directly
            let array: [u8; 32] = bytes.try_into()
                .map_err(|_| "Failed to convert to array")?;
            return Ok(Keypair::new_from_array(array));
        }
    }
    
    // Try to parse as base58
    Ok(Keypair::from_base58_string(key_str))
}