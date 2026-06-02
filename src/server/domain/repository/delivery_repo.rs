use crate::db::DB;
use chrono::{NaiveDate, Utc};
use sqlx::Row;
use std::sync::Arc;
use uuid::Uuid;

pub struct DeliveryRepo {
    db: Arc<DB>,
}

#[derive(Debug)]
pub struct DeliveryZone {
    pub id: String,
    pub tenant_id: String,
    pub polygon: String,
    pub flat_fee: f64,
    pub min_order_value: i32,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug)]
pub struct DeliveryTask {
    pub id: String,
    pub tenant_id: String,
    pub order_id: String,
    pub driver_id: Option<String>,
    pub status: String,
    pub estimated_arrival: Option<chrono::DateTime<Utc>>,
    pub delivery_location: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug)]
pub struct RoutePlan {
    pub id: String,
    pub tenant_id: String,
    pub delivery_date: NaiveDate,
    pub waypoint_sequence: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

impl DeliveryRepo {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn configure_delivery_zone(
        &self,
        tenant_id: &str,
        polygon: &str,
        flat_fee: f64,
        min_order_value: i32,
    ) -> Result<DeliveryZone, String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    r#"
                    INSERT INTO delivery_zones (id, tenant_id, polygon, flat_fee, min_order_value, created_at, updated_at)
                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                    ON CONFLICT (id) DO UPDATE SET
                        polygon = EXCLUDED.polygon,
                        flat_fee = EXCLUDED.flat_fee,
                        min_order_value = EXCLUDED.min_order_value,
                        updated_at = EXCLUDED.updated_at
                    RETURNING id, tenant_id, polygon, flat_fee, min_order_value, created_at, updated_at
                    "#,
                )
                .bind(&id)
                .bind(tenant_id)
                .bind(polygon)
                .bind(flat_fee)
                .bind(min_order_value)
                .bind(now)
                .bind(now)
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;

                Ok(DeliveryZone {
                    id: row.get("id"),
                    tenant_id: row.get("tenant_id"),
                    polygon: row.get("polygon"),
                    flat_fee: row.try_get::<f64, _>("flat_fee").unwrap_or(0.0),
                    min_order_value: row.try_get::<i32, _>("min_order_value").unwrap_or(0),
                    created_at: row.try_get("created_at").unwrap_or(now),
                    updated_at: row.try_get("updated_at").unwrap_or(now),
                })
            }
            crate::db::DbStore::Sqlite(pool) => {
                let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

                // For SQLite, ON CONFLICT requires a unique constraint. We assume `id` is primary key.
                let row = sqlx::query(
                    r#"
                    INSERT INTO delivery_zones (id, tenant_id, polygon, flat_fee, min_order_value, created_at, updated_at)
                    VALUES (?, ?, ?, ?, ?, ?, ?)
                    ON CONFLICT (id) DO UPDATE SET
                        polygon = EXCLUDED.polygon,
                        flat_fee = EXCLUDED.flat_fee,
                        min_order_value = EXCLUDED.min_order_value,
                        updated_at = EXCLUDED.updated_at
                    RETURNING id, tenant_id, polygon, flat_fee, min_order_value, created_at, updated_at
                    "#,
                )
                .bind(&id)
                .bind(tenant_id)
                .bind(polygon)
                .bind(flat_fee)
                .bind(min_order_value)
                .bind(now)
                .bind(now)
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;

                Ok(DeliveryZone {
                    id: row.get("id"),
                    tenant_id: row.get("tenant_id"),
                    polygon: row.get("polygon"),
                    flat_fee: row.try_get::<f64, _>("flat_fee").unwrap_or(0.0),
                    min_order_value: row.try_get::<i32, _>("min_order_value").unwrap_or(0),
                    created_at: now, // SQLite times can be tricky
                    updated_at: now,
                })
            }
        }
    }

    pub async fn get_delivery_zone(&self, tenant_id: &str) -> Result<Option<DeliveryZone>, String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let row = sqlx::query(
                    "SELECT id, tenant_id, polygon, flat_fee, min_order_value, created_at, updated_at FROM delivery_zones WHERE tenant_id = $1 LIMIT 1"
                )
                .bind(tenant_id)
                .fetch_optional(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    Ok(Some(DeliveryZone {
                        id: r.get("id"),
                        tenant_id: r.get("tenant_id"),
                        polygon: r.get("polygon"),
                        flat_fee: r.try_get::<f64, _>("flat_fee").unwrap_or(0.0),
                        min_order_value: r.try_get::<i32, _>("min_order_value").unwrap_or(0),
                        created_at: r.try_get("created_at").unwrap_or_else(|_| Utc::now()),
                        updated_at: r.try_get("updated_at").unwrap_or_else(|_| Utc::now()),
                    }))
                } else {
                    Ok(None)
                }
            }
            crate::db::DbStore::Sqlite(pool) => {
                let row = sqlx::query(
                    "SELECT id, tenant_id, polygon, flat_fee, min_order_value, created_at, updated_at FROM delivery_zones WHERE tenant_id = ? LIMIT 1"
                )
                .bind(tenant_id)
                .fetch_optional(pool)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    Ok(Some(DeliveryZone {
                        id: r.get("id"),
                        tenant_id: r.get("tenant_id"),
                        polygon: r.get("polygon"),
                        flat_fee: r.try_get::<f64, _>("flat_fee").unwrap_or(0.0),
                        min_order_value: r.try_get::<i32, _>("min_order_value").unwrap_or(0),
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                    }))
                } else {
                    Ok(None)
                }
            }
        }
    }

    pub async fn get_daily_itinerary(
        &self,
        tenant_id: &str,
        delivery_date: &str,
    ) -> Result<(Option<RoutePlan>, Vec<DeliveryTask>), String> {
        let route_plan = self.get_route_plan(tenant_id, delivery_date).await?;
        let tasks = self.get_delivery_tasks_for_date(tenant_id, delivery_date).await?;
        Ok((route_plan, tasks))
    }

    async fn get_route_plan(
        &self,
        tenant_id: &str,
        delivery_date_str: &str,
    ) -> Result<Option<RoutePlan>, String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let row = sqlx::query(
                    "SELECT id, tenant_id, delivery_date, waypoint_sequence, created_at, updated_at FROM route_plans WHERE tenant_id = $1 AND delivery_date = $2::date"
                )
                .bind(tenant_id)
                .bind(delivery_date_str)
                .fetch_optional(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    Ok(Some(RoutePlan {
                        id: r.get("id"),
                        tenant_id: r.get("tenant_id"),
                        delivery_date: r.get("delivery_date"),
                        waypoint_sequence: r.get("waypoint_sequence"),
                        created_at: r.try_get("created_at").unwrap_or_else(|_| Utc::now()),
                        updated_at: r.try_get("updated_at").unwrap_or_else(|_| Utc::now()),
                    }))
                } else {
                    Ok(None)
                }
            }
            crate::db::DbStore::Sqlite(pool) => {
                let row = sqlx::query(
                    "SELECT id, tenant_id, delivery_date, waypoint_sequence, created_at, updated_at FROM route_plans WHERE tenant_id = ? AND delivery_date = ?"
                )
                .bind(tenant_id)
                .bind(delivery_date_str)
                .fetch_optional(pool)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    let d_str: String = r.get("delivery_date");
                    let d_date = NaiveDate::parse_from_str(&d_str, "%Y-%m-%d").unwrap_or_else(|_| Utc::now().date_naive());
                    Ok(Some(RoutePlan {
                        id: r.get("id"),
                        tenant_id: r.get("tenant_id"),
                        delivery_date: d_date,
                        waypoint_sequence: r.get("waypoint_sequence"),
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                    }))
                } else {
                    Ok(None)
                }
            }
        }
    }

    pub async fn upsert_route_plan(
        &self,
        tenant_id: &str,
        delivery_date: NaiveDate,
        waypoint_sequence: &str,
    ) -> Result<RoutePlan, String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    r#"
                    INSERT INTO route_plans (id, tenant_id, delivery_date, waypoint_sequence, created_at, updated_at)
                    VALUES ($1, $2, $3, $4, $5, $6)
                    ON CONFLICT (id) DO UPDATE SET
                        waypoint_sequence = EXCLUDED.waypoint_sequence,
                        updated_at = EXCLUDED.updated_at
                    RETURNING id, tenant_id, delivery_date, waypoint_sequence, created_at, updated_at
                    "#,
                )
                .bind(&id)
                .bind(tenant_id)
                .bind(delivery_date)
                .bind(waypoint_sequence)
                .bind(now)
                .bind(now)
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;

                Ok(RoutePlan {
                    id: row.get("id"),
                    tenant_id: row.get("tenant_id"),
                    delivery_date: row.get("delivery_date"),
                    waypoint_sequence: row.get("waypoint_sequence"),
                    created_at: row.try_get("created_at").unwrap_or(now),
                    updated_at: row.try_get("updated_at").unwrap_or(now),
                })
            }
            crate::db::DbStore::Sqlite(pool) => {
                let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
                let date_str = delivery_date.format("%Y-%m-%d").to_string();
                let row = sqlx::query(
                    r#"
                    INSERT INTO route_plans (id, tenant_id, delivery_date, waypoint_sequence, created_at, updated_at)
                    VALUES (?, ?, ?, ?, ?, ?)
                    ON CONFLICT (id) DO UPDATE SET
                        waypoint_sequence = EXCLUDED.waypoint_sequence,
                        updated_at = EXCLUDED.updated_at
                    RETURNING id, tenant_id, delivery_date, waypoint_sequence, created_at, updated_at
                    "#,
                )
                .bind(&id)
                .bind(tenant_id)
                .bind(&date_str)
                .bind(waypoint_sequence)
                .bind(now)
                .bind(now)
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;

                let r_date_str: String = row.get("delivery_date");
                let r_date = NaiveDate::parse_from_str(&r_date_str, "%Y-%m-%d").unwrap_or(delivery_date);

                Ok(RoutePlan {
                    id: row.get("id"),
                    tenant_id: row.get("tenant_id"),
                    delivery_date: r_date,
                    waypoint_sequence: row.get("waypoint_sequence"),
                    created_at: now,
                    updated_at: now,
                })
            }
        }
    }

    async fn get_delivery_tasks_for_date(
        &self,
        tenant_id: &str,
        delivery_date: &str,
    ) -> Result<Vec<DeliveryTask>, String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                // Assuming created_at determines the delivery date for simplicity,
                // or we join with orders to get delivery_date if available.
                // Using created_at bounded by date for now.
                let rows = sqlx::query(
                    "SELECT id, tenant_id, order_id, driver_id, status, estimated_arrival, delivery_location, created_at, updated_at
                     FROM delivery_tasks
                     WHERE tenant_id = $1 AND DATE(created_at) = $2::date"
                )
                .bind(tenant_id)
                .bind(delivery_date)
                .fetch_all(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;

                let tasks = rows.into_iter().map(|r| DeliveryTask {
                    id: r.get("id"),
                    tenant_id: r.get("tenant_id"),
                    order_id: r.get("order_id"),
                    driver_id: r.try_get("driver_id").ok(),
                    status: r.get("status"),
                    estimated_arrival: r.try_get("estimated_arrival").ok(),
                    delivery_location: r.get("delivery_location"),
                    created_at: r.try_get("created_at").unwrap_or_else(|_| Utc::now()),
                    updated_at: r.try_get("updated_at").unwrap_or_else(|_| Utc::now()),
                }).collect();

                Ok(tasks)
            }
            crate::db::DbStore::Sqlite(pool) => {
                let rows = sqlx::query(
                    "SELECT id, tenant_id, order_id, driver_id, status, estimated_arrival, delivery_location, created_at, updated_at
                     FROM delivery_tasks
                     WHERE tenant_id = ? AND date(created_at) = date(?)"
                )
                .bind(tenant_id)
                .bind(delivery_date)
                .fetch_all(pool)
                .await
                .map_err(|e| e.to_string())?;

                let tasks = rows.into_iter().map(|r| DeliveryTask {
                    id: r.get("id"),
                    tenant_id: r.get("tenant_id"),
                    order_id: r.get("order_id"),
                    driver_id: r.try_get("driver_id").ok(),
                    status: r.get("status"),
                    estimated_arrival: None,
                    delivery_location: r.get("delivery_location"),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                }).collect();

                Ok(tasks)
            }
        }
    }

    pub async fn create_delivery_task(
        &self,
        tenant_id: &str,
        order_id: &str,
        delivery_location: &str,
    ) -> Result<DeliveryTask, String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    r#"
                    INSERT INTO delivery_tasks (id, tenant_id, order_id, status, delivery_location, created_at, updated_at)
                    VALUES ($1, $2, $3, 'PENDING', $4, $5, $6)
                    RETURNING id, tenant_id, order_id, driver_id, status, estimated_arrival, delivery_location, created_at, updated_at
                    "#,
                )
                .bind(&id)
                .bind(tenant_id)
                .bind(order_id)
                .bind(delivery_location)
                .bind(now)
                .bind(now)
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;

                Ok(DeliveryTask {
                    id: row.get("id"),
                    tenant_id: row.get("tenant_id"),
                    order_id: row.get("order_id"),
                    driver_id: row.try_get("driver_id").ok(),
                    status: row.get("status"),
                    estimated_arrival: row.try_get("estimated_arrival").ok(),
                    delivery_location: row.get("delivery_location"),
                    created_at: row.try_get("created_at").unwrap_or(now),
                    updated_at: row.try_get("updated_at").unwrap_or(now),
                })
            }
            crate::db::DbStore::Sqlite(pool) => {
                let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    r#"
                    INSERT INTO delivery_tasks (id, tenant_id, order_id, status, delivery_location, created_at, updated_at)
                    VALUES (?, ?, ?, 'PENDING', ?, ?, ?)
                    RETURNING id, tenant_id, order_id, driver_id, status, estimated_arrival, delivery_location, created_at, updated_at
                    "#,
                )
                .bind(&id)
                .bind(tenant_id)
                .bind(order_id)
                .bind(delivery_location)
                .bind(now)
                .bind(now)
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;

                Ok(DeliveryTask {
                    id: row.get("id"),
                    tenant_id: row.get("tenant_id"),
                    order_id: row.get("order_id"),
                    driver_id: row.try_get("driver_id").ok(),
                    status: row.get("status"),
                    estimated_arrival: None,
                    delivery_location: row.get("delivery_location"),
                    created_at: now,
                    updated_at: now,
                })
            }
        }
    }

    pub async fn update_delivery_task_status(
        &self,
        tenant_id: &str,
        task_id: &str,
        status: &str,
    ) -> Result<DeliveryTask, String> {
        let now = Utc::now();

        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    r#"
                    UPDATE delivery_tasks
                    SET status = $1, updated_at = $2
                    WHERE id = $3 AND tenant_id = $4
                    RETURNING id, tenant_id, order_id, driver_id, status, estimated_arrival, delivery_location, created_at, updated_at
                    "#,
                )
                .bind(status)
                .bind(now)
                .bind(task_id)
                .bind(tenant_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    Ok(DeliveryTask {
                        id: r.get("id"),
                        tenant_id: r.get("tenant_id"),
                        order_id: r.get("order_id"),
                        driver_id: r.try_get("driver_id").ok(),
                        status: r.get("status"),
                        estimated_arrival: r.try_get("estimated_arrival").ok(),
                        delivery_location: r.get("delivery_location"),
                        created_at: r.try_get("created_at").unwrap_or(now),
                        updated_at: r.try_get("updated_at").unwrap_or(now),
                    })
                } else {
                    Err("Task not found".to_string())
                }
            }
            crate::db::DbStore::Sqlite(pool) => {
                let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
                let row = sqlx::query(
                    r#"
                    UPDATE delivery_tasks
                    SET status = ?, updated_at = ?
                    WHERE id = ? AND tenant_id = ?
                    RETURNING id, tenant_id, order_id, driver_id, status, estimated_arrival, delivery_location, created_at, updated_at
                    "#,
                )
                .bind(status)
                .bind(now)
                .bind(task_id)
                .bind(tenant_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    Ok(DeliveryTask {
                        id: r.get("id"),
                        tenant_id: r.get("tenant_id"),
                        order_id: r.get("order_id"),
                        driver_id: r.try_get("driver_id").ok(),
                        status: r.get("status"),
                        estimated_arrival: None,
                        delivery_location: r.get("delivery_location"),
                        created_at: now,
                        updated_at: now,
                    })
                } else {
                    Err("Task not found".to_string())
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::db::DB;

    #[tokio::test]
    async fn test_delivery_repo() {
        let db = Arc::new(DB::new().await);
        let repo = DeliveryRepo::new(db.clone());

        let tenant_id = "tenant-1";

        // 1. Configure Zone
        let zone = repo.configure_delivery_zone(tenant_id, "{}", 10.0, 20).await.unwrap();
        assert_eq!(zone.flat_fee, 10.0);

        let fetched_zone = repo.get_delivery_zone(tenant_id).await.unwrap().unwrap();
        assert_eq!(fetched_zone.id, zone.id);

        // 2. Create Tasks
        let task1 = repo.create_delivery_task(tenant_id, "order-1", "loc-1").await.unwrap();
        assert_eq!(task1.status, "PENDING");

        // 3. Update task
        let updated_task = repo.update_delivery_task_status(tenant_id, &task1.id, "IN_TRANSIT").await.unwrap();
        assert_eq!(updated_task.status, "IN_TRANSIT");

        // 4. Get itinerary
        let today = Utc::now().format("%Y-%m-%d").to_string();
        let (plan_opt, tasks) = repo.get_daily_itinerary(tenant_id, &today).await.unwrap();
        assert!(plan_opt.is_none());
        assert_eq!(tasks.len(), 1);

        // 5. Upsert route plan
        let today_naive = Utc::now().date_naive();
        let plan = repo.upsert_route_plan(tenant_id, today_naive, "[\"loc-1\"]").await.unwrap();
        assert_eq!(plan.waypoint_sequence, "[\"loc-1\"]");

        let (plan_opt2, _) = repo.get_daily_itinerary(tenant_id, &today).await.unwrap();
        assert!(plan_opt2.is_some());
    }
}
