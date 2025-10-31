use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use std::str::FromStr;
use solana_pubkey::Pubkey;

use crate::{
    models::{
        requests::{
            CreateMainnetForkRequest, 
            LoadAccountRequest,
            LoadAccountsRequest,
            LoadTokenAccountsRequest,
        },
        responses::{
            CreateMainnetForkResponse,
            LoadAccountsResponse,
        },
    },
    services::MainnetClient,
    state::AppState,
    utils::resolve_fork_id,
};

/// Create a new fork and load accounts from mainnet
pub async fn create_mainnet_fork(
    State(state): State<AppState>,
    Json(payload): Json<CreateMainnetForkRequest>,
) -> Result<Json<CreateMainnetForkResponse>, (StatusCode, String)> {
    // Create the mainnet client
    let mainnet_client = if let Some(ref endpoint) = payload.rpc_endpoint {
        MainnetClient::with_endpoint(endpoint)
    } else {
        MainnetClient::new()
    };

    let user_id = payload.user_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    // Create fork with mainnet sync
    let mut fork_manager = state.fork_manager.lock()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Lock error".to_string()))?;

    let fork_id = fork_manager.create_fork_with_mainnet_sync(user_id.clone(), &mainnet_client)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to sync with mainnet: {}", e)))?;

    // Get fork metadata
    let fork = fork_manager.get_fork(&fork_id)
        .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Fork creation failed".to_string()))?;
    
    let created_at = fork.created_at;
    let mainnet_slot = fork.mainnet_slot;
    let mainnet_blockhash = fork.mainnet_blockhash.clone();

    // Load accounts from mainnet into the fork
    let mut loaded_addresses = Vec::new();
    
    for address in &payload.accounts {
        match mainnet_client.fetch_account(address) {
            Ok(account) => {
                // Parse the pubkey
                let pubkey = solana_pubkey::Pubkey::from_str(address)
                    .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid pubkey {}: {}", address, e)))?;

                // Get mutable fork reference
                let fork = fork_manager.get_fork_mut(&fork_id)
                    .ok_or((StatusCode::NOT_FOUND, "Fork not found".to_string()))?;

                // Set the account in the fork
                fork.svm.set_account(pubkey, account)
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to set account: {:?}", e)))?;

                loaded_addresses.push(address.clone());
            }
            Err(e) => {
                eprintln!("Warning: Failed to load account {}: {}", address, e);
            }
        }
    }

    // Calculate expires_at (15 minutes from creation)
    let expires_at = created_at + (15 * 60);

    drop(fork_manager);

   Ok(Json(CreateMainnetForkResponse {
    fork_id,
    user_id,
    created_at: format!("{}", created_at),
    expires_at: format!("{}", expires_at),
    mainnet_slot,           // ← NEW
    mainnet_blockhash,      // ← NEW
    accounts_loaded: loaded_addresses.len(),
    loaded_addresses,
}))
}
/// Load a single account from mainnet into an existing fork
pub async fn load_account(
    State(state): State<AppState>,
    Json(payload): Json<LoadAccountRequest>,
) -> Result<Json<LoadAccountsResponse>, (StatusCode, String)> {
    let mainnet_client = if let Some(ref endpoint) = payload.rpc_endpoint {
        MainnetClient::with_endpoint(endpoint)
    } else {
        MainnetClient::new()
    };

    let mut fork_manager = state.fork_manager.lock()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Lock error".to_string()))?;

    let fork_id = resolve_fork_id(&fork_manager, &payload.user_id)
        .map_err(|e| (e, "Fork not found".to_string()))?;

    let fork = fork_manager.get_fork_mut(&fork_id)
        .ok_or((StatusCode::NOT_FOUND, "Fork not found".to_string()))?;

    // Fetch account from mainnet
    let account = mainnet_client.fetch_account(&payload.address)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    // Load into fork
    let pubkey = Pubkey::from_str(&payload.address)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid pubkey: {}", e)))?;

    fork.svm.set_account(pubkey, account)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to set account: {:?}", e)))?;

    Ok(Json(LoadAccountsResponse {
        success: true,
        accounts_loaded: 1,
        loaded_addresses: vec![payload.address],
    }))
}

/// Load multiple accounts from mainnet into an existing fork
pub async fn load_accounts(
    State(state): State<AppState>,
    Json(payload): Json<LoadAccountsRequest>,
) -> Result<Json<LoadAccountsResponse>, (StatusCode, String)> {
    let mainnet_client = if let Some(ref endpoint) = payload.rpc_endpoint {
        MainnetClient::with_endpoint(endpoint)
    } else {
        MainnetClient::new()
    };

    let mut fork_manager = state.fork_manager.lock()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Lock error".to_string()))?;

    let fork_id = resolve_fork_id(&fork_manager, &payload.user_id)
        .map_err(|e| (e, "Fork not found".to_string()))?;

    let fork = fork_manager.get_fork_mut(&fork_id)
        .ok_or((StatusCode::NOT_FOUND, "Fork not found".to_string()))?;

    let mut loaded_addresses = Vec::new();

    for address in &payload.addresses {
        match mainnet_client.fetch_account(address) {
            Ok(account) => {
                let pubkey = Pubkey::from_str(address)
                    .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid pubkey {}: {}", address, e)))?;

                fork.svm.set_account(pubkey, account)
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to set account: {:?}", e)))?;

                loaded_addresses.push(address.clone());
            }
            Err(e) => {
                eprintln!("Warning: Failed to load account {}: {}", address, e);
            }
        }
    }

    Ok(Json(LoadAccountsResponse {
        success: true,
        accounts_loaded: loaded_addresses.len(),
        loaded_addresses,
    }))
}

/// Load all token accounts for an owner from mainnet
pub async fn load_token_accounts(
    State(state): State<AppState>,
    Json(payload): Json<LoadTokenAccountsRequest>,
) -> Result<Json<LoadAccountsResponse>, (StatusCode, String)> {
    let mainnet_client = if let Some(ref endpoint) = payload.rpc_endpoint {
        MainnetClient::with_endpoint(endpoint)
    } else {
        MainnetClient::new()
    };

    let mut fork_manager = state.fork_manager.lock()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Lock error".to_string()))?;

    let fork_id = resolve_fork_id(&fork_manager, &payload.user_id)
        .map_err(|e| (e, "Fork not found".to_string()))?;

    let fork = fork_manager.get_fork_mut(&fork_id)
        .ok_or((StatusCode::NOT_FOUND, "Fork not found".to_string()))?;

    // Fetch token accounts from mainnet
    let token_accounts = mainnet_client.fetch_token_accounts(&payload.owner)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    let mut loaded_addresses = Vec::new();

    for (address, account) in token_accounts {
        let pubkey = Pubkey::from_str(&address)
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid pubkey: {}", e)))?;

        fork.svm.set_account(pubkey, account)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to set account: {:?}", e)))?;

        loaded_addresses.push(address);
    }

    Ok(Json(LoadAccountsResponse {
        success: true,
        accounts_loaded: loaded_addresses.len(),
        loaded_addresses,
    }))
}