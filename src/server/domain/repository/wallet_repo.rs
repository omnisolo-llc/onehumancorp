use std::sync::Arc;
use crate::db::{DB, DbStore};
use super::models::{Wallet, VirtualCard};

pub struct WalletRepository {
    db: Arc<DB>,
}

impl WalletRepository {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn get_wallet_by_tenant(&self, tenant_id: &str) -> Result<Option<Wallet>, String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_as::<_, Wallet>(
                    r#"
                    SELECT id, tenant_id, available_balance_cents, currency, created_at, updated_at
                    FROM ohc_wallet
                    WHERE tenant_id = $1
                    "#
                )
                .bind(tenant_id)
                .fetch_optional(&self.db.pool)
                .await
                .map_err(|e| e.to_string())
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as::<_, Wallet>(
                    r#"
                    SELECT id, tenant_id, available_balance_cents, currency, created_at, updated_at
                    FROM ohc_wallet
                    WHERE tenant_id = ?
                    "#
                )
                .bind(tenant_id)
                .fetch_optional(sqlite_pool)
                .await
                .map_err(|e| e.to_string())
            }
        }
    }

    pub async fn create_wallet(&self, wallet: Wallet) -> Result<Wallet, String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    r#"
                    INSERT INTO ohc_wallet (id, tenant_id, available_balance_cents, currency, created_at, updated_at)
                    VALUES ($1, $2, $3, $4, $5, $6)
                    "#
                )
                .bind(&wallet.id).bind(&wallet.tenant_id).bind(wallet.available_balance_cents).bind(&wallet.currency)
                .bind(&wallet.created_at).bind(&wallet.updated_at)
                .execute(&self.db.pool).await.map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO ohc_wallet (id, tenant_id, available_balance_cents, currency, created_at, updated_at)
                    VALUES (?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(&wallet.id).bind(&wallet.tenant_id).bind(wallet.available_balance_cents).bind(&wallet.currency)
                .bind(&wallet.created_at).bind(&wallet.updated_at)
                .execute(sqlite_pool).await.map_err(|e| e.to_string())?;
            }
        }
        Ok(wallet)
    }

    pub async fn update_wallet_balance(&self, tenant_id: &str, amount_cents: i64) -> Result<(), String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    r#"
                    UPDATE ohc_wallet
                    SET available_balance_cents = available_balance_cents + $1, updated_at = CURRENT_TIMESTAMP
                    WHERE tenant_id = $2
                    "#
                )
                .bind(amount_cents).bind(tenant_id)
                .execute(&self.db.pool).await.map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query(
                    r#"
                    UPDATE ohc_wallet
                    SET available_balance_cents = available_balance_cents + ?, updated_at = CURRENT_TIMESTAMP
                    WHERE tenant_id = ?
                    "#
                )
                .bind(amount_cents).bind(tenant_id)
                .execute(sqlite_pool).await.map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    pub async fn get_virtual_card_by_tenant(&self, tenant_id: &str) -> Result<Option<VirtualCard>, String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_as::<_, VirtualCard>(
                    r#"
                    SELECT id, wallet_id, tenant_id, status, tokenized_pan, last_four, expiry_month, expiry_year, cardholder_name, created_at, updated_at
                    FROM ohc_virtual_card
                    WHERE tenant_id = $1
                    "#
                )
                .bind(tenant_id)
                .fetch_optional(&self.db.pool)
                .await
                .map_err(|e| e.to_string())
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as::<_, VirtualCard>(
                    r#"
                    SELECT id, wallet_id, tenant_id, status, tokenized_pan, last_four, expiry_month, expiry_year, cardholder_name, created_at, updated_at
                    FROM ohc_virtual_card
                    WHERE tenant_id = ?
                    "#
                )
                .bind(tenant_id)
                .fetch_optional(sqlite_pool)
                .await
                .map_err(|e| e.to_string())
            }
        }
    }

    pub async fn create_virtual_card(&self, card: VirtualCard) -> Result<VirtualCard, String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    r#"
                    INSERT INTO ohc_virtual_card (id, wallet_id, tenant_id, status, tokenized_pan, last_four, expiry_month, expiry_year, cardholder_name, created_at, updated_at)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                    "#
                )
                .bind(&card.id).bind(&card.wallet_id).bind(&card.tenant_id).bind(&card.status)
                .bind(&card.tokenized_pan).bind(&card.last_four).bind(card.expiry_month).bind(card.expiry_year)
                .bind(&card.cardholder_name).bind(&card.created_at).bind(&card.updated_at)
                .execute(&self.db.pool).await.map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO ohc_virtual_card (id, wallet_id, tenant_id, status, tokenized_pan, last_four, expiry_month, expiry_year, cardholder_name, created_at, updated_at)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(&card.id).bind(&card.wallet_id).bind(&card.tenant_id).bind(&card.status)
                .bind(&card.tokenized_pan).bind(&card.last_four).bind(card.expiry_month).bind(card.expiry_year)
                .bind(&card.cardholder_name).bind(&card.created_at).bind(&card.updated_at)
                .execute(sqlite_pool).await.map_err(|e| e.to_string())?;
            }
        }
        Ok(card)
    }
}
