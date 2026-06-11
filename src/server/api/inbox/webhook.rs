use axum::{extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::db::{DbStore, DB};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::hub::Hub;

#[derive(Clone)]
pub struct InboxWebhookState {
    pub hub: Arc<Hub>,
    pub db: Arc<DB>,
    pub orchestrator: Arc<DepartmentOrchestrator>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OmniInboxPayload {
    pub channel: String,      // e.g. "instagram", "whatsapp", "email"
    pub sender_id: String,    // phone, email, handle
    pub recipient_id: String, // to identify tenant
    pub content: String,
    // Optional additional context or metadata
    pub metadata: Option<serde_json::Value>,
}

pub async fn inbox_webhook_post_handler(
    State(state): State<InboxWebhookState>,
    axum::extract::Json(payload): axum::extract::Json<OmniInboxPayload>,
) -> impl IntoResponse {
    if payload.content.is_empty() {
        return StatusCode::BAD_REQUEST.into_response();
    }

    tracing::info!(
        "Received omni inbox message from {}: {}",
        payload.sender_id,
        payload.content
    );

    let pool = &state.db.pool;
    let tenant_id = payload.recipient_id.clone();

    // 2. Identity Resolution step
    // Attempt to match incoming sender identifiers (phone, email, social handle) to an existing OHC Customer record.
    let mut resolved_customer_id: Option<String> = None;

    match &state.db.store {
        DbStore::Postgres => {
            if let Ok(Some(cid)) = sqlx::query_scalar::<_, String>(
                "SELECT id FROM customers WHERE tenant_id = $1 AND (email = $2 OR phone = $2 OR preferences->>'social_handle' = $2) LIMIT 1"
            )
            .bind(&tenant_id)
            .bind(&payload.sender_id)
            .fetch_optional(pool)
            .await {
                resolved_customer_id = Some(cid);
            }
        },
        DbStore::Sqlite(sqlite_pool) => {
            if let Ok(Some(cid)) = sqlx::query_scalar::<_, String>(
                "SELECT id FROM customers WHERE tenant_id = ? AND (email = ? OR phone = ? OR json_extract(preferences, '$.social_handle') = ?) LIMIT 1"
            )
            .bind(&tenant_id)
            .bind(&payload.sender_id)
            .bind(&payload.sender_id)
            .bind(&payload.sender_id)
            .fetch_optional(sqlite_pool)
            .await {
                resolved_customer_id = Some(cid);
            }
        }
    }

    let inbox_id = Uuid::new_v4().to_string();
    let source = payload.channel.clone();

    // insert into inbox_messages
    let insert_result = match &state.db.store {
        DbStore::Postgres => {
            sqlx::query(
                "INSERT INTO inbox_messages (id, tenant_id, source, content, draft_reply, status, sender_id) VALUES ($1, $2, $3, $4, '', 'pending', $5)"
            )
            .bind(&inbox_id)
            .bind(&tenant_id)
            .bind(&source)
            .bind(&payload.content)
            .bind(&payload.sender_id)
            .execute(pool)
            .await.map(|_| ())
        },
        DbStore::Sqlite(sqlite_pool) => {
            sqlx::query(
                "INSERT INTO inbox_messages (id, tenant_id, source, content, draft_reply, status, sender_id) VALUES (?, ?, ?, ?, '', 'pending', ?)"
            )
            .bind(&inbox_id)
            .bind(&tenant_id)
            .bind(&source)
            .bind(&payload.content)
            .bind(&payload.sender_id)
            .execute(sqlite_pool)
            .await.map(|_| ())
        }
    };

    if let Err(e) = insert_result {
        tracing::error!("Failed to insert inbox message: {}", e);
    }

    // 3. Emit event mesh InboundMessage / tenant.omnichannel.message.received
    let event = crate::orchestration::departments::types::DepartmentEvent {
        id: Uuid::new_v4().to_string(),
        tenant_id: tenant_id.clone(),
        event_type: "tenant.omnichannel.message.received".to_string(),
        payload: serde_json::json!({
            "source": source,
            "message": payload.content,
            "sender_id": payload.sender_id,
            "inbox_message_id": inbox_id,
            "resolved_customer_id": resolved_customer_id,
        }),
    };

    let orchestrator_clone = state.orchestrator.clone();
    tokio::spawn(async move {
        let _ = orchestrator_clone.dispatch_event(event).await;
    });

    StatusCode::OK.into_response()
}

#[cfg(test)]
mod inbox_webhook_tests {
    use super::*;
    use axum::extract::State;
    use axum::Json;
    use crate::db::{DB, DbStore};
    use crate::hub::Hub;
    use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
    use crate::orchestration::mesh::CentrifugeNode;
    use ohc_builtin_agent::mesh::transport::InProcessTransport;
    use std::sync::Arc;
    use tokio;
    use tokio::sync::mpsc;
    use sqlx::sqlite::SqlitePoolOptions;

    // Helper to run a test with an in-memory db
    async fn setup_state() -> InboxWebhookState {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        let dummy_pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
            .unwrap();

        let db = Arc::new(DB {
            pool: dummy_pool,
            store: DbStore::Sqlite(pool.clone()),
        });

        // Run essential migrations to test
        sqlx::query("CREATE TABLE tenants (id TEXT PRIMARY KEY, name TEXT);")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("CREATE TABLE customers (id TEXT PRIMARY KEY, tenant_id TEXT, email TEXT, phone TEXT, name TEXT, preferences TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, _sync_status TEXT DEFAULT 'pending', version INTEGER DEFAULT 1);")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("CREATE TABLE inbox_messages (id TEXT PRIMARY KEY, tenant_id TEXT, source TEXT, content TEXT, draft_reply TEXT, status TEXT, sender_id TEXT);")
            .execute(&pool)
            .await
            .unwrap();

        // insert dummy tenant
        sqlx::query("INSERT INTO tenants (id, name) VALUES ('test_tenant', 'Test')")
            .execute(&pool)
            .await
            .unwrap();

        let (tx, _rx) = mpsc::channel(100);
        let hub = Arc::new(Hub::new(tx, db.pool.clone()));

        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let orchestrator = Arc::new(DepartmentOrchestrator::new(db.clone(), mesh));

        InboxWebhookState {
            hub: hub.clone(),
            db: db.clone(),
            orchestrator,
        }
    }

    #[tokio::test]
    async fn test_identity_resolution_existing_customer_phone() {
        let state = setup_state().await;

        // Insert customer
        match &state.db.store {
            DbStore::Sqlite(pool) => {
                sqlx::query("INSERT INTO customers (id, tenant_id, email, phone) VALUES ('cust_123', 'test_tenant', 'test@example.com', '+1234567890')")
                    .execute(pool)
                    .await
                    .unwrap();
            },
            _ => panic!("Expected Sqlite db store"),
        }

        let payload = OmniInboxPayload {
            channel: "whatsapp".to_string(),
            sender_id: "+1234567890".to_string(),
            recipient_id: "test_tenant".to_string(),
            content: "Hello!".to_string(),
            metadata: None,
        };

        let response = inbox_webhook_post_handler(State(state.clone()), Json(payload)).await.into_response();
        assert_eq!(response.status(), StatusCode::OK);

        // Let event mesh handle async loop
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        // Verify DB
        match &state.db.store {
            DbStore::Sqlite(pool) => {
                let (sender_id,): (String,) = sqlx::query_as("SELECT sender_id FROM inbox_messages LIMIT 1")
                    .fetch_one(pool)
                    .await
                    .unwrap();

                assert_eq!(sender_id, "+1234567890");
            },
            _ => (),
        }
    }

    #[tokio::test]
    async fn test_identity_resolution_not_found() {
        let state = setup_state().await;

        let payload = OmniInboxPayload {
            channel: "instagram".to_string(),
            sender_id: "unknown_user".to_string(),
            recipient_id: "test_tenant".to_string(),
            content: "Is this thing on?".to_string(),
            metadata: None,
        };

        let response = inbox_webhook_post_handler(State(state.clone()), Json(payload)).await.into_response();
        assert_eq!(response.status(), StatusCode::OK);

        // Verify DB
        match &state.db.store {
            DbStore::Sqlite(pool) => {
                let (sender_id,): (String,) = sqlx::query_as("SELECT sender_id FROM inbox_messages LIMIT 1")
                    .fetch_one(pool)
                    .await
                    .unwrap();

                assert_eq!(sender_id, "unknown_user");
            },
            _ => (),
        }
    }
}
