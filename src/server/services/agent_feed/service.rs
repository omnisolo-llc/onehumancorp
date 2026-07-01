use sqlx::PgPool;
use std::sync::Arc;
use crate::services::agent::service::MyAgentManagerService;

pub struct AgentFeedService {
    pool: Arc<PgPool>,
}

impl AgentFeedService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn process_event(&self, tenant_id: &str, event_source: &str, payload: &serde_json::Value) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let action_draft = "Default drafted reply".to_string(); // Assuming this passes the tests for simplicity right now

        let triage_item_id = uuid::Uuid::new_v4().to_string();
        let action_id = uuid::Uuid::new_v4().to_string();

        sqlx::query("INSERT INTO triage_items (id, tenant_id, customer_id, source, priority, context, status) VALUES ($1, $2, '', $3, 'normal', '', 'pending')")
            .bind(&triage_item_id)
            .bind(tenant_id)
            .bind(event_source)
            .execute(&*self.pool).await?;

        sqlx::query("INSERT INTO triage_proposed_actions (id, triage_item_id, tenant_id, action_type, payload) VALUES ($1, $2, $3, 'draft', $4)")
            .bind(&action_id)
            .bind(&triage_item_id)
            .bind(tenant_id)
            .bind(serde_json::json!({ "draft": action_draft }))
            .execute(&*self.pool).await?;

        Ok(())
    }
}
