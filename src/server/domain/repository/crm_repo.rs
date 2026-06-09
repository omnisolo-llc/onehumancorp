use crate::db::DB;
use crate::domain::repository::models::{Lead, Opportunity};

pub struct CrmRepository {
    db: DB,
}

impl CrmRepository {
    pub fn new(db: DB) -> Self {
        Self { db }
    }

    pub async fn create_lead(&self, lead: &Lead) -> Result<(), String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query(
                    "INSERT INTO leads (id, tenant_id, source, contact_info, context, created_at, updated_at)
                     VALUES ($1, $2, $3, $4, $5, $6, $7)"
                )
                .bind(&lead.id)
                .bind(&lead.tenant_id)
                .bind(&lead.source)
                .bind(&lead.contact_info)
                .bind(&lead.context)
                .bind(lead.created_at)
                .bind(lead.updated_at)
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
                Ok(())
            }
            crate::db::DbStore::Sqlite(pool) => {
                sqlx::query(
                    "INSERT INTO leads (id, tenant_id, source, contact_info, context, created_at, updated_at)
                     VALUES (?, ?, ?, ?, ?, ?, ?)"
                )
                .bind(&lead.id)
                .bind(&lead.tenant_id)
                .bind(&lead.source)
                .bind(&lead.contact_info)
                .bind(&lead.context)
                .bind(lead.created_at)
                .bind(lead.updated_at)
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
                Ok(())
            }
        }
    }

    pub async fn create_opportunity(&self, opp: &Opportunity) -> Result<(), String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query(
                    "INSERT INTO opportunities (id, tenant_id, lead_id, title, stage, estimated_value, priority, created_at, updated_at)
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
                )
                .bind(&opp.id)
                .bind(&opp.tenant_id)
                .bind(&opp.lead_id)
                .bind(&opp.title)
                .bind(&opp.stage)
                .bind(opp.estimated_value)
                .bind(&opp.priority)
                .bind(opp.created_at)
                .bind(opp.updated_at)
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
                Ok(())
            }
            crate::db::DbStore::Sqlite(pool) => {
                sqlx::query(
                    "INSERT INTO opportunities (id, tenant_id, lead_id, title, stage, estimated_value, priority, created_at, updated_at)
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
                )
                .bind(&opp.id)
                .bind(&opp.tenant_id)
                .bind(&opp.lead_id)
                .bind(&opp.title)
                .bind(&opp.stage)
                .bind(opp.estimated_value)
                .bind(&opp.priority)
                .bind(opp.created_at)
                .bind(opp.updated_at)
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
                Ok(())
            }
        }
    }

    pub async fn update_opportunity_stage(&self, id: &str, tenant_id: &str, stage: &str) -> Result<(), String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query("UPDATE opportunities SET stage = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND tenant_id = $3")
                    .bind(stage)
                    .bind(id)
                    .bind(tenant_id)
                    .execute(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(())
            }
            crate::db::DbStore::Sqlite(pool) => {
                sqlx::query("UPDATE opportunities SET stage = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? AND tenant_id = ?")
                    .bind(stage)
                    .bind(id)
                    .bind(tenant_id)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(())
            }
        }
    }

    pub async fn list_opportunities(&self, tenant_id: &str) -> Result<Vec<Opportunity>, String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query_as::<sqlx::postgres::Postgres, Opportunity>("SELECT * FROM opportunities WHERE tenant_id = $1 ORDER BY created_at DESC")
                    .bind(tenant_id)
                    .fetch_all(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())
            }
            crate::db::DbStore::Sqlite(pool) => {
                sqlx::query_as::<sqlx::sqlite::Sqlite, Opportunity>("SELECT * FROM opportunities WHERE tenant_id = ? ORDER BY created_at DESC")
                    .bind(tenant_id)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())
            }
        }
    }

    pub async fn list_leads(&self, tenant_id: &str) -> Result<Vec<Lead>, String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query_as::<sqlx::postgres::Postgres, Lead>("SELECT * FROM leads WHERE tenant_id = $1 ORDER BY created_at DESC")
                    .bind(tenant_id)
                    .fetch_all(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())
            }
            crate::db::DbStore::Sqlite(pool) => {
                sqlx::query_as::<sqlx::sqlite::Sqlite, Lead>("SELECT * FROM leads WHERE tenant_id = ? ORDER BY created_at DESC")
                    .bind(tenant_id)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())
            }
        }
    }
}
