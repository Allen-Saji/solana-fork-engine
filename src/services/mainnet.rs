use solana_client::rpc_client::RpcClient;
use solana_account::Account;
use solana_pubkey::Pubkey;
use std::str::FromStr;

pub struct MainnetClient {
    rpc_client: RpcClient,
}

impl MainnetClient {
    /// Create a new mainnet client with default RPC endpoint
    pub fn new() -> Self {
        Self::with_endpoint("https://api.mainnet-beta.solana.com")
    }

    /// Create a new mainnet client with custom RPC endpoint
    pub fn with_endpoint(endpoint: &str) -> Self {
        Self {
            rpc_client: RpcClient::new(endpoint.to_string()),
        }
    }

    /// Fetch an account from mainnet
    pub fn fetch_account(&self, address: &str) -> Result<Account, String> {
        let pubkey = Pubkey::from_str(address)
            .map_err(|e| format!("Invalid pubkey: {}", e))?;

        self.rpc_client
            .get_account(&pubkey)
            .map_err(|e| format!("Failed to fetch account: {}", e))
    }

    /// Fetch multiple accounts from mainnet
    pub fn fetch_accounts(&self, addresses: &[String]) -> Result<Vec<(String, Account)>, String> {
        let mut results = Vec::new();
        
        for address in addresses {
            match self.fetch_account(address) {
                Ok(account) => results.push((address.clone(), account)),
                Err(e) => return Err(format!("Failed to fetch {}: {}", address, e)),
            }
        }

        Ok(results)
    }

    /// Fetch all token accounts owned by an address
    pub fn fetch_token_accounts(&self, owner: &str) -> Result<Vec<(String, Account)>, String> {
        let owner_pubkey = Pubkey::from_str(owner)
            .map_err(|e| format!("Invalid owner pubkey: {}", e))?;

        // Get the SPL Token program ID
        let token_program_id = spl_token::id();

        // Fetch all accounts owned by the token program
        let accounts = self.rpc_client
            .get_program_accounts(&token_program_id)
            .map_err(|e| format!("Failed to fetch token accounts: {}", e))?;

        // Filter accounts where the owner field matches our owner pubkey
        let owned_accounts: Vec<(String, Account)> = accounts
            .into_iter()
            .filter_map(|(pubkey, account)| {
                // Token account data structure:
                // - First 32 bytes: mint pubkey
                // - Next 32 bytes: owner pubkey (bytes 32-64)
                // - Remaining: amount, delegate, state, etc.
                
                if account.data.len() >= 64 {
                    let owner_bytes = &account.data[32..64];
                    
                    // Check if this token account is owned by the specified address
                    if owner_bytes == owner_pubkey.to_bytes() {
                        Some((pubkey.to_string(), account))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        Ok(owned_accounts)
    }

    /// Get the latest blockhash from mainnet
    pub fn get_latest_blockhash(&self) -> Result<String, String> {
        self.rpc_client
            .get_latest_blockhash()
            .map(|hash| hash.to_string())
            .map_err(|e| format!("Failed to get blockhash: {}", e))
    }

    /// Get slot information
    pub fn get_slot(&self) -> Result<u64, String> {
        self.rpc_client
            .get_slot()
            .map_err(|e| format!("Failed to get slot: {}", e))
    }
}

impl Default for MainnetClient {
    fn default() -> Self {
        Self::new()
    }
}