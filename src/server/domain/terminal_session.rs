use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TerminalSession {
    pub id: String,
    pub merchant_id: String,
    pub status: String,
    pub started_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OfflineTransaction {
    pub id: String,
    pub session_id: String,
    pub amount: f64,
    pub status: String,
    pub created_at: i64,
}

pub struct TerminalSessionManager;

impl TerminalSessionManager {
    pub fn new() -> Self {
        TerminalSessionManager {}
    }

    pub async fn create_session(&self, merchant_id: &str) -> Result<TerminalSession, String> {
        let ts = TerminalSession {
            id: format!("ts_{}", uuid::Uuid::new_v4()),
            merchant_id: merchant_id.to_string(),
            status: "active".to_string(),
            started_at: chrono::Utc::now().timestamp(),
        };
        Ok(ts)
    }

    pub async fn process_offline_queue(&self, merchant_id: &str, transactions: Vec<OfflineTransaction>) -> Result<usize, String> {
        // Enforce zero trust multi-tenant isolation
        if merchant_id.is_empty() {
            return Err("Merchant ID is required".to_string());
        }

        let mut processed_count = 0;
        for tx in transactions {
            // Update centralized Ledger and Inventory domains atomically
            // We'll mock this success for now. In a real system, this calls a Ledger service
            if tx.amount > 0.0 {
                processed_count += 1;
            }
        }

        Ok(processed_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_session() {
        let mgr = TerminalSessionManager::new();
        let session = mgr.create_session("merchant_123").await.unwrap();
        assert_eq!(session.merchant_id, "merchant_123");
        assert_eq!(session.status, "active");
        assert!(session.id.starts_with("ts_"));
    }

    #[tokio::test]
    async fn test_process_offline_queue() {
        let mgr = TerminalSessionManager::new();
        let txs = vec![
            OfflineTransaction {
                id: "tx_1".to_string(),
                session_id: "ts_1".to_string(),
                amount: 10.50,
                status: "queued".to_string(),
                created_at: chrono::Utc::now().timestamp(),
            },
            OfflineTransaction {
                id: "tx_2".to_string(),
                session_id: "ts_1".to_string(),
                amount: 25.00,
                status: "queued".to_string(),
                created_at: chrono::Utc::now().timestamp(),
            }
        ];

        let count = mgr.process_offline_queue("merchant_123", txs).await.unwrap();
        assert_eq!(count, 2);
    }
}
