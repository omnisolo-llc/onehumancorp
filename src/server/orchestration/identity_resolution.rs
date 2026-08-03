use std::sync::Arc;
use crate::db::{DB, DbStore};
use uuid::Uuid;

pub struct IdentityResolver {
    db: Arc<DB>,
}

impl IdentityResolver {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    /// Resolves a sender_id (phone, email, handle) to an existing customer ID.
    /// If none exists, optionally creates a stub customer or returns None.
    pub async fn resolve_or_create_customer(&self, tenant_id: &str, sender_id: &str, source: &str) -> Result<String, String> {
        let pool = &self.db.pool;

        // Try to find a customer by email, phone, or preferences (social handle)
        match &self.db.store {
            DbStore::Postgres => {
                let row = sqlx::query(
                    r#"
                    SELECT id FROM customers
                    WHERE tenant_id = $1
                    AND (
                        email = $2 OR
                        phone = $2 OR
                        preferences->>'social_handle' = $2
                    ) LIMIT 1
                    "#
                )
                    .bind(tenant_id)
                    .bind(sender_id)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    use sqlx::Row;
                    let id: String = r.get("id");
                    return Ok(id);
                }
            },
            DbStore::Sqlite(sqlite_pool) => {
                 let row = sqlx::query(
                    r#"
                    SELECT id FROM customers
                    WHERE tenant_id = ?
                    AND (
                        email = ? OR
                        phone = ? OR
                        json_extract(preferences, '$.social_handle') = ?
                    ) LIMIT 1
                    "#
                )
                    .bind(tenant_id)
                    .bind(sender_id)
                    .bind(sender_id)
                    .bind(sender_id)
                    .fetch_optional(sqlite_pool)
                    .await
                    .map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    use sqlx::Row;
                    let id: String = r.get("id");
                    return Ok(id);
                }
            }
        }

        // If not found, create a lightweight customer profile
        let new_id = Uuid::new_v4().to_string();
        let name = format!("Unknown Lead ({})", source);
        let phone = if source == "whatsapp" || source == "sms" { Some(sender_id.to_string()) } else { None };
        let email = if source == "email" { Some(sender_id.to_string()) } else { None };
        let handle = if source == "instagram" || source == "facebook" { Some(sender_id.to_string()) } else { None };
        let preferences = if let Some(h) = handle {
             serde_json::json!({"social_handle": h})
        } else {
             serde_json::json!({})
        };

        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query("INSERT INTO customers (id, tenant_id, name, email, phone, preferences) VALUES ($1, $2, $3, $4, $5, $6)")
                    .bind(&new_id)
                    .bind(tenant_id)
                    .bind(&name)
                    .bind(&email)
                    .bind(&phone)
                    .bind(&preferences)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            },
            DbStore::Sqlite(sqlite_pool) => {
                 sqlx::query("INSERT INTO customers (id, tenant_id, name, email, phone, preferences) VALUES (?, ?, ?, ?, ?, ?)")
                    .bind(&new_id)
                    .bind(tenant_id)
                    .bind(&name)
                    .bind(&email)
                    .bind(&phone)
                    .bind(&preferences)
                    .execute(sqlite_pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }

        Ok(new_id)
    }
}
