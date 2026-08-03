use tauri::AppHandle;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PendingTransaction {
    pub id: String,
    pub payload: String,
    pub timestamp: i64,
}

use std::sync::Mutex;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

pub struct LocalDatabase {
    // File-backed persistent storage for offline cache
    pub storage_path: PathBuf,
    pub transactions: Mutex<Vec<PendingTransaction>>,
}

impl LocalDatabase {
    pub fn new() -> Self {
        let path = std::env::temp_dir().join("ohc_offline_sync.json");
        let mut txs = Vec::new();

        if let Ok(mut file) = File::open(&path) {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                if let Ok(parsed) = serde_json::from_str::<Vec<PendingTransaction>>(&contents) {
                    txs = parsed;
                }
            }
        }

        Self {
            storage_path: path,
            transactions: Mutex::new(txs),
        }
    }

    fn save_to_disk(&self, txs: &[PendingTransaction]) {
        if let Ok(file) = OpenOptions::new().write(true).create(true).truncate(true).open(&self.storage_path) {
            let _ = serde_json::to_writer(file, txs);
        }
    }

    pub fn insert_tx(&self, tx: PendingTransaction) {
        if let Ok(mut guard) = self.transactions.lock() {
            guard.push(tx);
            self.save_to_disk(&guard);
        }
    }

    pub fn get_pending_txs(&self) -> Vec<PendingTransaction> {
        let mut result = Vec::new();
        if let Ok(mut guard) = self.transactions.lock() {
            while let Some(tx) = guard.pop() {
                result.push(tx);
            }
            self.save_to_disk(&guard);
        }
        result.reverse();
        result
    }
}
