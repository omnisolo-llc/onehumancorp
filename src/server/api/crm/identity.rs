use sqlx::{Pool, Postgres, Sqlite};
use uuid::Uuid;
use crate::db::DbStore;

pub async fn resolve_customer_identity(
    pool: &DbStore,
    tenant_id: &str,
    channel: &str,
    identifier: &str,
) -> Result<Option<String>, sqlx::Error> {
    match pool {
        DbStore::Postgres => {
            let row: Option<(String,)> = sqlx::query_as(
                "SELECT customer_id FROM customer_identities WHERE tenant_id = $1 AND channel = $2 AND identifier = $3 LIMIT 1"
            )
            .bind(tenant_id)
            .bind(channel)
            .bind(identifier)
            .fetch_optional(&crate::db::get_pool())
            .await?;
            Ok(row.map(|(id,)| id))
        },
        DbStore::Sqlite(sqlite_pool) => {
            let row: Option<(String,)> = sqlx::query_as(
                "SELECT customer_id FROM customer_identities WHERE tenant_id = ? AND channel = ? AND identifier = ? LIMIT 1"
            )
            .bind(tenant_id)
            .bind(channel)
            .bind(identifier)
            .fetch_optional(sqlite_pool)
            .await?;
            Ok(row.map(|(id,)| id))
        }
    }
}

pub async fn create_customer_identity(
    pool: &DbStore,
    tenant_id: &str,
    customer_id: &str,
    channel: &str,
    identifier: &str,
) -> Result<(), sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    match pool {
        DbStore::Postgres => {
            sqlx::query(
                "INSERT INTO customer_identities (id, tenant_id, customer_id, channel, identifier) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (tenant_id, channel, identifier) DO NOTHING"
            )
            .bind(&id)
            .bind(tenant_id)
            .bind(customer_id)
            .bind(channel)
            .bind(identifier)
            .execute(&crate::db::get_pool())
            .await?;
        },
        DbStore::Sqlite(sqlite_pool) => {
            sqlx::query(
                "INSERT INTO customer_identities (id, tenant_id, customer_id, channel, identifier) VALUES (?, ?, ?, ?, ?) ON CONFLICT (tenant_id, channel, identifier) DO NOTHING"
            )
            .bind(&id)
            .bind(tenant_id)
            .bind(customer_id)
            .bind(channel)
            .bind(identifier)
            .execute(sqlite_pool)
            .await?;
        }
    }
    Ok(())
}

pub async fn get_customer_context(
    pool: &DbStore,
    tenant_id: &str,
    customer_id: &str,
) -> Result<String, sqlx::Error> {
    let mut context = format!("Customer ID: {}\n", customer_id);

    match pool {
        DbStore::Postgres => {
            let row: Option<(String, String)> = sqlx::query_as(
                "SELECT name, email FROM customers WHERE tenant_id = $1 AND id = $2"
            )
            .bind(tenant_id)
            .bind(customer_id)
            .fetch_optional(&crate::db::get_pool())
            .await?;
            if let Some((name, email)) = row {
                context.push_str(&format!("Name: {}\nEmail: {}\n", name, email));
            }

            let orders: Vec<(String, String)> = sqlx::query_as(
                "SELECT id, status FROM orders WHERE tenant_id = $1 AND customer_id = $2 ORDER BY created_at DESC LIMIT 5"
            )
            .bind(tenant_id)
            .bind(customer_id)
            .fetch_all(&crate::db::get_pool())
            .await?;
            if !orders.is_empty() {
                context.push_str("Recent Orders:\n");
                for (id, status) in orders {
                    context.push_str(&format!("- Order ID: {}, Status: {}\n", id, status));
                }
            }
        },
        DbStore::Sqlite(sqlite_pool) => {
            let row: Option<(String, String)> = sqlx::query_as(
                "SELECT name, COALESCE(email, '') FROM customers WHERE tenant_id = ? AND id = ?"
            )
            .bind(tenant_id)
            .bind(customer_id)
            .fetch_optional(sqlite_pool)
            .await?;
            if let Some((name, email)) = row {
                context.push_str(&format!("Name: {}\nEmail: {}\n", name, email));
            }

            let orders: Vec<(String, String)> = sqlx::query_as(
                "SELECT id, COALESCE(status, '') FROM orders WHERE tenant_id = ? AND customer_id = ? ORDER BY created_at DESC LIMIT 5"
            )
            .bind(tenant_id)
            .bind(customer_id)
            .fetch_all(sqlite_pool)
            .await?;
            if !orders.is_empty() {
                context.push_str("Recent Orders:\n");
                for (id, status) in orders {
                    context.push_str(&format!("- Order ID: {}, Status: {}\n", id, status));
                }
            }
        }
    }

    Ok(context)
}
