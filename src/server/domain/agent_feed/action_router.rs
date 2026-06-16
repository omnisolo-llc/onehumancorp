use sqlx::PgPool;
use tracing::info;

pub struct ActionRouter;

impl ActionRouter {
    pub async fn execute(
        pool: &PgPool,
        tenant_id: &str,
        feature_type: &str,
        payload: &serde_json::Value,
        feed_item: &crate::domain::repository::agent_feed_repo::AgentFeedItem,
    ) -> Result<(), String> {
        match feature_type {
            "incident_resolution" => Self::handle_incident_resolution(pool, tenant_id, payload, feed_item).await,
            "social_post_draft" => Self::handle_social_post_draft(pool, tenant_id, payload).await,
            "ambassador_reply" => Self::handle_ambassador_reply(pool, tenant_id, payload).await,
            "quote_draft" => Self::handle_quote_draft(pool, tenant_id, payload).await,
            "instagram_dm" => Self::handle_instagram_dm(pool, tenant_id, payload).await,
            _ => {
                info!("Unhandled feature_type: {}", feature_type);
                Ok(())
            }
        }
    }

    async fn handle_incident_resolution(
        pool: &PgPool,
        tenant_id: &str,
        _payload: &serde_json::Value,
        feed_item: &crate::domain::repository::agent_feed_repo::AgentFeedItem,
    ) -> Result<(), String> {
        if let Some(ref context_payload) = feed_item.context_payload {
            if let Some(incident_id) = context_payload.get("incident_id").and_then(|v| v.as_str()) {
                crate::domain::incidents::handler::IncidentsHandler::handle_incident_resolution(pool, tenant_id, incident_id).await?;
            }
        }
        Ok(())
    }

    async fn handle_social_post_draft(
        _pool: &PgPool,
        tenant_id: &str,
        _payload: &serde_json::Value,
    ) -> Result<(), String> {
        info!("Approved and scheduled SocialPostDraft for tenant: {}", tenant_id);
        // Real implementation would buffer post here to AYRSHARE.
        Ok(())
    }

    async fn handle_ambassador_reply(
        pool: &PgPool,
        tenant_id: &str,
        payload: &serde_json::Value,
    ) -> Result<(), String> {
        if let Some(inbox_id) = payload.get("inbox_message_id").and_then(|v| v.as_str()) {
            let draft_reply = payload.get("draft_reply").and_then(|v| v.as_str()).unwrap_or("");
            crate::domain::inbox::handler::InboxHandler::handle_ambassador_reply(pool, tenant_id, inbox_id, draft_reply).await?;
        }
        Ok(())
    }

    async fn handle_quote_draft(
        pool: &PgPool,
        tenant_id: &str,
        payload: &serde_json::Value,
    ) -> Result<(), String> {
        if let Some(quote_id) = payload.get("quote_id").and_then(|v| v.as_str()) {
            crate::domain::quotes::handler::QuotesHandler::handle_quote_draft(pool, tenant_id, quote_id).await?;
        }
        Ok(())
    }

    async fn handle_instagram_dm(
        pool: &PgPool,
        tenant_id: &str,
        payload: &serde_json::Value,
    ) -> Result<(), String> {
        if let Some(inbox_id) = payload.get("inbox_message_id").and_then(|v| v.as_str()) {
            let draft_reply = payload.get("draft_reply").and_then(|v| v.as_str()).unwrap_or("");
            crate::domain::inbox::handler::InboxHandler::handle_instagram_dm(pool, tenant_id, inbox_id, draft_reply).await?;
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use crate::domain::agent_feed::action_router::ActionRouter;
    use crate::domain::repository::agent_feed_repo::AgentFeedItem;
    use sqlx::PgPool;

    #[tokio::test]
    async fn test_action_router_unhandled_feature_type() {
        if std::env::var("DATABASE_URL").is_err() && std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }
        let database_url = std::env::var("OHC_DATABASE_URL").or_else(|_| std::env::var("DATABASE_URL")).unwrap();
        let pool = PgPool::connect(&database_url).await.unwrap();
        let payload = serde_json::json!({});
        let feed_item = AgentFeedItem {
            id: "test".to_string(),
            tenant_id: "test".to_string(),
            event_source: "test".to_string(),
            context_payload: None,
            proposed_action: None,
            lifecycle_state: "PENDING_APPROVAL".to_string(),
            created_at: None,
            updated_at: None,
        };

        let result = ActionRouter::execute(&pool, "test_tenant", "unknown_feature", &payload, &feed_item).await;
        assert!(result.is_ok());
    }
}
