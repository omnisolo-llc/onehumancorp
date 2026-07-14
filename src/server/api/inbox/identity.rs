
use crate::db::DB;
use sqlx::Row;

pub async fn resolve_identity(
    db: &DB,
    tenant_id: &str,
    _source: &str,
    sender_id: &str,
) -> Option<String> {
    if sender_id.is_empty() || sender_id == "unknown" {
        return None;
    }

    match &db.store {
        crate::db::DbStore::Postgres => {
            let pool = &db.pool;
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
            .ok()??;

            Some(row.get("id"))
        }
        crate::db::DbStore::Sqlite(sqlite_pool) => {
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
            .ok()??;

            Some(row.get("id"))
        }
    }
}
