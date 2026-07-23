
use crate::db::DB;
use sqlx::Row;

pub async fn resolve_identity(
    db: &DB,
    tenant_id: &str,
    source: &str,
    sender_id: &str,
) -> Option<String> {
    if sender_id.is_empty() || sender_id == "unknown" {
        return None;
    }

    let source = source.to_lowercase();
    let pool = &db.pool;

    // 1. Check if identity exists in customer_identities
    let existing_identity: Option<String> = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query_scalar("SELECT customer_id FROM customer_identities WHERE tenant_id = $1 AND channel = $2 AND channel_identity = $3")
                .bind(tenant_id)
                .bind(&source)
                .bind(sender_id)
                .fetch_optional(pool)
                .await.ok().flatten()
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query_scalar("SELECT customer_id FROM customer_identities WHERE tenant_id = ? AND channel = ? AND channel_identity = ?")
                .bind(tenant_id)
                .bind(&source)
                .bind(sender_id)
                .fetch_optional(sqlite_pool)
                .await.ok().flatten()
        }
    };

    if let Some(id) = existing_identity {
        return Some(id);
    }

    // 2. If not found, try to resolve by phone or email in customers table (basic resolution)
    let potential_customer_id: Option<String> = match &db.store {
        crate::db::DbStore::Postgres => {
            let query = if source == "whatsapp" || source == "sms" || source == "twilio" {
                "SELECT id FROM customers WHERE tenant_id = $1 AND (phone = $2 OR phone LIKE '%' || $2) LIMIT 1"
            } else if source == "email" {
                "SELECT id FROM customers WHERE tenant_id = $1 AND email = $2 LIMIT 1"
            } else {
                "SELECT id FROM customers WHERE tenant_id = $1 AND (preferences->>'social_handle' = $2 OR preferences->>'instagram_handle' = $2) LIMIT 1"
            };

            sqlx::query_scalar(query)
                .bind(tenant_id)
                .bind(sender_id)
                .fetch_optional(pool)
                .await
                .ok().flatten()
        }
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            let query = if source == "whatsapp" || source == "sms" || source == "twilio" {
                "SELECT id FROM customers WHERE tenant_id = ? AND (phone = ? OR phone LIKE '%' || ?) LIMIT 1"
            } else if source == "email" {
                "SELECT id FROM customers WHERE tenant_id = ? AND email = ? LIMIT 1"
            } else {
                "SELECT id FROM customers WHERE tenant_id = ? AND (json_extract(preferences, '$.social_handle') = ? OR json_extract(preferences, '$.instagram_handle') = ?) LIMIT 1"
            };

            let mut db_query = sqlx::query_scalar(query).bind(tenant_id).bind(sender_id);
            if source == "whatsapp" || source == "sms" || source == "twilio" || source != "email" {
                 db_query = db_query.bind(sender_id);
            }
            db_query.fetch_optional(sqlite_pool).await.ok().flatten()
        }
    };

    let id = if let Some(found_id) = potential_customer_id {
        found_id
    } else {
        let new_id = uuid::Uuid::new_v4().to_string();
        let email = if sender_id.contains('@') { sender_id } else { "" };
        let phone = if !sender_id.contains('@') && sender_id.chars().any(|c| c.is_digit(10)) { sender_id } else { "" };
        let name = "Unknown Customer";

        match &db.store {
            crate::db::DbStore::Postgres => {
                let _ = sqlx::query("INSERT INTO customers (id, tenant_id, name, email, phone) VALUES ($1, $2, $3, $4, $5)")
                    .bind(&new_id)
                    .bind(tenant_id)
                    .bind(name)
                    .bind(if email.is_empty() { None } else { Some(email.to_string()) })
                    .bind(if phone.is_empty() { None } else { Some(phone.to_string()) })
                    .execute(pool)
                    .await;
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                let _ = sqlx::query("INSERT INTO customers (id, tenant_id, name, email, phone) VALUES (?, ?, ?, ?, ?)")
                    .bind(&new_id)
                    .bind(tenant_id)
                    .bind(name)
                    .bind(if email.is_empty() { None } else { Some(email.to_string()) })
                    .bind(if phone.is_empty() { None } else { Some(phone.to_string()) })
                    .execute(sqlite_pool)
                    .await;
            }
        };
        new_id
    };

    // Cache this new identity link
    let identity_id = uuid::Uuid::new_v4().to_string();
    match &db.store {
        crate::db::DbStore::Postgres => {
            let _ = sqlx::query("INSERT INTO customer_identities (id, tenant_id, customer_id, channel, channel_identity) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING")
                .bind(&identity_id)
                .bind(tenant_id)
                .bind(&id)
                .bind(&source)
                .bind(sender_id)
                .execute(pool)
                .await;
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            let _ = sqlx::query("INSERT OR IGNORE INTO customer_identities (id, tenant_id, customer_id, channel, channel_identity) VALUES (?, ?, ?, ?, ?)")
                .bind(&identity_id)
                .bind(tenant_id)
                .bind(&id)
                .bind(&source)
                .bind(sender_id)
                .execute(sqlite_pool)
                .await;
        }
    };

    Some(id)
}
