use solana_keypair::Keypair;
use std::error::Error;

use axum::http::StatusCode;
use crate::services::fork_manager::ForkManager;

use crate::constants::LAMPORTS_PER_SOL;

/// Parse keypair from various string formats
/// Supports:
/// - JSON array format: [1,2,3,...] (64 or 32 bytes)
/// - Base58 string format
pub fn parse_keypair(key_str: &str) -> Result<Keypair, Box<dyn Error>> {
    // Try to parse as JSON array first (e.g., [1,2,3,...])
    if let Ok(bytes) = serde_json::from_str::<Vec<u8>>(key_str) {
        if bytes.len() == 64 {
            // Take only the first 32 bytes (secret key)
            let mut array = [0u8; 32];
            array.copy_from_slice(&bytes[0..32]);
            return Ok(Keypair::new_from_array(array));
        } else if bytes.len() == 32 {
            // If it's already 32 bytes, use directly
            let array: [u8; 32] = bytes
                .try_into()
                .map_err(|_| "Failed to convert to array")?;
            return Ok(Keypair::new_from_array(array));
        }
    }

    // Try to parse as base58
    Ok(Keypair::from_base58_string(key_str))
}

pub fn lamports_to_sol(lamports: u64) -> f64 {
    lamports as f64 / LAMPORTS_PER_SOL as f64
}

pub fn sol_to_lamports(sol: f64) -> u64 {
    (sol * LAMPORTS_PER_SOL as f64) as u64
}

pub fn resolve_fork_id(
    fork_manager: &ForkManager,
    user_id: &Option<String>,
) -> Result<String, StatusCode> {
    if let Some(uid) = user_id {
        // Try to find fork by user_id
        if let Some(fork_id) = fork_manager.get_user_fork_id(uid) {
            Ok(fork_id.clone())
        } else {
            Err(StatusCode::NOT_FOUND)
        }
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}