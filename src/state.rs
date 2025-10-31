use std::sync::{Arc, Mutex};
use crate::services::fork_manager::ForkManager;

#[derive(Clone)]  // ← ADD THIS
pub struct AppState {
    pub fork_manager: Arc<Mutex<ForkManager>>,
}

impl AppState {
    pub fn new(fork_manager: Arc<Mutex<ForkManager>>) -> Self {
        Self { fork_manager }
    }
}