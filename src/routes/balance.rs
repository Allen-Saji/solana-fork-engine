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

/// Set account balance
pub async fn set_balance(
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

/// Get account balance
pub async fn get_balance(
    State(state): State<AppState>,
    Json(payload): Json<GetBalanceRequest>,
) -> Result<Json<GetBalanceResponse>, StatusCode> {
    let address = Pubkey::from_str(&payload.address)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let fork_guard = state.fork.lock().unwrap();

    match fork_guard.as_ref() {
        Some(fork) => {
            let lamports = fork.get_balance(&address);
            let sol = lamports_to_sol(lamports);

            Ok(Json(GetBalanceResponse {
                address: payload.address,
                lamports,
                sol,
            }))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Get detailed account information
pub async fn get_account(
    State(state): State<AppState>,
    Json(payload): Json<GetBalanceRequest>,
) -> Result<Json<AccountInfo>, StatusCode> {
    let address = Pubkey::from_str(&payload.address)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let fork_guard = state.fork.lock().unwrap();

    match fork_guard.as_ref() {
        Some(fork) => fork
            .get_account_info(&address)
            .map(Json)
            .ok_or(StatusCode::NOT_FOUND),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Airdrop SOL to an account
pub async fn airdrop(
    State(state): State<AppState>,
    Json(payload): Json<AirdropRequest>,
) -> Result<Json<AirdropResponse>, StatusCode> {
    let address = Pubkey::from_str(&payload.address)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let lamports = sol_to_lamports(payload.sol);

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