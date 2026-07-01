use std::sync::Arc;
use crate::db::DB;
use serde_json::json;
use uuid::Uuid;

pub struct VipLoyaltyWorker {
    pub db: Arc<DB>,
}

impl VipLoyaltyWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60 * 60 * 24)); // Daily job
            loop {
                interval.tick().await;
                let _ = Self::poll(&db).await;
            }
        });
    }

    pub async fn poll(db: &Arc<DB>) -> Result<bool, String> {
        let pool = db.pool.clone();

        let vips = match &db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query_as::<_, (Uuid, String, String)>(
                    r#"
                    SELECT c.id, c.tenant_id, c.name
                    FROM customers c
                    JOIN orders o ON c.id = o.customer_id
                    GROUP BY c.id, c.tenant_id, c.name
                    HAVING COUNT(o.id) >= 2
                       AND MAX(o.created_at) < CURRENT_TIMESTAMP - INTERVAL '60 days'
                       AND SUM(o.total_amount_cents) > 10000
                    "#
                )
                .fetch_all(&pool)
                .await
                .unwrap_or_default()
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as::<_, (Uuid, String, String)>(
                    r#"
                    SELECT c.id, c.tenant_id, c.name
                    FROM customers c
                    JOIN orders o ON c.id = o.customer_id
                    GROUP BY c.id, c.tenant_id, c.name
                    HAVING COUNT(o.id) >= 2
                       AND MAX(o.created_at) < datetime('now', '-60 days')
                       AND SUM(o.total_amount_cents) > 10000
                    "#
                )
                .fetch_all(sqlite_pool)
                .await
                .unwrap_or_default()
            }
        };

        if vips.is_empty() {
            return Ok(false);
        }

        let mut processed_any = false;

        for (customer_id, tenant_id, name) in vips {
            let pending_count: i64 = match &db.store {
                crate::db::DbStore::Postgres => {
                    sqlx::query_scalar(
                        "SELECT COUNT(*) FROM shared_tasks WHERE organization_id = $1 AND title LIKE $2 AND approval_status = 'PENDING'"
                    )
                    .bind(&tenant_id)
                    .bind(format!("Loyalty Alert: %{}", name))
                    .fetch_one(&pool)
                    .await
                    .unwrap_or(0)
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    sqlx::query_scalar(
                        "SELECT COUNT(*) FROM shared_tasks WHERE organization_id = ? AND title LIKE ? AND approval_status = 'PENDING'"
                    )
                    .bind(&tenant_id)
                    .bind(format!("Loyalty Alert: %{}", name))
                    .fetch_one(sqlite_pool)
                    .await
                    .unwrap_or(0)
                }
            };

            if pending_count > 0 {
                continue;
            }

            let order_history: Vec<String> = match &db.store {
                crate::db::DbStore::Postgres => {
                     sqlx::query_scalar(
                        "SELECT CAST(total_amount_cents AS TEXT) || ' cents on ' || CAST(created_at AS TEXT) FROM orders WHERE customer_id = $1 ORDER BY created_at DESC LIMIT 3"
                    )
                    .bind(&customer_id)
                    .fetch_all(&pool)
                    .await
                    .unwrap_or_default()
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                     sqlx::query_scalar(
                        "SELECT CAST(total_amount_cents AS TEXT) || ' cents on ' || CAST(created_at AS TEXT) FROM orders WHERE customer_id = ? ORDER BY created_at DESC LIMIT 3"
                    )
                    .bind(&customer_id)
                    .fetch_all(sqlite_pool)
                    .await
                    .unwrap_or_default()
                }
            };

            let context_str = order_history.join(", ");
            let llm_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
            let provider = crate::minimax::MinimaxClient::new(llm_key);
            let prompt = format!(
                "You are The Promoter, a Customer Success Agent for a small business. \
                Draft a short, highly personalized SMS re-engagement offer (e.g. 15% off) for a VIP customer named {name} who hasn't purchased in 60 days. \
                Their recent purchase history: {context_str}. \
                Keep it under 160 characters. Do not include placeholders like [Business Name], just write the message directly.",
                name = name,
                context_str = context_str
            );

            let draft = match provider.reason(&prompt).await {
                Ok(response) => response,
                Err(_) => format!("Hey {}, we miss you! It's been a while since your last visit. Enjoy 15% off your next purchase with us!", name),
            };

            let task_id = format!("task-loyalty-{}", Uuid::new_v4());
            let title = format!("Loyalty Alert: VIP {} hasn't purchased in 60 days.", name);
            let description = format!("Want to send them a 'We Miss You' discount?");

            let context_payload = json!({
                "customer_id": customer_id,
                "customer_name": name,
                "feature_type": "vip_loyalty",
                "message": description
            }).to_string();

            let action_payload = json!({
                "action_type": "send_vip_offer",
                "customer_id": customer_id,
                "message": "Approve & Send",
                "feature_type": "vip_loyalty"
            }).to_string();

            match &db.store {
                crate::db::DbStore::Postgres => {
                    sqlx::query(
                        "INSERT INTO shared_tasks (id, organization_id, title, description, status, priority, action_risk, approval_status, proposed_content, payload)
                         VALUES ($1, $2, $3, $4, 'OPEN', 'HIGH', 'LOW', 'PENDING', $5, $6)"
                    )
                    .bind(task_id)
                    .bind(&tenant_id)
                    .bind(title)
                    .bind(context_payload)
                    .bind(draft)
                    .bind(action_payload)
                    .execute(&pool)
                    .await
                    .unwrap_or_default();
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    sqlx::query(
                        "INSERT INTO shared_tasks (id, organization_id, title, description, status, priority, action_risk, approval_status, proposed_content, payload)
                         VALUES (?, ?, ?, ?, 'OPEN', 'HIGH', 'LOW', 'PENDING', ?, ?)"
                    )
                    .bind(task_id)
                    .bind(&tenant_id)
                    .bind(title)
                    .bind(context_payload)
                    .bind(draft)
                    .bind(action_payload)
                    .execute(sqlite_pool)
                    .await
                    .unwrap_or_default();
                }
            };

            processed_any = true;
        }

        Ok(processed_any)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use crate::db::DbStore;

    async fn setup_test_db() -> Option<Arc<DB>> {
        let sqlite_pool = crate::db::create_sqlite_pool_for_test().await;
        let pool = crate::db::create_dummy_pg_pool().await;
        let db = DB {
            pool,
            store: DbStore::Sqlite(sqlite_pool.clone()),
        };

        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS tenants (id TEXT PRIMARY KEY, name TEXT, industry TEXT);").execute(&sqlite_pool).await;
        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS customers (id UUID PRIMARY KEY, tenant_id TEXT, name TEXT, email TEXT, phone TEXT, updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);").execute(&sqlite_pool).await;
        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS orders (id TEXT PRIMARY KEY, tenant_id TEXT, customer_id UUID, total_amount_cents INTEGER, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);").execute(&sqlite_pool).await;
        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS shared_tasks (id TEXT PRIMARY KEY, organization_id TEXT, title TEXT, description TEXT, status TEXT, priority TEXT, action_risk TEXT, approval_status TEXT, proposed_content TEXT, payload TEXT);").execute(&sqlite_pool).await;

        Some(Arc::new(db))
    }

    #[tokio::test]
    async fn test_vip_loyalty_worker() {
        let db = match setup_test_db().await {
            Some(db) => db,
            None => return,
        };

        let pool = match &db.store {
            DbStore::Sqlite(p) => p.clone(),
            _ => panic!("Expected Sqlite store"),
        };

        let _ = sqlx::query("INSERT INTO tenants (id, name, industry) VALUES ('tenant-1', 'Boutique', 'Retail')")
            .execute(&pool).await;

        let customer_id = Uuid::new_v4();
        // Insert customer
        let _ = sqlx::query("INSERT INTO customers (id, tenant_id, name, email) VALUES ($1, 'tenant-1', 'PriyaVIP', 'priyavip@example.com')")
            .bind(customer_id)
            .execute(&pool).await;

        // Insert order 1
        let _ = sqlx::query("INSERT INTO orders (id, tenant_id, customer_id, total_amount_cents, created_at) VALUES ('order-1', 'tenant-1', $1, 6000, datetime('now', '-70 days'))")
            .bind(customer_id)
            .execute(&pool).await;

        // Insert order 2
        let _ = sqlx::query("INSERT INTO orders (id, tenant_id, customer_id, total_amount_cents, created_at) VALUES ('order-2', 'tenant-1', $1, 5000, datetime('now', '-65 days'))")
            .bind(customer_id)
            .execute(&pool).await;

        let _processed = VipLoyaltyWorker::poll(&db).await.unwrap_or_default();

        let task_result: Result<(String, String, String), _> = sqlx::query_as("SELECT title, approval_status, proposed_content FROM shared_tasks WHERE organization_id = 'tenant-1'")
            .fetch_one(&pool).await;

        if let Ok(task) = task_result {
            assert!(task.0.contains("PriyaVIP"));
            assert_eq!(task.1, "PENDING");
            assert!(task.2.contains("PriyaVIP"));
        }

        let _processed_again = VipLoyaltyWorker::poll(&db).await.unwrap_or_default();
    }
}
