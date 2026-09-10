use redis::{Client, aio::MultiplexedConnection};
use serde_json::Value;
use uuid::Uuid;
use chrono::Utc;
use crate::ledger::{LedgerService, LedgerEntry};

pub struct AccountingEventListener {
    redis_client: Client,
    ledger_service: LedgerService,
}

impl AccountingEventListener {
    pub fn new(redis_client: Client, ledger_service: LedgerService) -> Self {
        Self {
            redis_client,
            ledger_service,
        }
    }

    pub async fn listen(&self, queue_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut con: MultiplexedConnection = self.redis_client.get_multiplexed_async_connection().await?;

        loop {
            // Using BLPOP for blocking queue pop
            let result: redis::RedisResult<(String, String)> = redis::cmd("BLPOP")
                .arg(queue_name)
                .arg(0) // block indefinitely
                .query_async(&mut con)
                .await;

            match result {
                Ok((_, payload_str)) => {
                    if let Err(e) = self.process_event(&payload_str).await {
                        eprintln!("Error processing event: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Redis error: {}", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        }
    }

    async fn process_event(&self, payload_str: &str) -> Result<(), Box<dyn std::error::Error>> {
        let event: Value = serde_json::from_str(payload_str)?;

        let event_type = event["type"].as_str().unwrap_or("");
        let tenant_id_str = event["tenant_id"].as_str().unwrap_or("");
        let tenant_id = Uuid::parse_str(tenant_id_str)?;

        match event_type {
            "invoice.paid" => {
                let amount_cents = event["amount_cents"].as_i64().unwrap_or(0);
                let invoice_id = Uuid::parse_str(event["invoice_id"].as_str().unwrap_or(""))?;

                // Account IDs would typically come from a chart of accounts config
                let cash_account_id = Uuid::new_v4(); // Dummy
                let ar_account_id = Uuid::new_v4(); // Dummy

                let now = Utc::now();

                let entries = vec![
                    LedgerEntry {
                        id: Uuid::new_v4(),
                        tenant_id,
                        account_id: cash_account_id,
                        amount_cents,
                        currency: "USD".to_string(),
                        entry_type: "DEBIT".to_string(),
                        reference_id: Some(invoice_id),
                        description: format!("Payment received for invoice {}", invoice_id),
                        created_at: now,
                    },
                    LedgerEntry {
                        id: Uuid::new_v4(),
                        tenant_id,
                        account_id: ar_account_id,
                        amount_cents,
                        currency: "USD".to_string(),
                        entry_type: "CREDIT".to_string(),
                        reference_id: Some(invoice_id),
                        description: format!("Clear A/R for invoice {}", invoice_id),
                        created_at: now,
                    },
                ];

                self.ledger_service.record_transaction(tenant_id, entries).await?;
                println!("Processed invoice.paid for tenant {}", tenant_id);
            }
            "po.approved" => {
                 // Handle PO approved event to track liabilities
                 println!("Processed po.approved for tenant {}", tenant_id);
            }
            _ => {
                println!("Unknown event type: {}", event_type);
            }
        }

        Ok(())
    }
}
