use solana_pubkey::Pubkey;

/// System program ID constant
pub const SYSTEM_PROGRAM_ID: Pubkey = solana_pubkey::pubkey!("11111111111111111111111111111111");

/// Lamports per SOL
pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

/// Default server address
pub const DEFAULT_SERVER_ADDR: &str = "0.0.0.0:8899";

/// API version
pub const API_VERSION: &str = "v1";