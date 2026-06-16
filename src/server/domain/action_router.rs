use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionIntent {
    pub feature_type: String,
    #[serde(flatten)]
    pub payload: serde_json::Value,
}

pub struct ActionRouter {
    pool: PgPool,
}

impl ActionRouter {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn execute_intent(
        &self,
        tenant_id: &str,
        intent: ActionIntent,
    ) -> Result<(), String> {
        let feature_type = intent.feature_type.as_str();

        match feature_type {
            "incident_resolution" => {
                crate::domain::sre::handle_incident_resolution(&self.pool, tenant_id, &intent.payload).await
            }
            "social_post_draft" => {
                crate::domain::marketing::handle_social_post_draft(&self.pool, tenant_id, &intent.payload).await
            }
            "ambassador_reply" | "instagram_dm" => {
                crate::domain::inbox::handle_ambassador_reply(&self.pool, tenant_id, feature_type, &intent.payload).await
            }
            "quote_draft" => {
                crate::domain::quotes::handle_quote_draft(&self.pool, tenant_id, &intent.payload).await
            }
            _ => {
                tracing::warn!("Unsupported feature_type for ActionRouter: {}", feature_type);
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_action_router_unsupported_type() -> Result<(), Box<dyn std::error::Error>> {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@127.0.0.1:5432/postgres".to_string());
        let pool_res = sqlx::PgPool::connect(&db_url).await;
        if pool_res.is_err() { return Ok(()); } // Skip if no DB available
        let pool = pool_res?;
        let router = ActionRouter::new(pool);
        let intent = ActionIntent {
            feature_type: "unknown_feature".to_string(),
            payload: json!({}),
        };

        let result = router.execute_intent("tenant_123", intent).await;
        assert!(result.is_ok());
        Ok(())
    }
}
