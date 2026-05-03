#[cfg(test)]
mod tests {
    use super::super::mcp_webhook::{mcp_webhook_handler, WebhookPayload, WebhookState, router};
    use axum::extract::{Path, State};
    use axum::Json;
    use axum::http::HeaderMap;
    use crate::db::{DB, DbStore};
    use std::sync::Arc;
    use axum::response::IntoResponse;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_mcp_webhook_handler_postgres() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }
        let database_url = "postgres://postgres:postgres@localhost:5432/test";
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(database_url)
            .await
            .unwrap();

        let db = Arc::new(DB {
            pool: pool.clone(),
            store: DbStore::Postgres,
        });

        let state = WebhookState {
            db_pool: db,
            secret: "test_secret".to_string(),
        };

        let payload = WebhookPayload {
            payload: "{\"test\":\"ok\"}".to_string()
        };

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "Bearer test_secret".parse().unwrap());

        // Let's create the task first so it doesn't fail with "not found"
        use crate::integrations::mcp::async_task_tracker::AsyncTaskTracker;
        let tracker = AsyncTaskTracker::new_postgres(pool);
        let _ = tracker.create_task("test-task-1", "system", "agent-1", "{}").await;

        let resp = mcp_webhook_handler(headers, State(state), Path("test-task-1".to_string()), Json(payload)).await.into_response();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_mcp_webhook_handler_sqlite() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS mcp_async_tasks (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT NOT NULL,
                status TEXT NOT NULL,
                payload TEXT,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
            );"
        ).execute(&pool).await.unwrap();

        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(),
            store: DbStore::Sqlite(pool.clone()),
        });

        let state = WebhookState {
            db_pool: db,
            secret: "test_secret".to_string(),
        };

        let payload = WebhookPayload {
            payload: "{\"test\":\"ok\"}".to_string()
        };

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "Bearer test_secret".parse().unwrap());

        use crate::integrations::mcp::async_task_tracker::AsyncTaskTracker;
        let tracker = AsyncTaskTracker::new_sqlite(pool);
        let _ = tracker.create_task("test-task-2", "system", "agent-1", "{}").await;

        let resp = mcp_webhook_handler(headers, State(state), Path("test-task-2".to_string()), Json(payload)).await.into_response();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_mcp_webhook_handler_unauthorized() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(),
            store: DbStore::Sqlite(pool),
        });

        let state = WebhookState {
            db_pool: db,
            secret: "test_secret".to_string(),
        };

        let payload = WebhookPayload {
            payload: "{\"test\":\"ok\"}".to_string()
        };

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "Bearer wrong_secret".parse().unwrap());

        let resp = mcp_webhook_handler(headers, State(state), Path("test-task-1".to_string()), Json(payload)).await.into_response();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
}
