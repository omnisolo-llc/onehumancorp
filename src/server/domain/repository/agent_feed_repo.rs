use sqlx::FromRow;
use std::sync::Arc;
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
    db: Arc<crate::db::DB>,
}

impl AgentFeedRepository {
    pub fn new(db: Arc<crate::db::DB>) -> Self {
        Self { db }
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
        .fetch_one(&self.db.pool)
        .await?;

        Ok(rec)
    }

    pub async fn get(&self, tenant_id: &str, id: &str) -> Result<Option<AgentFeedItem>, sqlx::Error> {
        let rec = sqlx::query_as::<_, AgentFeedItem>(
            r#"
            SELECT
                id,
                tenant_id,
                event_source,
                context_payload,
                proposed_action,
                lifecycle_state,
                created_at,
                updated_at
            FROM agent_feed_items
            WHERE tenant_id = $1 AND id = $2

            UNION ALL

            SELECT
                id,
                tenant_id,
                department as event_source,
                jsonb_build_object('description', description) as context_payload,
                payload as proposed_action,
                CASE
                    WHEN status = 'DRAFT' THEN 'PENDING_APPROVAL'
                    WHEN status = 'REJECTED' THEN 'DISMISSED'
                    ELSE status
                END as lifecycle_state,
                created_at,
                updated_at
            FROM agent_approvals
            WHERE tenant_id = $1 AND id = $2

            UNION ALL

            SELECT
                id,
                tenant_id,
                COALESCE(agent_type, 'operations') as event_source,
                jsonb_build_object('description', 'Action Request: ' || action_type) as context_payload,
                payload as proposed_action,
                CASE
                    WHEN status = 'Pending' THEN 'PENDING_APPROVAL'
                    WHEN status = 'Rejected' THEN 'DISMISSED'
                    ELSE status
                END as lifecycle_state,
                created_at,
                updated_at
            FROM agent_action_requests
            WHERE tenant_id = $1 AND id = $2
            "#
        )
        .bind(tenant_id)
        .bind(id)
        .fetch_optional(&self.db.pool)
        .await?;

        Ok(rec)
    }

    pub async fn list(&self, tenant_id: &str, limit: i64, offset: i64, mobile_optimized: bool) -> Result<Vec<AgentFeedItem>, sqlx::Error> {
        let query = if mobile_optimized {
            r#"
            SELECT
                id,
                tenant_id,
                event_source,
                NULL as context_payload,
                proposed_action,
                lifecycle_state,
                created_at,
                updated_at
            FROM agent_feed_items
            WHERE tenant_id = $1

            UNION ALL

            SELECT
                id,
                tenant_id,
                department as event_source,
                NULL as context_payload,
                payload as proposed_action,
                CASE
                    WHEN status = 'DRAFT' THEN 'PENDING_APPROVAL'
                    WHEN status = 'REJECTED' THEN 'DISMISSED'
                    ELSE status
                END as lifecycle_state,
                created_at,
                updated_at
            FROM agent_approvals
            WHERE tenant_id = $1 AND status IN ('DRAFT', 'PAUSED', 'APPROVED', 'REJECTED', 'DISMISSED')

            UNION ALL

            SELECT
                id,
                tenant_id,
                COALESCE(agent_type, 'operations') as event_source,
                NULL as context_payload,
                payload as proposed_action,
                CASE
                    WHEN status = 'Pending' THEN 'PENDING_APPROVAL'
                    WHEN status = 'Rejected' THEN 'DISMISSED'
                    ELSE status
                END as lifecycle_state,
                created_at,
                updated_at
            FROM agent_action_requests
            WHERE tenant_id = $1 AND status IN ('Pending', 'Approved', 'Rejected')

            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#
        } else {
            r#"
            SELECT
                id,
                tenant_id,
                event_source,
                context_payload,
                proposed_action,
                lifecycle_state,
                created_at,
                updated_at
            FROM agent_feed_items
            WHERE tenant_id = $1

            UNION ALL

            SELECT
                id,
                tenant_id,
                department as event_source,
                jsonb_build_object('description', description) as context_payload,
                payload as proposed_action,
                CASE
                    WHEN status = 'DRAFT' THEN 'PENDING_APPROVAL'
                    WHEN status = 'REJECTED' THEN 'DISMISSED'
                    ELSE status
                END as lifecycle_state,
                created_at,
                updated_at
            FROM agent_approvals
            WHERE tenant_id = $1 AND status IN ('DRAFT', 'PAUSED', 'APPROVED', 'REJECTED', 'DISMISSED')

            UNION ALL

            SELECT
                id,
                tenant_id,
                COALESCE(agent_type, 'operations') as event_source,
                jsonb_build_object('description', 'Action Request: ' || action_type) as context_payload,
                payload as proposed_action,
                CASE
                    WHEN status = 'Pending' THEN 'PENDING_APPROVAL'
                    WHEN status = 'Rejected' THEN 'DISMISSED'
                    ELSE status
                END as lifecycle_state,
                created_at,
                updated_at
            FROM agent_action_requests
            WHERE tenant_id = $1 AND status IN ('Pending', 'Approved', 'Rejected')

            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#
        };

        let items = sqlx::query_as::<_, AgentFeedItem>(query)
        .bind(tenant_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.db.pool)
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
        .fetch_optional(&self.db.pool)
        .await?;

        if let Some(r) = rec {
            return Ok(r);
        }

        // Fallback to agent_approvals
        let legacy_status = if new_state == "APPROVED" { "APPROVED" } else if new_state == "DISMISSED" { "REJECTED" } else { "DRAFT" };
        let rows_affected = sqlx::query("UPDATE agent_approvals SET status = $1, updated_at = NOW() WHERE tenant_id = $2 AND id = $3")
            .bind(legacy_status)
            .bind(tenant_id)
            .bind(id)
            .execute(&self.db.pool)
            .await?
            .rows_affected();

        if rows_affected == 0 {
             // Fallback to agent_action_requests
             let request_status = if new_state == "APPROVED" { "Approved" } else if new_state == "DISMISSED" { "Rejected" } else { "Pending" };
             sqlx::query("UPDATE agent_action_requests SET status = $1, updated_at = NOW() WHERE tenant_id = $2 AND id = $3")
                 .bind(request_status)
                 .bind(tenant_id)
                 .bind(id)
                 .execute(&self.db.pool)
                 .await?;
        }

        let fetched = self.get(tenant_id, id).await?;
        if let Some(f) = fetched {
            return Ok(f);
        }

        Err(sqlx::Error::RowNotFound)
    }

    pub async fn update_payloads(&self, tenant_id: &str, id: &str, context_payload: Option<sqlx::types::Json<serde_json::Value>>, proposed_action: Option<sqlx::types::Json<serde_json::Value>>) -> Result<(), sqlx::Error> {
        let res = sqlx::query(
            r#"
            UPDATE agent_feed_items
            SET context_payload = $1, proposed_action = $2, updated_at = NOW()
            WHERE tenant_id = $3 AND id = $4
            "#
        )
        .bind(&context_payload)
        .bind(&proposed_action)
        .bind(tenant_id)
        .bind(id)
        .execute(&self.db.pool)
        .await?;

        if res.rows_affected() > 0 {
            return Ok(());
        }

        // Fallback for agent_approvals
        if let Some(action) = &proposed_action {
            let rows_affected = sqlx::query(
                r#"
                UPDATE agent_approvals
                SET payload = $1, updated_at = NOW()
                WHERE tenant_id = $2 AND id = $3
                "#
            )
            .bind(action)
            .bind(tenant_id)
            .bind(id)
            .execute(&self.db.pool)
            .await?
            .rows_affected();

            if rows_affected == 0 {
                // Fallback for agent_action_requests
                sqlx::query(
                    r#"
                    UPDATE agent_action_requests
                    SET payload = $1, updated_at = NOW()
                    WHERE tenant_id = $2 AND id = $3
                    "#
                )
                .bind(action)
                .bind(tenant_id)
                .bind(id)
                .execute(&self.db.pool)
                .await?;
            }
        }

        Ok(())
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
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        if std::env::var("OHC_DATABASE_URL").is_err() { return; }
        let pool = PgPool::connect(&database_url).await.unwrap();
        let repo = AgentFeedRepository::new(std::sync::Arc::new(crate::db::DB { pool: pool.clone(), store: crate::db::DbStore::Postgres }));

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
        let list = repo.list(tenant_id, 10, 0, false).await.expect("Failed to list feed items");
        assert!(!list.is_empty());
        assert!(list.iter().any(|i| i.id == new_item.id));

        // 5. List items with mobile_optimized = true
        let list_mobile = repo.list(tenant_id, 10, 0, true).await.expect("Failed to list feed items (mobile)");
        assert!(!list_mobile.is_empty());
        let mobile_item = list_mobile.iter().find(|i| i.id == new_item.id).unwrap();
        assert!(mobile_item.context_payload.is_none());
        // mobile optimized feed should include proposed action for cards
        assert!(mobile_item.proposed_action.is_some());
    }
}
