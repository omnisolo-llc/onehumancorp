use sqlx::{PgPool, FromRow};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AgentFeedItem {
    pub id: String,
    pub tenant_id: String,
    pub event_source: String,
    pub context_payload: Option<sqlx::types::Json<serde_json::Value>>,
    pub proposed_action: Option<sqlx::types::Json<serde_json::Value>>,
    pub lifecycle_state: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct AgentFeedRepository {
    pool: PgPool,
}

impl AgentFeedRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, item: AgentFeedItem) -> Result<AgentFeedItem, sqlx::Error> {
        let rec = sqlx::query_as::<_, AgentFeedItem>(
            r#"
            INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#
        )
        .bind(item.id)
        .bind(item.tenant_id)
        .bind(item.event_source)
        .bind(item.context_payload)
        .bind(item.proposed_action)
        .bind(item.lifecycle_state)
        .bind(item.created_at)
        .bind(item.updated_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn get(&self, tenant_id: &str, id: &str) -> Result<Option<AgentFeedItem>, sqlx::Error> {
        let rec = sqlx::query_as::<_, AgentFeedItem>(
            "SELECT * FROM agent_feed_items WHERE tenant_id = $1 AND id = $2"
        )
        .bind(tenant_id)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn list(&self, tenant_id: &str, limit: i64, offset: i64) -> Result<Vec<AgentFeedItem>, sqlx::Error> {
        let items = sqlx::query_as::<_, AgentFeedItem>(
            "SELECT * FROM agent_feed_items WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(tenant_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(items)
    }

    pub async fn update_state(&self, tenant_id: &str, id: &str, new_state: &str) -> Result<AgentFeedItem, sqlx::Error> {
        let rec = sqlx::query_as::<_, AgentFeedItem>(
            r#"
            UPDATE agent_feed_items
            SET lifecycle_state = $1, updated_at = NOW()
            WHERE tenant_id = $2 AND id = $3
            RETURNING *
            "#
        )
        .bind(new_state)
        .bind(tenant_id)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::repository::agent_feed_repo::{AgentFeedRepository, AgentFeedItem};
    use chrono::Utc;
    use sqlx::PgPool;
    use uuid::Uuid;

    #[tokio::test]
    #[ignore] // Integration test requiring database
    async fn test_agent_feed_repo_lifecycle() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool = PgPool::connect(&database_url).await.unwrap();
        let repo = AgentFeedRepository::new(pool);

        let tenant_id = "test-tenant-123";

        // 1. Create an item
        let new_item = AgentFeedItem {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            event_source: "test_source".to_string(),
            context_payload: Some(sqlx::types::Json(serde_json::json!({"test": "data"}))),
            proposed_action: Some(sqlx::types::Json(serde_json::json!({"action": "test"}))),
            lifecycle_state: "PENDING".to_string(),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };

        let created = repo.create(new_item.clone()).await.expect("Failed to create feed item");
        assert_eq!(created.id, new_item.id);
        assert_eq!(created.lifecycle_state, "PENDING");

        // 2. Get the item
        let fetched = repo.get(tenant_id, &new_item.id).await.expect("Failed to get feed item").expect("Item not found");
        assert_eq!(fetched.id, new_item.id);

        // 3. Update the state
        let updated = repo.update_state(tenant_id, &new_item.id, "APPROVED").await.expect("Failed to update state");
        assert_eq!(updated.lifecycle_state, "APPROVED");

        // 4. List items
        let list = repo.list(tenant_id, 10, 0).await.expect("Failed to list feed items");
        assert!(!list.is_empty());
        assert!(list.iter().any(|i| i.id == new_item.id));
    }
}
