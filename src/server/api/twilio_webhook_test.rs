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
use crate::api::twilio_webhook::{
    twilio_signature_middleware, twilio_webhook_post_handler, TwilioWebhookState,
};
use std::collections::HashMap;

#[test]
fn validates_twilio_signature_using_the_canonical_url_and_sorted_form_fields() {
    let url = "https://mycompany.com/myapp.php?foo=1&bar=2";
    let body = b"CallSid=CA1234567890ABCDE&Caller=%2B14158675310&Digits=1234&From=%2B14158675310&To=%2B18005551212";

    assert!(crate::api::twilio_webhook::valid_twilio_signature(
        "12345",
        url,
        body,
        Some("L/OH5YylLD5NRKLltdqwSvS0BnU="),
    ));
    assert!(!crate::api::twilio_webhook::valid_twilio_signature(
        "wrong-token",
        url,
        body,
        Some("L/OH5YylLD5NRKLltdqwSvS0BnU="),
    ));
    assert!(!crate::api::twilio_webhook::valid_twilio_signature(
        "12345", url, body, None,
    ));
}

#[tokio::test]
async fn twilio_signature_middleware_fails_closed_and_accepts_a_valid_callback() {
    let app = axum::Router::new()
        .route(
            "/myapp.php",
            axum::routing::post(|| async { StatusCode::NO_CONTENT }),
        )
        .route_layer(axum::middleware::from_fn(twilio_signature_middleware));
    let body = "CallSid=CA1234567890ABCDE&Caller=%2B14158675310&Digits=1234&From=%2B14158675310&To=%2B18005551212";

    temp_env::async_with_vars(
        [
            ("TWILIO_AUTH_TOKEN", Some("12345")),
            ("TWILIO_WEBHOOK_BASE_URL", Some("https://mycompany.com")),
        ],
        async {
            let unsigned = Request::builder()
                .method("POST")
                .uri("/myapp.php?foo=1&bar=2")
                .body(Body::from(body))
                .unwrap();
            assert_eq!(
                app.clone().oneshot(unsigned).await.unwrap().status(),
                StatusCode::UNAUTHORIZED,
            );

            let signed = Request::builder()
                .method("POST")
                .uri("/myapp.php?foo=1&bar=2")
                .header("x-twilio-signature", "L/OH5YylLD5NRKLltdqwSvS0BnU=")
                .body(Body::from(body))
                .unwrap();
            assert_eq!(
                app.oneshot(signed).await.unwrap().status(),
                StatusCode::NO_CONTENT,
            );
        },
    )
    .await;
}

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

    let response = app.clone().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Test with media
    let body_media = Body::from("From=whatsapp%3A%2B14155238886&To=whatsapp%3A%2B1234567890&Body=Check+out+this+cake&NumMedia=1&MediaUrl0=https%3A%2F%2Fexample.com%2Fimage.jpg&MediaContentType0=image%2Fjpeg");
    let request_media = Request::builder()
        .method("POST")
        .uri("/api/v1/webhooks/twilio")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body_media)
        .unwrap();

    let response_media = app.oneshot(request_media).await.unwrap();
    assert_eq!(response_media.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_twilio_voice_webhook_handler_success() {
    let pool = crate::db::get_pool();
    let db = Arc::new(DB { pool: pool.clone(), store: crate::db::DbStore::Sqlite(pool.clone()) });

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
        .route("/api/v1/webhooks/twilio/voice", axum::routing::post(crate::api::twilio_webhook::twilio_voice_webhook_handler))
        .with_state(state);

    let body = Body::from("CallSid=CA123&From=%2B0987654321&To=%2B1234567890");

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/webhooks/twilio/voice")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
