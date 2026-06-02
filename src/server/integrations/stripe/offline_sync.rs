use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflinePaymentIntent {
    pub intent_id: String,
    pub amount: u64,
    pub currency: String,
    pub idempotency_key: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub success: bool,
    pub idempotency_key: String,
    pub message: String,
    pub already_processed: bool,
}

pub struct TerminalOfflineSyncService {
    processed_idempotency_keys: Arc<Mutex<HashMap<String, OfflinePaymentIntent>>>,
    _stale_threshold_seconds: u64,
}

impl TerminalOfflineSyncService {
    pub fn new() -> Self {
        Self {
            processed_idempotency_keys: Arc::new(Mutex::new(HashMap::new())),
            _stale_threshold_seconds: 7200, // 2 hours
        }
    }

    /// Process a payment intent received from the mobile client
    pub async fn sync_offline_intent(&self, intent: OfflinePaymentIntent) -> SyncResult {
        let mut keys = self.processed_idempotency_keys.lock().unwrap();

        if keys.contains_key(&intent.idempotency_key) {
            return SyncResult {
                success: true,
                idempotency_key: intent.idempotency_key,
                message: "Payment intent already processed".to_string(),
                already_processed: true,
            };
        }

        // Simulate processing payment via Stripe API using the intent token
        // In a real scenario, this would call the Stripe SDK/API

        let mut processed_intent = intent.clone();
        processed_intent.status = "succeeded".to_string();

        keys.insert(intent.idempotency_key.clone(), processed_intent);

        SyncResult {
            success: true,
            idempotency_key: intent.idempotency_key,
            message: "Payment intent processed successfully".to_string(),
            already_processed: false,
        }
    }

    /// Recurring check for stale offline queues for AI Finance agent (Simulated)
    pub async fn check_stale_queues(&self, current_queued_intents: &[OfflinePaymentIntent]) -> bool {
        // In reality, this would check timestamps in DB. We simulate by checking if there's any pending intent.
        // For the CUJ requirement: "AI Finance agent must have a recurring check for stale offline queues (simulated via backend flag)."

        if current_queued_intents.is_empty() {
            return false;
        }

        // Simulating the condition where intents have been queued for >2 hours
        // For testing, if we provide intents to this function, we assume they are stale if status is "pending".
        for intent in current_queued_intents {
             if intent.status == "pending" {
                 return true; // We have stale offline intents, trigger AI notification
             }
        }

        false
    }
}

impl Default for TerminalOfflineSyncService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sync_offline_intent_success() {
        let service = TerminalOfflineSyncService::new();
        let intent = OfflinePaymentIntent {
            intent_id: "pi_123".to_string(),
            amount: 5000,
            currency: "usd".to_string(),
            idempotency_key: Uuid::new_v4().to_string(),
            status: "pending".to_string(),
        };

        let result = service.sync_offline_intent(intent).await;
        assert!(result.success);
        assert!(!result.already_processed);
    }

    #[tokio::test]
    async fn test_sync_offline_intent_idempotency() {
        let service = TerminalOfflineSyncService::new();
        let idempotency_key = Uuid::new_v4().to_string();
        let intent = OfflinePaymentIntent {
            intent_id: "pi_123".to_string(),
            amount: 5000,
            currency: "usd".to_string(),
            idempotency_key: idempotency_key.clone(),
            status: "pending".to_string(),
        };

        // First sync
        let result1 = service.sync_offline_intent(intent.clone()).await;
        assert!(result1.success);
        assert!(!result1.already_processed);

        // Second sync with same idempotency key
        let result2 = service.sync_offline_intent(intent).await;
        assert!(result2.success);
        assert!(result2.already_processed);
    }

    #[tokio::test]
    async fn test_check_stale_queues() {
        let service = TerminalOfflineSyncService::new();

        let stale_intents = vec![OfflinePaymentIntent {
            intent_id: "pi_123".to_string(),
            amount: 5000,
            currency: "usd".to_string(),
            idempotency_key: Uuid::new_v4().to_string(),
            status: "pending".to_string(),
        }];

        let result = service.check_stale_queues(&stale_intents).await;
        assert!(result); // Should trigger AI notification

        let processed_intents = vec![OfflinePaymentIntent {
            intent_id: "pi_456".to_string(),
            amount: 5000,
            currency: "usd".to_string(),
            idempotency_key: Uuid::new_v4().to_string(),
            status: "succeeded".to_string(),
        }];

        let result2 = service.check_stale_queues(&processed_intents).await;
        assert!(!result2); // Should not trigger AI notification
    }
}
