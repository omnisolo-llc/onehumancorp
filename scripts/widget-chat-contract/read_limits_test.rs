use super::*;
use axum::body::to_bytes;

#[tokio::test]
async fn widget_auth_and_input_errors_always_disable_shared_caching() {
    let f = Fixture::new().await;
    for (token, body) in [
        (None, "{}".to_string()),
        (Some(f.token.as_str()), "{".into()),
    ] {
        let mut req = Request::builder()
            .method("POST")
            .uri("/api/widget/conversations")
            .header("content-type", "application/json");
        if let Some(token) = token {
            req = req.header("authorization", format!("Bearer {token}"));
        }
        let response = f
            .app
            .clone()
            .oneshot(req.body(Body::from(body)).unwrap())
            .await
            .unwrap();
        assert!(!response.status().is_success());
        assert_eq!(
            response
                .headers()
                .get("cache-control")
                .and_then(|v| v.to_str().ok()),
            Some("private, no-store")
        );
    }
}
#[tokio::test]
async fn widget_transport_rejects_oversized_bodies_before_json_or_storage() {
    let f = Fixture::new().await;
    let response = f
        .app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/widget/messages")
                .header("authorization", format!("Bearer {}", f.token))
                .header("content-type", "application/json")
                .body(Body::from("x".repeat(65_537)))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    assert_eq!(response.headers()["cache-control"], "private, no-store");
    assert!(to_bytes(response.into_body(), 4096).await.unwrap().len() < 4096);
}
