use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use solana_message::Message;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use solana_system_interface::instruction::transfer;
use solana_transaction::Transaction;
use std::str::FromStr;

use crate::{
    models::{
        SendTransactionRequest,
        SendTransactionResponse,
        TransferRequest,
        TransferResponse,
    },
    state::AppState,
    utils::{parse_keypair, sol_to_lamports},
};

/// Helper function to resolve fork_id from request
fn resolve_fork_id(
    manager: &crate::services::fork_manager::ForkManager,
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

/// Send a raw transaction
pub async fn send_transaction(
    State(state): State<AppState>,
    Json(payload): Json<SendTransactionRequest>,
) -> Result<Json<SendTransactionResponse>, StatusCode> {
    use base64::Engine;

    // Decode base64 transaction
    let tx_bytes = base64::engine::general_purpose::STANDARD
        .decode(&payload.transaction)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Deserialize transaction
    let transaction: Transaction = bincode::deserialize(&tx_bytes)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut manager = state
        .fork_manager
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let fork_id = resolve_fork_id(&manager, payload.fork_id, payload.user_id)?;

    let fork = manager
        .get_fork_mut(&fork_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    let result = fork
        .send_transaction(transaction)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(SendTransactionResponse {
        success: result.success,
        signature: result.signature,
        error: result.error,
    }))
}

/// Transfer SOL between accounts
pub async fn transfer_sol(
    State(state): State<AppState>,
    Json(payload): Json<TransferRequest>,
) -> Result<Json<TransferResponse>, StatusCode> {
    // Parse addresses
    let from_pubkey = Pubkey::from_str(&payload.from).map_err(|_| StatusCode::BAD_REQUEST)?;
    let to_pubkey = Pubkey::from_str(&payload.to).map_err(|_| StatusCode::BAD_REQUEST)?;

    // Convert SOL to lamports
    let lamports = sol_to_lamports(payload.amount_sol);

    // Parse private key
    let keypair = parse_keypair(&payload.private_key).map_err(|_| StatusCode::BAD_REQUEST)?;

    // Verify the from address matches the keypair
    if keypair.pubkey() != from_pubkey {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut manager = state
        .fork_manager
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let fork_id = resolve_fork_id(&manager, payload.fork_id, payload.user_id)?;

    let fork = manager
        .get_fork_mut(&fork_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    // Create transfer instruction
    let instruction = transfer(&from_pubkey, &to_pubkey, lamports);

    // Get latest blockhash from fork
    let recent_blockhash = fork.svm.latest_blockhash();

    // Create message
    let message = Message::new(&[instruction], Some(&from_pubkey));

    // Create and sign transaction
    let transaction = Transaction::new(&[&keypair], message, recent_blockhash);

    // Send transaction
    let result = fork
        .send_transaction(transaction)
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