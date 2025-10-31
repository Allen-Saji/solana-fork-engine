#![allow(deprecated)]

use axum::{extract::State, http::StatusCode, Json};
use solana_pubkey::Pubkey;
use solana_keypair::Keypair;
use solana_signer::Signer;
use solana_transaction::versioned::VersionedTransaction;
use solana_message::{Message, VersionedMessage};
use solana_instruction::{AccountMeta, Instruction};
use std::str::FromStr;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

use crate::models::program::*;
use crate::state::AppState;
use crate::services::MainnetClient;

/// Helper function to resolve fork_id from user_id
fn resolve_fork_id(
    manager: &crate::services::fork_manager::ForkManager,
    user_id: &str,
) -> Result<String, StatusCode> {
    manager
        .get_user_fork_id(user_id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)
}

/// Helper function to parse base58 keypair
fn parse_keypair(keypair_str: &str) -> Result<Keypair, String> {
    Ok(Keypair::from_base58_string(keypair_str))
}

/// Deploy a program to the fork
pub async fn deploy_program(
    State(state): State<AppState>,
    Json(payload): Json<DeployProgramRequest>,
) -> Result<Json<DeployProgramResponse>, (StatusCode, String)> {
    let mut fork_manager = state.fork_manager.lock()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Lock error".to_string()))?;
    
    let fork_id = resolve_fork_id(&fork_manager, &payload.user_id)
        .map_err(|e| (e, "Fork not found".to_string()))?;
    
    let fork = fork_manager.get_fork_mut(&fork_id)
        .ok_or((StatusCode::NOT_FOUND, "Fork not found".to_string()))?;

    // Parse program keypair
    let program_keypair = parse_keypair(&payload.program_keypair)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid program keypair: {}", e)))?;

    // Decode program data from base64
    let program_data = BASE64.decode(&payload.program_data)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid base64 program data: {}", e)))?;

    let program_id = program_keypair.pubkey();
    let program_size = program_data.len();

    // Use liteSVM's add_program method to deploy
    // This directly adds the program without needing deployment transactions
    // FIXED: Properly handle the Result returned by add_program
    fork.svm.add_program(program_id, &program_data)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to deploy program: {:?}", e)))?;

    Ok(Json(DeployProgramResponse {
        program_id: program_id.to_string(),
        signature: "program_deployed_directly".to_string(),
        success: true,
        deployed_size: program_size,
    }))
}

/// Invoke a program instruction
pub async fn invoke_program(
    State(state): State<AppState>,
    Json(payload): Json<InvokeProgramRequest>,
) -> Result<Json<InvokeProgramResponse>, (StatusCode, String)> {
    let mut fork_manager = state.fork_manager.lock()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Lock error".to_string()))?;
    
    let fork_id = resolve_fork_id(&fork_manager, &payload.user_id)
        .map_err(|e| (e, "Fork not found".to_string()))?;
    
    let fork = fork_manager.get_fork_mut(&fork_id)
        .ok_or((StatusCode::NOT_FOUND, "Fork not found".to_string()))?;

    // Parse program ID
    let program_id = Pubkey::from_str(&payload.program_id)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid program ID: {}", e)))?;

    // Decode instruction data from base64
    let instruction_data = BASE64.decode(&payload.instruction_data)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid base64 instruction data: {}", e)))?;

    // Parse signers
    let signers: Vec<Keypair> = payload.signers.iter()
        .map(|s| parse_keypair(s))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid signer: {}", e)))?;

    if signers.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "At least one signer is required".to_string()));
    }

    // Convert AccountMetaData to AccountMeta
    let accounts: Vec<AccountMeta> = payload.accounts.iter()
        .map(|acc| {
            let pubkey = Pubkey::from_str(&acc.pubkey)
                .map_err(|e| format!("Invalid account pubkey: {}", e))?;
            Ok(AccountMeta {
                pubkey,
                is_signer: acc.is_signer,
                is_writable: acc.is_writable,
            })
        })
        .collect::<Result<Vec<_>, String>>()
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    // Create instruction
    let instruction = Instruction {
        program_id,
        accounts,
        data: instruction_data,
    };

    // Get recent blockhash
    let blockhash = fork.svm.latest_blockhash();

    // Create and sign transaction
    let msg = Message::new_with_blockhash(
        &[instruction],
        Some(&signers[0].pubkey()),
        &blockhash,
    );
    let versioned_msg = VersionedMessage::Legacy(msg);
    
    let signer_refs: Vec<&Keypair> = signers.iter().collect();
    let tx = VersionedTransaction::try_new(versioned_msg, &signer_refs)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create transaction: {:?}", e)))?;

    // Send transaction
    let result = fork.svm.send_transaction(tx);

    match result {
        Ok(_meta) => {
            // TODO: Extract logs from transaction metadata if available
            Ok(Json(InvokeProgramResponse {
                signature: "instruction_executed".to_string(),
                success: true,
                logs: vec!["Program executed successfully".to_string()],
                error: None,
            }))
        },
        Err(e) => {
            Ok(Json(InvokeProgramResponse {
                signature: "execution_failed".to_string(),
                success: false,
                logs: vec![],
                error: Some(format!("{:?}", e)),
            }))
        }
    }
}

/// Load a program from mainnet
pub async fn load_program(
    State(state): State<AppState>,
    Json(payload): Json<LoadProgramRequest>,
) -> Result<Json<LoadProgramResponse>, (StatusCode, String)> {
    let mut fork_manager = state.fork_manager.lock()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Lock error".to_string()))?;
    
    let fork_id = resolve_fork_id(&fork_manager, &payload.user_id)
        .map_err(|e| (e, "Fork not found".to_string()))?;
    
    let fork = fork_manager.get_fork_mut(&fork_id)
        .ok_or((StatusCode::NOT_FOUND, "Fork not found".to_string()))?;

    // Create mainnet client
    let mainnet_client = if let Some(ref endpoint) = payload.rpc_endpoint {
        MainnetClient::with_endpoint(endpoint)
    } else {
        MainnetClient::new()
    };

    // Parse program ID
    let program_id = Pubkey::from_str(&payload.program_id)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid program ID: {}", e)))?;

    // Fetch program account from mainnet
    let program_account = mainnet_client.fetch_account(&payload.program_id)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Failed to fetch program: {}", e)))?;

    let program_size = program_account.data.len();
    let is_executable = program_account.executable;

    // Load program into fork
    fork.svm.set_account(program_id, program_account)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to load program: {:?}", e)))?;

    Ok(Json(LoadProgramResponse {
        program_id: payload.program_id,
        success: true,
        program_size,
        is_executable,
    }))
}

/// Get program information
pub async fn get_program_info(
    State(state): State<AppState>,
    Json(payload): Json<GetProgramRequest>,
) -> Result<Json<ProgramInfo>, (StatusCode, String)> {
    let fork_manager = state.fork_manager.lock()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Lock error".to_string()))?;
    
    let fork_id = resolve_fork_id(&fork_manager, &payload.user_id)
        .map_err(|e| (e, "Fork not found".to_string()))?;
    
    let fork = fork_manager.get_fork(&fork_id)
        .ok_or((StatusCode::NOT_FOUND, "Fork not found".to_string()))?;

    let program_id = Pubkey::from_str(&payload.program_id)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid program ID: {}", e)))?;

    let account = fork.svm.get_account(&program_id)
        .ok_or((StatusCode::NOT_FOUND, "Program not found in fork".to_string()))?;

    Ok(Json(ProgramInfo {
        program_id: payload.program_id,
        executable: account.executable,
        owner: account.owner.to_string(),
        data_size: account.data.len(),
        lamports: account.lamports,
    }))
}