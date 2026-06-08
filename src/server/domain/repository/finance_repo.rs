use std::sync::Arc;
use crate::db::{DB, DbStore};
use super::models::{TaxObligation, VirtualEnvelope};
use chrono::Utc;

pub struct FinanceRepository {
    db: Arc<DB>,
}

impl FinanceRepository {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn create_tax_obligation(&self, obligation: TaxObligation) -> Result<(), String> {
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                sqlx::query(
                    r#"
                    INSERT INTO tax_obligations (
                        id, tenant_id, transaction_id, tax_type, amount, jurisdiction, status, created_at, updated_at
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                    "#
                )
                .bind(&obligation.id).bind(&obligation.tenant_id).bind(&obligation.transaction_id)
                .bind(&obligation.tax_type).bind(obligation.amount).bind(&obligation.jurisdiction)
                .bind(&obligation.status).bind(obligation.created_at).bind(obligation.updated_at)
                .execute(&mut *tx).await.map_err(|e| e.to_string())?;
                tx.commit().await.map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;
                sqlx::query(
                    r#"
                    INSERT INTO tax_obligations (
                        id, tenant_id, transaction_id, tax_type, amount, jurisdiction, status, created_at, updated_at
                    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(&obligation.id).bind(&obligation.tenant_id).bind(&obligation.transaction_id)
                .bind(&obligation.tax_type).bind(obligation.amount).bind(&obligation.jurisdiction)
                .bind(&obligation.status).bind(obligation.created_at).bind(obligation.updated_at)
                .execute(&mut *tx).await.map_err(|e| e.to_string())?;
                tx.commit().await.map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    pub async fn create_virtual_envelope(&self, envelope: VirtualEnvelope) -> Result<(), String> {
         match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                sqlx::query(
                    r#"
                    INSERT INTO virtual_envelopes (
                        id, tenant_id, name, balance, target_amount, created_at, updated_at
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7)
                    "#
                )
                .bind(&envelope.id).bind(&envelope.tenant_id).bind(&envelope.name)
                .bind(envelope.balance).bind(envelope.target_amount)
                .bind(envelope.created_at).bind(envelope.updated_at)
                .execute(&mut *tx).await.map_err(|e| e.to_string())?;
                tx.commit().await.map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                 let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;
                sqlx::query(
                    r#"
                    INSERT INTO virtual_envelopes (
                        id, tenant_id, name, balance, target_amount, created_at, updated_at
                    ) VALUES (?, ?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(&envelope.id).bind(&envelope.tenant_id).bind(&envelope.name)
                .bind(envelope.balance).bind(envelope.target_amount)
                .bind(envelope.created_at).bind(envelope.updated_at)
                .execute(&mut *tx).await.map_err(|e| e.to_string())?;
                tx.commit().await.map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    pub async fn get_tax_obligations(&self, tenant_id: &str) -> Result<Vec<TaxObligation>, String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_as::<_, TaxObligation>(
                    r#"SELECT * FROM tax_obligations WHERE tenant_id = $1"#
                )
                .bind(tenant_id)
                .fetch_all(&self.db.pool)
                .await
                .map_err(|e| e.to_string())
            }
             DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as::<_, TaxObligation>(
                    r#"SELECT * FROM tax_obligations WHERE tenant_id = ?"#
                )
                .bind(tenant_id)
                .fetch_all(sqlite_pool)
                .await
                .map_err(|e| e.to_string())
            }
        }
    }

     pub async fn get_virtual_envelopes(&self, tenant_id: &str) -> Result<Vec<VirtualEnvelope>, String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_as::<_, VirtualEnvelope>(
                    r#"SELECT * FROM virtual_envelopes WHERE tenant_id = $1"#
                )
                .bind(tenant_id)
                .fetch_all(&self.db.pool)
                .await
                .map_err(|e| e.to_string())
            }
             DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as::<_, VirtualEnvelope>(
                    r#"SELECT * FROM virtual_envelopes WHERE tenant_id = ?"#
                )
                .bind(tenant_id)
                .fetch_all(sqlite_pool)
                .await
                .map_err(|e| e.to_string())
            }
        }
    }
}
