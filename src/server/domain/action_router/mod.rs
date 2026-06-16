use sqlx::PgPool;
use serde_json::Value;

use crate::domain::sales::SalesHandler;
use crate::domain::inbox::InboxHandler;
use crate::domain::incident::IncidentHandler;

pub struct ActionRouter {
    pool: PgPool,
}

impl ActionRouter {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn execute(&self, tenant_id: &str, feature_type: &str, payload: &Value) -> Result<(), Box<dyn std::error::Error>> {
        match feature_type {
            "incident_resolution" => {
                if let Some(incident_id) = payload.get("incident_id").and_then(|v| v.as_str()) {
                    IncidentHandler::handle_resolution(&self.pool, tenant_id, incident_id).await?;
                }
                Ok(())
            },
            "social_post_draft" => {
                tracing::info!("Approved and scheduled SocialPostDraft for tenant: {}", tenant_id);
                Ok(())
            },
            "ambassador_reply" | "instagram_dm" => {
                if let Some(inbox_id) = payload.get("inbox_message_id").and_then(|v| v.as_str()) {
                    let draft_reply = payload.get("draft_reply").and_then(|v| v.as_str()).unwrap_or("");
                    InboxHandler::handle_ambassador_reply(&self.pool, tenant_id, inbox_id, draft_reply).await?;
                }
                Ok(())
            },
            "quote_draft" => {
                if let Some(quote_id) = payload.get("quote_id").and_then(|v| v.as_str()) {
                    SalesHandler::handle_quote_draft(&self.pool, tenant_id, quote_id).await?;
                }
                Ok(())
            },
            _ => {
                tracing::warn!("Unsupported feature type: {}", feature_type);
                Ok(())
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn test_action_router_unsupported_feature() {
        // Create a lazy pool that doesn't actually connect,
        // since unsupported features shouldn't hit the DB.
        let pool = PgPoolOptions::new()
            .connect_lazy("postgres://fake:fake@localhost:5432/fake")
            .unwrap();

        let router = ActionRouter::new(pool);
        let tenant_id = "test_tenant";
        let feature_type = "some_unknown_feature";
        let payload = json!({"foo": "bar"});

        let result = router.execute(tenant_id, feature_type, &payload).await;

        // Should handle gracefully and return Ok(())
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_action_router_social_post_draft() {
        let pool = PgPoolOptions::new()
            .connect_lazy("postgres://fake:fake@localhost:5432/fake")
            .unwrap();

        let router = ActionRouter::new(pool);
        let tenant_id = "test_tenant";
        let feature_type = "social_post_draft";
        let payload = json!({"foo": "bar"});

        let result = router.execute(tenant_id, feature_type, &payload).await;

        // Should handle gracefully and return Ok(()) without hitting DB
        assert!(result.is_ok());
    }
}
