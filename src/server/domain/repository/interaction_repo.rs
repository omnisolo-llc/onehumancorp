use std::sync::Arc;
use crate::db::{DB, DbStore};
use super::models::Interaction;

pub struct InteractionRepository {
    db: Arc<DB>,
}

impl InteractionRepository {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn create_interaction(&self, interaction: Interaction) -> Result<Interaction, String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    r#"
                    INSERT INTO interactions (
                        id, tenant_id, customer_id, channel, content, metadata, created_at, updated_at
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                    "#
                )
                .bind(&interaction.id)
                .bind(&interaction.tenant_id)
                .bind(&interaction.customer_id)
                .bind(&interaction.channel)
                .bind(&interaction.content)
                .bind(&interaction.metadata)
                .bind(&interaction.created_at)
                .bind(&interaction.updated_at)
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                let metadata = interaction.metadata.as_ref().map(|m| m.to_string());
                sqlx::query(
                    r#"
                    INSERT INTO interactions (
                        id, tenant_id, customer_id, channel, content, metadata, created_at, updated_at
                    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(&interaction.id)
                .bind(&interaction.tenant_id)
                .bind(&interaction.customer_id)
                .bind(&interaction.channel)
                .bind(&interaction.content)
                .bind(metadata)
                .bind(&interaction.created_at)
                .bind(&interaction.updated_at)
                .execute(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }
        Ok(interaction)
    }

    pub async fn get_interactions_by_tenant(&self, tenant_id: &str) -> Result<Vec<Interaction>, String> {
        let interactions = match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_as::<_, Interaction>(
                    r#"
                    SELECT id, tenant_id, customer_id, channel, content, metadata, created_at, updated_at
                    FROM interactions
                    WHERE tenant_id = $1
                    "#
                )
                .bind(tenant_id)
                .fetch_all(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as::<_, Interaction>(
                    r#"
                    SELECT id, tenant_id, customer_id, channel, content, metadata, created_at, updated_at
                    FROM interactions
                    WHERE tenant_id = ?
                    "#
                )
                .bind(tenant_id)
                .fetch_all(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?
            }
        };
        Ok(interactions)
    }
}
