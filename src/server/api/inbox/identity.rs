
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
    let existing_id = match &db.store {
        crate::db::DbStore::Postgres => {
            let pool = &db.pool;
            let query = if source == "whatsapp" || source == "sms" || source == "twilio" {
                "SELECT id FROM customers WHERE tenant_id = $1 AND (phone = $2 OR phone LIKE '%' || $2) LIMIT 1"
            } else if source == "email" {
                "SELECT id FROM customers WHERE tenant_id = $1 AND email = $2 LIMIT 1"
            } else {
                "SELECT id FROM customers WHERE tenant_id = $1 AND (preferences->>'social_handle' = $2 OR preferences->>'instagram_handle' = $2) LIMIT 1"
            };

            sqlx::query(query)
                .bind(tenant_id)
                .bind(sender_id)
                .fetch_optional(pool)
                .await
                .ok()?
                .map(|row| row.get("id"))
        }
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            let query = if source == "whatsapp" || source == "sms" || source == "twilio" {
                "SELECT id FROM customers WHERE tenant_id = ? AND (phone = ? OR phone LIKE '%' || ?) LIMIT 1"
            } else if source == "email" {
                "SELECT id FROM customers WHERE tenant_id = ? AND email = ? LIMIT 1"
            } else {
                "SELECT id FROM customers WHERE tenant_id = ? AND (json_extract(preferences, '$.social_handle') = ? OR json_extract(preferences, '$.instagram_handle') = ?) LIMIT 1"
            };

            let mut db_query = sqlx::query(query).bind(tenant_id).bind(sender_id);
            if source == "whatsapp" || source == "sms" || source == "twilio" || source != "email" {
                 db_query = db_query.bind(sender_id);
            }

            db_query
                .fetch_optional(sqlite_pool)
                .await
                .ok()?
                .map(|row| row.get("id"))
        }
    };

    if existing_id.is_some() {
        return existing_id;
    }

    // Create a new "Lead" customer
    let new_id = uuid::Uuid::new_v4().to_string();
    let name = format!("Lead: {}", sender_id);
    let mut email = None;
    let mut phone = None;
    let mut preferences = serde_json::json!({});

    if source == "email" {
        email = Some(sender_id.to_string());
    } else if source == "whatsapp" || source == "sms" || source == "twilio" {
        phone = Some(sender_id.to_string());
    } else {
        preferences["social_handle"] = serde_json::json!(sender_id);
    }

    match &db.store {
        crate::db::DbStore::Postgres => {
            let pool = &db.pool;
            let _ = sqlx::query(
                "INSERT INTO customers (id, tenant_id, name, email, phone, preferences, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())"
            )
            .bind(&new_id)
            .bind(tenant_id)
            .bind(name)
            .bind(email)
            .bind(phone)
            .bind(preferences)
            .execute(pool)
            .await;
        }
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            let _ = sqlx::query(
                "INSERT INTO customers (id, tenant_id, name, email, phone, preferences, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
            )
            .bind(&new_id)
            .bind(tenant_id)
            .bind(name)
            .bind(email)
            .bind(phone)
            .bind(serde_json::to_string(&preferences).unwrap_or_else(|_| "{}".to_string()))
            .execute(sqlite_pool)
            .await;
        }
    }

    Some(new_id)
}
