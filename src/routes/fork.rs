use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use crate::{
    models::{CreateForkRequest, CreateForkResponse, ForkInfo},
    state::AppState,
};

/// Create a new fork for a user
pub async fn create_fork(
    State(state): State<AppState>,
    Json(payload): Json<CreateForkRequest>,
) -> Result<Json<CreateForkResponse>, StatusCode> {
    let user_id = payload.user_id.unwrap_or_else(|| "anonymous".to_string());

    let mut manager = state
        .fork_manager
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Create fork for user
    let fork_id = manager.create_fork(user_id.clone());

    Ok(Json(CreateForkResponse {
        success: true,
        message: format!("Fork created for user: {}", user_id),
        fork_id: fork_id.clone(),
        rpc_url: format!("http://localhost:8899/rpc/fork/{}", fork_id),
    }))
}

/// List all active forks
pub async fn list_forks(
    State(state): State<AppState>,
) -> Result<Json<Vec<ForkInfo>>, StatusCode> {
    let manager = state
        .fork_manager
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let fork_ids = manager.get_all_fork_ids();
    let mut fork_infos = Vec::new();

    for fork_id in fork_ids {
        if let Some(fork) = manager.get_fork(&fork_id) {
            fork_infos.push(fork.get_info());
        }
    }

    Ok(Json(fork_infos))
}

/// Clean up expired forks
pub async fn cleanup_forks(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut manager = state
        .fork_manager
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let cleaned = manager.cleanup_expired_forks();

    Ok(Json(serde_json::json!({
        "success": true,
        "message": format!("Cleaned up {} expired forks", cleaned),
        "cleaned_count": cleaned,
        "active_forks": manager.active_fork_count(),
    })))
}