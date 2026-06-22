#[cfg(test)]
mod tests {
    use axum::{body::Body, http::{Request, StatusCode}};
    use tower::ServiceExt;
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

        // In tests we typically use SQLite store and a dummy postgres pool if we don't connect
        let pg_pool = PgPoolOptions::new()
            .connect_lazy("postgres://dummy:dummy@localhost/dummy")
            .unwrap();

        Arc::new(DB {
            pool: pg_pool,
            store: DbStore::Sqlite(sqlite_pool),
        })
    }

    #[tokio::test]
    async fn test_cfo_projection() {
        // Just mock it since we only want to test routing
        let db = test_db().await;
        let app = super::super::cfo::router(db.clone());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/projection?tenant_id=test_tenant")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
