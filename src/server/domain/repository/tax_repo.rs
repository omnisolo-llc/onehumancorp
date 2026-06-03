use std::sync::Arc;
use crate::db::{DB, DbStore};
use super::models::{TaxJurisdiction, TaxLedgerEntry};
use chrono::Utc;
use uuid::Uuid;

pub struct TaxRepository {
    db: Arc<DB>,
}

impl TaxRepository {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn get_jurisdiction(&self, country_code: &str, region_code: Option<&str>) -> Result<Option<TaxJurisdiction>, String> {
        match &self.db.store {
            DbStore::Postgres => {
                let jurisdiction = if let Some(region) = region_code {
                    sqlx::query_as::<_, TaxJurisdiction>(
                        "SELECT * FROM tax_jurisdictions WHERE country_code = $1 AND region_code = $2"
                    )
                    .bind(country_code)
                    .bind(region)
                    .fetch_optional(&self.db.pool)
                    .await.map_err(|e| e.to_string())?
                } else {
                    sqlx::query_as::<_, TaxJurisdiction>(
                        "SELECT * FROM tax_jurisdictions WHERE country_code = $1 AND region_code IS NULL"
                    )
                    .bind(country_code)
                    .fetch_optional(&self.db.pool)
                    .await.map_err(|e| e.to_string())?
                };
                Ok(jurisdiction)
            },
            DbStore::Sqlite => {
                 let jurisdiction = if let Some(region) = region_code {
                    sqlx::query_as::<_, TaxJurisdiction>(
                        "SELECT * FROM tax_jurisdictions WHERE country_code = $1 AND region_code = $2"
                    )
                    .bind(country_code)
                    .bind(region)
                    .fetch_optional(&self.db.sqlite_pool)
                    .await.map_err(|e| e.to_string())?
                } else {
                    sqlx::query_as::<_, TaxJurisdiction>(
                        "SELECT * FROM tax_jurisdictions WHERE country_code = $1 AND region_code IS NULL"
                    )
                    .bind(country_code)
                    .fetch_optional(&self.db.sqlite_pool)
                    .await.map_err(|e| e.to_string())?
                };
                Ok(jurisdiction)
            }
        }
    }

    pub async fn add_ledger_entry(&self, entry: TaxLedgerEntry) -> Result<(), String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    r#"
                    INSERT INTO tax_ledgers (
                        id, tenant_id, transaction_id, jurisdiction_id, taxable_amount_cents,
                        tax_rate, tax_collected_cents, created_at
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                    "#
                )
                .bind(&entry.id).bind(&entry.tenant_id).bind(&entry.transaction_id)
                .bind(&entry.jurisdiction_id).bind(&entry.taxable_amount_cents)
                .bind(&entry.tax_rate).bind(&entry.tax_collected_cents)
                .bind(&entry.created_at)
                .execute(&self.db.pool).await.map_err(|e| e.to_string())?;
                Ok(())
            },
            DbStore::Sqlite => {
                sqlx::query(
                    r#"
                    INSERT INTO tax_ledgers (
                        id, tenant_id, transaction_id, jurisdiction_id, taxable_amount_cents,
                        tax_rate, tax_collected_cents, created_at
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                    "#
                )
                .bind(&entry.id).bind(&entry.tenant_id).bind(&entry.transaction_id)
                .bind(&entry.jurisdiction_id).bind(&entry.taxable_amount_cents)
                .bind(&entry.tax_rate.to_string()).bind(&entry.tax_collected_cents)
                .bind(&entry.created_at)
                .execute(&self.db.sqlite_pool).await.map_err(|e| e.to_string())?;
                Ok(())
            }
        }
    }

    pub async fn get_ledger_entries(&self, tenant_id: &str) -> Result<Vec<TaxLedgerEntry>, String> {
        match &self.db.store {
            DbStore::Postgres => {
                let entries = sqlx::query_as::<_, TaxLedgerEntry>(
                    "SELECT * FROM tax_ledgers WHERE tenant_id = $1 ORDER BY created_at DESC"
                )
                .bind(tenant_id)
                .fetch_all(&self.db.pool)
                .await.map_err(|e| e.to_string())?;
                Ok(entries)
            },
            DbStore::Sqlite => {
                let entries = sqlx::query_as::<_, TaxLedgerEntry>(
                    "SELECT * FROM tax_ledgers WHERE tenant_id = $1 ORDER BY created_at DESC"
                )
                .bind(tenant_id)
                .fetch_all(&self.db.sqlite_pool)
                .await.map_err(|e| e.to_string())?;
                Ok(entries)
            }
        }
    }
}
