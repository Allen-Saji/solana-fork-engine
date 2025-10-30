use crate::fork_manager::SharedForkManager;

/// Application state shared across all route handlers
#[derive(Clone)]
pub struct AppState {
    pub fork_manager: SharedForkManager,
}

impl AppState {
    /// Create new application state with fork manager
    pub fn new(fork_manager: SharedForkManager) -> Self {
        Self { fork_manager }
    }
}