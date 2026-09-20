#[cfg(test)]
mod tests {
    use crate::db::{DB, DbStore};
    use crate::workers::invoice_followup_worker::prepare_reminders;

    #[tokio::test]
    async fn startup_spawns_immediately_and_exposes_a_cancellable_task() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::query("CREATE TABLE invoices(tenant_id TEXT)")
            .execute(&pool)
            .await
            .unwrap();
        let db = std::sync::Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new()
                .connect_lazy("postgres://unused:unused@127.0.0.1:1/unused")
                .unwrap(),
            store: DbStore::Sqlite(pool),
        });
        let task = crate::workers::invoice_followup_worker::start_invoice_followup_worker(db);
        tokio::task::yield_now().await;
        assert!(
            !task.is_finished(),
            "startup must return a running task, not a discarded future"
        );
        task.abort();
        assert!(task.await.unwrap_err().is_cancelled());
    }

    #[tokio::test]
    async fn reminder_drafts_are_real_deduplicated_scoped_and_retired_after_payment() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::query("CREATE TABLE invoices(id TEXT,tenant_id TEXT,client_id TEXT,currency TEXT,total_amount REAL,amount_paid_cents INTEGER,due_date INTEGER,payment_status TEXT,status TEXT)").execute(&pool).await.unwrap();
        sqlx::query("CREATE TABLE agent_feed_items(id TEXT PRIMARY KEY,tenant_id TEXT,event_source TEXT,context_payload TEXT,proposed_action TEXT,lifecycle_state TEXT,created_at TEXT,updated_at TEXT)").execute(&pool).await.unwrap();
        let db = DB {
            pool: sqlx::postgres::PgPoolOptions::new()
                .connect_lazy("postgres://invalid:invalid@127.0.0.1:1/unused")
                .unwrap(),
            store: DbStore::Sqlite(pool.clone()),
        };
        sqlx::query("INSERT INTO invoices VALUES('invoice-1','owner-a','client-1','USD',50.25,1000,100,'unpaid','sent'),('invoice-2','owner-b','client-2','USD',100,0,100,'unpaid','sent'),('draft','owner-a','client-1','USD',10,0,100,'unpaid','draft')").execute(&pool).await.unwrap();
        assert_eq!(prepare_reminders(&db, "owner-a", 200).await.unwrap(), 1);
        assert_eq!(prepare_reminders(&db, "owner-a", 200).await.unwrap(), 0);
        let (tenant, content, state): (String, String, String) = sqlx::query_as(
            "SELECT tenant_id,proposed_action,lifecycle_state FROM agent_feed_items",
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(tenant, "owner-a");
        assert_eq!(state, "PENDING_APPROVAL");
        let content: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(content["balance_cents"], 4025);
        assert_eq!(content["delivery_status"], "not_sent");
        assert!(
            content["generated_response"]
                .as_str()
                .unwrap()
                .contains("40.25")
        );
        sqlx::query("UPDATE invoices SET payment_status='paid',amount_paid_cents=5025 WHERE tenant_id='owner-a' AND id='invoice-1'").execute(&pool).await.unwrap();
        assert_eq!(prepare_reminders(&db, "owner-a", 300).await.unwrap(), 0);
        let state: String = sqlx::query_scalar("SELECT lifecycle_state FROM agent_feed_items")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(state, "CANCELLED");
        assert!(prepare_reminders(&db, "", 300).await.is_err());
    }
}
