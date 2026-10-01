use super::*;
use axum::{
    Extension,
    body::Body,
    http::{Method, Request},
};
use tower::ServiceExt;

fn claims(tenant: &str, role: &str) -> server_common::Claims {
    server_common::Claims {
        sub: format!("user-{tenant}"),
        exp: chrono::Utc::now().timestamp() + 3600,
        iat: 0,
        organization_id: Some(tenant.into()),
        username: "test".into(),
        email: String::new(),
        roles: vec![role.into()],
        session_id: None,
        jti: "local-test".into(),
    }
}
fn policy(standalone: bool, configured_enabled: bool) -> Policy {
    Policy {
        standalone,
        multitenant: !standalone,
        configured_enabled,
    }
}
fn app(
    store: std::sync::Arc<crate::settings::Store>,
    policy: Policy,
    claims: server_common::Claims,
) -> axum::Router {
    router_with_policy(store, policy).layer(Extension(claims))
}
async fn request(
    app: axum::Router,
    method: Method,
    payload: serde_json::Value,
) -> (StatusCode, serde_json::Value) {
    let response = app
        .oneshot(
            Request::builder()
                .method(method)
                .uri("/api/v1/settings/telemetry")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null),
    )
}
fn run(test: impl std::future::Future<Output = ()>) {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(test);
}

#[test]
fn tenant_admins_cannot_mutate_process_global_collection_for_other_tenants() {
    let _guard = crate::settings::tests::telemetry_guard();
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("settings.json");
    let store = std::sync::Arc::new(crate::settings::Store::from_file(path.clone()).unwrap());
    run(async {
        for tenant in ["tenant-a", "tenant-b"] {
            let app = app(store.clone(), policy(false, false), claims(tenant, "ADMIN"));
            let (status, body) = request(
                app.clone(),
                Method::POST,
                serde_json::json!({"product_telemetry_enabled":true}),
            )
            .await;
            assert_eq!(status, StatusCode::FORBIDDEN);
            assert_eq!(body["success"], false);
            assert_eq!(body["can_change"], false);
            let (_, state) = request(app, Method::GET, serde_json::json!({})).await;
            assert_eq!(state["effective_enabled"], false);
            assert_eq!(
                state["change_block_reason"],
                "hosted_global_control_unavailable"
            );
        }
    });
    assert!(!path.exists());
    assert!(!store.get().product_telemetry_enabled);
    assert!(!server_config::DYNAMIC_TELEMETRY_ENABLED.load(std::sync::atomic::Ordering::Relaxed));
}
#[test]
fn standalone_admin_can_save_but_viewer_cannot_change_collection() {
    let _guard = crate::settings::tests::telemetry_guard();
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("settings.json");
    let store = std::sync::Arc::new(crate::settings::Store::from_file(path.clone()).unwrap());
    run(async {
        let (status, body) = request(
            app(
                store.clone(),
                policy(true, false),
                claims("local", "VIEWER"),
            ),
            Method::POST,
            serde_json::json!({"product_telemetry_enabled":true}),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN);
        assert_eq!(body["change_block_reason"], "admin_required");
        let (status, body) = request(
            app(store.clone(), policy(true, false), claims("local", "ADMIN")),
            Method::POST,
            serde_json::json!({"product_telemetry_enabled":true}),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["success"], true);
        assert_eq!(body["preference_enabled"], true);
        assert_eq!(body["effective_enabled"], true);
        assert_eq!(body["operator_enforced"], false);
        assert_eq!(body["can_change"], true);
    });
    let disk: crate::settings::AppSettings =
        serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    assert!(disk.product_telemetry_enabled);
}
#[test]
fn forced_configuration_cannot_be_misreported_as_disabled() {
    let _guard = crate::settings::tests::telemetry_guard();
    let directory = tempfile::tempdir().unwrap();
    let store = std::sync::Arc::new(
        crate::settings::Store::from_file(directory.path().join("settings.json")).unwrap(),
    );
    run(async {
        let app = app(store.clone(), policy(true, true), claims("local", "ADMIN"));
        let (status, state) = request(app.clone(), Method::GET, serde_json::json!({})).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(state["product_telemetry_enabled"], false);
        assert_eq!(state["preference_enabled"], false);
        assert_eq!(state["effective_enabled"], true);
        assert_eq!(state["operator_enforced"], true);
        assert_eq!(state["can_change"], false);
        let (status, body) = request(
            app,
            Method::POST,
            serde_json::json!({"product_telemetry_enabled":false}),
        )
        .await;
        assert_eq!(status, StatusCode::CONFLICT);
        assert_eq!(body["success"], false);
        assert_eq!(body["effective_enabled"], true);
    });
    assert!(!store.get().product_telemetry_enabled);
}
#[test]
fn failing_persistence_returns_false_and_preserves_effective_state() {
    let _guard = crate::settings::tests::telemetry_guard();
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("settings.json");
    let store = std::sync::Arc::new(crate::settings::Store::from_file(path.clone()).unwrap());
    std::fs::create_dir(&path).unwrap();
    run(async {
        let app = app(store.clone(), policy(true, false), claims("local", "ADMIN"));
        let (status, body) = request(
            app.clone(),
            Method::POST,
            serde_json::json!({"product_telemetry_enabled":true}),
        )
        .await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body["success"], false);
        assert_eq!(body["effective_enabled"], false);
        assert_eq!(body["preference_enabled"], false);
        let (_, state) = request(app, Method::GET, serde_json::json!({})).await;
        assert_eq!(state["effective_enabled"], false);
    });
}
#[test]
fn unavailable_persistent_store_is_read_only() {
    let _guard = crate::settings::tests::telemetry_guard();
    let store = std::sync::Arc::new(crate::settings::Store::new());
    run(async {
        let (_, state) = request(
            app(store, policy(true, false), claims("local", "ADMIN")),
            Method::GET,
            serde_json::json!({}),
        )
        .await;
        assert_eq!(state["can_change"], false);
        assert_eq!(
            state["change_block_reason"],
            "persistent_storage_unavailable"
        );
    });
}

#[test]
fn missing_identity_and_malformed_updates_never_change_settings() {
    let _guard = crate::settings::tests::telemetry_guard();
    let directory = tempfile::tempdir().unwrap();
    let store = std::sync::Arc::new(
        crate::settings::Store::from_file(directory.path().join("settings.json")).unwrap(),
    );
    run(async {
        let unauthenticated: axum::Router = router_with_policy(store.clone(), policy(true, false));
        for method in [Method::GET, Method::POST] {
            let (status, _) = request(
                unauthenticated.clone(),
                method,
                serde_json::json!({"product_telemetry_enabled":true}),
            )
            .await;
            assert_eq!(status, StatusCode::UNAUTHORIZED);
        }
        let admin = app(store.clone(), policy(true, false), claims("local", "ADMIN"));
        let (status, _) = request(
            admin.clone(),
            Method::POST,
            serde_json::json!({"product_telemetry_enabled":true}),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        for payload in [
            serde_json::json!({}),
            serde_json::json!({"product_telemetry_enabled":"false"}),
            serde_json::json!({"product_telemetry_enabled":false,"operator":true}),
        ] {
            let (status, _) = request(admin.clone(), Method::POST, payload).await;
            assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
        }
    });
    assert!(store.get().product_telemetry_enabled);
}
