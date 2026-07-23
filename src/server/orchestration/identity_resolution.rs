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

        // 1. Check if identity exists in customer_identities
        let existing_identity: Option<String> = match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_scalar("SELECT customer_id FROM customer_identities WHERE tenant_id = $1 AND channel = $2 AND channel_identity = $3")
                    .bind(tenant_id)
                    .bind(source)
                    .bind(sender_id)
                    .fetch_optional(pool)
                    .await.ok().flatten()
            },
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_scalar("SELECT customer_id FROM customer_identities WHERE tenant_id = ? AND channel = ? AND channel_identity = ?")
                    .bind(tenant_id)
                    .bind(source)
                    .bind(sender_id)
                    .fetch_optional(sqlite_pool)
                    .await.ok().flatten()
            }
        };

        if let Some(id) = existing_identity {
            return Ok(id);
        }

        // 2. Try to find a customer by email, phone, or preferences (social handle)
        let potential_customer_id: Option<String> = match &self.db.store {
            DbStore::Postgres => {
                let row = sqlx::query(
                    r#"
                    SELECT id FROM customers
                    WHERE tenant_id = $1
                    AND (
                        email = $2 OR
                        phone = $2 OR
                        preferences->>'social_handle' = $2 OR
                        preferences->>'instagram_handle' = $2
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
                    Some(r.get("id"))
                } else {
                    None
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
                        json_extract(preferences, '$.social_handle') = ? OR
                        json_extract(preferences, '$.instagram_handle') = ?
                    ) LIMIT 1
                    "#
                )
                    .bind(tenant_id)
                    .bind(sender_id)
                    .bind(sender_id)
                    .bind(sender_id)
                    .bind(sender_id)
                    .fetch_optional(sqlite_pool)
                    .await
                    .map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    use sqlx::Row;
                    Some(r.get("id"))
                } else {
                    None
                }
            }
        };

        let customer_id = if let Some(found_id) = potential_customer_id {
            found_id
        } else {
            // If not found, create a lightweight customer profile
            let new_id = Uuid::new_v4().to_string();
            let name = format!("Unknown Lead ({})", source);
            let phone = if source == "whatsapp" || source == "sms" || source == "twilio" || source == "voice" { Some(sender_id.to_string()) } else { None };
            let email = if source == "email" { Some(sender_id.to_string()) } else { None };
            let handle = if source == "instagram" || source == "facebook" || source == "instagram_dm" { Some(sender_id.to_string()) } else { None };

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
            };

            new_id
        };

        // 3. Cache this new identity link in customer_identities
        let identity_id = Uuid::new_v4().to_string();
        match &self.db.store {
            DbStore::Postgres => {
                let _ = sqlx::query("INSERT INTO customer_identities (id, tenant_id, customer_id, channel, channel_identity) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING")
                    .bind(&identity_id)
                    .bind(tenant_id)
                    .bind(&customer_id)
                    .bind(source)
                    .bind(sender_id)
                    .execute(pool)
                    .await;
            },
            DbStore::Sqlite(sqlite_pool) => {
                let _ = sqlx::query("INSERT OR IGNORE INTO customer_identities (id, tenant_id, customer_id, channel, channel_identity) VALUES (?, ?, ?, ?, ?)")
                    .bind(&identity_id)
                    .bind(tenant_id)
                    .bind(&customer_id)
                    .bind(source)
                    .bind(sender_id)
                    .execute(sqlite_pool)
                    .await;
            }
        };

        Ok(customer_id)
    }
}
