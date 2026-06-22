#[cfg(test)]
mod tests {
    use crate::workers::cfo_worker::CfoWorker;
    use crate::db::{DB, DbStore};
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::postgres::PgPoolOptions;
    use std::sync::Arc;

    async fn test_db() -> Arc<DB> {
        let sqlite_pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        let pg_pool = PgPoolOptions::new()
            .connect_lazy("postgres://dummy:dummy@localhost/dummy")
            .unwrap();

        Arc::new(DB {
            pool: pg_pool,
            store: DbStore::Sqlite(sqlite_pool),
        })
    }

    #[tokio::test]
    async fn test_cfo_worker_process() {
        let db = test_db().await;

        let worker = Arc::new(CfoWorker::new(db.clone()));

        let _result = worker.process_cashflow().await;
    }
}
