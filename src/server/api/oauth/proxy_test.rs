use super::proxy::*;
use axum::extract::Query;
use std::collections::HashMap;
use axum::response::IntoResponse;
use axum::http::StatusCode;

#[tokio::test]
async fn test_proxy_valid_tunnel_id() {
    let q = OAuthCallbackQuery {
        code: "testcode".to_string(),
        state: "standalone_my-tunnel-123_realstate".to_string(),
        extra: HashMap::new(),
    };

    let res = handle_oauth_callback(Query(q)).await.into_response();
    assert_eq!(res.status(), StatusCode::TEMPORARY_REDIRECT);
}

#[tokio::test]
async fn test_proxy_invalid_tunnel_id() {
    let q = OAuthCallbackQuery {
        code: "testcode".to_string(),
        state: "standalone_my-tunnel-123/../../_realstate".to_string(),
        extra: HashMap::new(),
    };

    let res = handle_oauth_callback(Query(q)).await.into_response();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}
