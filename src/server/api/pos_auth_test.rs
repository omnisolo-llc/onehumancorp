use super::*;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

fn claims(tenant: Option<&str>) -> ::server_common::Claims {
    ::server_common::Claims {
        sub: "signed-user".into(),
        exp: chrono::Utc::now().timestamp() + 3600,
        iat: 0,
        organization_id: tenant.map(str::to_owned),
        username: "Signed staff".into(),
        email: String::new(),
        roles: vec!["CASHIER".into()],
        session_id: None,
        jti: "pos-auth-contract".into(),
    }
}

async fn authenticate(
    claims: Option<::server_common::Claims>,
    payload: serde_json::Value,
) -> (StatusCode, serde_json::Value) {
    // Authentication reads claims only; no database or provider connection is needed.
    let pool = sqlx::postgres::PgPoolOptions::new()
        .min_connections(0)
        .max_connections(1)
        .connect_lazy("postgres://unused:unused@127.0.0.1:1/unused")
        .unwrap();
    let (event_tx, _event_rx) = tokio::sync::mpsc::channel(1);
    let mut app = axum::Router::new()
        .route("/auth", axum::routing::post(pos_auth_handler))
        .with_state(Arc::new(Hub::new(event_tx, pool)));
    if let Some(claims) = claims {
        app = app.layer(Extension(claims));
    }
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let body = axum::body::to_bytes(response.into_body(), 16 * 1024)
        .await
        .unwrap();
    (
        status,
        serde_json::from_slice(&body).unwrap_or(serde_json::Value::Null),
    )
}

#[tokio::test]
async fn empty_request_returns_only_the_signed_staff_and_tenant() {
    let (status, body) = authenticate(Some(claims(Some("tenant-a"))), json!({})).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        body,
        json!({"success":true,"staff":{"id":"signed-user","name":"Signed staff","role":"CASHIER","tenant_id":"tenant-a"}})
    );
}

#[tokio::test]
async fn no_request_field_can_act_as_a_pin_or_replace_signed_identity() {
    for payload in [
        json!({"pin":"1234"}),
        json!({"pin":""}),
        json!({"pin":"0000"}),
        json!({"staff_id":"another-user"}),
        json!({"tenant_id":"another-tenant"}),
        json!({"role":"ADMIN"}),
        json!({"unexpected":true}),
        json!(null),
        json!([]),
        json!([{}]),
        json!("1234"),
    ] {
        let (status, _) = authenticate(Some(claims(Some("tenant-a"))), payload.clone()).await;
        assert_eq!(
            status,
            StatusCode::UNPROCESSABLE_ENTITY,
            "accepted unsupported request: {payload}"
        );
    }
}

#[tokio::test]
async fn empty_request_never_substitutes_for_a_signed_tenant_account() {
    let (status, _) = authenticate(None, json!({})).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    for tenant in [None, Some(""), Some(" "), Some("system"), Some(" SyStEm ")] {
        let (status, _) = authenticate(Some(claims(tenant)), json!({})).await;
        assert_eq!(
            status,
            StatusCode::UNAUTHORIZED,
            "accepted invalid tenant {tenant:?}"
        );
    }
}
