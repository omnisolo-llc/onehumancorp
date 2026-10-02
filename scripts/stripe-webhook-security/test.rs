use axum::{Router, extract::State, http::StatusCode, routing::post};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
const BILLING_SECRET: &str = "whsec_isolated_billing_fixture_never_a_credential";
const LEDGER_SECRET: &str = "whsec_isolated_ledger_fixture_never_a_credential";
fn configure() {
    unsafe {
        std::env::set_var("STRIPE_WEBHOOK_SECRET", BILLING_SECRET);
        std::env::remove_var("STRIPE_WEBHOOK_SECRET_FILE");
        std::env::set_var("STRIPE_LEDGER_WEBHOOK_SECRET", LEDGER_SECRET);
        std::env::remove_var("STRIPE_LEDGER_WEBHOOK_SECRET_FILE");
    }
}
async fn serve(app: Router) -> (String, tokio::task::JoinHandle<()>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let origin = format!("http://{}", listener.local_addr().unwrap());
    let task = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    (origin, task)
}
fn client() -> reqwest::Client {
    reqwest::Client::builder().no_proxy().build().unwrap()
}
async fn effect(State(count): State<Arc<AtomicUsize>>) -> StatusCode {
    count.fetch_add(1, Ordering::SeqCst);
    StatusCode::OK
}
fn billing() -> (
    Router,
    Arc<AtomicUsize>,
    Arc<crate::api::billing_webhook::NoRedis>,
) {
    let effects = Arc::new(AtomicUsize::new(0));
    let redis = Arc::new(crate::api::billing_webhook::NoRedis {
        calls: AtomicUsize::new(0),
    });
    let state = crate::api::billing_webhook::WebhookState {
        rate_limiter: redis.clone(),
    };
    let app = Router::new()
        .route("/api/v1/webhooks/stripe", post(effect))
        .route_layer(axum::middleware::from_fn_with_state(
            state,
            crate::api::billing_webhook::webhook_security_middleware,
        ))
        .with_state(effects.clone());
    (app, effects, redis)
}
#[tokio::test]
async fn billing_rejects_current_timestamp_with_forged_signature_before_any_effect() {
    configure();
    let (app, effects, redis) = billing();
    let (origin, task) = serve(app).await;
    let response = client()
        .post(format!("{origin}/api/v1/webhooks/stripe"))
        .header(
            "Stripe-Signature",
            format!("t={},v1={}", chrono::Utc::now().timestamp(), "0".repeat(64)),
        )
        .json(&serde_json::json!({"id":"evt_isolated","type":"unknown","data":{"object":{}}}))
        .send()
        .await
        .unwrap();
    let status = response.status();
    task.abort();
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(effects.load(Ordering::SeqCst), 0);
    assert_eq!(redis.calls.load(Ordering::SeqCst), 0);
}
#[tokio::test]
async fn ledger_requires_authenticity_before_payload_or_ledger_effects() {
    configure();
    let effects = Arc::new(AtomicUsize::new(0));
    let app = Router::new().nest(
        "/api/v1/payments/ledger",
        crate::api::payment_ledger::router().with_state(effects.clone()),
    );
    let (origin, task) = serve(app).await;
    let response=client().post(format!("{origin}/api/v1/payments/ledger/webhook"))
        .json(&serde_json::json!({"type_field":"payment_intent.succeeded","data":{"object":{"id":"pi_isolated","metadata":{"tenant_id":"tenant-isolated","idempotency_key":"isolated"}}}}))
        .send().await.unwrap();
    let status = response.status();
    task.abort();
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(effects.load(Ordering::SeqCst), 0);
}

fn sign(raw: &[u8], secret: &str, time: i64) -> String {
    use hmac::Mac;
    let mut mac = hmac::Hmac::<sha2::Sha256>::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(format!("{time}.").as_bytes());
    mac.update(raw);
    format!("t={time},v1={}", hex::encode(mac.finalize().into_bytes()))
}
fn billing_payload() -> Vec<u8> {
    serde_json::to_vec(&serde_json::json!({"id":"evt_local_crypto","type":"unhandled.fixture","data":{"object":{}}})).unwrap()
}
fn ledger_payload(standard: bool) -> Vec<u8> {
    let field = if standard { "type" } else { "type_field" };
    serde_json::to_vec(&serde_json::json!({field:"payment_intent.succeeded","data":{"object":{"id":"pi_fixture","metadata":{"tenant_id":"fixture-tenant","idempotency_key":"fixture-operation"}}}})).unwrap()
}
async fn dispatch_count(count: &AtomicUsize) -> usize {
    for _ in 0..100 {
        if count.load(Ordering::SeqCst) > 0 {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }
    count.load(Ordering::SeqCst)
}
#[tokio::test]
async fn authentic_billing_body_reaches_the_existing_dispatch_boundary() {
    configure();
    let (app, effects, redis) = billing();
    let (origin, task) = serve(app).await;
    let raw = billing_payload();
    let signature = sign(&raw, BILLING_SECRET, chrono::Utc::now().timestamp());
    let response = client()
        .post(format!("{origin}/api/v1/webhooks/stripe"))
        .header("content-type", "application/json")
        .header("Stripe-Signature", signature)
        .body(raw)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(dispatch_count(&effects).await, 1);
    assert_eq!(redis.calls.load(Ordering::SeqCst), 1);
    task.abort();
}
#[tokio::test]
async fn billing_rejects_modified_bytes_and_stale_or_future_signed_events_before_dispatch() {
    configure();
    let (app, effects, redis) = billing();
    let (origin, task) = serve(app).await;
    let raw = billing_payload();
    let now = chrono::Utc::now().timestamp();
    let modified = [raw.as_slice(), b"\n"].concat();
    for (body, signature) in [
        (modified, sign(&raw, BILLING_SECRET, now)),
        (raw.clone(), sign(&raw, BILLING_SECRET, now - 600)),
        (raw.clone(), sign(&raw, BILLING_SECRET, now + 600)),
        (raw.clone(), sign(&raw, LEDGER_SECRET, now)),
    ] {
        let response = client()
            .post(format!("{origin}/api/v1/webhooks/stripe?delivery=fixture"))
            .header("content-type", "application/json")
            .header("Stripe-Signature", signature)
            .body(body)
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
    assert_eq!(effects.load(Ordering::SeqCst), 0);
    assert_eq!(redis.calls.load(Ordering::SeqCst), 0);
    task.abort();
}
#[tokio::test]
async fn duplicate_headers_and_legacy_alias_never_substitute_for_stripe_authentication() {
    configure();
    let (app, effects, redis) = billing();
    let (origin, task) = serve(app).await;
    let raw = billing_payload();
    let signature = sign(&raw, BILLING_SECRET, chrono::Utc::now().timestamp());
    let duplicated = client()
        .post(format!("{origin}/api/v1/webhooks/stripe"))
        .header("content-type", "application/json")
        .header("Stripe-Signature", &signature)
        .header("Stripe-Signature", &signature)
        .body(raw.clone())
        .send()
        .await
        .unwrap();
    assert_eq!(duplicated.status(), StatusCode::UNAUTHORIZED);
    let aliased = client()
        .post(format!("{origin}/api/v1/webhooks/stripe"))
        .header("content-type", "application/json")
        .header("X-Signature", signature)
        .body(raw)
        .send()
        .await
        .unwrap();
    assert_eq!(aliased.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(effects.load(Ordering::SeqCst), 0);
    assert_eq!(redis.calls.load(Ordering::SeqCst), 0);
    task.abort();
}
#[tokio::test]
async fn missing_or_ambiguous_endpoint_secret_is_explicitly_unavailable_without_effects() {
    configure();
    let (app, effects, redis) = billing();
    let (origin, task) = serve(app).await;
    let raw = billing_payload();
    let signature = sign(&raw, BILLING_SECRET, chrono::Utc::now().timestamp());
    unsafe {
        std::env::remove_var("STRIPE_WEBHOOK_SECRET");
    }
    let missing = client()
        .post(format!("{origin}/api/v1/webhooks/stripe"))
        .header("content-type", "application/json")
        .header("Stripe-Signature", &signature)
        .body(raw.clone())
        .send()
        .await
        .unwrap();
    assert_eq!(missing.status(), StatusCode::SERVICE_UNAVAILABLE);
    configure();
    unsafe {
        std::env::set_var(
            "STRIPE_WEBHOOK_SECRET_FILE",
            "unused-because-two-sources-are-invalid",
        );
    }
    let ambiguous = client()
        .post(format!("{origin}/api/v1/webhooks/stripe"))
        .header("content-type", "application/json")
        .header("Stripe-Signature", signature)
        .body(raw)
        .send()
        .await
        .unwrap();
    assert_eq!(ambiguous.status(), StatusCode::SERVICE_UNAVAILABLE);
    configure();
    assert_eq!(effects.load(Ordering::SeqCst), 0);
    assert_eq!(redis.calls.load(Ordering::SeqCst), 0);
    task.abort();
}
#[tokio::test]
async fn oversized_or_authenticated_malformed_billing_payload_cannot_claim_or_dispatch() {
    configure();
    let (app, effects, redis) = billing();
    let (origin, task) = serve(app).await;
    let now = chrono::Utc::now().timestamp();
    for (raw, status) in [
        (vec![b'x'; 1024 * 1024 + 1], StatusCode::PAYLOAD_TOO_LARGE),
        (b"{".to_vec(), StatusCode::BAD_REQUEST),
        (
            br#"{"id":"evt_missing_data","type":"invalid"}"#.to_vec(),
            StatusCode::BAD_REQUEST,
        ),
    ] {
        let sig = sign(&raw, BILLING_SECRET, now);
        let response = client()
            .post(format!("{origin}/api/v1/webhooks/stripe"))
            .header("content-type", "application/json")
            .header("Stripe-Signature", sig)
            .body(raw)
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), status);
    }
    assert_eq!(effects.load(Ordering::SeqCst), 0);
    assert_eq!(redis.calls.load(Ordering::SeqCst), 0);
    task.abort();
}
#[tokio::test]
async fn multiple_v1_rotation_signatures_preserve_authenticated_raw_bytes() {
    configure();
    let (app, effects, _) = billing();
    let (origin, task) = serve(app).await;
    let mut raw = billing_payload();
    raw.push(b'\n');
    let sig = sign(&raw, BILLING_SECRET, chrono::Utc::now().timestamp());
    let header = sig.replace(
        ",v1=",
        &format!(",v0={},v1={},v1=", "0".repeat(64), "0".repeat(64)),
    );
    let response = client()
        .post(format!("{origin}/api/v1/webhooks/stripe"))
        .header("content-type", "application/json")
        .header("Stripe-Signature", header)
        .body(raw)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(dispatch_count(&effects).await, 1);
    task.abort();
}
#[tokio::test]
async fn ledger_uses_its_own_endpoint_secret_and_keeps_real_and_legacy_wire_shapes() {
    configure();
    let effects = Arc::new(AtomicUsize::new(0));
    let app = Router::new().nest(
        "/api/v1/payments/ledger",
        crate::api::payment_ledger::router().with_state(effects.clone()),
    );
    let (origin, task) = serve(app).await;
    for standard in [true, false] {
        let raw = ledger_payload(standard);
        let now = chrono::Utc::now().timestamp();
        let wrong = client()
            .post(format!("{origin}/api/v1/payments/ledger/webhook"))
            .header("content-type", "application/json")
            .header("Stripe-Signature", sign(&raw, BILLING_SECRET, now))
            .body(raw.clone())
            .send()
            .await
            .unwrap();
        assert_eq!(wrong.status(), StatusCode::UNAUTHORIZED);
        let valid = client()
            .post(format!("{origin}/api/v1/payments/ledger/webhook"))
            .header("content-type", "application/json")
            .header("Stripe-Signature", sign(&raw, LEDGER_SECRET, now))
            .body(raw)
            .send()
            .await
            .unwrap();
        assert_eq!(valid.status(), StatusCode::OK);
    }
    assert_eq!(effects.load(Ordering::SeqCst), 2);
    task.abort();
}
#[tokio::test]
async fn ledger_authenticates_before_json_and_rejects_duplicate_type_fields() {
    configure();
    let effects = Arc::new(AtomicUsize::new(0));
    let app = Router::new().nest(
        "/api/v1/payments/ledger",
        crate::api::payment_ledger::router().with_state(effects.clone()),
    );
    let (origin, task) = serve(app).await;
    let now = chrono::Utc::now().timestamp();
    for (raw, sig, status) in [
        (b"{".to_vec(), "invalid".into(), StatusCode::UNAUTHORIZED),
        (
            b"{".to_vec(),
            sign(b"{", LEDGER_SECRET, now),
            StatusCode::BAD_REQUEST,
        ),
        (
            br#"{"type":"one","type_field":"two","data":{"object":{"id":"pi","metadata":{}}}}"#
                .to_vec(),
            sign(
                br#"{"type":"one","type_field":"two","data":{"object":{"id":"pi","metadata":{}}}}"#,
                LEDGER_SECRET,
                now,
            ),
            StatusCode::UNPROCESSABLE_ENTITY,
        ),
    ] {
        let response = client()
            .post(format!("{origin}/api/v1/payments/ledger/webhook"))
            .header("content-type", "application/json")
            .header("Stripe-Signature", sig)
            .body(raw)
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), status);
    }
    assert_eq!(effects.load(Ordering::SeqCst), 0);
    task.abort();
}
#[tokio::test]
async fn ledger_configuration_never_falls_back_to_the_billing_key() {
    configure();
    unsafe {
        std::env::remove_var("STRIPE_LEDGER_WEBHOOK_SECRET");
    }
    let effects = Arc::new(AtomicUsize::new(0));
    let app = Router::new().nest(
        "/api/v1/payments/ledger",
        crate::api::payment_ledger::router().with_state(effects.clone()),
    );
    let (origin, task) = serve(app).await;
    let raw = ledger_payload(true);
    let response = client()
        .post(format!("{origin}/api/v1/payments/ledger/webhook"))
        .header("content-type", "application/json")
        .header(
            "Stripe-Signature",
            sign(&raw, BILLING_SECRET, chrono::Utc::now().timestamp()),
        )
        .body(raw)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(effects.load(Ordering::SeqCst), 0);
    configure();
    task.abort();
}
#[tokio::test]
async fn non_stripe_legacy_middleware_branch_is_unchanged() {
    configure();
    unsafe {
        std::env::remove_var("STRIPE_WEBHOOK_SECRET");
    }
    let effects = Arc::new(AtomicUsize::new(0));
    let redis = Arc::new(crate::api::billing_webhook::NoRedis {
        calls: AtomicUsize::new(0),
    });
    let app = Router::new()
        .route("/api/v1/webhooks/calcom", post(effect))
        .route_layer(axum::middleware::from_fn_with_state(
            crate::api::billing_webhook::WebhookState {
                rate_limiter: redis,
            },
            crate::api::billing_webhook::webhook_security_middleware,
        ))
        .with_state(effects.clone());
    let (origin, task) = serve(app).await;
    let response = client()
        .post(format!("{origin}/api/v1/webhooks/calcom"))
        .header(
            "X-Signature",
            format!("t={},v1=legacy", chrono::Utc::now().timestamp()),
        )
        .json(&serde_json::json!({"legacy_fixture":true}))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(dispatch_count(&effects).await, 1);
    configure();
    task.abort();
}

#[tokio::test]
async fn malformed_signature_fields_fail_before_effects_on_both_http_routes() {
    configure();
    let (billing_app, billing_effects, redis) = billing();
    let ledger_effects = Arc::new(AtomicUsize::new(0));
    let app = billing_app.nest(
        "/api/v1/payments/ledger",
        crate::api::payment_ledger::router().with_state(ledger_effects.clone()),
    );
    let (origin, task) = serve(app).await;
    for (path, raw, secret) in [
        ("/api/v1/webhooks/stripe", billing_payload(), BILLING_SECRET),
        (
            "/api/v1/payments/ledger/webhook",
            ledger_payload(true),
            LEDGER_SECRET,
        ),
    ] {
        let now = chrono::Utc::now().timestamp();
        let valid = sign(&raw, secret, now);
        for signature in [
            valid.replace("v1=", "v0="),
            format!("t={now},{valid}"),
            format!("t={now},v1=not-hex"),
            "x".repeat(4097),
        ] {
            let response = client()
                .post(format!("{origin}{path}"))
                .header("content-type", "application/json")
                .header("Stripe-Signature", signature)
                .body(raw.clone())
                .send()
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        }
    }
    assert_eq!(billing_effects.load(Ordering::SeqCst), 0);
    assert_eq!(ledger_effects.load(Ordering::SeqCst), 0);
    assert_eq!(redis.calls.load(Ordering::SeqCst), 0);
    task.abort();
}

#[tokio::test]
async fn empty_whitespace_and_oversized_configured_secrets_fail_closed() {
    configure();
    let (app, effects, redis) = billing();
    let (origin, task) = serve(app).await;
    let raw = billing_payload();
    let signature = sign(&raw, BILLING_SECRET, chrono::Utc::now().timestamp());
    for secret in [String::new(), " \t\n".into(), "x".repeat(4097)] {
        unsafe {
            std::env::set_var("STRIPE_WEBHOOK_SECRET", secret);
        }
        let response = client()
            .post(format!("{origin}/api/v1/webhooks/stripe"))
            .header("content-type", "application/json")
            .header("Stripe-Signature", &signature)
            .body(raw.clone())
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
    assert_eq!(effects.load(Ordering::SeqCst), 0);
    assert_eq!(redis.calls.load(Ordering::SeqCst), 0);
    configure();
    task.abort();
}

#[tokio::test]
async fn ledger_stream_body_limit_precedes_business_handler() {
    configure();
    let effects = Arc::new(AtomicUsize::new(0));
    let app = Router::new().nest(
        "/api/v1/payments/ledger",
        crate::api::payment_ledger::router().with_state(effects.clone()),
    );
    let (origin, task) = serve(app).await;
    let raw = vec![b' '; 1024 * 1024 + 1];
    let response = client()
        .post(format!("{origin}/api/v1/payments/ledger/webhook"))
        .header("content-type", "application/json")
        .header(
            "Stripe-Signature",
            sign(&raw, LEDGER_SECRET, chrono::Utc::now().timestamp()),
        )
        .body(raw)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    assert_eq!(effects.load(Ordering::SeqCst), 0);
    task.abort();
}

#[test]
fn actual_stripe_library_shared_verifier_rejects_unauthenticated_bytes() {
    use server_integrations_stripe::webhook_signature::{SignatureError, verify_at};
    assert_eq!(
        verify_at(b"{}", "", BILLING_SECRET.as_bytes(), 0),
        Err(SignatureError::InvalidHeader)
    );
}
