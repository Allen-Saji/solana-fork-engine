use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use solana_pubkey::Pubkey;
use std::str::FromStr;

use crate::{
    models::{
        AccountInfo,
        AirdropRequest,
        AirdropResponse,
        GetBalanceRequest,
        GetBalanceResponse,
        SetBalanceRequest,
        SetBalanceResponse,
    },
    state::AppState,
    utils::{lamports_to_sol, sol_to_lamports},
};

/// Helper function to resolve fork_id from request
fn resolve_fork_id(
    manager: &crate::fork_manager::ForkManager,
    fork_id: Option<String>,
    user_id: Option<String>,
) -> Result<String, StatusCode> {
    if let Some(fid) = fork_id {
        Ok(fid)
    } else if let Some(uid) = user_id {
        manager
            .get_user_fork_id(&uid)
            .cloned()
            .ok_or(StatusCode::NOT_FOUND)
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

/// Set account balance
pub async fn set_balance(
    State(state): State<AppState>,
    Json(payload): Json<SetBalanceRequest>,
) -> Result<Json<SetBalanceResponse>, StatusCode> {
    let address = Pubkey::from_str(&payload.address).map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut manager = state
        .fork_manager
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let fork_id = resolve_fork_id(&manager, payload.fork_id, payload.user_id)?;

    let fork = manager
        .get_fork_mut(&fork_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    fork.set_balance(&address, payload.lamports)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(SetBalanceResponse {
        success: true,
        message: "Balance updated successfully".to_string(),
        address: payload.address,
        new_balance: payload.lamports,
    }))
}

/// Get account balance
pub async fn get_balance(
    State(state): State<AppState>,
    Json(payload): Json<GetBalanceRequest>,
) -> Result<Json<GetBalanceResponse>, StatusCode> {
    let address = Pubkey::from_str(&payload.address).map_err(|_| StatusCode::BAD_REQUEST)?;

    let manager = state
        .fork_manager
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let fork_id = resolve_fork_id(&manager, payload.fork_id, payload.user_id)?;

    let fork = manager.get_fork(&fork_id).ok_or(StatusCode::NOT_FOUND)?;

    let lamports = fork.get_balance(&address);
    let sol = lamports_to_sol(lamports);

    Ok(Json(GetBalanceResponse {
        address: payload.address,
        lamports,
        sol,
    }))
}

/// Get detailed account information
pub async fn get_account(
    State(state): State<AppState>,
    Json(payload): Json<GetBalanceRequest>,
) -> Result<Json<AccountInfo>, StatusCode> {
    let address = Pubkey::from_str(&payload.address).map_err(|_| StatusCode::BAD_REQUEST)?;

    let manager = state
        .fork_manager
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let fork_id = resolve_fork_id(&manager, payload.fork_id, payload.user_id)?;

    let fork = manager.get_fork(&fork_id).ok_or(StatusCode::NOT_FOUND)?;

    fork.get_account_info(&address)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

/// Airdrop SOL to an account
pub async fn airdrop(
    State(state): State<AppState>,
    Json(payload): Json<AirdropRequest>,
) -> Result<Json<AirdropResponse>, StatusCode> {
    let address = Pubkey::from_str(&payload.address).map_err(|_| StatusCode::BAD_REQUEST)?;

    let lamports = sol_to_lamports(payload.sol);

    let mut manager = state
        .fork_manager
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let fork_id = resolve_fork_id(&manager, payload.fork_id, payload.user_id)?;

    let fork = manager
        .get_fork_mut(&fork_id)
        .ok_or(StatusCode::NOT_FOUND)?;

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