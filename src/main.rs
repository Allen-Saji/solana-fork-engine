use axum::{
    routing::{get, post},
    Router,
};
use solana_fork_engine::{
    constants::DEFAULT_SERVER_ADDR,
    create_shared_fork_manager,
    routes,
    state::AppState,
};

#[tokio::main]
async fn main() {
    println!("🚀 Starting Solana Fork Engine v0.3.0");
    println!("   Features: Multiple Forks, User Isolation, Mainnet Forking");
    println!();
    
    // Create application state (no initial fork)
    let fork_manager = create_shared_fork_manager();
    let state = AppState::new(fork_manager);
    
    // Build the router with all routes
    let app = Router::new()
        // Health and info routes
        .route("/", get(routes::root))
        .route("/health", get(routes::health_check))
        .route("/api/v1/fork/info", get(routes::get_fork_info))
        
        // Fork management
        .route("/api/v1/forks", get(routes::list_forks))
        .route("/api/v1/forks", post(routes::create_fork))
        
        // Mainnet forking routes (NEW)
        .route("/api/v1/forks/mainnet", post(routes::create_mainnet_fork))
        .route("/api/v1/fork/load-account", post(routes::load_account))
        .route("/api/v1/fork/load-accounts", post(routes::load_accounts))
        .route("/api/v1/fork/load-token-accounts", post(routes::load_token_accounts))
        
        // Balance operations (require user_id query param)
        .route("/api/v1/fork/balance/set", post(routes::set_balance))
        .route("/api/v1/fork/balance/get", post(routes::get_balance))
        .route("/api/v1/fork/account", post(routes::get_account))
        .route("/api/v1/fork/airdrop", post(routes::airdrop))
        
        // Transaction operations (require user_id query param)
        .route(
            "/api/v1/fork/transaction/send",
            post(routes::send_transaction),
        )
        .route("/api/v1/fork/transfer", post(routes::transfer_sol))
        
        // Token operations
        .route("/api/v1/token/create-mint", post(routes::create_token_mint))
        .route("/api/v1/token/create-account", post(routes::create_token_account))
        .route("/api/v1/token/mint", post(routes::mint_tokens))
        .route("/api/v1/token/transfer", post(routes::transfer_tokens))
        .route("/api/v1/token/balance", post(routes::get_token_balance))
        .with_state(state);
    
    // Print available endpoints
    println!("🌐 Server listening on http://{}", DEFAULT_SERVER_ADDR);
    println!();
    println!("📝 Available endpoints:");
    println!("  GET  http://localhost:8899/");
    println!("  GET  http://localhost:8899/health");
    println!("  GET  http://localhost:8899/api/v1/forks");
    println!("  POST http://localhost:8899/api/v1/forks");
    println!();
    println!("🔗 Mainnet Forking endpoints:");
    println!("  POST http://localhost:8899/api/v1/forks/mainnet");
    println!("  POST http://localhost:8899/api/v1/fork/load-account");
    println!("  POST http://localhost:8899/api/v1/fork/load-accounts");
    println!("  POST http://localhost:8899/api/v1/fork/load-token-accounts");
    println!();
    println!("  User-specific endpoints (require ?user_id=<id>):");
    println!("  GET  http://localhost:8899/api/v1/fork/info?user_id=<id>");
    println!("  POST http://localhost:8899/api/v1/fork/balance/set?user_id=<id>");
    println!("  POST http://localhost:8899/api/v1/fork/balance/get?user_id=<id>");
    println!("  POST http://localhost:8899/api/v1/fork/account?user_id=<id>");
    println!("  POST http://localhost:8899/api/v1/fork/airdrop?user_id=<id>");
    println!("  POST http://localhost:8899/api/v1/fork/transaction/send?user_id=<id>");
    println!("  POST http://localhost:8899/api/v1/fork/transfer?user_id=<id>");
    println!();
    println!("💡 Tip: Each user_id gets their own isolated fork!");
    println!("🌍 New: Fork from mainnet and test with real accounts!");
    
    // Start the server
    let listener = tokio::net::TcpListener::bind(DEFAULT_SERVER_ADDR)
        .await
        .expect("Failed to bind to address");
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}