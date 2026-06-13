use std::sync::Arc;
use crate::db::{DB, DbStore};
use super::models::{PaymentRoutingRule, TransactionGroup, InvisibleLedgerEntry};
use chrono::Utc;
use uuid::Uuid;

pub struct InvisibleLedgerRepository {
    db: Arc<DB>,
}

impl InvisibleLedgerRepository {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn create_routing_rule(&self, rule: PaymentRoutingRule) -> Result<PaymentRoutingRule, String> {
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                sqlx::query(
                    r#"
                    INSERT INTO payment_routing_rules (
                        id, tenant_id, product_service_id, split_percentage, destination_party_id, created_at, updated_at
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7)
                    "#
                )
                .bind(&rule.id).bind(&rule.tenant_id).bind(&rule.product_service_id)
                .bind(&rule.split_percentage).bind(&rule.destination_party_id)
                .bind(&rule.created_at).bind(&rule.updated_at)
                .execute(&mut *tx).await.map_err(|e| e.to_string())?;
                tx.commit().await.map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;
                sqlx::query(
                    r#"
                    INSERT INTO payment_routing_rules (
                        id, tenant_id, product_service_id, split_percentage, destination_party_id, created_at, updated_at
                    ) VALUES (?, ?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(&rule.id).bind(&rule.tenant_id).bind(&rule.product_service_id)
                .bind(&rule.split_percentage).bind(&rule.destination_party_id)
                .bind(&rule.created_at).bind(&rule.updated_at)
                .execute(&mut *tx).await.map_err(|e| e.to_string())?;
                tx.commit().await.map_err(|e| e.to_string())?;
            }
        }
        Ok(rule)
    }

    pub async fn get_routing_rules(&self, tenant_id: &str, product_service_id: &str) -> Result<Vec<PaymentRoutingRule>, String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_as::<_, PaymentRoutingRule>(
                    r#"
                    SELECT id, tenant_id, product_service_id, split_percentage, destination_party_id, created_at, updated_at
                    FROM payment_routing_rules
                    WHERE tenant_id = $1 AND product_service_id = $2
                    "#
                )
                .bind(tenant_id).bind(product_service_id)
                .fetch_all(&self.db.pool).await.map_err(|e| e.to_string())
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as::<_, PaymentRoutingRule>(
                    r#"
                    SELECT id, tenant_id, product_service_id, split_percentage, destination_party_id, created_at, updated_at
                    FROM payment_routing_rules
                    WHERE tenant_id = ? AND product_service_id = ?
                    "#
                )
                .bind(tenant_id).bind(product_service_id)
                .fetch_all(sqlite_pool).await.map_err(|e| e.to_string())
            }
        }
    }

    pub async fn record_transaction_group(&self, group: TransactionGroup, total_amount: f64, source_party_id: &str, product_service_id: &str) -> Result<TransactionGroup, String> {
        let rules = self.get_routing_rules(&group.tenant_id, product_service_id).await?;
        let now = Utc::now();

        let mut entries = Vec::new();
        let mut retained_amount = total_amount;

        for rule in rules {
            let split_amount = total_amount * (rule.split_percentage / 100.0);
            retained_amount -= split_amount;

            entries.push(InvisibleLedgerEntry {
                id: Uuid::new_v4().to_string(),
                tenant_id: group.tenant_id.clone(),
                transaction_group_id: group.id.clone(),
                entry_type: "Contractor Payout".to_string(),
                amount: split_amount,
                currency: "USD".to_string(),
                source_party_id: source_party_id.to_string(),
                destination_party_id: rule.destination_party_id.clone(),
                status: Some("QUEUED".to_string()),
                created_at: Some(now),
            });
        }

        // Retained margin entry
        entries.push(InvisibleLedgerEntry {
            id: Uuid::new_v4().to_string(),
            tenant_id: group.tenant_id.clone(),
            transaction_group_id: group.id.clone(),
            entry_type: "Retained Margin".to_string(),
            amount: retained_amount,
            currency: "USD".to_string(),
            source_party_id: source_party_id.to_string(),
            destination_party_id: "OWNER".to_string(),
            status: Some("CLEARED".to_string()),
            created_at: Some(now),
        });

        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                sqlx::query(
                    r#"
                    INSERT INTO transaction_groups (
                        id, tenant_id, reference_type, reference_id, status, created_at, updated_at
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7)
                    "#
                )
                .bind(&group.id).bind(&group.tenant_id).bind(&group.reference_type)
                .bind(&group.reference_id).bind(&group.status).bind(&group.created_at).bind(&group.updated_at)
                .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                for entry in entries {
                    sqlx::query(
                        r#"
                        INSERT INTO invisible_ledger_entries (
                            id, tenant_id, transaction_group_id, entry_type, amount, currency, source_party_id, destination_party_id, status, created_at
                        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                        "#
                    )
                    .bind(&entry.id).bind(&entry.tenant_id).bind(&entry.transaction_group_id)
                    .bind(&entry.entry_type).bind(&entry.amount).bind(&entry.currency)
                    .bind(&entry.source_party_id).bind(&entry.destination_party_id)
                    .bind(&entry.status).bind(&entry.created_at)
                    .execute(&mut *tx).await.map_err(|e| e.to_string())?;
                }

                tx.commit().await.map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;

                sqlx::query(
                    r#"
                    INSERT INTO transaction_groups (
                        id, tenant_id, reference_type, reference_id, status, created_at, updated_at
                    ) VALUES (?, ?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(&group.id).bind(&group.tenant_id).bind(&group.reference_type)
                .bind(&group.reference_id).bind(&group.status).bind(&group.created_at).bind(&group.updated_at)
                .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                for entry in entries {
                    sqlx::query(
                        r#"
                        INSERT INTO invisible_ledger_entries (
                            id, tenant_id, transaction_group_id, entry_type, amount, currency, source_party_id, destination_party_id, status, created_at
                        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                        "#
                    )
                    .bind(&entry.id).bind(&entry.tenant_id).bind(&entry.transaction_group_id)
                    .bind(&entry.entry_type).bind(&entry.amount).bind(&entry.currency)
                    .bind(&entry.source_party_id).bind(&entry.destination_party_id)
                    .bind(&entry.status).bind(&entry.created_at)
                    .execute(&mut *tx).await.map_err(|e| e.to_string())?;
                }

                tx.commit().await.map_err(|e| e.to_string())?;
            }
        }
        Ok(group)
    }

    pub async fn get_tenant_balances(&self, tenant_id: &str) -> Result<Vec<InvisibleLedgerEntry>, String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_as::<_, InvisibleLedgerEntry>(
                    r#"
                    SELECT id, tenant_id, transaction_group_id, entry_type, amount, currency, source_party_id, destination_party_id, status, created_at
                    FROM invisible_ledger_entries
                    WHERE tenant_id = $1
                    "#
                )
                .bind(tenant_id)
                .fetch_all(&self.db.pool).await.map_err(|e| e.to_string())
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as::<_, InvisibleLedgerEntry>(
                    r#"
                    SELECT id, tenant_id, transaction_group_id, entry_type, amount, currency, source_party_id, destination_party_id, status, created_at
                    FROM invisible_ledger_entries
                    WHERE tenant_id = ?
                    "#
                )
                .bind(tenant_id)
                .fetch_all(sqlite_pool).await.map_err(|e| e.to_string())
            }
        }
    }
}

#[cfg(test)]
mod invisible_ledger_tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;
    use crate::db::DbStore;

    async fn setup_test_db() -> Arc<DB> {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            r#"
            CREATE TABLE payment_routing_rules (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                product_service_id TEXT NOT NULL,
                split_percentage REAL NOT NULL,
                destination_party_id TEXT NOT NULL,
                created_at TEXT,
                updated_at TEXT
            );

            CREATE TABLE transaction_groups (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                reference_type TEXT NOT NULL,
                reference_id TEXT NOT NULL,
                status TEXT,
                created_at TEXT,
                updated_at TEXT
            );

            CREATE TABLE invisible_ledger_entries (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                transaction_group_id TEXT NOT NULL,
                entry_type TEXT NOT NULL,
                amount REAL NOT NULL,
                currency TEXT NOT NULL,
                source_party_id TEXT NOT NULL,
                destination_party_id TEXT NOT NULL,
                status TEXT,
                created_at TEXT
            );
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        let pg_pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
            .unwrap();

        Arc::new(DB {
            pool: pg_pool,
            store: DbStore::Sqlite(pool),
        })
    }

    #[tokio::test]
    async fn test_invisible_ledger_split_scenario() {
        let db = setup_test_db().await;
        let repo = InvisibleLedgerRepository::new(db);

        // 1. Nora sets up a routing rule: 70% to Alex for "Design Work"
        let rule = PaymentRoutingRule {
            id: Uuid::new_v4().to_string(),
            tenant_id: "nora_tenant".to_string(),
            product_service_id: "design_work".to_string(),
            split_percentage: 70.0,
            destination_party_id: "alex_contractor".to_string(),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        repo.create_routing_rule(rule).await.unwrap();

        // 2. Client pays $1000
        let tx_group = TransactionGroup {
            id: Uuid::new_v4().to_string(),
            tenant_id: "nora_tenant".to_string(),
            reference_type: "INVOICE".to_string(),
            reference_id: "inv_123".to_string(),
            status: Some("PAID".to_string()),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };

        repo.record_transaction_group(tx_group, 1000.0, "client_abc", "design_work").await.unwrap();

        // 3. Verify ledger entries
        let entries = repo.get_tenant_balances("nora_tenant").await.unwrap();

        assert_eq!(entries.len(), 2);

        let alex_entry = entries.iter().find(|e| e.destination_party_id == "alex_contractor").unwrap();
        assert_eq!(alex_entry.amount, 700.0);
        assert_eq!(alex_entry.status, Some("QUEUED".to_string()));

        let owner_entry = entries.iter().find(|e| e.destination_party_id == "OWNER").unwrap();
        assert_eq!(owner_entry.amount, 300.0);
        assert_eq!(owner_entry.status, Some("CLEARED".to_string()));
    }

    #[tokio::test]
    async fn test_invisible_ledger_tenant_isolation() {
        let db = setup_test_db().await;
        let repo = InvisibleLedgerRepository::new(db);

        let tx_group1 = TransactionGroup {
            id: Uuid::new_v4().to_string(),
            tenant_id: "tenant_A".to_string(),
            reference_type: "INVOICE".to_string(),
            reference_id: "inv_A".to_string(),
            status: Some("PAID".to_string()),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        repo.record_transaction_group(tx_group1, 100.0, "client_A", "service_A").await.unwrap();

        let tx_group2 = TransactionGroup {
            id: Uuid::new_v4().to_string(),
            tenant_id: "tenant_B".to_string(),
            reference_type: "INVOICE".to_string(),
            reference_id: "inv_B".to_string(),
            status: Some("PAID".to_string()),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        repo.record_transaction_group(tx_group2, 200.0, "client_B", "service_B").await.unwrap();

        let entries_a = repo.get_tenant_balances("tenant_A").await.unwrap();
        assert_eq!(entries_a.len(), 1); // 1 owner entry (no routing rules)
        assert_eq!(entries_a[0].amount, 100.0);

        let entries_b = repo.get_tenant_balances("tenant_B").await.unwrap();
        assert_eq!(entries_b.len(), 1); // 1 owner entry
        assert_eq!(entries_b[0].amount, 200.0);
    }
}
