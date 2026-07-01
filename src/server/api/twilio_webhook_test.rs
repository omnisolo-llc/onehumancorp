use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use std::sync::Arc;
use tower::ServiceExt;
use uuid::Uuid;

use crate::db::DB;
use crate::hub::Hub;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::orchestrator::MeshNode;
use crate::api::twilio_webhook::{twilio_webhook_post_handler, TwilioWebhookState};
use std::collections::HashMap;

#[tokio::test]
async fn test_twilio_webhook_post_handler_success() {
    let pool = crate::db::get_pool();
    let db = Arc::new(DB { pool: pool.clone(), store: crate::db::DbStore::Sqlite(pool.clone()) });

    // Create test user and settings
    let _ = sqlx::query("INSERT OR IGNORE INTO settings (tenant_id, sms_critical_phone, voice_receptionist_number) VALUES ('test_tenant', '+1234567890', '+1234567890')")
        .execute(&pool)
        .await;

    let transport = crate::mesh::local::LocalMeshTransport::new();
    let node = Arc::new(crate::mesh::CentrifugeNode::new(transport));
    let orchestrator = Arc::new(DepartmentOrchestrator::new(db.clone(), node));
    let hub = Arc::new(Hub::new());

    let state = TwilioWebhookState {
        hub,
        db: db.clone(),
        orchestrator,
        voice_engine: Arc::new(crate::voice::VoiceAIEdgeEngine::new()),
        voice_router: Arc::new(crate::voice::VoiceContextRouter::new()),
        voice_sessions: Arc::new(dashmap::DashMap::new()),
    };

    let app = axum::Router::new()
        .route("/api/v1/webhooks/twilio", axum::routing::post(twilio_webhook_post_handler))
        .with_state(state);

    let body = Body::from("From=whatsapp%3A%2B14155238886&To=whatsapp%3A%2B1234567890&Body=Hello%21+Id+like+to+order+a+vegan+cake+over+WhatsApp.");

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/webhooks/twilio")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
