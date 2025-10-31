use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use solana_pubkey::Pubkey;
use std::str::FromStr;

use crate::{
    models::{RpcRequest, RpcResponse},
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct RpcQuery {
    pub user_id: Option<String>,
    pub fork_id: Option<String>,
}

/// Main RPC endpoint - handles Solana JSON-RPC requests
pub async fn handle_rpc(
    State(state): State<AppState>,
    Query(query): Query<RpcQuery>,
    Json(request): Json<RpcRequest>,
) -> Result<Json<RpcResponse>, StatusCode> {
    // Validate JSON-RPC version
    if request.jsonrpc != "2.0" {
        return Ok(Json(RpcResponse::error(
            request.id,
            -32600,
            "Invalid JSON-RPC version".to_string(),
        )));
    }

    // Get fork manager
    let manager = state
        .fork_manager
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Resolve fork_id
    let fork_id = if let Some(fid) = query.fork_id {
        fid
    } else if let Some(uid) = query.user_id {
        manager
            .get_user_fork_id(&uid)
            .cloned()
            .ok_or(StatusCode::NOT_FOUND)?
    } else {
        return Ok(Json(RpcResponse::error(
            request.id,
            -32602,
            "Missing fork_id or user_id parameter".to_string(),
        )));
    };

    // Get fork
    let fork = manager.get_fork(&fork_id).ok_or(StatusCode::NOT_FOUND)?;

    // Handle different RPC methods
    let result = match request.method.as_str() {
        "getBalance" => handle_get_balance(fork, &request.params),
        "getAccountInfo" => handle_get_account_info(fork, &request.params),
        "getSlot" => handle_get_slot(fork),
        "getLatestBlockhash" => handle_get_latest_blockhash(fork),
        "getBlockHeight" => handle_get_block_height(fork),
        "getHealth" => handle_get_health(),
        "getVersion" => handle_get_version(),
        _ => Err(format!("Method not supported: {}", request.method)),
    };

    match result {
        Ok(value) => Ok(Json(RpcResponse::success(request.id, value))),
        Err(msg) => Ok(Json(RpcResponse::error(request.id, -32601, msg))),
    }
}

/// Handle getBalance RPC method
fn handle_get_balance(
    fork: &crate::models::Fork,
    params: &Option<Vec<Value>>,
) -> Result<Value, String> {
    let params = params.as_ref().ok_or("Missing parameters")?;
    
    if params.is_empty() {
        return Err("Missing address parameter".to_string());
    }

    let address_str = params[0]
        .as_str()
        .ok_or("Invalid address format")?;

    let pubkey = Pubkey::from_str(address_str)
        .map_err(|e| format!("Invalid pubkey: {}", e))?;

    let lamports = fork.get_balance(&pubkey);

    Ok(json!({
        "context": {
            "slot": fork.slot
        },
        "value": lamports
    }))
}

/// Handle getAccountInfo RPC method
fn handle_get_account_info(
    fork: &crate::models::Fork,
    params: &Option<Vec<Value>>,
) -> Result<Value, String> {
    let params = params.as_ref().ok_or("Missing parameters")?;
    
    if params.is_empty() {
        return Err("Missing address parameter".to_string());
    }

    let address_str = params[0]
        .as_str()
        .ok_or("Invalid address format")?;

    let pubkey = Pubkey::from_str(address_str)
        .map_err(|e| format!("Invalid pubkey: {}", e))?;

    match fork.get_account_info(&pubkey) {
        Some(account) => Ok(json!({
            "context": {
                "slot": fork.slot
            },
            "value": {
                "lamports": account.lamports,
                "owner": account.owner,
                "executable": account.executable,
                "rentEpoch": account.rent_epoch,
                "data": ["", "base64"]  // Simplified - not returning actual data
            }
        })),
        None => Ok(json!({
            "context": {
                "slot": fork.slot
            },
            "value": null
        })),
    }
}

/// Handle getSlot RPC method
fn handle_get_slot(fork: &crate::models::Fork) -> Result<Value, String> {
    Ok(json!(fork.slot))
}

/// Handle getLatestBlockhash RPC method
fn handle_get_latest_blockhash(fork: &crate::models::Fork) -> Result<Value, String> {
    Ok(json!({
        "context": {
            "slot": fork.slot
        },
        "value": {
            "blockhash": fork.mainnet_blockhash,
            "lastValidBlockHeight": fork.slot + 150
        }
    }))
}

/// Handle getBlockHeight RPC method
fn handle_get_block_height(fork: &crate::models::Fork) -> Result<Value, String> {
    Ok(json!(fork.slot))
}

/// Handle getHealth RPC method
fn handle_get_health() -> Result<Value, String> {
    Ok(json!("ok"))
}

/// Handle getVersion RPC method
fn handle_get_version() -> Result<Value, String> {
    Ok(json!({
        "solana-core": "1.18.0",
        "feature-set": 0
    }))
}