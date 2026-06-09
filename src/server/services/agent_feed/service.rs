use std::sync::Arc;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentFeedCard {
    pub id: String,
    pub tenant_id: String,
    pub agent_type: String,
    pub card_type: String,
    pub title: String,
    pub description: Option<String>,
    pub proposed_action_payload: serde_json::Value,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
#[derive(Debug, Deserialize)]
pub struct CreateCardRequest {
    pub agent_type: String,
    pub card_type: String,
    pub title: String,
    pub description: Option<String>,
    pub proposed_action_payload: serde_json::Value,
}

#[derive(Serialize)]
#[derive(Debug, Deserialize)]
pub struct ResolveCardRequest {
    pub status: String, // e.g., "Approved", "Dismissed", "Edited"
}

pub struct AgentFeedService {
    db: Arc<crate::db::DB>,
}

impl AgentFeedService {
    pub fn new(db: Arc<crate::db::DB>) -> Self {
        Self { db }
    }

    pub async fn create_card(
        &self,
        tenant_id: &str,
        req: CreateCardRequest,
    ) -> Result<AgentFeedCard, String> {
        let id = uuid::Uuid::new_v4().to_string();
        let query = r#"
            INSERT INTO agent_feed_cards (
                id, tenant_id, agent_type, card_type, title, description, proposed_action_payload, status
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, 'Pending')
            RETURNING *
        "#;

        let row = sqlx::query_as::<_, AgentFeedCard>(query)
            .bind(&id)
            .bind(tenant_id)
            .bind(&req.agent_type)
            .bind(&req.card_type)
            .bind(&req.title)
            .bind(&req.description)
            .bind(&req.proposed_action_payload)
            .fetch_one(&self.db.pool)
            .await
            .map_err(|e| format!("Failed to create agent feed card: {}", e))?;

        Ok(row)
    }

    pub async fn list_pending_cards(&self, tenant_id: &str) -> Result<Vec<AgentFeedCard>, String> {
        let query = r#"
            SELECT * FROM agent_feed_cards
            WHERE tenant_id = $1 AND status = 'Pending'
            ORDER BY created_at DESC
        "#;

        let rows = sqlx::query_as::<_, AgentFeedCard>(query)
            .bind(tenant_id)
            .fetch_all(&self.db.pool)
            .await
            .map_err(|e| format!("Failed to list pending agent feed cards: {}", e))?;

        Ok(rows)
    }

    pub async fn resolve_card(
        &self,
        tenant_id: &str,
        card_id: &str,
        req: ResolveCardRequest,
    ) -> Result<AgentFeedCard, String> {
        let query = r#"
            UPDATE agent_feed_cards
            SET status = $1, updated_at = CURRENT_TIMESTAMP
            WHERE id = $2 AND tenant_id = $3
            RETURNING *
        "#;

        let row = sqlx::query_as::<_, AgentFeedCard>(query)
            .bind(&req.status)
            .bind(card_id)
            .bind(tenant_id)
            .fetch_one(&self.db.pool)
            .await
            .map_err(|e| format!("Failed to resolve agent feed card: {}", e))?;

        Ok(row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    async fn setup_db() -> Arc<crate::db::DB> {
        let database_url = std::env::var("OHC_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());

        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(&database_url)
            .await
            .unwrap();

        // Run migrations
        sqlx::query("CREATE TABLE IF NOT EXISTS agent_feed_cards (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            agent_type TEXT NOT NULL,
            card_type TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            proposed_action_payload JSONB DEFAULT '{}'::jsonb,
            status TEXT NOT NULL DEFAULT 'Pending',
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        )").execute(&pool).await.unwrap();

        Arc::new(crate::db::DB { pool: pool, store: crate::db::DbStore::Postgres })
    }

    #[tokio::test]
    async fn test_create_and_list_cards() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        let db = setup_db().await;
        let service = AgentFeedService::new(db);
        let tenant_id = "test-tenant-1";

        let req = CreateCardRequest {
            agent_type: "The Ambassador".to_string(),
            card_type: "Actionable".to_string(),
            title: "Reply to Customer".to_string(),
            description: Some("Customer asked about Vegan Cake.".to_string()),
            proposed_action_payload: serde_json::json!({"action": "send_dm"}),
        };

        let created = service.create_card(tenant_id, req).await.unwrap();
        assert_eq!(created.agent_type, "The Ambassador");
        assert_eq!(created.status, "Pending");

        let pending_cards = service.list_pending_cards(tenant_id).await.unwrap();
        assert!(pending_cards.iter().any(|c| c.id == created.id));
    }

    #[tokio::test]
    async fn test_resolve_card() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        let db = setup_db().await;
        let service = AgentFeedService::new(db);
        let tenant_id = "test-tenant-2";

        let req = CreateCardRequest {
            agent_type: "The Promoter".to_string(),
            card_type: "Actionable".to_string(),
            title: "Post to Instagram".to_string(),
            description: None,
            proposed_action_payload: serde_json::json!({"action": "post"}),
        };

        let created = service.create_card(tenant_id, req).await.unwrap();

        let resolve_req = ResolveCardRequest {
            status: "Approved".to_string(),
        };

        let resolved = service.resolve_card(tenant_id, &created.id, resolve_req).await.unwrap();
        assert_eq!(resolved.status, "Approved");

        let pending_cards = service.list_pending_cards(tenant_id).await.unwrap();
        assert!(!pending_cards.iter().any(|c| c.id == resolved.id));
    }
}
