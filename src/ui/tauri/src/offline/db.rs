use tauri::AppHandle;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PendingTransaction {
    pub id: String,
    pub payload: String,
    pub timestamp: i64,
}

pub struct LocalDatabase {
    // Basic SQLite integration for offline storage
}

impl LocalDatabase {
    pub fn new() -> Self {
        Self {}
    }
}
