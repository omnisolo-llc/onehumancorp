use std::sync::Arc;
use crate::db::{DB, DbStore};
use super::models::{Invoice, InvoiceLineItem, PaymentEvent, LedgerEntry};
use chrono::Utc;
use uuid::Uuid;

pub struct LedgerRepository {
    db: Arc<DB>,
}

impl LedgerRepository {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn create_invoice(&self, invoice: Invoice, items: Vec<InvoiceLineItem>) -> Result<Invoice, String> {
        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                sqlx::query(
                    r#"
                    INSERT INTO invoices (
                        id, tenant_id, customer_id, status, due_date,
                        total_amount, currency, tax_nexus, split_partner_id, split_percentage, created_at, updated_at
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
                    "#
                )
                .bind(&invoice.id).bind(&invoice.tenant_id).bind(&invoice.customer_id).bind(&invoice.status)
                .bind(&invoice.due_date).bind(&invoice.total_amount).bind(&invoice.currency).bind(&invoice.tax_nexus)
                .bind(&invoice.split_partner_id).bind(&invoice.split_percentage)
                .bind(&invoice.created_at).bind(&invoice.updated_at)
                .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                for item in items {
                    sqlx::query(
                        r#"
                        INSERT INTO invoice_line_items (
                            id, tenant_id, invoice_id, description, quantity, unit_price, amount, created_at
                        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                        "#
                    )
                    .bind(&item.id).bind(&item.tenant_id).bind(&item.invoice_id).bind(&item.description)
                    .bind(&item.quantity).bind(&item.unit_price).bind(&item.amount).bind(&item.created_at)
                    .execute(&mut *tx).await.map_err(|e| e.to_string())?;
                }
                tx.commit().await.map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;
                sqlx::query(
                    r#"
                    INSERT INTO invoices (
                        id, tenant_id, customer_id, status, due_date,
                        total_amount, currency, tax_nexus, split_partner_id, split_percentage, created_at, updated_at
                    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(&invoice.id).bind(&invoice.tenant_id).bind(&invoice.customer_id).bind(&invoice.status)
                .bind(&invoice.due_date).bind(&invoice.total_amount).bind(&invoice.currency).bind(&invoice.tax_nexus)
                .bind(&invoice.split_partner_id).bind(&invoice.split_percentage)
                .bind(&invoice.created_at).bind(&invoice.updated_at)
                .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                for item in items {
                    sqlx::query(
                        r#"
                        INSERT INTO invoice_line_items (
                            id, tenant_id, invoice_id, description, quantity, unit_price, amount, created_at
                        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                        "#
                    )
                    .bind(&item.id).bind(&item.tenant_id).bind(&item.invoice_id).bind(&item.description)
                    .bind(&item.quantity).bind(&item.unit_price).bind(&item.amount).bind(&item.created_at)
                    .execute(&mut *tx).await.map_err(|e| e.to_string())?;
                }
                tx.commit().await.map_err(|e| e.to_string())?;
            }
        }
        Ok(invoice)
    }

    pub async fn get_invoice(&self, tenant_id: &str, invoice_id: &str) -> Result<Option<Invoice>, String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_as::<_, Invoice>(
                    r#"
                    SELECT id, tenant_id, customer_id, status, due_date, total_amount, currency, tax_nexus, split_partner_id, split_percentage, created_at, updated_at
                    FROM invoices
                    WHERE tenant_id = $1 AND id = $2
                    "#
                )
                .bind(tenant_id)
                .bind(invoice_id)
                .fetch_optional(&self.db.pool)
                .await
                .map_err(|e| e.to_string())
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as::<_, Invoice>(
                    r#"
                    SELECT id, tenant_id, customer_id, status, due_date, total_amount, currency, tax_nexus, split_partner_id, split_percentage, created_at, updated_at
                    FROM invoices
                    WHERE tenant_id = ? AND id = ?
                    "#
                )
                .bind(tenant_id)
                .bind(invoice_id)
                .fetch_optional(sqlite_pool)
                .await
                .map_err(|e| e.to_string())
            }
        }
    }

    pub async fn update_invoice_status(&self, tenant_id: &str, invoice_id: &str, status: &str) -> Result<(), String> {
        let now = Utc::now();
        match &self.db.store {
            DbStore::Postgres => {
                let result = sqlx::query(
                    r#"
                    UPDATE invoices SET payment_status = $1, updated_at = $2
                    WHERE tenant_id = $3 AND id = $4
                    RETURNING id
                    "#
                )
                .bind(status)
                .bind(now)
                .bind(tenant_id)
                .bind(invoice_id)
                .fetch_optional(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
                if result.is_none() {
                    return Err("Invoice not found or does not belong to tenant".to_string());
                }
            }
            DbStore::Sqlite(sqlite_pool) => {
                let result = sqlx::query(
                    r#"
                    UPDATE invoices SET payment_status = ?, updated_at = ?
                    WHERE tenant_id = ? AND id = ?
                    RETURNING id
                    "#
                )
                .bind(status)
                .bind(now)
                .bind(tenant_id)
                .bind(invoice_id)
                .fetch_optional(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;
                if result.is_none() {
                    return Err("Invoice not found or does not belong to tenant".to_string());
                }
            }
        }
        Ok(())
    }

    pub async fn apply_payment_event(&self, event: PaymentEvent) -> Result<(), String> {
        let credit_entry_id = Uuid::new_v4().to_string();
        let debit_entry_id = Uuid::new_v4().to_string();
        let now = Utc::now();

        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                sqlx::query(
                    r#"INSERT INTO payment_events (id, tenant_id, invoice_id, amount, method, completed_at, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7)"#
                )
                .bind(&event.id).bind(&event.tenant_id).bind(&event.invoice_id).bind(&event.amount).bind(&event.method).bind(&event.completed_at).bind(&event.created_at)
                .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                sqlx::query(r#"INSERT INTO ledger_entries (id, tenant_id, payment_event_id, credit, debit, entry_type, posted_at, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#)
                .bind(&credit_entry_id).bind(&event.tenant_id).bind(&event.id).bind(event.amount).bind(0.0).bind("Revenue").bind(now).bind(now)
                .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                sqlx::query(r#"INSERT INTO ledger_entries (id, tenant_id, payment_event_id, credit, debit, entry_type, posted_at, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#)
                .bind(&debit_entry_id).bind(&event.tenant_id).bind(&event.id).bind(0.0).bind(event.amount).bind("Cash").bind(now).bind(now)
                .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                let invoice = sqlx::query_as::<_, Invoice>(
                    r#"SELECT * FROM invoices WHERE tenant_id = $1 AND id = $2"#
                )
                .bind(&event.tenant_id).bind(&event.invoice_id)
                .fetch_optional(&mut *tx)
                .await.map_err(|e| e.to_string())?;

                if let Some(inv) = invoice {
                    if let (Some(partner_id), Some(pct)) = (inv.split_partner_id, inv.split_percentage) {
                        let partner_amount = event.amount * (pct / 100.0);
                        let partner_credit_id = Uuid::new_v4().to_string();
                        let partner_debit_id = Uuid::new_v4().to_string();

                        sqlx::query(r#"INSERT INTO ledger_entries (id, tenant_id, payment_event_id, credit, debit, entry_type, posted_at, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#)
                        .bind(&partner_credit_id).bind(&event.tenant_id).bind(&event.id).bind(partner_amount).bind(0.0).bind(format!("Payable to {}", partner_id)).bind(now).bind(now)
                        .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                        sqlx::query(r#"INSERT INTO ledger_entries (id, tenant_id, payment_event_id, credit, debit, entry_type, posted_at, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#)
                        .bind(&partner_debit_id).bind(&event.tenant_id).bind(&event.id).bind(0.0).bind(partner_amount).bind("Revenue Split").bind(now).bind(now)
                        .execute(&mut *tx).await.map_err(|e| e.to_string())?;
                    }
                }

                let invoice = sqlx::query_as::<_, Invoice>(
                    r#"SELECT * FROM invoices WHERE tenant_id = $1 AND id = $2"#
                )
                .bind(&event.tenant_id).bind(&event.invoice_id)
                .fetch_optional(&mut *tx)
                .await.map_err(|e| e.to_string())?;

                if let Some(inv) = invoice {
                    if let (Some(partner_id), Some(pct)) = (inv.split_partner_id, inv.split_percentage) {
                        let partner_amount = event.amount * (pct / 100.0);
                        let partner_credit_id = Uuid::new_v4().to_string();
                        let partner_debit_id = Uuid::new_v4().to_string();

                        sqlx::query(r#"INSERT INTO ledger_entries (id, tenant_id, payment_event_id, credit, debit, entry_type, posted_at, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#)
                        .bind(&partner_credit_id).bind(&event.tenant_id).bind(&event.id).bind(partner_amount).bind(0.0).bind(format!("Payable to {}", partner_id)).bind(now).bind(now)
                        .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                        sqlx::query(r#"INSERT INTO ledger_entries (id, tenant_id, payment_event_id, credit, debit, entry_type, posted_at, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#)
                        .bind(&partner_debit_id).bind(&event.tenant_id).bind(&event.id).bind(0.0).bind(partner_amount).bind("Revenue Split").bind(now).bind(now)
                        .execute(&mut *tx).await.map_err(|e| e.to_string())?;
                    }
                }

                sqlx::query(r#"UPDATE invoices SET payment_status = 'paid', updated_at = $1 WHERE tenant_id = $2 AND id = $3"#)
                .bind(now).bind(&event.tenant_id).bind(&event.invoice_id)
                .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;
                sqlx::query(
                    r#"INSERT INTO payment_events (id, tenant_id, invoice_id, amount, method, completed_at, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)"#
                )
                .bind(&event.id).bind(&event.tenant_id).bind(&event.invoice_id).bind(&event.amount).bind(&event.method).bind(&event.completed_at).bind(&event.created_at)
                .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                sqlx::query(r#"INSERT INTO ledger_entries (id, tenant_id, payment_event_id, credit, debit, entry_type, posted_at, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#)
                .bind(&credit_entry_id).bind(&event.tenant_id).bind(&event.id).bind(event.amount).bind(0.0).bind("Revenue").bind(now).bind(now)
                .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                sqlx::query(r#"INSERT INTO ledger_entries (id, tenant_id, payment_event_id, credit, debit, entry_type, posted_at, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#)
                .bind(&debit_entry_id).bind(&event.tenant_id).bind(&event.id).bind(0.0).bind(event.amount).bind("Cash").bind(now).bind(now)
                .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                let invoice = sqlx::query_as::<_, Invoice>(
                    r#"SELECT * FROM invoices WHERE tenant_id = ? AND id = ?"#
                )
                .bind(&event.tenant_id).bind(&event.invoice_id)
                .fetch_optional(&mut *tx)
                .await.map_err(|e| e.to_string())?;

                if let Some(inv) = invoice {
                    if let (Some(partner_id), Some(pct)) = (inv.split_partner_id, inv.split_percentage) {
                        let partner_amount = event.amount * (pct / 100.0);
                        let partner_credit_id = Uuid::new_v4().to_string();
                        let partner_debit_id = Uuid::new_v4().to_string();

                        sqlx::query(r#"INSERT INTO ledger_entries (id, tenant_id, payment_event_id, credit, debit, entry_type, posted_at, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#)
                        .bind(&partner_credit_id).bind(&event.tenant_id).bind(&event.id).bind(partner_amount).bind(0.0).bind(format!("Payable to {}", partner_id)).bind(now).bind(now)
                        .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                        sqlx::query(r#"INSERT INTO ledger_entries (id, tenant_id, payment_event_id, credit, debit, entry_type, posted_at, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#)
                        .bind(&partner_debit_id).bind(&event.tenant_id).bind(&event.id).bind(0.0).bind(partner_amount).bind("Revenue Split").bind(now).bind(now)
                        .execute(&mut *tx).await.map_err(|e| e.to_string())?;
                    }
                }

                let invoice = sqlx::query_as::<_, Invoice>(
                    r#"SELECT * FROM invoices WHERE tenant_id = ? AND id = ?"#
                )
                .bind(&event.tenant_id).bind(&event.invoice_id)
                .fetch_optional(&mut *tx)
                .await.map_err(|e| e.to_string())?;

                if let Some(inv) = invoice {
                    if let (Some(partner_id), Some(pct)) = (inv.split_partner_id, inv.split_percentage) {
                        let partner_amount = event.amount * (pct / 100.0);
                        let partner_credit_id = Uuid::new_v4().to_string();
                        let partner_debit_id = Uuid::new_v4().to_string();

                        sqlx::query(r#"INSERT INTO ledger_entries (id, tenant_id, payment_event_id, credit, debit, entry_type, posted_at, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#)
                        .bind(&partner_credit_id).bind(&event.tenant_id).bind(&event.id).bind(partner_amount).bind(0.0).bind(format!("Payable to {}", partner_id)).bind(now).bind(now)
                        .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                        sqlx::query(r#"INSERT INTO ledger_entries (id, tenant_id, payment_event_id, credit, debit, entry_type, posted_at, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#)
                        .bind(&partner_debit_id).bind(&event.tenant_id).bind(&event.id).bind(0.0).bind(partner_amount).bind("Revenue Split").bind(now).bind(now)
                        .execute(&mut *tx).await.map_err(|e| e.to_string())?;
                    }
                }

                sqlx::query(r#"UPDATE invoices SET payment_status = 'paid', updated_at = ? WHERE tenant_id = ? AND id = ?"#)
                .bind(now).bind(&event.tenant_id).bind(&event.invoice_id)
                .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    pub async fn get_ledger_entries(&self, tenant_id: &str) -> Result<Vec<LedgerEntry>, String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_as::<_, LedgerEntry>(
                    r#"
                    SELECT id, tenant_id, payment_event_id, credit, debit, entry_type, posted_at, created_at
                    FROM ledger_entries
                    WHERE tenant_id = $1
                    "#
                )
                .bind(tenant_id)
                .fetch_all(&self.db.pool)
                .await
                .map_err(|e| e.to_string())
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as::<_, LedgerEntry>(
                    r#"
                    SELECT id, tenant_id, payment_event_id, credit, debit, entry_type, posted_at, created_at
                    FROM ledger_entries
                    WHERE tenant_id = ?
                    "#
                )
                .bind(tenant_id)
                .fetch_all(sqlite_pool)
                .await
                .map_err(|e| e.to_string())
            }
        }
    }
}

#[cfg(test)]
mod ledger_tests {
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
            CREATE TABLE invoices (
                id TEXT PRIMARY KEY,
                tenant_id TEXT,
                customer_id TEXT,
                status TEXT,
                due_date TEXT,
                total_amount REAL,
                currency TEXT,
                tax_nexus TEXT,
                split_partner_id TEXT,
                split_percentage REAL,
                created_at TEXT,
                updated_at TEXT
            );

            CREATE TABLE invoice_line_items (
                id TEXT PRIMARY KEY,
                tenant_id TEXT,
                invoice_id TEXT,
                description TEXT,
                quantity INTEGER,
                unit_price REAL,
                amount REAL,
                created_at TEXT
            );

            CREATE TABLE payment_events (
                id TEXT PRIMARY KEY,
                tenant_id TEXT,
                invoice_id TEXT,
                amount REAL,
                method TEXT,
                completed_at TEXT,
                created_at TEXT
            );

            CREATE TABLE ledger_entries (
                id TEXT PRIMARY KEY,
                tenant_id TEXT,
                payment_event_id TEXT,
                credit REAL,
                debit REAL,
                entry_type TEXT,
                posted_at TEXT,
                created_at TEXT
            );
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        let pg_pool = crate::db::secure_pg_pool_options()
            .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
            .unwrap();

        Arc::new(DB {
            pool: pg_pool,
            store: DbStore::Sqlite(pool),
        })
    }

    #[tokio::test]
    async fn test_create_invoice_success() {
        let db = setup_test_db().await;
        let repo = LedgerRepository::new(db);

        let invoice = Invoice {
            id: "inv_1".into(),
            tenant_id: "tenant_1".into(),
            customer_id: "cust_1".into(),
            status: Some("Draft".into()),
            due_date: Some(Utc::now()),
            total_amount: Some(100.0),
            currency: Some("USD".into()),
            tax_nexus: None,
            split_partner_id: None,
            split_percentage: None,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };

        let items = vec![InvoiceLineItem {
            id: "item_1".into(),
            tenant_id: "tenant_1".into(),
            invoice_id: "inv_1".into(),
            description: "Service".into(),
            quantity: Some(1),
            unit_price: Some(100.0),
            amount: Some(100.0),
            created_at: Some(Utc::now()),
        }];

        let result = repo.create_invoice(invoice.clone(), items).await;
        assert!(result.is_ok());

        let fetched = repo.get_invoice("tenant_1", "inv_1").await.unwrap();
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().id, "inv_1");
    }

    #[tokio::test]
    async fn test_apply_payment_updates_ledger_entries() {
        let db = setup_test_db().await;
        let repo = LedgerRepository::new(db);

        let invoice = Invoice {
            id: "inv_2".into(),
            tenant_id: "tenant_2".into(),
            customer_id: "cust_2".into(),
            status: Some("Sent".into()),
            due_date: Some(Utc::now()),
            total_amount: Some(250.0),
            currency: Some("USD".into()),
            tax_nexus: None,
            split_partner_id: None,
            split_percentage: None,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        repo.create_invoice(invoice, vec![]).await.unwrap();

        let event = PaymentEvent {
            id: "evt_1".into(),
            tenant_id: "tenant_2".into(),
            invoice_id: "inv_2".into(),
            amount: 250.0,
            method: "Card".into(),
            completed_at: Some(Utc::now()),
            created_at: Some(Utc::now()),
        };

        repo.apply_payment_event(event).await.unwrap();

        let fetched = repo.get_invoice("tenant_2", "inv_2").await.unwrap().unwrap();
        assert_eq!(fetched.status.unwrap(), "Paid");

        let entries = repo.get_ledger_entries("tenant_2").await.unwrap();
        assert_eq!(entries.len(), 2);
    }

    #[tokio::test]
    async fn test_get_ledger_entries_fetches_tenant_records_only() {
        let db = setup_test_db().await;
        let repo = LedgerRepository::new(db);

        let event1 = PaymentEvent {
            id: "evt_3".into(),
            tenant_id: "tenant_3".into(),
            invoice_id: "inv_3".into(),
            amount: 50.0,
            method: "Cash".into(),
            completed_at: Some(Utc::now()),
            created_at: Some(Utc::now()),
        };
        let event2 = PaymentEvent {
            id: "evt_4".into(),
            tenant_id: "tenant_4".into(),
            invoice_id: "inv_4".into(),
            amount: 70.0,
            method: "Cash".into(),
            completed_at: Some(Utc::now()),
            created_at: Some(Utc::now()),
        };

        let invoice1 = Invoice {
            id: "inv_3".into(),
            tenant_id: "tenant_3".into(),
            customer_id: "cust_3".into(),
            status: Some("Sent".into()),
            due_date: Some(Utc::now()),
            total_amount: Some(50.0),
            currency: Some("USD".into()),
            tax_nexus: None,
            split_partner_id: None,
            split_percentage: None,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        repo.create_invoice(invoice1, vec![]).await.unwrap();

        let invoice2 = Invoice {
            id: "inv_4".into(),
            tenant_id: "tenant_4".into(),
            customer_id: "cust_4".into(),
            status: Some("Sent".into()),
            due_date: Some(Utc::now()),
            total_amount: Some(70.0),
            currency: Some("USD".into()),
            tax_nexus: None,
            split_partner_id: None,
            split_percentage: None,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        repo.create_invoice(invoice2, vec![]).await.unwrap();

        repo.apply_payment_event(event1).await.unwrap();
        repo.apply_payment_event(event2).await.unwrap();

        let entries_tenant3 = repo.get_ledger_entries("tenant_3").await.unwrap();
        assert_eq!(entries_tenant3.len(), 2);

        let entries_tenant4 = repo.get_ledger_entries("tenant_4").await.unwrap();
        assert_eq!(entries_tenant4.len(), 2);
    }

    #[tokio::test]
    async fn test_apply_payment_with_split() {
        let db = setup_test_db().await;
        let repo = LedgerRepository::new(db);

        let invoice = Invoice {
            id: "inv_split".into(),
            tenant_id: "tenant_split".into(),
            customer_id: "cust_split".into(),
            status: Some("Sent".into()),
            due_date: Some(Utc::now()),
            total_amount: Some(100.0),
            currency: Some("USD".into()),
            tax_nexus: None,
            split_partner_id: Some("Sarah".into()),
            split_percentage: Some(70.0),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        repo.create_invoice(invoice, vec![]).await.unwrap();

        let event = PaymentEvent {
            id: "evt_split".into(),
            tenant_id: "tenant_split".into(),
            invoice_id: "inv_split".into(),
            amount: 100.0,
            method: "Card".into(),
            completed_at: Some(Utc::now()),
            created_at: Some(Utc::now()),
        };

        repo.apply_payment_event(event).await.unwrap();

        let entries = repo.get_ledger_entries("tenant_split").await.unwrap();
        // Should have 4 entries: Revenue, Cash, Payable to Sarah, Revenue Split
        assert_eq!(entries.len(), 4);

        let partner_credit = entries.iter().find(|e| e.entry_type == "Payable to Sarah").unwrap();
        assert_eq!(partner_credit.credit, 70.0);

        let split_debit = entries.iter().find(|e| e.entry_type == "Revenue Split").unwrap();
        assert_eq!(split_debit.debit, 70.0);
    }

    #[tokio::test]
    async fn test_apply_payment_with_split() {
        let db = setup_test_db().await;
        let repo = LedgerRepository::new(db);

        let invoice = Invoice {
            id: "inv_split".into(),
            tenant_id: "tenant_split".into(),
            customer_id: "cust_split".into(),
            status: Some("Sent".into()),
            due_date: Some(Utc::now()),
            total_amount: Some(100.0),
            currency: Some("USD".into()),
            tax_nexus: None,
            split_partner_id: Some("Sarah".into()),
            split_percentage: Some(70.0),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        repo.create_invoice(invoice, vec![]).await.unwrap();

        let event = PaymentEvent {
            id: "evt_split".into(),
            tenant_id: "tenant_split".into(),
            invoice_id: "inv_split".into(),
            amount: 100.0,
            method: "Card".into(),
            completed_at: Some(Utc::now()),
            created_at: Some(Utc::now()),
        };

        repo.apply_payment_event(event).await.unwrap();

        let entries = repo.get_ledger_entries("tenant_split").await.unwrap();
        // Should have 4 entries: Revenue, Cash, Payable to Sarah, Revenue Split
        assert_eq!(entries.len(), 4);

        let partner_credit = entries.iter().find(|e| e.entry_type == "Payable to Sarah").unwrap();
        assert_eq!(partner_credit.credit, 70.0);

        let split_debit = entries.iter().find(|e| e.entry_type == "Revenue Split").unwrap();
        assert_eq!(split_debit.debit, 70.0);
    }
}


    pub async fn create_ledger_account(&self, tenant_id: &str, currency: &str) -> Result<super::models::LedgerAccount, String> {
        let account_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let account = super::models::LedgerAccount {
            id: account_id.clone(),
            tenant_id: tenant_id.to_string(),
            currency: currency.to_string(),
            current_balance: 0.0,
            last_synced: Some(now),
            created_at: Some(now),
            updated_at: Some(now),
        };

        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    r#"
                    INSERT INTO ledger_account (
                        id, tenant_id, currency, current_balance, last_synced, created_at, updated_at
                    ) VALUES ($1::uuid, $2::uuid, $3, $4, $5, $6, $7)
                    "#
                )
                .bind(Uuid::parse_str(&account.id).unwrap())
                .bind(Uuid::parse_str(&account.tenant_id).unwrap())
                .bind(&account.currency)
                .bind(&account.current_balance)
                .bind(&account.last_synced)
                .bind(&account.created_at)
                .bind(&account.updated_at)
                .execute(&self.db.pool).await.map_err(|e| e.to_string())?;
                Ok(account)
            }
            DbStore::Sqlite(_sqlite_pool) => {
                // SQLite mock execution
                Ok(account)
            }
        }
    }

    pub async fn add_ledger_entry(&self, tenant_id: &str, account_id: &str, amount: f64, entry_type: &str, idempotency_key: Option<&str>) -> Result<LedgerEntry, String> {
        let entry_id = Uuid::new_v4().to_string();
        let now = Utc::now();

        let entry = LedgerEntry {
            id: entry_id.clone(),
            tenant_id: tenant_id.to_string(),
            account_id: account_id.to_string(),
            amount,
            entry_type: entry_type.to_string(),
            reference_id: idempotency_key.unwrap_or("").to_string(),
            created_at: Some(now),
        };

        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                sqlx::query(
                    r#"
                    INSERT INTO ledger_entry (
                        id, ledger_account_id, amount, type, idempotency_key, created_at
                    ) VALUES ($1::uuid, $2::uuid, $3, $4, $5, $6)
                    "#
                )
                .bind(Uuid::parse_str(&entry.id).unwrap())
                .bind(Uuid::parse_str(&entry.account_id).unwrap())
                .bind(&entry.amount)
                .bind(&entry.entry_type)
                .bind(idempotency_key)
                .bind(&entry.created_at)
                .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                sqlx::query(
                    r#"
                    UPDATE ledger_account
                    SET current_balance = current_balance + $1, updated_at = $2
                    WHERE id = $3::uuid AND tenant_id = $4::uuid
                    "#
                )
                .bind(&entry.amount)
                .bind(&entry.created_at)
                .bind(Uuid::parse_str(&entry.account_id).unwrap())
                .bind(Uuid::parse_str(&entry.tenant_id).unwrap())
                .execute(&mut *tx).await.map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;

                Ok(entry)
            }
            DbStore::Sqlite(_sqlite_pool) => {
                // SQLite mock execution
                Ok(entry)
            }
        }
    }
}
