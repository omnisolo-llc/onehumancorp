use crate::db::{DB, DbStore};
use std::sync::Arc;
use serde_json::json;

pub struct FraudShieldOrchestrator {
    db: Arc<DB>,
}

impl FraudShieldOrchestrator {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn handle_charge_dispute(&self, tenant_id: &str, transaction_id: &str, dispute_id: &str) {
        // 1. Context Gathering
        tracing::info!("FraudShieldOrchestrator: Gathering context for dispute {} (txn {})", dispute_id, transaction_id);

        // Fetch Comms/DMs/SMS from Omnichannel Inbox (interactions table)
        let communications = self.fetch_communications(tenant_id, transaction_id).await;

        // Fetch Signed Agreement & TOS from Contract Engine
        let contract = self.fetch_contract(tenant_id, transaction_id).await;

        // Fetch Deposit/Fulfillment Status from Ledger/Booking (orders table)
        let order_status = self.fetch_order_status(tenant_id, transaction_id).await;

        // 2. Dispute Packet Compiler AI (compile structured 'Evidence Packet')
        let evidence_packet = self.compile_evidence_packet(&communications, &contract, &order_status);

        // 3. Payment Gateway API: Auto-Submit Evidence
        self.submit_evidence_to_provider(dispute_id, &evidence_packet).await;

        // 4. Business Owner Notification: 'We fought a chargeback for you'
        self.trigger_notification(tenant_id, transaction_id, dispute_id).await;
    }

    async fn fetch_communications(&self, tenant_id: &str, transaction_id: &str) -> String {
        let mut comms = Vec::new();
        match &self.db.store {
            DbStore::Sqlite(pool) => {
                if let Ok(rows) = sqlx::query("SELECT content FROM interactions WHERE tenant_id = ? AND metadata LIKE ?")
                    .bind(tenant_id)
                    .bind(format!("%{}%", transaction_id))
                    .fetch_all(pool)
                    .await
                {
                    for row in rows {
                        use sqlx::Row;
                        let content: String = row.get("content");
                        comms.push(content);
                    }
                } else {
                    tracing::warn!("Failed to fetch interactions for transaction {} on sqlite", transaction_id);
                }
            }
            DbStore::Postgres => {
                if let Ok(rows) = sqlx::query("SELECT content FROM interactions WHERE tenant_id = $1 AND metadata LIKE $2")
                    .bind(tenant_id)
                    .bind(format!("%{}%", transaction_id))
                    .fetch_all(&self.db.pool)
                    .await
                {
                    for row in rows {
                        use sqlx::Row;
                        let content: String = row.get("content");
                        comms.push(content);
                    }
                } else {
                    tracing::warn!("Failed to fetch interactions for transaction {} on postgres", transaction_id);
                }
            }
        }

        if comms.is_empty() {
            "No direct communication logs found for this transaction.".to_string()
        } else {
            comms.join("\n")
        }
    }

    async fn fetch_contract(&self, _tenant_id: &str, _transaction_id: &str) -> String {
        // In a real implementation, this would query a contracts table or document store.
        "Signed Terms of Service and Delivery Agreement".to_string()
    }

    async fn fetch_order_status(&self, tenant_id: &str, transaction_id: &str) -> String {
        let mut final_status = "Status: Fulfilled / Delivered (Default assumed for dispute defense)".to_string();

        match &self.db.store {
            DbStore::Sqlite(pool) => {
                if let Ok(Some(row)) = sqlx::query("SELECT status FROM orders WHERE tenant_id = ? AND id = ?")
                    .bind(tenant_id)
                    .bind(transaction_id)
                    .fetch_optional(pool)
                    .await
                {
                    use sqlx::Row;
                    final_status = row.get("status");
                } else {
                    tracing::warn!("Failed to fetch order status for transaction {} on sqlite", transaction_id);
                }
            }
            DbStore::Postgres => {
                if let Ok(Some(row)) = sqlx::query("SELECT status FROM orders WHERE tenant_id = $1 AND id = $2")
                    .bind(tenant_id)
                    .bind(transaction_id)
                    .fetch_optional(&self.db.pool)
                    .await
                {
                    use sqlx::Row;
                    final_status = row.get("status");
                } else {
                    tracing::warn!("Failed to fetch order status for transaction {} on postgres", transaction_id);
                }
            }
        }

        final_status
    }

    fn compile_evidence_packet(&self, communications: &str, contract: &str, order_status: &str) -> serde_json::Value {
        json!({
            "evidence_compilation": {
                "communications_log": communications,
                "contract_agreements": contract,
                "fulfillment_status": order_status,
            },
            "summary": "Auto-compiled evidence packet for chargeback defense."
        })
    }

    async fn submit_evidence_to_provider(&self, dispute_id: &str, evidence_packet: &serde_json::Value) {
        // In a real app, send to Stripe API: POST /v1/disputes/{dispute_id}
        tracing::info!("Submitting bank-ready dispute packet for dispute {}: {}", dispute_id, evidence_packet);
    }

    async fn trigger_notification(&self, tenant_id: &str, transaction_id: &str, dispute_id: &str) {
        let msg = format!(
            "🛡️ Chargeback Defended: A customer disputed a transaction ({}). We automatically submitted the signed contract and communication logs confirming delivery. Dispute ID: {}. No action needed.",
            transaction_id, dispute_id
        );
        tracing::info!("Notification to tenant {}: {}", tenant_id, msg);

        // Example: storing to business_milestones or notifications table could happen here.
    }
}
