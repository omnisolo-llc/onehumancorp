use std::sync::Arc;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;
use crate::db::{DB, DbStore};
use crate::domain::repository::models::{UcalResource, UcalLedger, UcalDynamicBuffer};

pub struct UcalRepository {
    db: Arc<DB>,
}

impl UcalRepository {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn create_resource(&self, resource: UcalResource) -> Result<(), String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    "INSERT INTO ucal_resources (id, tenant_id, name, resource_type, base_capacity)
                     VALUES ($1, $2, $3, $4, $5)"
                )
                .bind(resource.id)
                .bind(&resource.tenant_id)
                .bind(&resource.name)
                .bind(&resource.resource_type)
                .bind(resource.base_capacity)
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(pool) => {
                sqlx::query(
                    "INSERT INTO ucal_resources (id, tenant_id, name, resource_type, base_capacity)
                     VALUES (?, ?, ?, ?, ?)"
                )
                .bind(resource.id.to_string())
                .bind(&resource.tenant_id)
                .bind(&resource.name)
                .bind(&resource.resource_type)
                .bind(resource.base_capacity)
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    pub async fn get_resources(&self, tenant_id: &str) -> Result<Vec<UcalResource>, String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_as::<_, UcalResource>("SELECT * FROM ucal_resources WHERE tenant_id = $1")
                    .bind(tenant_id)
                    .fetch_all(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())
            }
            DbStore::Sqlite(pool) => {
                let rows = sqlx::query("SELECT * FROM ucal_resources WHERE tenant_id = ?")
                    .bind(tenant_id)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())?;

                let mut res = Vec::new();
                for row in rows {
                    res.push(UcalResource {
                        id: Uuid::parse_str(row.get("id")).unwrap_or_default(),
                        tenant_id: row.get("tenant_id"),
                        name: row.get("name"),
                        resource_type: row.get("resource_type"),
                        base_capacity: row.get("base_capacity"),
                        created_at: row.get("created_at"),
                        updated_at: row.get("updated_at"),
                    });
                }
                Ok(res)
            }
        }
    }

    pub async fn check_and_lock_capacity(
        &self,
        tenant_id: &str,
        resource_id: Uuid,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        units: i32,
        status: &str,
        reference_id: Option<&str>,
    ) -> Result<UcalLedger, String> {
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                // 1. Lock resource row to serialize capacity checks for this specific resource
                let res_row = sqlx::query("SELECT base_capacity FROM ucal_resources WHERE id = $1 AND tenant_id = $2 FOR UPDATE")
                    .bind(resource_id)
                    .bind(tenant_id)
                    .fetch_one(&mut *tx)
                    .await
                    .map_err(|e| format!("Resource not found: {}", e))?;

                let base_cap: i32 = res_row.get("base_capacity");

                // 2. Check current consumption in time range (with buffer overlap logic simplified for core impl)
                let consumed: i64 = sqlx::query_scalar(
                    "SELECT COALESCE(SUM(consumed_units), 0) FROM ucal_ledger
                     WHERE resource_id = $1 AND tenant_id = $2
                     AND (start_time, end_time) OVERLAPS ($3, $4)"
                )
                .bind(resource_id)
                .bind(tenant_id)
                .bind(start_time)
                .bind(end_time)
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                if (consumed as i32 + units) > base_cap {
                    return Err("Insufficient capacity".to_string());
                }

                // 3. Insert ledger entry
                let ledger_id = Uuid::new_v4();
                let ledger = sqlx::query_as::<_, UcalLedger>(
                    "INSERT INTO ucal_ledger (id, tenant_id, resource_id, start_time, end_time, consumed_units, status, reference_id)
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                     RETURNING *"
                )
                .bind(ledger_id)
                .bind(tenant_id)
                .bind(resource_id)
                .bind(start_time)
                .bind(end_time)
                .bind(units)
                .bind(status)
                .bind(reference_id)
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;
                Ok(ledger)
            }
            DbStore::Sqlite(_) => {
                // Simplified SQLite implementation for standalone mode (no OVERLAPS)
                Err("UCAL Atomic locking not fully implemented for SQLite yet".to_string())
            }
        }
    }

    pub async fn get_ledger_entries(
        &self,
        tenant_id: &str,
        start_range: DateTime<Utc>,
        end_range: DateTime<Utc>,
    ) -> Result<Vec<UcalLedger>, String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_as::<_, UcalLedger>(
                    "SELECT * FROM ucal_ledger WHERE tenant_id = $1 AND start_time < $2 AND end_time > $3"
                )
                .bind(tenant_id)
                .bind(end_range)
                .bind(start_range)
                .fetch_all(&self.db.pool)
                .await
                .map_err(|e| e.to_string())
            }
            DbStore::Sqlite(_) => Err("Not implemented for SQLite".to_string())
        }
    }
}
