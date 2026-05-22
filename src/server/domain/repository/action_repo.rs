use std::sync::Arc;
use crate::db::{DB, DbStore};
use super::models::AgentAction;

pub struct ActionRepository {
    db: Arc<DB>,
}

impl ActionRepository {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn create_action(&self, action: AgentAction) -> Result<AgentAction, String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    r#"
                    INSERT INTO agent_actions (
                        id, tenant_id, agent_id, interaction_id, action_type, payload, created_at
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7)
                    "#
                )
                .bind(&action.id)
                .bind(&action.tenant_id)
                .bind(&action.agent_id)
                .bind(&action.interaction_id)
                .bind(&action.action_type)
                .bind(&action.payload)
                .bind(&action.created_at)
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                let payload = action.payload.as_ref().map(|p| p.to_string());
                sqlx::query(
                    r#"
                    INSERT INTO agent_actions (
                        id, tenant_id, agent_id, interaction_id, action_type, payload, created_at
                    ) VALUES (?, ?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(&action.id)
                .bind(&action.tenant_id)
                .bind(&action.agent_id)
                .bind(&action.interaction_id)
                .bind(&action.action_type)
                .bind(payload)
                .bind(&action.created_at)
                .execute(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }
        Ok(action)
    }

    pub async fn get_actions_by_tenant(&self, tenant_id: &str) -> Result<Vec<AgentAction>, String> {
        let actions = match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_as::<_, AgentAction>(
                    r#"
                    SELECT id, tenant_id, agent_id, interaction_id, action_type, payload, created_at
                    FROM agent_actions
                    WHERE tenant_id = $1
                    "#
                )
                .bind(tenant_id)
                .fetch_all(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as::<_, AgentAction>(
                    r#"
                    SELECT id, tenant_id, agent_id, interaction_id, action_type, payload, created_at
                    FROM agent_actions
                    WHERE tenant_id = ?
                    "#
                )
                .bind(tenant_id)
                .fetch_all(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?
            }
        };
        Ok(actions)
    }
}
