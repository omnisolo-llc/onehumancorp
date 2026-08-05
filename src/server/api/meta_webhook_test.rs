use super::meta_webhook::{meta_webhook_post_handler, MetaWebhookState};
use axum::{body::Bytes, http::HeaderMap, extract::State};
use std::sync::Arc;
use crate::hub::Hub;

#[tokio::test]
async fn test_meta_webhook_post_handler_whatsapp() {
    use sqlx::SqlitePool;
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

    let schema = r#"
        CREATE TABLE IF NOT EXISTS customers (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            name TEXT NOT NULL,
            email TEXT,
            phone TEXT
        );
        CREATE TABLE IF NOT EXISTS customer_identities (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            customer_id TEXT NOT NULL,
            channel TEXT NOT NULL,
            channel_identity TEXT NOT NULL,
            UNIQUE(tenant_id, channel, channel_identity)
        );
        CREATE TABLE IF NOT EXISTS inbox_messages (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            source TEXT NOT NULL,
            original_content TEXT NOT NULL,
            content TEXT NOT NULL,
            draft_reply TEXT,
            status TEXT NOT NULL DEFAULT 'unread',
            sender_id TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        CREATE TABLE IF NOT EXISTS omni_inbox_messages (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            source TEXT NOT NULL,
            original_content TEXT NOT NULL,
            translated_content TEXT NOT NULL,
            target_language TEXT NOT NULL,
            draft_reply TEXT,
            status TEXT NOT NULL DEFAULT 'unread',
            sender_id TEXT,
            customer_id TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        CREATE TABLE IF NOT EXISTS ohc_job_queue (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            job_type TEXT NOT NULL,
            payload TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'PENDING'
        );
        CREATE TABLE IF NOT EXISTS whatsapp_channels (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            phone_number TEXT NOT NULL,
            phone_number_id TEXT NOT NULL,
            business_account_id TEXT NOT NULL,
            api_token TEXT NOT NULL,
            calling_enabled BOOLEAN DEFAULT FALSE,
            webhook_verify_token TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        CREATE TABLE IF NOT EXISTS work_intents (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            source TEXT NOT NULL,
            intent_type TEXT NOT NULL,
            payload TEXT NOT NULL,
            status TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
    "#;
    sqlx::query(schema).execute(&pool).await.unwrap();

    // Seed a whatsapp channel for the webhook test
    sqlx::query(
        "INSERT INTO whatsapp_channels (id, tenant_id, phone_number, phone_number_id, business_account_id, api_token, webhook_verify_token)
         VALUES ('chan-1', 'e2e-tenant', '+15550199', '1234567890', '987654321', 'test_api_token', 'verify-token-xyz')"
    )
    .execute(&pool)
    .await
    .unwrap();

    let db = Arc::new(crate::db::DB {
        pool: sqlx::PgPool::connect_lazy("postgres://dummy").unwrap(),
        store: crate::db::DbStore::Sqlite(pool.clone()),
    });

    let transport = Arc::new(ohc_builtin_agent::mesh::transport::InProcessTransport::new());
    let mesh = Arc::new(crate::orchestration::mesh::CentrifugeNode::new(transport));
    let orchestrator = Arc::new(crate::orchestration::departments::DepartmentOrchestrator::new(db.clone(), mesh));

    let hub = Arc::new(Hub::new());
    let state = MetaWebhookState {
        hub,
        db,
        orchestrator,
    };

    // Construct valid Meta webhook payload
    let payload = serde_json::json!({
        "object": "whatsapp_business_account",
        "entry": [
            {
                "id": "waba-123",
                "changes": [
                    {
                        "field": "messages",
                        "value": {
                            "messaging_product": "whatsapp",
                            "metadata": {
                                "display_phone_number": "1234567890",
                                "phone_number_id": "1234567890"
                            },
                            "messages": [
                                {
                                    "from": "0987654321",
                                    "id": "msg-999",
                                    "timestamp": "1600000000",
                                    "text": {
                                        "body": "Hello from OHC WhatsApp integration!"
                                    },
                                    "type": "text"
                                }
                            ]
                        }
                    }
                ]
            }
        ]
    });

    let secret = "configured-secret";
    std::env::set_var("META_APP_SECRET", secret);

    let body_bytes = Bytes::from(payload.to_string());
    let mut mac = hmac::Hmac::<sha2::Sha256>::new_from_slice(secret.as_bytes()).unwrap();
    use hmac::Mac;
    mac.update(&body_bytes);
    let signature = format!("sha256={}", hex::encode(mac.finalize().into_bytes()));

    let mut headers = HeaderMap::new();
    headers.insert("x-hub-signature-256", signature.parse().unwrap());

    let response = meta_webhook_post_handler(headers, State(state), body_bytes).await;
    let res_parts = response.into_response();
    assert_eq!(res_parts.status(), axum::http::StatusCode::OK);

    // Verify message has been ingested under e2e-tenant
    let inbox_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM omni_inbox_messages WHERE tenant_id = 'e2e-tenant'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(inbox_count, 2); // It attempts two inserts in meta_webhook.rs (one fallback block, one main insert block)
}
