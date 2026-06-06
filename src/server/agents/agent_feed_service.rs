use crate::queue::{Job, TaskQueue};
use crate::hub::Hub;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use tracing::{info, error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventPayload {
    pub source: String,
    pub event_type: String,
    pub data: serde_json::Value,
}

pub struct AgentFeedService {
    hub: Arc<Hub>,
}

impl AgentFeedService {
    pub fn new(hub: Arc<Hub>) -> Self {
        Self { hub }
    }

    pub async fn process_event(&self, tenant_id: &str, event: EventPayload) -> Result<(), String> {
        info!("Processing event for tenant {}: {:?}", tenant_id, event);

        let lock_key = format!("ohc:lock:{}:agent_action:{}", tenant_id, event.event_type);

        let lock_acquired = {
            if let Some(client) = &self.hub.redis_client {
                if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                    let result: redis::RedisResult<bool> = redis::cmd("SET")
                        .arg(&lock_key)
                        .arg("locked")
                        .arg("NX")
                        .arg("EX")
                        .arg(60)
                        .query_async(&mut conn)
                        .await;

                    result.unwrap_or(false)
                } else {
                    false
                }
            } else {
                true
            }
        };

        if !lock_acquired {
            info!("Lock not acquired for {}, skipping event", lock_key);
            return Ok(());
        }

        let draft_payload = match event.event_type.as_str() {
            "instagram_dm" => {
                serde_json::json!({
                    "message": "Drafted reply: Yes, we have vegan cakes available! Would you like to order?",
                    "suggested_action": "send_dm"
                })
            },
            "low_inventory" => {
                serde_json::json!({
                    "message": "Drafted social post: We're almost out of our famous chocolate chip cookies! Come get them before they're gone.",
                    "suggested_action": "post_to_instagram"
                })
            },
            _ => {
                serde_json::json!({
                    "message": format!("Suggested action for event: {}", event.event_type),
                    "suggested_action": "review"
                })
            }
        };

        let action_id = Uuid::new_v4().to_string();

        let pool = crate::db::get_pool();
        let result = sqlx::query(
            "INSERT INTO agent_actions (id, tenant_id, agent_id, action_type, payload, _sync_status)
                VALUES ($1, $2, $3, $4, $5, 'PENDING')"
        )
        .bind(&action_id)
        .bind(tenant_id)
        .bind("system_agent")
        .bind(&event.event_type)
        .bind(draft_payload.to_string())
        .execute(&pool)
        .await;

        if let Err(e) = result {
            error!("Failed to insert action card: {}", e);
            return Err(e.to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hub::Hub;
    use std::env;

    #[tokio::test]
    async fn test_agent_feed_service_process_event() {
        if let Ok(db_url) = env::var("OHC_DATABASE_URL") {
            let pool = sqlx::postgres::PgPoolOptions::new()
                .connect_lazy(&db_url)
                .unwrap();
            let (tx, _) = tokio::sync::mpsc::channel(100);
            let hub = Arc::new(Hub::new(tx, pool));

            let service = AgentFeedService::new(hub);

            let event = EventPayload {
                source: "instagram".to_string(),
                event_type: "instagram_dm".to_string(),
                data: serde_json::json!({"message": "Do you have vegan cakes?"}),
            };

            let result = service.process_event("tenant_123", event).await;
            assert!(result.is_ok());
        }
    }
}
