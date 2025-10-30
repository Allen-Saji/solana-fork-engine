use axum::{
    routing::{get, post},
    Router,
};

use solana_fork_engine::{
    constants::DEFAULT_SERVER_ADDR,
    routes,
    state::AppState,
};

#[tokio::main]
async fn main() {
    println!("🚀 Starting Solana Fork Engine...");

    // Create application state with initial fork
    println!("📦 Creating initial fork...");
    let state = AppState::new("fork-001".to_string());
    println!("✅ Fork created with ID: fork-001");

    // Build the router with all routes
    let app = Router::new()
        // Health and info routes
        .route("/", get(routes::root))
        .route("/health", get(routes::health_check))
        .route("/api/v1/fork/info", get(routes::get_fork_info))
        // Fork management
        .route("/api/v1/forks", post(routes::create_fork))
        // Balance operations
        .route("/api/v1/fork/balance/set", post(routes::set_balance))
        .route("/api/v1/fork/balance/get", post(routes::get_balance))
        .route("/api/v1/fork/account", post(routes::get_account))
        .route("/api/v1/fork/airdrop", post(routes::airdrop))
        // Transaction operations
        .route(
            "/api/v1/fork/transaction/send",
            post(routes::send_transaction),
        )
        .route("/api/v1/fork/transfer", post(routes::transfer_sol))
        .with_state(state);

    // Print available endpoints
    println!("🌐 Server listening on http://{}", DEFAULT_SERVER_ADDR);
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

    // Start the server
    let listener = tokio::net::TcpListener::bind(DEFAULT_SERVER_ADDR)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}