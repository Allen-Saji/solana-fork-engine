use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;

use crate::{models::ForkInfo, state::AppState};

#[derive(Deserialize)]
pub struct ForkInfoQuery {
    pub fork_id: Option<String>,
    pub user_id: Option<String>,
}

/// Root endpoint - API information
pub async fn root() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "Solana Fork Engine API",
        "version": "0.2.0",
        "features": ["multi-fork", "fork-expiration", "user-isolation"],
        "endpoints": {
            "health": "GET /health",
            "fork_info": "GET /api/v1/fork/info?fork_id=... or ?user_id=...",
            "create_fork": "POST /api/v1/forks",
            "list_forks": "GET /api/v1/forks/list",
            "cleanup_forks": "POST /api/v1/forks/cleanup",
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

/// Get fork information by fork_id or user_id
pub async fn get_fork_info(
    State(state): State<AppState>,
    Query(params): Query<ForkInfoQuery>,
) -> Result<Json<ForkInfo>, StatusCode> {
    let manager = state
        .fork_manager
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Determine which fork to get info for
    let fork_id = if let Some(fid) = params.fork_id {
        Some(fid)
    } else if let Some(uid) = params.user_id {
        manager.get_user_fork_id(&uid).cloned()
    } else {
        None
    };

    let fork_id = fork_id.ok_or(StatusCode::BAD_REQUEST)?;

    manager
        .get_fork(&fork_id)
        .map(|fork| Json(fork.get_info()))
        .ok_or(StatusCode::NOT_FOUND)
}