use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::Fork;

/// Maximum fork lifetime in seconds (15 minutes)
const FORK_LIFETIME_SECONDS: u64 = 15 * 60;

/// Manages multiple forks for different users
pub struct ForkManager {
    /// Map of fork_id -> Fork
    forks: HashMap<String, Fork>,
    /// Map of user_id -> fork_id
    user_forks: HashMap<String, String>,
}

impl ForkManager {
    /// Create a new fork manager
    pub fn new() -> Self {
        Self {
            forks: HashMap::new(),
            user_forks: HashMap::new(),
        }
    }

    /// Create a new fork for a user
    pub fn create_fork(&mut self, user_id: String) -> String {
        // Check if user already has a fork
        if let Some(fork_id) = self.user_forks.get(&user_id) {
            // Check if fork is still valid
            if let Some(fork) = self.forks.get(fork_id) {
                if !self.is_fork_expired(fork) {
                    return fork_id.clone();
                }
            }
        }

        // Create new fork
        let fork_id = format!("fork-{}-{}", user_id, Self::current_timestamp());
        let fork = Fork::new(fork_id.clone());

        self.forks.insert(fork_id.clone(), fork);
        self.user_forks.insert(user_id, fork_id.clone());

        fork_id
    }

    /// Get a fork by ID
    pub fn get_fork(&self, fork_id: &str) -> Option<&Fork> {
        self.forks.get(fork_id)
    }

    /// Get a mutable fork by ID
    pub fn get_fork_mut(&mut self, fork_id: &str) -> Option<&mut Fork> {
        self.forks.get_mut(fork_id)
    }

    /// Get fork ID for a user
    pub fn get_user_fork_id(&self, user_id: &str) -> Option<&String> {
        self.user_forks.get(user_id)
    }

    /// Check if a fork has expired
    fn is_fork_expired(&self, fork: &Fork) -> bool {
        let current_time = Self::current_timestamp();
        current_time - fork.created_at > FORK_LIFETIME_SECONDS
    }

    /// Clean up expired forks
    pub fn cleanup_expired_forks(&mut self) -> usize {
        let mut expired_fork_ids = Vec::new();

        // Find expired forks
        for (fork_id, fork) in &self.forks {
            if self.is_fork_expired(fork) {
                expired_fork_ids.push(fork_id.clone());
            }
        }

        let count = expired_fork_ids.len();

        // Remove expired forks
        for fork_id in expired_fork_ids {
            self.forks.remove(&fork_id);

            // Remove from user mapping
            self.user_forks.retain(|_, fid| fid != &fork_id);
        }

        count
    }

    /// Get total number of active forks
    pub fn active_fork_count(&self) -> usize {
        self.forks.len()
    }

    /// Get all fork IDs
    pub fn get_all_fork_ids(&self) -> Vec<String> {
        self.forks.keys().cloned().collect()
    }

    /// Delete a specific fork
    pub fn delete_fork(&mut self, fork_id: &str) -> bool {
        if self.forks.remove(fork_id).is_some() {
            // Remove from user mapping
            self.user_forks.retain(|_, fid| fid != fork_id);
            true
        } else {
            false
        }
    }

    /// Get current timestamp in seconds
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

/// Thread-safe fork manager
pub type SharedForkManager = Arc<Mutex<ForkManager>>;

/// Create a new shared fork manager
pub fn create_shared_fork_manager() -> SharedForkManager {
    Arc::new(Mutex::new(ForkManager::new()))
}