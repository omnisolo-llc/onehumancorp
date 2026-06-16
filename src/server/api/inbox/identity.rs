
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
    match &db.store {
        crate::db::DbStore::Postgres => {
            let pool = &db.pool;
            let query = if source == "whatsapp" || source == "sms" || source == "twilio" {
                "SELECT id FROM customers WHERE tenant_id = $1 AND (phone = $2 OR phone LIKE '%' || $2) LIMIT 1"
            } else if source == "email" {
                "SELECT id FROM customers WHERE tenant_id = $1 AND email = $2 LIMIT 1"
            } else {
                "SELECT id FROM customers WHERE tenant_id = $1 AND (preferences->>'social_handle' = $2 OR preferences->>'instagram_handle' = $2) LIMIT 1"
            };

            let row = sqlx::query(query)
                .bind(tenant_id)
                .bind(sender_id)
                .fetch_optional(pool)
                .await
                .ok()??;

            Some(row.get("id"))
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

            let row = db_query.fetch_optional(sqlite_pool).await.ok()??;

            Some(row.get("id"))
        }
    }
}
