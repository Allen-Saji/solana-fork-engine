use axum::{
    routing::{get, post},
    Router,
    Json,
    http::StatusCode,
    extract::State,
};
use serde::{Deserialize, Serialize};
use litesvm::LiteSVM;
use solana_sdk::{
    pubkey::Pubkey,
    account::Account
};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use std::str::FromStr;

const SYSTEM_PROGRAM_ID: Pubkey = solana_sdk::pubkey!("11111111111111111111111111111111");

// ============================================
// DATA STRUCTURES
// ============================================

/// Our Fork with metadata
struct Fork {
    id: String,
    svm: LiteSVM,
    created_at: u64,
    slot: u64,
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
        }
    }
    
    /// Set account balance
    fn set_balance(&mut self, address: Pubkey, lamports: u64) -> Result<(), String> {
        // Get existing account or create a new one
        let mut account = self.svm.get_account(&address)
            .unwrap_or_else(|| Account {
                lamports: 0,
                data: vec![],
                owner: SYSTEM_PROGRAM_ID,
                executable: false,
                rent_epoch: 0,
            });
        
        // Update lamports
        account.lamports = lamports;
        
        // Set the account in the fork
        self.svm.set_account(address, account)
            .map_err(|e| format!("Failed to set account: {:?}", e))?;
        
        Ok(())
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
    sol: f64,  // Amount in SOL (easier than lamports)
}

#[derive(Serialize)]
struct AirdropResponse {
    success: bool,
    message: String,
    address: String,
    amount_sol: f64,
    amount_lamports: u64,
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
    
    // Create initial fork
    println!("📦 Creating initial fork...");
    let fork = Fork::new("fork-001".to_string());
    println!("✅ Fork created with ID: {}", fork.id);
    
    // Create application state
    let state = AppState {
        fork: Arc::new(Mutex::new(Some(fork))),
    };
    
    // Create router with new endpoints
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/api/v1/fork/info", get(get_fork_info))
        .route("/api/v1/forks", post(create_fork))
        // NEW: Balance operations
        .route("/api/v1/fork/balance/set", post(set_balance))
        .route("/api/v1/fork/balance/get", post(get_balance))
        .route("/api/v1/fork/account", post(get_account))
        .route("/api/v1/fork/airdrop", post(airdrop))
        .with_state(state);
    
    // Start server
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
            "get_account": "POST /api/v1/fork/account"
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

// NEW: Set balance handler
async fn set_balance(
    State(state): State<AppState>,
    Json(payload): Json<SetBalanceRequest>,
) -> Result<Json<SetBalanceResponse>, StatusCode> {
    // Parse the address string to Pubkey
    let address = Pubkey::from_str(&payload.address)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Lock the fork
    let mut fork_guard = state.fork.lock().unwrap();
    
    match fork_guard.as_mut() {
        Some(fork) => {
            // Set the balance
            fork.set_balance(address, payload.lamports)
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

// NEW: Get balance handler
async fn get_balance(
    State(state): State<AppState>,
    Json(payload): Json<GetBalanceRequest>,
) -> Result<Json<GetBalanceResponse>, StatusCode> {
    // Parse the address
    let address = Pubkey::from_str(&payload.address)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Lock the fork
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

// NEW: Get account info handler
async fn get_account(
    State(state): State<AppState>,
    Json(payload): Json<GetBalanceRequest>,
) -> Result<Json<AccountInfo>, StatusCode> {
    // Parse the address
    let address = Pubkey::from_str(&payload.address)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Lock the fork
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

// NEW: Airdrop handler (convenience method)
async fn airdrop(
    State(state): State<AppState>,
    Json(payload): Json<AirdropRequest>,
) -> Result<Json<AirdropResponse>, StatusCode> {
    // Parse address
    let address = Pubkey::from_str(&payload.address)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Convert SOL to lamports
    let lamports = (payload.sol * 1_000_000_000.0) as u64;
    
    // Lock the fork
    let mut fork_guard = state.fork.lock().unwrap();
    
    match fork_guard.as_mut() {
        Some(fork) => {
            // Set the balance
            fork.set_balance(address, lamports)
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