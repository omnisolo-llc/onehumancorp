#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;
    use std::sync::Arc;
    use crate::api::omni_inbox_webhook::{omni_inbox_post_handler, OmniInboxWebhookState, OmniInboxPayload};
    use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
    use crate::orchestration::mesh::CentrifugeNode;
    use ohc_builtin_agent::mesh::transport::InProcessTransport;
    use crate::db::{DB, DbStore};
    use crate::hub::Hub;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_omni_inbox_webhook_triggers_ambassador() {
        // 1. Setup Mock DB (Sqlite in-memory)
        let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query("CREATE TABLE IF NOT EXISTS help_articles (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, topic TEXT, title TEXT NOT NULL, content_markdown TEXT NOT NULL)")
            .execute(&sqlite_pool).await.unwrap();
        sqlx::query("CREATE TABLE IF NOT EXISTS products (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, title TEXT, name TEXT, inventory_count INT)")
            .execute(&sqlite_pool).await.unwrap();
        sqlx::query("CREATE TABLE IF NOT EXISTS omni_inbox_messages (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, source TEXT NOT NULL, original_content TEXT NOT NULL, translated_content TEXT NOT NULL, source_language TEXT, target_language TEXT NOT NULL, draft_reply TEXT, status TEXT NOT NULL, sender_id TEXT, customer_id TEXT, created_at TIMESTAMPTZ)")
            .execute(&sqlite_pool).await.unwrap();
        sqlx::query("CREATE TABLE IF NOT EXISTS ohc_job_queue (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, job_type TEXT NOT NULL, payload TEXT NOT NULL, status TEXT NOT NULL, next_retry_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP)")
            .execute(&sqlite_pool).await.unwrap();
        sqlx::query("CREATE TABLE IF NOT EXISTS agent_feed_items (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, event_source TEXT NOT NULL, context_payload TEXT, proposed_action TEXT, lifecycle_state TEXT NOT NULL, created_at TIMESTAMPTZ, updated_at TIMESTAMPTZ)")
            .execute(&sqlite_pool).await.unwrap();
        sqlx::query("CREATE TABLE IF NOT EXISTS agent_approvals (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, department TEXT NOT NULL, description TEXT NOT NULL, status TEXT NOT NULL, action_risk TEXT NOT NULL, payload TEXT, created_at TIMESTAMPTZ, updated_at TIMESTAMPTZ)")
            .execute(&sqlite_pool).await.unwrap();
        sqlx::query("CREATE TABLE IF NOT EXISTS customers (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, phone TEXT, email TEXT, name TEXT)")
            .execute(&sqlite_pool).await.unwrap();

        let db = Arc::new(DB {
            pool: sqlx::PgPool::connect_lazy("postgres://localhost/dummy").unwrap(),
            store: DbStore::Sqlite(sqlite_pool),
        });

        // 2. Setup Orchestrator & Hub
        let (tx, _rx) = mpsc::channel(10);
        let hub = Arc::new(Hub::new(tx, db.pool.clone()));
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let orchestrator = Arc::new(DepartmentOrchestrator::new(db.clone(), mesh));

        // Register Ambassador Agent
        let ambassador = Arc::new(tokio::sync::RwLock::new(
            crate::orchestration::departments::ambassador_agent::AmbassadorAgent::new(orchestrator.clone())
        ));
        orchestrator.register_department(ambassador).await;

        let state = OmniInboxWebhookState {
            hub,
            db: db.clone(),
            orchestrator,
        };

        // 3. Create Request
        let payload = OmniInboxPayload {
            tenant_id: "test-tenant".to_string(),
            source: "instagram".to_string(),
            sender_id: "sender-123".to_string(),
            message: "Do you have vegan cakes?".to_string(),
        };

        let app = axum::Router::new()
            .route("/webhook", axum::routing::post(omni_inbox_post_handler))
            .with_state(state);

        // 4. Send Request
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/webhook")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        // 5. Assert Success
        assert_eq!(response.status(), StatusCode::OK);

        // 6. Verify Ambassador drafting (background task)
        // Since Ambassador runs in tokio::spawn, we might need a small sleep or poll
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        if let DbStore::Sqlite(pool) = &db.store {
            let row: (String,) = sqlx::query_as("SELECT lifecycle_state FROM agent_feed_items WHERE tenant_id = 'test-tenant' LIMIT 1")
                .fetch_one(pool).await.unwrap();
            assert_eq!(row.0, "PENDING_APPROVAL");

            let row: (String,) = sqlx::query_as("SELECT description FROM agent_approvals WHERE tenant_id = 'test-tenant' LIMIT 1")
                .fetch_one(pool).await.unwrap();
            assert!(row.0.contains("The Ambassador"));
        }
    }
}
