use std::sync::{Arc, Mutex};

use crate::models::Fork;

/// Application state shared across all route handlers
#[derive(Clone)]
pub struct AppState {
    pub fork: Arc<Mutex<Option<Fork>>>,
}

impl AppState {
    /// Create new application state with an initial fork
    pub fn new(fork_id: String) -> Self {
        let fork = Fork::new(fork_id);
        Self {
            fork: Arc::new(Mutex::new(Some(fork))),
        }
    }
}