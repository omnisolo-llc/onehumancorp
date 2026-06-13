use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use uuid::Uuid;
use serde_json::json;
use tokio::time::timeout;

pub struct DynamicPricingWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}

const AI_AGENT_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_RETRIES: i32 = 3;

impl DynamicPricingWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(120), // Run every 2 minutes
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let interval_duration = self.poll_interval;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval_duration);
            loop {
                interval.tick().await;
                if let Err(e) = Self::run_analysis(&db).await {
                    ::server_telemetry::record_error_signal("DynamicPricingWorker analysis error");
                    tracing::error!("DynamicPricingWorker analysis error: {}", e);
                }
            }
        });
    }

    pub async fn run_analysis(db: &Arc<DB>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let pool = &db.pool;

        // 1. Analyze Inventory Velocity
        // Find products with high inventory but low recent sales, or low inventory but high sales.
        // For simplicity, we trigger on inventory > 50 (simulate stagnant) and inventory < 5 (simulate high demand)
        let mut conn = pool.acquire().await?;

        // Stagnant inventory
        let stagnant_products = sqlx::query(
            "SELECT id, tenant_id, title, price_cents, inventory_count FROM products WHERE inventory_count > 50 LIMIT 5"
        )
        .fetch_all(&mut *conn)
        .await?;

        for row in stagnant_products {
            use sqlx::Row;
            let product_id: String = row.get("id");
            let tenant_id: String = row.get("tenant_id");
            let title: String = row.get("title");
            let base_price: i64 = row.get("price_cents");

            // Check if we already proposed recently
            let existing: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM agent_feed_items WHERE tenant_id = $1 AND event_source = 'dynamic_pricing' AND lifecycle_state = 'PENDING_APPROVAL' AND context_payload->>'product_id' = $2"
            )
            .bind(&tenant_id)
            .bind(&product_id)
            .fetch_one(&mut *conn)
            .await?;

            if existing > 0 {
                continue;
            }

            let prompt = format!("You are the Dynamic Pricing Agent for a business owner. The product '{}' has high stagnant inventory. Suggest a dynamic pricing rule to clear inventory. Give a short rationale message, a discount percentage (e.g. 15), and a new minimum price. Output a JSON object with 'message', 'discount_percent', 'min_price_cents'. Example: {{\"message\": \"...\", \"discount_percent\": 15, \"min_price_cents\": 850}}", title);

            let mut attempts = 0;
            let mut ai_response = String::new();
            while attempts < MAX_RETRIES {
                let ai_op = async {
                    if let Ok(mut client) = ::server_ohc::orchestration::hub_service_client::HubServiceClient::connect(std::env::var("OHC_HUB_URL").unwrap_or_else(|_| "http://127.0.0.1:8081".to_string())).await {
                        let reason_req = ::server_ohc::orchestration::ReasonRequest {
                            prompt: prompt.clone(),
                            from_agent_id: "Dynamic Pricing Agent".into(),
                        };
                        if let Ok(res) = client.reason(tonic::Request::new(reason_req)).await {
                            return Ok(res.into_inner().content);
                        }
                    }
                    Err("AI call failed".to_string())
                };

                match timeout(AI_AGENT_TIMEOUT, ai_op).await {
                    Ok(Ok(content)) => {
                        ai_response = content;
                        break;
                    },
                    _ => {
                        attempts += 1;
                        tokio::time::sleep(std::time::Duration::from_secs(2u64.pow(attempts as u32))).await;
                    }
                }
            }

            if !ai_response.is_empty() {
                let json_start = ai_response.find('{').unwrap_or(0);
                let json_end = ai_response.rfind('}').unwrap_or(ai_response.len() - 1) + 1;
                if json_start < json_end {
                    let json_str = &ai_response[json_start..json_end];
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(json_str) {
                        let message = parsed.get("message").and_then(|m| m.as_str()).unwrap_or("Consider discounting this item to clear inventory.");
                        let min_price = parsed.get("min_price_cents").and_then(|v| v.as_i64()).unwrap_or(base_price / 2);
                        let discount = parsed.get("discount_percent").and_then(|v| v.as_i64()).unwrap_or(15);

                        let context_payload = json!({
                            "product_id": product_id,
                            "product_name": title,
                            "rationale": message,
                            "trigger": "stagnant_inventory"
                        });

                        let proposed_action = json!({
                            "action_type": "apply_discount_rule",
                            "discount_percent": discount,
                            "base_price_cents": base_price,
                            "min_price_cents": min_price,
                            "description": format!("Apply {}% discount to {}", discount, title)
                        });

                        let item_id = Uuid::new_v4().to_string();
                        let _ = sqlx::query(
                            "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES ($1, $2, 'dynamic_pricing', $3, $4, 'PENDING_APPROVAL', NOW(), NOW())"
                        )
                        .bind(&item_id)
                        .bind(&tenant_id)
                        .bind(sqlx::types::Json(context_payload))
                        .bind(sqlx::types::Json(proposed_action))
                        .execute(&mut *conn)
                        .await;
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dynamic_pricing_worker_instantiation() {
        let db = Arc::new(crate::db::DB::new_for_test().await);
        let worker = DynamicPricingWorker::new(db);
        assert_eq!(worker.poll_interval.as_secs(), 120);
    }
}
