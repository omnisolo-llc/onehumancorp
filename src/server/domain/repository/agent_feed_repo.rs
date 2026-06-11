use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::db::{DB, DbStore};

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
    db: std::sync::Arc<DB>,
}

impl AgentFeedRepository {
    pub fn new(db: std::sync::Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn create(&self, item: AgentFeedItem) -> Result<AgentFeedItem, sqlx::Error> {
        match &self.db.store {
            DbStore::Postgres => {
                let rec = sqlx::query_as::<_, AgentFeedItem>(
                    r#"
                    INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                    RETURNING *
                    "#
                )
                .bind(&item.id)
                .bind(&item.tenant_id)
                .bind(&item.event_source)
                .bind(&item.context_payload)
                .bind(&item.proposed_action)
                .bind(&item.lifecycle_state)
                .bind(&item.created_at)
                .bind(&item.updated_at)
                .fetch_one(&self.db.pool)
                .await?;
                Ok(rec)
            }
            DbStore::Sqlite(pool) => {
                let rec = sqlx::query_as::<_, AgentFeedItem>(
                    r#"
                    INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                    RETURNING *
                    "#
                )
                .bind(&item.id)
                .bind(&item.tenant_id)
                .bind(&item.event_source)
                .bind(&item.context_payload)
                .bind(&item.proposed_action)
                .bind(&item.lifecycle_state)
                .bind(&item.created_at)
                .bind(&item.updated_at)
                .fetch_one(pool)
                .await?;
                Ok(rec)
            }
        }
    }

    pub async fn get(&self, tenant_id: &str, id: &str) -> Result<Option<AgentFeedItem>, sqlx::Error> {
        match &self.db.store {
            DbStore::Postgres => {
                let rec = sqlx::query_as::<_, AgentFeedItem>(
                    "SELECT * FROM agent_feed_items WHERE tenant_id = $1 AND id = $2"
                )
                .bind(tenant_id)
                .bind(id)
                .fetch_optional(&self.db.pool)
                .await?;
                Ok(rec)
            }
            DbStore::Sqlite(pool) => {
                let rec = sqlx::query_as::<_, AgentFeedItem>(
                    "SELECT * FROM agent_feed_items WHERE tenant_id = ? AND id = ?"
                )
                .bind(tenant_id)
                .bind(id)
                .fetch_optional(pool)
                .await?;
                Ok(rec)
            }
        }
    }

    pub async fn list(&self, tenant_id: &str, limit: i64, offset: i64) -> Result<Vec<AgentFeedItem>, sqlx::Error> {
        match &self.db.store {
            DbStore::Postgres => {
                let items = sqlx::query_as::<_, AgentFeedItem>(
                    "SELECT * FROM agent_feed_items WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
                )
                .bind(tenant_id)
                .bind(limit)
                .bind(offset)
                .fetch_all(&self.db.pool)
                .await?;
                Ok(items)
            }
            DbStore::Sqlite(pool) => {
                let items = sqlx::query_as::<_, AgentFeedItem>(
                    "SELECT * FROM agent_feed_items WHERE tenant_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
                )
                .bind(tenant_id)
                .bind(limit)
                .bind(offset)
                .fetch_all(pool)
                .await?;
                Ok(items)
            }
        }
    }

    pub async fn update_state(&self, tenant_id: &str, id: &str, new_state: &str) -> Result<AgentFeedItem, sqlx::Error> {
        match &self.db.store {
            DbStore::Postgres => {
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
                .fetch_one(&self.db.pool)
                .await?;
                Ok(rec)
            }
            DbStore::Sqlite(pool) => {
                let rec = sqlx::query_as::<_, AgentFeedItem>(
                    r#"
                    UPDATE agent_feed_items
                    SET lifecycle_state = ?, updated_at = CURRENT_TIMESTAMP
                    WHERE tenant_id = ? AND id = ?
                    RETURNING *
                    "#
                )
                .bind(new_state)
                .bind(tenant_id)
                .bind(id)
                .fetch_one(pool)
                .await?;
                Ok(rec)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::repository::agent_feed_repo::{AgentFeedRepository, AgentFeedItem};
    use chrono::Utc;
    use sqlx::PgPool;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_agent_feed_repo_lifecycle() {
        let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS agent_feed_items (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                event_source TEXT NOT NULL,
                context_payload JSON,
                proposed_action JSON,
                lifecycle_state TEXT NOT NULL,
                created_at DATETIME,
                updated_at DATETIME
            )"
        )
        .execute(&sqlite_pool)
        .await
        .unwrap();

        let dummy_pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
            .unwrap();

        let db = std::sync::Arc::new(crate::db::DB { pool: dummy_pool, store: crate::db::DbStore::Sqlite(sqlite_pool) });
        let repo = AgentFeedRepository::new(db);

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
