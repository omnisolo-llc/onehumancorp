use super::webhook::*;
use axum::{
    body::Body,
    http::{Request, StatusCode, header},
    routing::post,
    Router,
};
use tower::ServiceExt;

#[tokio::test]
async fn test_twilio_voice_webhook_handler() {
    std::env::set_var("TWILIO_AUTH_TOKEN", "test_token");
    std::env::set_var("TWILIO_WEBHOOK_URL", "https://api.ohc.com/webhook");

    let app = Router::new().route("/webhook", post(twilio_voice_webhook_handler));

    let body = "CallSid=CA12345&From=%2B1234567890&To=%2B0987654321&CallStatus=ringing";

    // Compute the correct mock signature
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    use base64::{Engine as _, engine::general_purpose::STANDARD};

    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(b"test_token").expect("HMAC can take key of any size");

    // The keys sorted: CallSid, CallStatus, From, To
    let data = "https://api.ohc.com/webhookCallSidCA12345CallStatusringingFrom+1234567890To+0987654321";
    mac.update(data.as_bytes());
    let signature = STANDARD.encode(mac.finalize().into_bytes());

    let request = Request::builder()
        .uri("/webhook")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header("X-Twilio-Signature", signature)
        .body(Body::from(body))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response.headers().get(header::CONTENT_TYPE).unwrap();
    assert_eq!(content_type, "text/xml");

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

    assert!(body_str.contains("Connecting you to the OHC AI Receptionist"));
    assert!(body_str.contains("<Stream url="));
}
