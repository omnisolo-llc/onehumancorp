use std::sync::Arc;
use crate::db::{DB, DbStore};
use super::models::{TaxJurisdiction, TaxLedgerEntry};
use chrono::Utc;

pub struct TaxRepository {
    db: Arc<DB>,
}

impl TaxRepository {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn get_jurisdiction(
        &self,
        country_code: &str,
        state_code: Option<&str>,
        zip_code: Option<&str>,
    ) -> Result<Option<TaxJurisdiction>, String> {
        let mut query = String::from("SELECT * FROM tax_jurisdictions WHERE country_code = $1");

        let mut idx = 2;
        if state_code.is_some() {
            query.push_str(&format!(" AND state_code = ${}", idx));
            idx += 1;
        } else {
            query.push_str(" AND state_code IS NULL");
        }

        if zip_code.is_some() {
            query.push_str(&format!(" AND zip_code = ${}", idx));
        } else {
            query.push_str(" AND zip_code IS NULL");
        }

        match &self.db.store {
            DbStore::Postgres => {
                let mut q = sqlx::query_as::<_, TaxJurisdiction>(&query)
                    .bind(country_code);

                if let Some(state) = state_code {
                    q = q.bind(state);
                }
                if let Some(zip) = zip_code {
                    q = q.bind(zip);
                }

                q.fetch_optional(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())
            }
            DbStore::Sqlite(sqlite_pool) => {
                let mut query = String::from("SELECT * FROM tax_jurisdictions WHERE country_code = ?");
                if state_code.is_some() {
                    query.push_str(" AND state_code = ?");
                } else {
                    query.push_str(" AND state_code IS NULL");
                }

                if zip_code.is_some() {
                    query.push_str(" AND zip_code = ?");
                } else {
                    query.push_str(" AND zip_code IS NULL");
                }

                let mut q = sqlx::query_as::<_, TaxJurisdiction>(&query)
                    .bind(country_code);

                if let Some(state) = state_code {
                    q = q.bind(state);
                }
                if let Some(zip) = zip_code {
                    q = q.bind(zip);
                }

                q.fetch_optional(sqlite_pool)
                    .await
                    .map_err(|e| e.to_string())
            }
        }
    }

    pub async fn record_tax_ledger(&self, entry: TaxLedgerEntry) -> Result<(), String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    r#"
                    INSERT INTO tax_ledgers (id, tenant_id, transaction_id, jurisdiction_id, taxable_amount, tax_amount, product_category, collected_at)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                    "#
                )
                .bind(&entry.id)
                .bind(&entry.tenant_id)
                .bind(&entry.transaction_id)
                .bind(&entry.jurisdiction_id)
                .bind(&entry.taxable_amount)
                .bind(&entry.tax_amount)
                .bind(&entry.product_category)
                .bind(&entry.collected_at)
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
                Ok(())
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO tax_ledgers (id, tenant_id, transaction_id, jurisdiction_id, taxable_amount, tax_amount, product_category, collected_at)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(&entry.id)
                .bind(&entry.tenant_id)
                .bind(&entry.transaction_id)
                .bind(&entry.jurisdiction_id)
                .bind(&entry.taxable_amount)
                .bind(&entry.tax_amount)
                .bind(&entry.product_category)
                .bind(&entry.collected_at)
                .execute(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;
                Ok(())
            }
        }
    }

    pub async fn get_tenant_tax_ledgers(&self, tenant_id: &str) -> Result<Vec<TaxLedgerEntry>, String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_as::<_, TaxLedgerEntry>("SELECT * FROM tax_ledgers WHERE tenant_id = $1")
                    .bind(tenant_id)
                    .fetch_all(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as::<_, TaxLedgerEntry>("SELECT * FROM tax_ledgers WHERE tenant_id = ?")
                    .bind(tenant_id)
                    .fetch_all(sqlite_pool)
                    .await
                    .map_err(|e| e.to_string())
            }
        }
    }
}
