use axum::{
    extract::State,
    Json,
};

use crate::{
    models::{CreateForkRequest, CreateForkResponse},
    state::AppState,
};

/// Create or get existing fork
pub async fn create_fork(
    State(state): State<AppState>,
    Json(payload): Json<CreateForkRequest>,
) -> Json<CreateForkResponse> {
    let user = payload.user_id.unwrap_or_else(|| "anonymous".to_string());

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
            fork_id: String::new(),
            rpc_url: String::new(),
        })
    }
}