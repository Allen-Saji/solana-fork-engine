#![allow(deprecated)]

use axum::{extract::State, http::StatusCode, Json};
use solana_pubkey::Pubkey;
use solana_keypair::Keypair;
use solana_signer::Signer;
use solana_transaction::versioned::VersionedTransaction;
use solana_message::{Message, VersionedMessage};
use solana_system_interface::instruction as system_instruction;
use spl_token::instruction as token_instruction;
use spl_token::solana_program::program_pack::Pack;
use spl_associated_token_account::{
    get_associated_token_address,
    instruction::create_associated_token_account,
};
use std::str::FromStr;

use crate::token_models::*;
use crate::state::AppState;

/// Helper function to resolve fork_id from user_id
fn resolve_fork_id(
    manager: &crate::fork_manager::ForkManager,
    user_id: &str,
) -> Result<String, StatusCode> {
    manager
        .get_user_fork_id(user_id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)
}

// Helper function to parse base58 keypair
fn parse_keypair(keypair_str: &str) -> Result<Keypair, String> {
    Ok(Keypair::from_base58_string(keypair_str))
}

// Create a new token mint
pub async fn create_token_mint(
    State(state): State<AppState>,
    Json(payload): Json<CreateTokenRequest>,
) -> Result<Json<CreateTokenResponse>, (StatusCode, String)> {
    let mut fork_manager = state.fork_manager.lock()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Lock error".to_string()))?;
    
    let fork_id = resolve_fork_id(&fork_manager, &payload.user_id)
        .map_err(|e| (e, "Fork not found".to_string()))?;
    
    let fork = fork_manager.get_fork_mut(&fork_id)
        .ok_or((StatusCode::NOT_FOUND, "Fork not found".to_string()))?;

    // Parse payer keypair
    let payer = parse_keypair(&payload.payer_keypair)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    // Generate new mint keypair
    let mint_keypair = Keypair::new();
    
    // Get recent blockhash
    let blockhash = fork.svm.latest_blockhash();

    // Calculate rent for mint account
    let rent = fork.svm.minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN);

    // Create mint account instruction
    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint_keypair.pubkey(),
        rent,
        spl_token::state::Mint::LEN as u64,
        &spl_token::id(),
    );

    // Initialize mint instruction
    let init_mint_ix = token_instruction::initialize_mint(
        &spl_token::id(),
        &mint_keypair.pubkey(),
        &payer.pubkey(),
        None,
        payload.decimals,
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e)))?;

    // Create and sign transaction
    let msg = Message::new_with_blockhash(
        &[create_account_ix, init_mint_ix],
        Some(&payer.pubkey()),
        &blockhash,
    );
    let versioned_msg = VersionedMessage::Legacy(msg);
    let tx = VersionedTransaction::try_new(versioned_msg, &[&payer, &mint_keypair])
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e)))?;

    // Send transaction
    let result = fork.svm.send_transaction(tx);
    
    match result {
        Ok(signature) => Ok(Json(CreateTokenResponse {
            mint_address: mint_keypair.pubkey().to_string(),
            signature: format!("{:?}", signature),
        })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

// Create an associated token account
pub async fn create_token_account(
    State(state): State<AppState>,
    Json(payload): Json<CreateTokenAccountRequest>,
) -> Result<Json<CreateTokenAccountResponse>, (StatusCode, String)> {
    let mut fork_manager = state.fork_manager.lock()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Lock error".to_string()))?;
    
    let fork_id = resolve_fork_id(&fork_manager, &payload.user_id)
        .map_err(|e| (e, "Fork not found".to_string()))?;
    
    let fork = fork_manager.get_fork_mut(&fork_id)
        .ok_or((StatusCode::NOT_FOUND, "Fork not found".to_string()))?;

    // Parse inputs
    let payer = parse_keypair(&payload.payer_keypair)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    
    let mint = Pubkey::from_str(&payload.mint_address)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid mint address: {}", e)))?;
    
    let owner = Pubkey::from_str(&payload.owner_address)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid owner address: {}", e)))?;

    // Get associated token address
    let token_account = get_associated_token_address(&owner, &mint);

    // Create associated token account instruction
    let create_ata_ix = create_associated_token_account(
        &payer.pubkey(),
        &owner,
        &mint,
        &spl_token::id(),
    );

    // Get recent blockhash
    let blockhash = fork.svm.latest_blockhash();

    // Create and sign transaction
    let msg = Message::new_with_blockhash(
        &[create_ata_ix],
        Some(&payer.pubkey()),
        &blockhash,
    );
    let versioned_msg = VersionedMessage::Legacy(msg);
    let tx = VersionedTransaction::try_new(versioned_msg, &[&payer])
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e)))?;

    // Send transaction
    let result = fork.svm.send_transaction(tx);
    
    match result {
        Ok(signature) => Ok(Json(CreateTokenAccountResponse {
            token_account: token_account.to_string(),
            signature: format!("{:?}", signature),
        })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

// Mint tokens to an account
pub async fn mint_tokens(
    State(state): State<AppState>,
    Json(payload): Json<MintTokensRequest>,
) -> Result<Json<MintTokensResponse>, (StatusCode, String)> {
    let mut fork_manager = state.fork_manager.lock()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Lock error".to_string()))?;
    
    let fork_id = resolve_fork_id(&fork_manager, &payload.user_id)
        .map_err(|e| (e, "Fork not found".to_string()))?;
    
    let fork = fork_manager.get_fork_mut(&fork_id)
        .ok_or((StatusCode::NOT_FOUND, "Fork not found".to_string()))?;

    // Parse inputs
    let mint_authority = parse_keypair(&payload.mint_authority_keypair)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    
    let mint = Pubkey::from_str(&payload.mint_address)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid mint address: {}", e)))?;
    
    let destination = Pubkey::from_str(&payload.destination_account)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid destination: {}", e)))?;

    // Create mint to instruction
    let mint_to_ix = token_instruction::mint_to(
        &spl_token::id(),
        &mint,
        &destination,
        &mint_authority.pubkey(),
        &[],
        payload.amount,
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e)))?;

    // Get recent blockhash
    let blockhash = fork.svm.latest_blockhash();

    // Create and sign transaction
    let msg = Message::new_with_blockhash(
        &[mint_to_ix],
        Some(&mint_authority.pubkey()),
        &blockhash,
    );
    let versioned_msg = VersionedMessage::Legacy(msg);
    let tx = VersionedTransaction::try_new(versioned_msg, &[&mint_authority])
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e)))?;

    // Send transaction
    let result = fork.svm.send_transaction(tx);
    
    match result {
        Ok(signature) => {
            // Get new balance
            let account = fork.svm.get_account(&destination)
                .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Failed to get account".to_string()))?;
            
            let token_account = spl_token::state::Account::unpack(&account.data)
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e)))?;

            Ok(Json(MintTokensResponse {
                signature: format!("{:?}", signature),
                new_balance: token_account.amount,
            }))
        },
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

// Transfer tokens between accounts
pub async fn transfer_tokens(
    State(state): State<AppState>,
    Json(payload): Json<TransferTokensRequest>,
) -> Result<Json<TransferTokensResponse>, (StatusCode, String)> {
    let mut fork_manager = state.fork_manager.lock()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Lock error".to_string()))?;
    
    let fork_id = resolve_fork_id(&fork_manager, &payload.user_id)
        .map_err(|e| (e, "Fork not found".to_string()))?;
    
    let fork = fork_manager.get_fork_mut(&fork_id)
        .ok_or((StatusCode::NOT_FOUND, "Fork not found".to_string()))?;

    // Parse inputs
    let owner = parse_keypair(&payload.from_keypair)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    
    let source = Pubkey::from_str(&payload.source_account)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid source: {}", e)))?;
    
    let destination = Pubkey::from_str(&payload.destination_account)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid destination: {}", e)))?;

    // Create transfer instruction
    let transfer_ix = token_instruction::transfer(
        &spl_token::id(),
        &source,
        &destination,
        &owner.pubkey(),
        &[],
        payload.amount,
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e)))?;

    // Get recent blockhash
    let blockhash = fork.svm.latest_blockhash();

    // Create and sign transaction
    let msg = Message::new_with_blockhash(
        &[transfer_ix],
        Some(&owner.pubkey()),
        &blockhash,
    );
    let versioned_msg = VersionedMessage::Legacy(msg);
    let tx = VersionedTransaction::try_new(versioned_msg, &[&owner])
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e)))?;

    // Send transaction
    let result = fork.svm.send_transaction(tx);
    
    match result {
        Ok(signature) => {
            // Get balances
            let source_account = fork.svm.get_account(&source)
                .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Failed to get source account".to_string()))?;
            let dest_account = fork.svm.get_account(&destination)
                .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Failed to get dest account".to_string()))?;
            
            let source_token = spl_token::state::Account::unpack(&source_account.data)
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e)))?;
            let dest_token = spl_token::state::Account::unpack(&dest_account.data)
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e)))?;

            Ok(Json(TransferTokensResponse {
                signature: format!("{:?}", signature),
                source_balance: source_token.amount,
                destination_balance: dest_token.amount,
            }))
        },
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
    }
}

// Get token account balance
pub async fn get_token_balance(
    State(state): State<AppState>,
    Json(payload): Json<GetTokenBalanceRequest>,
) -> Result<Json<GetTokenBalanceResponse>, (StatusCode, String)> {
    let fork_manager = state.fork_manager.lock()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Lock error".to_string()))?;
    
    let fork_id = resolve_fork_id(&fork_manager, &payload.user_id)
        .map_err(|e| (e, "Fork not found".to_string()))?;
    
    let fork = fork_manager.get_fork(&fork_id)
        .ok_or((StatusCode::NOT_FOUND, "Fork not found".to_string()))?;

    let token_account_pubkey = Pubkey::from_str(&payload.token_account)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid token account: {}", e)))?;

    let account = fork.svm.get_account(&token_account_pubkey)
        .ok_or((StatusCode::NOT_FOUND, "Token account not found".to_string()))?;

    let token_account = spl_token::state::Account::unpack(&account.data)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to parse token account: {:?}", e)))?;

    Ok(Json(GetTokenBalanceResponse {
        token_account: payload.token_account,
        balance: token_account.amount,
        mint: token_account.mint.to_string(),
        owner: token_account.owner.to_string(),
    }))
}