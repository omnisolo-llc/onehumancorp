use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::{Value, json};
use std::{sync::Arc, time::Duration};
use tower::ServiceExt;
use uuid::Uuid;

fn disconnected_db() -> Arc<crate::db::DB> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_millis(50))
        .connect_lazy("postgres://synthetic_widget_fixture@127.0.0.1:1/ohc_widget_test")
        .unwrap();
    Arc::new(crate::db::DB {
        pool,
        store: crate::db::DbStore::Postgres,
    })
}
struct Fixture {
    app: axum::Router,
    tenant: Uuid,
    foreign: Uuid,
    token: String,
}
impl Fixture {
    async fn new() -> Self {
        let store = Arc::new(server_auth::Store::new());
        let tenant = Uuid::new_v4();
        let foreign = Uuid::new_v4();
        let user = store
            .create_user(
                "synthetic-widget-owner".into(),
                "widget-owner@example.test".into(),
                "public-synthetic-fixture-password".into(),
                vec!["ADMIN".into()],
                tenant.to_string(),
            )
            .await
            .unwrap();
        let token = store.issue_token(&user).unwrap();
        let app = crate::actual_parent_mount(disconnected_db(), store);
        Self {
            app,
            tenant,
            foreign,
            token,
        }
    }
    async fn request(
        &self,
        method: &str,
        path: &str,
        token: Option<&str>,
        body: Value,
    ) -> StatusCode {
        let mut request = Request::builder()
            .method(method)
            .uri(path)
            .header("content-type", "application/json")
            .header("x-tenant-id", self.tenant.to_string())
            .header("x-user-id", "forged-owner");
        if let Some(token) = token {
            request = request.header("authorization", format!("Bearer {token}"));
        }
        let response = tokio::time::timeout(
            Duration::from_secs(2),
            self.app
                .clone()
                .oneshot(request.body(Body::from(body.to_string())).unwrap()),
        )
        .await
        .expect("bounded local request")
        .unwrap();
        response.status()
    }
}
#[tokio::test]
async fn actual_parent_router_builds_with_current_axum_capture_syntax() {
    let store = Arc::new(server_auth::Store::new());
    let _app = crate::actual_parent_mount(disconnected_db(), store);
}
#[tokio::test]
async fn all_widget_routes_reject_anonymous_requests_before_database_access() {
    let f = Fixture::new().await;
    for (method, path, body) in [
        (
            "POST",
            "/api/widget/conversations".to_string(),
            json!({"tenant_id":f.tenant,"inbox_id":Uuid::new_v4(),"contact_id":Uuid::new_v4()}),
        ),
        (
            "POST",
            "/api/widget/messages".to_string(),
            json!({"tenant_id":f.tenant,"conversation_id":Uuid::new_v4(),"content":"fixture"}),
        ),
        (
            "GET",
            format!(
                "/api/widget/{}/conversations/{}/messages",
                f.tenant,
                Uuid::new_v4()
            ),
            Value::Null,
        ),
    ] {
        assert_eq!(
            f.request(method, &path, None, body).await,
            StatusCode::UNAUTHORIZED,
            "{path}"
        );
    }
}
#[tokio::test]
async fn malformed_bearer_and_forged_identity_headers_do_not_authorize_widget_access() {
    let f = Fixture::new().await;
    assert_eq!(
        f.request(
            "POST",
            "/api/widget/conversations",
            Some("invalid"),
            json!({"tenant_id":f.tenant,"inbox_id":Uuid::new_v4(),"contact_id":Uuid::new_v4()})
        )
        .await,
        StatusCode::UNAUTHORIZED
    );
}
#[tokio::test]
async fn signed_identity_cannot_choose_a_foreign_conversation_tenant() {
    let f = Fixture::new().await;
    assert_eq!(
        f.request(
            "POST",
            "/api/widget/conversations",
            Some(&f.token),
            json!({"tenant_id":f.foreign,"inbox_id":Uuid::new_v4(),"contact_id":Uuid::new_v4()})
        )
        .await,
        StatusCode::FORBIDDEN
    );
}
#[tokio::test]
async fn signed_identity_cannot_choose_a_foreign_message_tenant() {
    let f = Fixture::new().await;
    assert_eq!(
        f.request(
            "POST",
            "/api/widget/messages",
            Some(&f.token),
            json!({"tenant_id":f.foreign,"conversation_id":Uuid::new_v4(),"content":"fixture"})
        )
        .await,
        StatusCode::FORBIDDEN
    );
}
#[tokio::test]
async fn signed_identity_cannot_read_a_foreign_path_tenant() {
    let f = Fixture::new().await;
    let path = format!(
        "/api/widget/{}/conversations/{}/messages",
        f.foreign,
        Uuid::new_v4()
    );
    assert_eq!(
        f.request("GET", &path, Some(&f.token), Value::Null).await,
        StatusCode::FORBIDDEN
    );
}
#[tokio::test]
async fn unavailable_storage_is_not_reported_as_an_empty_history_or_successful_write() {
    let f = Fixture::new().await;
    let path = format!(
        "/api/widget/{}/conversations/{}/messages",
        f.tenant,
        Uuid::new_v4()
    );
    assert!(
        f.request("GET", &path, Some(&f.token), Value::Null)
            .await
            .is_server_error()
    );
    assert!(
        f.request(
            "POST",
            "/api/widget/conversations",
            Some(&f.token),
            json!({"tenant_id":f.tenant,"inbox_id":Uuid::new_v4(),"contact_id":Uuid::new_v4()})
        )
        .await
        .is_server_error()
    );
}

#[path = "postgres_test.rs"]
mod postgres;

#[path = "read_limits_test.rs"]
mod read_limits;
