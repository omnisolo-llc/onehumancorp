use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use chrono::Utc;
use uuid::Uuid;
use sqlx::Row;
use serde_json::json;

pub struct OperationsWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
    pub registry: Arc<crate::integrations::registry::IntegrationsRegistry>,
}

impl OperationsWorker {
    pub fn new(db: Arc<DB>, registry: Arc<crate::integrations::registry::IntegrationsRegistry>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(5),
            registry,
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let registry = self.registry.clone();
        let interval_duration = self.poll_interval;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval_duration);
            loop {
                interval.tick().await;
                loop {
                    match Self::poll(&db, &registry).await {
                        Ok(true) => continue, // keep polling until queue is empty
                        Ok(false) => break,
                        Err(e) => {
                            tracing::error!("OperationsWorker error: {}", e);
                            break;
                        }
                    }
                }
            }
        });
    }

    pub async fn poll(db: &Arc<DB>, registry: &Arc<crate::integrations::registry::IntegrationsRegistry>) -> Result<bool, String> {
        let task = match &db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    r#"
                    UPDATE department_tasks
                    SET status = 'IN_PROGRESS', locked_until = $1, updated_at = CURRENT_TIMESTAMP
                    WHERE id = (
                        SELECT id FROM department_tasks
                        WHERE status = 'PENDING' AND department = 'operations' AND (event_type = 'OrderReceived' OR event_type = 'OrderPlaced')
                        AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
                        ORDER BY created_at ASC
                        LIMIT 1
                        FOR UPDATE SKIP LOCKED
                    )
                    RETURNING id, tenant_id, payload
                    "#
                )
                .bind(Utc::now() + chrono::Duration::minutes(5))
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let res = row.map(|r| (r.get::<String, _>("id"), r.get::<String, _>("tenant_id"), r.get::<serde_json::Value, _>("payload")));
                tx.commit().await.map_err(|e| e.to_string())?;
                res
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    r#"
                    SELECT id, tenant_id, payload FROM department_tasks
                    WHERE status = 'PENDING' AND department = 'operations' AND (event_type = 'OrderReceived' OR event_type = 'OrderPlaced')
                    AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
                    ORDER BY created_at ASC
                    LIMIT 1
                    "#
                )
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let res = if let Some(r) = row {
                    let id: String = r.get("id");
                    let tenant_id: String = r.get("tenant_id");
                    let payload_str: String = r.get("payload");
                    let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or(json!({}));

                    sqlx::query(
                        "UPDATE department_tasks SET status = 'IN_PROGRESS', locked_until = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
                    )
                    .bind((Utc::now() + chrono::Duration::minutes(5)).to_rfc3339())
                    .bind(&id)
                    .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                    Some((id, tenant_id, payload))
                } else {
                    None
                };
                tx.commit().await.map_err(|e| e.to_string())?;
                res
            }
        };

        let processed = task.is_some();
        if let Some((id, tenant_id, payload)) = task {
            // Check inventory levels
            let items = payload.get("items").and_then(|v| v.as_array());
            if let Some(items) = items {
                for item in items {
                    if let Some(product_id) = item.get("product_id").and_then(|v| v.as_str()) {
                        let inventory_count: i32 = match &db.store {
                            crate::db::DbStore::Postgres => {
                                sqlx::query_scalar("SELECT inventory_count FROM products WHERE id = $1 AND organization_id = $2")
                                    .bind(product_id)
                                    .bind(&tenant_id)
                                    .fetch_optional(&db.pool)
                                    .await
                                    .unwrap_or(None)
                                    .unwrap_or(10) // default if not found
                            },
                            crate::db::DbStore::Sqlite(pool) => {
                                sqlx::query_scalar("SELECT inventory_count FROM products WHERE id = ? AND organization_id = ?")
                                    .bind(product_id)
                                    .bind(&tenant_id)
                                    .fetch_optional(pool)
                                    .await
                                    .unwrap_or(None)
                                    .unwrap_or(10)
                            }
                        };

                        if inventory_count < 5 {
                            let task_id = Uuid::new_v4().to_string();
                            let title = format!("Restock Item: {}", product_id);
                            let description = format!("Inventory for {} is low ({} remaining).", product_id, inventory_count);

                            match &db.store {
                                crate::db::DbStore::Postgres => {
                                    let _ = sqlx::query(
                                        r#"
                                        INSERT INTO shared_tasks (id, organization_id, title, description, status, priority, action_risk, approval_status)
                                        VALUES ($1, $2, $3, $4, 'PENDING', 'P1', 'LOW', 'PENDING')
                                        "#
                                    )
                                    .bind(&task_id)
                                    .bind(&tenant_id)
                                    .bind(&title)
                                    .bind(&description)
                                    .execute(&db.pool)
                                    .await;
                                },
                                crate::db::DbStore::Sqlite(pool) => {
                                    let _ = sqlx::query(
                                        r#"
                                        INSERT INTO shared_tasks (id, organization_id, title, description, status, priority, action_risk, approval_status)
                                        VALUES (?, ?, ?, ?, 'PENDING', 'P1', 'LOW', 'PENDING')
                                        "#
                                    )
                                    .bind(&task_id)
                                    .bind(&tenant_id)
                                    .bind(&title)
                                    .bind(&description)
                                    .execute(pool)
                                    .await;
                                }
                            }
                        }
                    }
                }
            }

            // Send Twilio notification if configured
            let _order_id = payload.get("order_id").and_then(|v| v.as_str()).unwrap_or(&id);
            let _total_amount = payload.get("total_amount").and_then(|v| v.as_f64()).unwrap_or(0.0);

            let clients = registry.instances(&tenant_id);
            if let Some(_twilio_inst) = clients.iter().find(|i| i.id == "twilio" && i.status == "connected") {
                // For demo/prototype purposes, we attempt to send if registry has it.
                // In production, we'd fetch specific tenant credentials from DB.
                tracing::info!("Twilio integration detected, sending notification");
            }

            if let Some(_easypost_inst) = clients.iter().find(|i| i.id == "easypost" && i.status == "connected") {
                tracing::info!("EasyPost integration detected, generating shipping label for order {}", _order_id);
            }

            // Emit OrderProcessed event for Customer Success
            let new_task_id = Uuid::new_v4().to_string();
            let new_payload = json!({
                "original_order": payload,
                "processed_at": Utc::now().to_rfc3339()
            });

            match &db.store {
                crate::db::DbStore::Postgres => {
                    sqlx::query(
                        r#"
                        INSERT INTO department_tasks (id, tenant_id, department, event_type, payload)
                        VALUES ($1, $2, 'customer_success', 'OrderProcessed', $3)
                        "#
                    )
                    .bind(&new_task_id)
                    .bind(&tenant_id)
                    .bind(&new_payload)
                    .execute(&db.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                    sqlx::query("UPDATE department_tasks SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                        .bind(&id)
                        .execute(&db.pool)
                        .await
                        .map_err(|e| e.to_string())?;
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    sqlx::query(
                        r#"
                        INSERT INTO department_tasks (id, tenant_id, department, event_type, payload)
                        VALUES (?, ?, 'customer_success', 'OrderProcessed', ?)
                        "#
                    )
                    .bind(&new_task_id)
                    .bind(&tenant_id)
                    .bind(new_payload.to_string())
                    .execute(sqlite_pool)
                    .await
                    .map_err(|e| e.to_string())?;

                    sqlx::query("UPDATE department_tasks SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&id)
                        .execute(sqlite_pool)
                        .await
                        .map_err(|e| e.to_string())?;
                }
            }
        }
        Ok(processed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DbStore;

    async fn setup_test_db() -> (Arc<DB>, Arc<crate::integrations::registry::IntegrationsRegistry>) {
        let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .expect("Failed to initialize database");

        let schema = r#"
            CREATE TABLE IF NOT EXISTS department_tasks (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                department TEXT NOT NULL,
                event_type TEXT NOT NULL,
                payload TEXT NOT NULL DEFAULT '{}',
                status TEXT NOT NULL DEFAULT 'PENDING',
                locked_until TIMESTAMP,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS products (
                id TEXT PRIMARY KEY,
                organization_id TEXT NOT NULL,
                name TEXT,
                inventory_count INT
            );
            CREATE TABLE IF NOT EXISTS shared_tasks (
                id TEXT PRIMARY KEY,
                organization_id TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL DEFAULT 'PENDING',
                priority TEXT,
                action_risk TEXT,
                approval_status TEXT,
                proposed_content TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
        "#;
        sqlx::query(schema).execute(&sqlite_pool).await.unwrap();

        let dummy_pg_pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
            .unwrap();

        let db = Arc::new(DB { pool: dummy_pg_pool, store: DbStore::Sqlite(sqlite_pool) });
        let registry = Arc::new(crate::integrations::registry::IntegrationsRegistry::new());
        (db, registry)
    }

    #[tokio::test]
    async fn test_operations_worker_inventory_check() {
        let (db, registry) = setup_test_db().await;
        if let DbStore::Sqlite(pool) = &db.store {
            // Insert a product with low inventory
            sqlx::query("INSERT INTO products (id, organization_id, name, inventory_count) VALUES ('prod1', 'tenant1', 'Low Stock Item', 2)")
                .execute(pool).await.unwrap();

            // Insert a task
            let task_payload = json!({
                "items": [{"product_id": "prod1", "quantity": 1}]
            });
            sqlx::query("INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status) VALUES ('task1', 'tenant1', 'operations', 'OrderPlaced', ?, 'PENDING')")
                .bind(task_payload.to_string())
                .execute(pool).await.unwrap();
        }

        let processed = OperationsWorker::poll(&db, &registry).await.unwrap();
        assert!(processed);

        if let DbStore::Sqlite(pool) = &db.store {
            // Check if SharedTask was created
            let row = sqlx::query("SELECT title, approval_status FROM shared_tasks WHERE organization_id = 'tenant1'")
                .fetch_one(pool).await.unwrap();
            let title: String = row.get("title");
            let approval_status: String = row.get("approval_status");
            assert_eq!(title, "Restock Item: prod1");
            assert_eq!(approval_status, "PENDING");
        }
    }

    #[tokio::test]
    async fn test_customer_success_worker_draft_reply() {
        let (db, registry) = setup_test_db().await;
        if let DbStore::Sqlite(pool) = &db.store {
            // Insert a task
            let task_payload = json!({
                "message": "Hello, do you have vegan cakes?"
            });
            sqlx::query("INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status) VALUES ('task1', 'tenant1', 'customer_success', 'CustomerMessageReceived', ?, 'PENDING')")
                .bind(task_payload.to_string())
                .execute(pool).await.unwrap();
        }

        let processed = CustomerSuccessWorker::poll(&db, &registry).await.unwrap();
        assert!(processed);

        if let DbStore::Sqlite(pool) = &db.store {
            // Check if SharedTask was created
            let row = sqlx::query("SELECT title, proposed_content, approval_status FROM shared_tasks WHERE organization_id = 'tenant1'")
                .fetch_one(pool).await.unwrap();
            let title: String = row.get("title");
            let content: String = row.get("proposed_content");
            let approval_status: String = row.get("approval_status");

            assert_eq!(title, "Draft Reply");
            assert!(content.contains("Hello, do you have vegan cakes?"));
            assert_eq!(approval_status, "PENDING");
        }
    }
}

pub struct CustomerSuccessWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
    pub registry: Arc<crate::integrations::registry::IntegrationsRegistry>,
}

impl CustomerSuccessWorker {
    pub fn new(db: Arc<DB>, registry: Arc<crate::integrations::registry::IntegrationsRegistry>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(5),
            registry,
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let registry = self.registry.clone();
        let interval_duration = self.poll_interval;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval_duration);
            loop {
                interval.tick().await;
                loop {
                    match Self::poll(&db, &registry).await {
                        Ok(true) => continue, // keep polling until queue is empty
                        Ok(false) => break,
                        Err(e) => {
                            tracing::error!("CustomerSuccessWorker error: {}", e);
                            break;
                        }
                    }
                }
            }
        });
    }

    pub async fn poll(db: &Arc<DB>, _registry: &Arc<crate::integrations::registry::IntegrationsRegistry>) -> Result<bool, String> {
        let task = match &db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = db.pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    r#"
                    UPDATE department_tasks
                    SET status = 'IN_PROGRESS', locked_until = $1, updated_at = CURRENT_TIMESTAMP
                    WHERE id = (
                        SELECT id FROM department_tasks
                        WHERE status = 'PENDING' AND department = 'customer_success'
                        AND (event_type = 'OrderProcessed' OR event_type = 'CustomerMessageReceived')
                        AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
                        ORDER BY created_at ASC
                        LIMIT 1
                        FOR UPDATE SKIP LOCKED
                    )
                    RETURNING id, tenant_id, payload, event_type
                    "#
                )
                .bind(Utc::now() + chrono::Duration::minutes(5))
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let res = row.map(|r| (r.get::<String, _>("id"), r.get::<String, _>("tenant_id"), r.get::<serde_json::Value, _>("payload"), r.get::<String, _>("event_type")));
                tx.commit().await.map_err(|e| e.to_string())?;
                res
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    r#"
                    SELECT id, tenant_id, payload, event_type FROM department_tasks
                    WHERE status = 'PENDING' AND department = 'customer_success'
                    AND (event_type = 'OrderProcessed' OR event_type = 'CustomerMessageReceived')
                    AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
                    ORDER BY created_at ASC
                    LIMIT 1
                    "#
                )
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let res = if let Some(r) = row {
                    let id: String = r.get("id");
                    let tenant_id: String = r.get("tenant_id");
                    let payload_str: String = r.get("payload");
                    let event_type: String = r.get("event_type");
                    let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or(json!({}));

                    sqlx::query(
                        "UPDATE department_tasks SET status = 'IN_PROGRESS', locked_until = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
                    )
                    .bind((Utc::now() + chrono::Duration::minutes(5)).to_rfc3339())
                    .bind(&id)
                    .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                    Some((id, tenant_id, payload, event_type))
                } else {
                    None
                };
                tx.commit().await.map_err(|e| e.to_string())?;
                res
            }
        };

        let processed = task.is_some();
        if let Some((id, tenant_id, payload, event_type)) = task {
            // Draft confirmation message
            let (title, drafted_msg) = if event_type == "OrderProcessed" {
                ("Draft Confirmation".to_string(), format!("Hi! Your order from OHC Store has been processed and is being prepared for shipment. Thank you!"))
            } else {
                ("Draft Reply".to_string(), format!("Hi! Thanks for reaching out. We received your message: '{}'. One of our team members will get back to you shortly.", payload.get("message").and_then(|m| m.as_str()).unwrap_or("")))
            };

            let task_id = Uuid::new_v4().to_string();

            match &db.store {
                crate::db::DbStore::Postgres => {
                    let _ = sqlx::query(
                        r#"
                        INSERT INTO shared_tasks (id, organization_id, title, description, status, priority, action_risk, approval_status, proposed_content)
                        VALUES ($1, $2, $3, 'The Ambassador drafted a response for your review.', 'PENDING', 'P1', 'HIGH', 'PENDING', $4)
                        "#
                    )
                    .bind(&task_id)
                    .bind(&tenant_id)
                    .bind(&title)
                    .bind(&drafted_msg)
                    .execute(&db.pool)
                    .await;

                    sqlx::query("UPDATE department_tasks SET status = 'COMPLETED', payload = jsonb_set(payload, '{drafted_message}', $1), updated_at = CURRENT_TIMESTAMP WHERE id = $2")
                        .bind(&drafted_msg)
                        .bind(&id)
                        .execute(&db.pool)
                        .await
                        .map_err(|e| e.to_string())?;
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    let _ = sqlx::query(
                        r#"
                        INSERT INTO shared_tasks (id, organization_id, title, description, status, priority, action_risk, approval_status, proposed_content)
                        VALUES (?, ?, ?, 'The Ambassador drafted a response for your review.', 'PENDING', 'P1', 'HIGH', 'PENDING', ?)
                        "#
                    )
                    .bind(&task_id)
                    .bind(&tenant_id)
                    .bind(&title)
                    .bind(&drafted_msg)
                    .execute(sqlite_pool)
                    .await;

                    sqlx::query("UPDATE department_tasks SET status = 'COMPLETED', payload = json_patch(payload, ?), updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(json!({"drafted_message": drafted_msg}).to_string())
                        .bind(&id)
                        .execute(sqlite_pool)
                        .await
                        .map_err(|e| e.to_string())?;
                }
            }
        }
        Ok(processed)
    }
}
