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
    pub correlation_id: Option<String>,
    pub priority_score: Option<i32>,
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
            INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at, correlation_id, priority_score)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
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
        .bind(item.correlation_id)
        .bind(item.priority_score)
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn update(&self, item: AgentFeedItem) -> Result<AgentFeedItem, sqlx::Error> {
        let rec = sqlx::query_as::<_, AgentFeedItem>(
            r#"
            UPDATE agent_feed_items
            SET event_source = $3, context_payload = $4, proposed_action = $5, lifecycle_state = $6, updated_at = NOW(), correlation_id = $7, priority_score = $8
            WHERE id = $1 AND tenant_id = $2
            RETURNING *
            "#
        )
        .bind(item.id)
        .bind(item.tenant_id)
        .bind(item.event_source)
        .bind(item.context_payload)
        .bind(item.proposed_action)
        .bind(item.lifecycle_state)
        .bind(item.correlation_id)
        .bind(item.priority_score)
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn get_pending_by_correlation_id(&self, tenant_id: &str, correlation_id: &str) -> Result<Option<AgentFeedItem>, sqlx::Error> {
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
                updated_at,
                correlation_id,
                priority_score
            FROM agent_feed_items
            WHERE tenant_id = $1 AND correlation_id = $2 AND lifecycle_state = 'PENDING_APPROVAL'
            ORDER BY created_at DESC
            LIMIT 1
            "#
        )
        .bind(tenant_id)
        .bind(correlation_id)
        .fetch_optional(&self.pool)
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
                updated_at,
                correlation_id,
                priority_score
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
                updated_at,
                NULL as correlation_id,
                NULL as priority_score
            FROM agent_approvals
            WHERE tenant_id = $1 AND id = $2
            "#
        )
        .bind(tenant_id)
        .bind(id)
        .fetch_optional(&self.pool)
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
                NULL::jsonb as context_payload,
                NULL::jsonb as proposed_action,
                lifecycle_state,
                created_at,
                updated_at,
                correlation_id,
                priority_score
            FROM agent_feed_items
            WHERE tenant_id = $1

            UNION ALL

            SELECT
                id,
                tenant_id,
                department as event_source,
                NULL::jsonb as context_payload,
                NULL::jsonb as proposed_action,
                CASE
                    WHEN status = 'DRAFT' THEN 'PENDING_APPROVAL'
                    WHEN status = 'REJECTED' THEN 'DISMISSED'
                    ELSE status
                END as lifecycle_state,
                created_at,
                updated_at,
                NULL as correlation_id,
                NULL as priority_score
            FROM agent_approvals
            WHERE tenant_id = $1 AND status IN ('DRAFT', 'PAUSED', 'APPROVED', 'REJECTED', 'DISMISSED')

            ORDER BY COALESCE(priority_score, 0) DESC, created_at DESC
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
                updated_at,
                correlation_id,
                priority_score
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
                updated_at,
                NULL as correlation_id,
                NULL as priority_score
            FROM agent_approvals
            WHERE tenant_id = $1 AND status IN ('DRAFT', 'PAUSED', 'APPROVED', 'REJECTED', 'DISMISSED')

            ORDER BY COALESCE(priority_score, 0) DESC, created_at DESC
            LIMIT $2 OFFSET $3
            "#
        };

        let items = sqlx::query_as::<_, AgentFeedItem>(query)
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
        .fetch_optional(&self.pool)
        .await?;

        if let Some(r) = rec {
            return Ok(r);
        }

        // Fallback to agent_approvals
        let legacy_status = if new_state == "APPROVED" { "APPROVED" } else if new_state == "DISMISSED" { "REJECTED" } else { "DRAFT" };
        sqlx::query("UPDATE agent_approvals SET status = $1, updated_at = NOW() WHERE tenant_id = $2 AND id = $3")
            .bind(legacy_status)
            .bind(tenant_id)
            .bind(id)
            .execute(&self.pool)
            .await?;

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
        .execute(&self.pool)
        .await?;

        if res.rows_affected() > 0 {
            return Ok(());
        }

        // Fallback for agent_approvals
        if let Some(action) = proposed_action {
            sqlx::query(
                r#"
                UPDATE agent_approvals
                SET payload = $1, updated_at = NOW()
                WHERE tenant_id = $2 AND id = $3
                "#
            )
            .bind(action)
            .bind(tenant_id)
            .bind(id)
            .execute(&self.pool)
            .await?;
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
            correlation_id: Some("test_correlation".to_string()),
            priority_score: Some(10),
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
        assert!(mobile_item.proposed_action.is_none());
    }
}
