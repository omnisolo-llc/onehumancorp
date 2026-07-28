use std::sync::Arc;
use axum::{
    extract::{State, Extension},
    response::IntoResponse,
    Json,
};
use axum::http::StatusCode;

use server_common::Claims;
use server_lib::db::{DbStore, DB};
use server_lib::orchestration::departments::orchestrator::DepartmentOrchestrator;
use server_lib::orchestration::mesh::TeammateMesh;
use hub_proto::ohc::orchestration::TeammateMeshEvent;
use async_trait::async_trait;

use super::{
    get_conversations, get_messages, handle_omnichannel_webhook,
    OmnichannelWebhookState, OmnichannelPayload,
};

fn create_test_claims(tenant_id: &str) -> Claims {
    Claims {
        sub: "user_1".to_string(),
        organization_id: Some(tenant_id.to_string()),
        username: "".to_string(),
        email: "".to_string(),
        roles: vec!["owner".to_string()],
        session_id: Some("sess_1".to_string()),
        jti: "".to_string(),
        exp: 2000000000,
        iat: 1000000000,
    }
}

struct DummyMesh {}

#[async_trait]
impl TeammateMesh for DummyMesh {
    async fn publish(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn publish_with_ack(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn subscribe(&self, _topic: &str, _handler: Box<dyn Fn(TeammateMeshEvent) + Send + Sync + 'static>) -> Result<Box<dyn Fn() + Send + Sync + 'static>, String> { Ok(Box::new(|| {})) }
    async fn acquire_lock(&self, _lock_key: &str, _owner: &str, _ttl_ms: u64) -> Result<bool, String> { Ok(true) }
    async fn release_lock(&self, _lock_key: &str, _owner: &str) -> Result<(), String> { Ok(()) }
    async fn register_presence(&self, _agent_id: &str, _status: &str, _ttl_ms: u64) -> Result<(), String> { Ok(()) }
    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> { Ok(vec![]) }
    async fn ping(&self) -> Result<(), String> { Ok(()) }
    async fn start_health_responder(&self) -> Result<Box<dyn Fn() + Send + Sync + 'static>, String> { Ok(Box::new(|| {})) }
    async fn publish_state_handoff(&self, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn subscribe_state_handoff(&self, _handler: Box<dyn Fn(TeammateMeshEvent) + Send + Sync + 'static>) -> Result<Box<dyn Fn() + Send + Sync + 'static>, String> { Ok(Box::new(|| {})) }
}

async fn setup_test_state() -> OmnichannelWebhookState {
    let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE unified_threads (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            customer_id TEXT,
            channel TEXT NOT NULL,
            status TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        CREATE TABLE omni_inbox_messages (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            customer_id TEXT,
            source TEXT NOT NULL,
            sender_id TEXT,
            original_content TEXT NOT NULL,
            translated_content TEXT,
            target_language TEXT,
            draft_reply TEXT,
            status TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        CREATE TABLE omnichannel_identities (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            customer_id TEXT NOT NULL,
            provider TEXT NOT NULL,
            provider_id TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        "#
    )
    .execute(&sqlite_pool)
    .await
    .unwrap();

    let dummy_pg_pool = server_lib::db::create_dummy_pg_pool().await;

    let db = DB {
        pool: dummy_pg_pool,
        store: DbStore::Sqlite(sqlite_pool),
    };

    let transport = DummyMesh {};
    let orchestrator = DepartmentOrchestrator::new(Arc::new(db.clone()), Arc::new(transport));

    OmnichannelWebhookState {
        db: Arc::new(db),
        orchestrator: Arc::new(orchestrator),
    }
}

#[tokio::test]
async fn test_get_conversations() {
    let state = setup_test_state().await;
    let claims = create_test_claims("t-1");

    let res = get_conversations(
        State(state.clone()),
        Extension(claims.clone()),
        axum::extract::Path("t-1".to_string())
    ).await.into_response();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_messages() {
    let state = setup_test_state().await;
    let claims = create_test_claims("t-1");

    let res = get_messages(
        State(state.clone()),
        Extension(claims.clone()),
        axum::extract::Path(("t-1".to_string(), "conv-1".to_string()))
    ).await.into_response();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_handle_omnichannel_webhook() {
    let state = setup_test_state().await;
    let claims = create_test_claims("t-1");

    let payload = OmnichannelPayload {
        tenant_id: "t-1".to_string(),
        source: "sms".to_string(),
        sender_id: "phone-123".to_string(),
        message: "hello".to_string(),
        target_language: None,
    };

    let res = handle_omnichannel_webhook(
        State(state.clone()),
        Extension(claims.clone()),
        Json(payload)
    ).await.into_response();
    assert_eq!(res.status(), StatusCode::OK);
}
