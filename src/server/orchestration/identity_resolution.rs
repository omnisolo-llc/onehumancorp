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

        if sender_id.is_empty() || sender_id == "unknown" {
            // Cannot reliably resolve without a sender id, let's just make one
            let new_id = Uuid::new_v4().to_string();
            return Ok(new_id);
        }

        // Try to find a customer by email, phone, or preferences (social handle), OR check customer_identities
        match &self.db.store {
            DbStore::Postgres => {
                let row = sqlx::query(
                    r#"
                    SELECT c.id FROM customers c
                    LEFT JOIN customer_identities ci ON c.id::text = ci.customer_id AND ci.tenant_id = c.tenant_id
                    WHERE c.tenant_id = $1
                    AND (
                        c.email = $2 OR
                        c.phone = $2 OR
                        c.phone LIKE '%' || $2 OR
                        c.preferences->>'social_handle' = $2 OR
                        c.preferences->>'instagram_handle' = $2 OR
                        ci.channel_identity = $2
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
                    SELECT c.id FROM customers c
                    LEFT JOIN customer_identities ci ON c.id = ci.customer_id AND ci.tenant_id = c.tenant_id
                    WHERE c.tenant_id = ?
                    AND (
                        c.email = ? OR
                        c.phone = ? OR
                        c.phone LIKE '%' || ? OR
                        json_extract(c.preferences, '$.social_handle') = ? OR
                        json_extract(c.preferences, '$.instagram_handle') = ? OR
                        ci.channel_identity = ?
                    ) LIMIT 1
                    "#
                )
                    .bind(tenant_id)
                    .bind(sender_id)
                    .bind(sender_id)
                    .bind(sender_id)
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
        let preferences = if let Some(ref h) = handle {
             serde_json::json!({"social_handle": h})
        } else {
             serde_json::json!({})
        };

        let identity_id = Uuid::new_v4().to_string();

        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
                sqlx::query("INSERT INTO customers (id, tenant_id, name, email, phone, preferences) VALUES ($1, $2, $3, $4, $5, $6)")
                    .bind(&new_id)
                    .bind(tenant_id)
                    .bind(&name)
                    .bind(&email)
                    .bind(&phone)
                    .bind(&preferences)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                sqlx::query("INSERT INTO customer_identities (id, tenant_id, customer_id, channel, channel_identity) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING")
                    .bind(&identity_id)
                    .bind(tenant_id)
                    .bind(&new_id)
                    .bind(source)
                    .bind(sender_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
                tx.commit().await.map_err(|e| e.to_string())?;
            },
            DbStore::Sqlite(sqlite_pool) => {
                 let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;
                 sqlx::query("INSERT INTO customers (id, tenant_id, name, email, phone, preferences) VALUES (?, ?, ?, ?, ?, ?)")
                    .bind(&new_id)
                    .bind(tenant_id)
                    .bind(&name)
                    .bind(&email)
                    .bind(&phone)
                    .bind(&preferences)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                 sqlx::query("INSERT INTO customer_identities (id, tenant_id, customer_id, channel, channel_identity) VALUES (?, ?, ?, ?, ?) ON CONFLICT DO NOTHING")
                    .bind(&identity_id)
                    .bind(tenant_id)
                    .bind(&new_id)
                    .bind(source)
                    .bind(sender_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
                 tx.commit().await.map_err(|e| e.to_string())?;
            }
        }

        Ok(new_id)
    }
}
