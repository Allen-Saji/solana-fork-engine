use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use crate::{
    models::ForkInfo,
    state::AppState,
};

/// Root endpoint - API information
pub async fn root() -> Json<serde_json::Value> {
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

/// Health check endpoint
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

/// Get fork information
pub async fn get_fork_info(
    State(state): State<AppState>,
) -> Result<Json<ForkInfo>, StatusCode> {
    let fork_guard = state.fork.lock().unwrap();

    match fork_guard.as_ref() {
        Some(fork) => Ok(Json(fork.get_info())),
        None => Err(StatusCode::NOT_FOUND),
    }
}