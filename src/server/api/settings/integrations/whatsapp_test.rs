use super::whatsapp::{connect_whatsapp_cloud_api, connect_whatsapp_twilio};
use ::server_common::Claims;
use axum::{
    Router,
    body::{Body, to_bytes},
    extract::Extension,
    http::{Request, StatusCode},
    routing::post,
};
use tower::ServiceExt;

fn claims(tenant: Option<&str>, roles: &[&str]) -> Claims {
    Claims {
        sub: "fixture-owner".into(),
        exp: 0,
        iat: 0,
        organization_id: tenant.map(str::to_string),
        username: "fixture-owner".into(),
        email: "fixture@example.test".into(),
        roles: roles.iter().map(|role| (*role).to_string()).collect(),
        session_id: None,
        jti: "fixture-jti".into(),
    }
}

// These are the production adapters mounted by the main router. Deliberately
// supply no database or provider state: an unsupported connection must not use it.
fn routes(user: Claims) -> Router {
    Router::new()
        .route("/cloud", post(connect_whatsapp_cloud_api))
        .route("/twilio", post(connect_whatsapp_twilio))
        .layer(Extension(user))
}

#[tokio::test]
async fn both_providers_reject_repeated_unverified_inputs_without_storage_or_network() {
    for route in ["/cloud", "/twilio"] {
        for role in ["OWNER", "ADMIN"] {
            for payload in [
                "{}",
                r#"{"bot_token":"fixture-sid","api_token":"fixture-secret","from_phone":"+15555550123"}"#,
            ] {
                for _ in 0..2 {
                    let response = routes(claims(Some("fixture-tenant"), &[role]))
                        .oneshot(
                            Request::builder()
                                .method("POST")
                                .uri(route)
                                .header("content-type", "application/json")
                                .body(Body::from(payload))
                                .unwrap(),
                        )
                        .await
                        .unwrap();
                    assert_eq!(response.status(), StatusCode::NOT_IMPLEMENTED);
                    assert_eq!(response.headers()["cache-control"], "no-store");
                    let bytes = to_bytes(response.into_body(), 4096).await.unwrap();
                    let text = std::str::from_utf8(&bytes).unwrap();
                    assert!(!text.contains("fixture-secret"));
                    let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
                    assert_eq!(value["success"], false);
                    assert_eq!(value["usable"], false);
                    assert_eq!(value["status"], "pending_verification");
                    assert!(value.get("receipt").is_none());
                }
            }
        }
    }
}

#[tokio::test]
async fn missing_tenant_and_non_owner_requests_do_not_reach_connection_handling() {
    for route in ["/cloud", "/twilio"] {
        for (user, status) in [
            (claims(None, &["ADMIN"]), StatusCode::UNAUTHORIZED),
            (claims(Some("  "), &["ADMIN"]), StatusCode::UNAUTHORIZED),
            (
                claims(Some("fixture-tenant"), &["MEMBER"]),
                StatusCode::FORBIDDEN,
            ),
        ] {
            let response = routes(user)
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri(route)
                        .header("content-type", "application/json")
                        .body(Body::from("{}"))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), status);
            let bytes = to_bytes(response.into_body(), 4096).await.unwrap();
            let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(value["success"], false);
            assert_eq!(value["usable"], false);
        }
    }
}
