use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use std::sync::Arc;
use tower::ServiceExt;

use crate::db::DB;
use crate::hub::Hub;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::orchestrator::MeshNode;
use crate::api::twilio_voice::{twilio_voice_incoming_handler, TwilioVoiceWebhookState};
use ::server_integrations_twilio::provider::TwilioProvider;

#[tokio::test]
async fn test_twilio_voice_incoming_handler() {
    let pool = crate::db::get_pool();
    let db = Arc::new(DB { pool: pool.clone(), store: crate::db::DbStore::Sqlite(pool.clone()) });

    let transport = crate::mesh::local::LocalMeshTransport::new();
    let node = Arc::new(crate::mesh::CentrifugeNode::new(transport));
    let orchestrator = Arc::new(DepartmentOrchestrator::new(db.clone(), node));
    let hub = Arc::new(Hub::new());

    let twilio = Arc::new(TwilioProvider::new("test".to_string(), "test".to_string()));

    let state = TwilioVoiceWebhookState {
        hub,
        db: db.clone(),
        orchestrator,
        voice_engine: Arc::new(crate::voice::VoiceAIEdgeEngine::new()),
        twilio,
    };

    let app = axum::Router::new()
        .route("/api/v1/webhooks/twilio_voice/incoming", axum::routing::post(twilio_voice_incoming_handler))
        .with_state(state);

    let body = Body::from("CallSid=CA123&From=%2B1234567890&To=%2B0987654321");

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/webhooks/twilio_voice/incoming")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
