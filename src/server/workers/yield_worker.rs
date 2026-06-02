use std::sync::Arc;
use crate::db::{DB, DbStore};
use crate::orchestration::departments::types::DepartmentEvent;
use std::time::Duration;
use chrono::Utc;
use uuid::Uuid;
use sqlx::Row;

pub struct YieldWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}

impl YieldWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(60), // Run less frequently since yield events are time-based
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let interval_duration = self.poll_interval;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval_duration);
            loop {
                interval.tick().await;
                if let Err(e) = Self::poll(&db).await {
                    tracing::error!("YieldWorker error: {}", e);
                }
            }
        });
    }

    pub async fn poll(db: &Arc<DB>) -> Result<(), String> {
        Self::check_perishable_inventory(db).await?;
        Self::check_idle_capacity(db).await?;
        Ok(())
    }

    async fn check_perishable_inventory(db: &Arc<DB>) -> Result<(), String> {
        // Find products with inventory > 0 that might be expiring today.
        // For simplicity, we assume products with 'perishable' tag or metadata are tracked.
        // In this implementation, we look for items tagged 'perishable' or simply having a short shelf life.

        let query = match &db.store {
            DbStore::Postgres => {
                r#"
                SELECT p.id, p.tenant_id, p.title, p.price_cents, p.inventory_count
                FROM products p
                WHERE p.inventory_count > 0
                  AND p.metadata::jsonb ->> 'perishable' = 'true'
                  AND NOT EXISTS (
                      SELECT 1 FROM yield_proposals yp
                      WHERE yp.resource_id = p.id AND yp.status IN ('PENDING', 'APPROVED')
                  )
                "#
            },
            DbStore::Sqlite(_) => {
                r#"
                SELECT p.id, p.tenant_id, p.title, p.price_cents, p.inventory_count
                FROM products p
                WHERE p.inventory_count > 0
                  AND json_extract(p.metadata, '$.perishable') = 'true'
                  AND NOT EXISTS (
                      SELECT 1 FROM yield_proposals yp
                      WHERE yp.resource_id = p.id AND yp.status IN ('PENDING', 'APPROVED')
                  )
                "#
            }
        };

        match &db.store {
            DbStore::Postgres => {
                let rows = sqlx::query(query).fetch_all(&db.pool).await.map_err(|e| e.to_string())?;
                for row in rows {
                    let product_id: String = row.get("id");
                    let tenant_id: String = row.get("tenant_id");
                    let title: String = row.get("title");
                    let price_cents: i64 = row.get("price_cents");
                    let count: i32 = row.get("inventory_count");

                    let proposed_price_cents = (price_cents as f64 * 0.70) as i64;
                    let reason = format!("{} units of {} remaining today. Recommend 30% discount to clear stock.", count, title);

                    Self::create_proposal(db, tenant_id, "inventory".to_string(), product_id, price_cents, proposed_price_cents, reason).await?;
                }
            },
            DbStore::Sqlite(pool) => {
                let rows = sqlx::query(query).fetch_all(pool).await.map_err(|e| e.to_string())?;
                for row in rows {
                    let product_id: String = row.get("id");
                    let tenant_id: String = row.get("tenant_id");
                    let title: String = row.get("title");
                    let price_cents: i64 = row.get("price_cents");
                    let count: i32 = row.get("inventory_count");

                    let proposed_price_cents = (price_cents as f64 * 0.70) as i64;
                    let reason = format!("{} units of {} remaining today. Recommend 30% discount to clear stock.", count, title);

                    Self::create_proposal(db, tenant_id, "inventory".to_string(), product_id, price_cents, proposed_price_cents, reason).await?;
                }
            }
        };

        Ok(())
    }

    async fn check_idle_capacity(db: &Arc<DB>) -> Result<(), String> {
        // In a real scenario we would check available empty slots in the calendar for the next 24h.
        // For this demo, we assume the OperationsWorker or this worker checks for unbooked slots.
        Ok(())
    }

    async fn create_proposal(
        db: &Arc<DB>,
        tenant_id: String,
        resource_type: String,
        resource_id: String,
        original_price_cents: i64,
        proposed_price_cents: i64,
        reason: String,
    ) -> Result<(), String> {
        let proposal_id = Uuid::new_v4().to_string();
        let expires_at = Utc::now() + chrono::Duration::hours(24);

        let insert_query = r#"
            INSERT INTO yield_proposals (id, tenant_id, resource_type, resource_id, status, proposed_price_cents, original_price_cents, reason, expires_at)
            VALUES ($1, $2, $3, $4, 'PENDING', $5, $6, $7, $8)
        "#;

        match &db.store {
            DbStore::Postgres => {
                sqlx::query(insert_query)
                    .bind(&proposal_id)
                    .bind(&tenant_id)
                    .bind(&resource_type)
                    .bind(&resource_id)
                    .bind(proposed_price_cents)
                    .bind(original_price_cents)
                    .bind(&reason)
                    .bind(expires_at)
                    .execute(&db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
            },
            DbStore::Sqlite(pool) => {
                let insert_sqlite = r#"
                    INSERT INTO yield_proposals (id, tenant_id, resource_type, resource_id, status, proposed_price_cents, original_price_cents, reason, expires_at)
                    VALUES (?, ?, ?, ?, 'PENDING', ?, ?, ?, ?)
                "#;
                sqlx::query(insert_sqlite)
                    .bind(&proposal_id)
                    .bind(&tenant_id)
                    .bind(&resource_type)
                    .bind(&resource_id)
                    .bind(proposed_price_cents)
                    .bind(original_price_cents)
                    .bind(&reason)
                    .bind(expires_at.format("%Y-%m-%d %H:%M:%S").to_string())
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }

        let event = DepartmentEvent {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.yield.proposal_created".to_string(),
            payload: serde_json::json!({
                "proposal_id": proposal_id,
                "resource_type": resource_type,
                "resource_id": resource_id,
                "reason": reason,
                "original_price_cents": original_price_cents,
                "proposed_price_cents": proposed_price_cents
            }),
        };

        if let Ok(mut client) = ::server_ohc::orchestration::hub_service_client::HubServiceClient::connect(std::env::var("OHC_HUB_URL").unwrap_or_else(|_| "http://127.0.0.1:8081".to_string())).await {
            let req = ::server_ohc::orchestration::PublishMeshEventRequest {
                event: Some(::server_ohc::orchestration::MeshEvent {
                    id: event.id,
                    topic: "department:operations".to_string(),
                    payload: event.payload.to_string(),
                    source: "yield_worker".to_string(),
                    timestamp_unix: chrono::Utc::now().timestamp(),
                }),
            };
            let _ = client.publish_mesh_event(tonic::Request::new(req)).await;
        }

        Ok(())
    }
}
