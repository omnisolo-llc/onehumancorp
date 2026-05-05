use std::sync::Arc;
use tonic::{Request, Response, Status};
use serde_json::Value;

use crate::ohc::billing::tier_service_server::TierService;
use crate::ohc::billing::{SyncStripeWebhookRequest, SyncStripeWebhookResponse};
use crate::pricing::rate_limit::{RedisRateLimiter, PlanTier};
use sqlx::PgPool;

pub struct MyTierService {
    db_pool: Arc<PgPool>,
    rate_limiter: Arc<RedisRateLimiter>,
}

impl MyTierService {
    pub fn new(db_pool: Arc<PgPool>, rate_limiter: Arc<RedisRateLimiter>) -> Self {
        Self {
            db_pool,
            rate_limiter,
        }
    }
}

#[tonic::async_trait]
impl TierService for MyTierService {
    async fn sync_stripe_webhook(
        &self,
        request: Request<SyncStripeWebhookRequest>,
    ) -> Result<Response<SyncStripeWebhookResponse>, Status> {
        let req = request.into_inner();
        let _secret = std::env::var("STRIPE_WEBHOOK_SECRET").unwrap_or_default();
        let payload: Value = serde_json::from_str(&req.payload_json)
            .map_err(|e| Status::invalid_argument(format!("Invalid JSON: {}", e)))?;

        // Simple mock for parsing the event
        let type_val = payload.get("type").and_then(|t| t.as_str()).unwrap_or("");

        if type_val == "customer.subscription.updated" || type_val == "customer.subscription.created" {
            let data = payload.get("data").and_then(|d| d.get("object"));
            if let Some(obj) = data {
                let customer_id = obj.get("customer").and_then(|c| c.as_str()).unwrap_or("");
                let plan_id = obj
                    .get("items")
                    .and_then(|i| i.get("data"))
                    .and_then(|d| d.get(0))
                    .and_then(|item| item.get("plan"))
                    .and_then(|p| p.get("id"))
                    .and_then(|id| id.as_str())
                    .unwrap_or("");

                // Translate plan_id to tier. In reality, we map Stripe product/price to tier
                let new_tier = if plan_id.contains("pro") {
                    "Pro"
                } else if plan_id.contains("starter") {
                    "Starter"
                } else if plan_id.contains("business") {
                    "Business"
                } else {
                    "Free"
                };

                // Find tenant_id by customer_id
                let row = sqlx::query("SELECT id FROM organizations WHERE stripe_customer_id = $1")
                    .bind(customer_id)
                    .fetch_optional(&*self.db_pool)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?;

                if let Some(record) = row {
                    let org_id: String = sqlx::Row::get(&record, "id");

                    // Update DB
                    sqlx::query("UPDATE organizations SET plan_tier = $1 WHERE id = $2")
                        .bind(new_tier)
                        .bind(&org_id)
                        .execute(&*self.db_pool)
                        .await
                        .map_err(|e| Status::internal(e.to_string()))?;

                    // Map to PlanTier enum
                    let plan_tier_enum = match new_tier {
                        "Starter" => PlanTier::Starter,
                        "Pro" => PlanTier::Pro,
                        "Business" => PlanTier::Business,
                        _ => PlanTier::Free,
                    };

                    // Update Redis
                    let _ = self.rate_limiter.set_tenant_tier(&org_id, plan_tier_enum).await;
                }
            }
        }

        Ok(Response::new(SyncStripeWebhookResponse { success: true }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use crate::pricing::rate_limit::RedisRateLimiter;
    use redis::Client;

    #[tokio::test]
    async fn test_sync_stripe_webhook_parsing() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost/ohc".to_string());
        if !db_url.contains("test") { return; }

        let pool = PgPool::connect(&db_url).await.unwrap();
        let client = Client::open("redis://127.0.0.1/").unwrap();
        let rate_limiter = Arc::new(RedisRateLimiter::new(client));

        let service = MyTierService::new(Arc::new(pool), rate_limiter);

        let req = SyncStripeWebhookRequest {
            payload_json: r#"{
                "type": "customer.subscription.updated",
                "data": {
                    "object": {
                        "customer": "cus_123",
                        "items": {
                            "data": [
                                {
                                    "plan": { "id": "price_pro" }
                                }
                            ]
                        }
                    }
                }
            }"#.to_string()
        };

        let res = service.sync_stripe_webhook(Request::new(req)).await;
        assert!(res.is_ok());
    }
}
