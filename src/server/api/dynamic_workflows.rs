use axum::{
    Json, Router,
    extract::{Extension, Path, State, rejection::JsonRejection},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
};
use serde_json::json;
use server_auth::commit_authority::{AuthorityError, verify_owner};
use server_common::{Claims, auth_utils::signed_tenant_id};
use std::sync::Arc;

use crate::orchestration::dynamic_workflows::{
    DynamicWorkflowError, DynamicWorkflowManager, DynamicWorkflowRequest,
};

#[derive(Clone)]
struct WorkflowState {
    manager: Arc<DynamicWorkflowManager>,
    auth: Arc<server_auth::Store>,
}

pub fn router<S>(manager: Arc<DynamicWorkflowManager>, auth: Arc<server_auth::Store>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(start_workflow))
        .route("/{id}", get(get_workflow))
        .route("/{id}/confirm", post(confirm_workflow))
        .with_state(WorkflowState { manager, auth })
}

async fn start_workflow(
    State(state): State<WorkflowState>,
    Extension(claims): Extension<Claims>,
    headers: HeaderMap,
    request: Result<Json<DynamicWorkflowRequest>, JsonRejection>,
) -> axum::response::Response {
    let owner = match verify_owner(&state.auth, &claims, &headers).await {
        Ok(owner) => owner,
        Err(error) => return authority_error(error),
    };
    let request = match request {
        Ok(Json(request)) => request,
        Err(error) => {
            return (
                error.status(),
                Json(json!({ "error": "Invalid workflow request" })),
            )
                .into_response();
        }
    };
    if request.prompt.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "prompt is required" })),
        )
            .into_response();
    }

    match state.manager.start_workflow(&owner, request).await {
        Ok(start) => (StatusCode::OK, Json(json!(start))).into_response(),
        Err(error) => workflow_error(error),
    }
}

async fn confirm_workflow(
    State(state): State<WorkflowState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    headers: HeaderMap,
) -> axum::response::Response {
    let owner = match verify_owner(&state.auth, &claims, &headers).await {
        Ok(owner) => owner,
        Err(error) => return authority_error(error),
    };
    match state.manager.confirm_workflow(&owner, &id).await {
        Ok(start) => (StatusCode::OK, Json(json!(start))).into_response(),
        Err(error) => workflow_error(error),
    }
}

async fn get_workflow(
    State(state): State<WorkflowState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> axum::response::Response {
    let Some(tenant) = signed_tenant_id(&claims) else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    match state.manager.get_workflow(&tenant, &id) {
        Ok(Some(plan)) => (StatusCode::OK, Json(json!(plan))).into_response(),
        Ok(None) => workflow_error(DynamicWorkflowError::NotFound),
        Err(error) => workflow_error(error),
    }
}

fn authority_error(error: AuthorityError) -> axum::response::Response {
    let (status, message) = match error {
        AuthorityError::Forbidden => (StatusCode::FORBIDDEN, "current owner authority is required"),
        AuthorityError::Unavailable | AuthorityError::Database(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            "owner authority is unavailable",
        ),
    };
    (status, Json(json!({ "error": message }))).into_response()
}

fn workflow_error(error: DynamicWorkflowError) -> axum::response::Response {
    let (status, message) = match error {
        DynamicWorkflowError::NotFound => (StatusCode::NOT_FOUND, "workflow not found"),
        DynamicWorkflowError::NotTriggered => (
            StatusCode::BAD_REQUEST,
            "task does not require a dynamic workflow",
        ),
        DynamicWorkflowError::Internal(error) => {
            tracing::warn!(%error, "Dynamic workflow operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "workflow operation failed",
            )
        }
    };
    (status, Json(json!({ "error": message }))).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::queue::{Job, TaskQueue};
    use async_trait::async_trait;
    use axum::{
        body::{Body, to_bytes},
        http::Request,
        middleware::from_fn_with_state,
    };
    use std::sync::Mutex;
    use tower::ServiceExt;

    // Observe the dispatch boundary without running a worker or provider.
    #[derive(Default)]
    struct RecordingQueue(Mutex<Vec<Job>>);

    #[async_trait]
    impl TaskQueue for RecordingQueue {
        async fn enqueue(&self, job: Job) -> Result<(), String> {
            self.0.lock().unwrap().push(job);
            Ok(())
        }
        async fn dequeue(&self, _: Vec<String>) -> Result<Option<Job>, String> {
            Ok(None)
        }
        async fn complete(&self, _: &str, _: &str) -> Result<(), String> {
            Ok(())
        }
        async fn fail(&self, _: &str, _: &str, _: &str) -> Result<(), String> {
            Ok(())
        }
        async fn requeue(&self, job: Job) -> Result<(), String> {
            self.enqueue(job).await
        }
        async fn cleanup_stale_jobs(&self) -> Result<u64, String> {
            Ok(0)
        }
    }

    struct Fixture {
        auth: Arc<server_auth::Store>,
        token: String,
        manager: Arc<DynamicWorkflowManager>,
        queue: Arc<RecordingQueue>,
        state: tempfile::TempDir,
    }

    struct Reply {
        status: StatusCode,
        content_type: String,
        body: String,
    }

    impl Reply {
        fn json(&self) -> serde_json::Value {
            assert_eq!(self.content_type, "application/json", "{}", self.body);
            serde_json::from_str(&self.body).expect("workflow response must be JSON")
        }
    }

    impl Fixture {
        async fn new() -> Self {
            let pool = sqlx::sqlite::SqlitePoolOptions::new()
                .max_connections(1)
                .connect("sqlite::memory:")
                .await
                .unwrap();
            let database = crate::persistence::AppDatabase::from_connection(
                sea_orm::SqlxSqliteConnector::from_sqlx_sqlite_pool(pool),
            );
            crate::persistence::migration::migrate(&database)
                .await
                .unwrap();
            let auth = Arc::new(server_auth::Store::with_portable_repo(Arc::new(
                server_auth::seaorm_store::SeaOrmAuthRepository::new(database.connection().clone()),
            )));
            let token = Self::actor(&auth, "tenant-owned", "OWNER").await;
            let queue = Arc::new(RecordingQueue::default());
            let state = tempfile::tempdir().unwrap();
            let manager = Arc::new(DynamicWorkflowManager::with_state_dir(
                queue.clone(),
                state.path().into(),
            ));
            Self {
                auth,
                token,
                manager,
                queue,
                state,
            }
        }

        async fn actor(auth: &server_auth::Store, tenant: &str, role: &str) -> String {
            let user = auth
                .create_user(
                    format!("workflow-{tenant}"),
                    format!("workflow-{tenant}@example.test"),
                    "public-local-workflow-fixture".into(),
                    vec![role.into()],
                    tenant.into(),
                )
                .await
                .unwrap();
            auth.issue_token(&user).unwrap()
        }

        fn restart_manager(&mut self) {
            self.manager = Arc::new(DynamicWorkflowManager::with_state_dir(
                self.queue.clone(),
                self.state.path().into(),
            ));
        }

        async fn call(&self, token: Option<&str>, method: &str, path: &str, body: &str) -> Reply {
            let app = router(self.manager.clone(), self.auth.clone()).layer(from_fn_with_state(
                self.auth.clone(),
                server_auth::strict_bearer_auth_middleware,
            ));
            let mut request = Request::builder()
                .method(method)
                .uri(path)
                .header("content-type", "application/json");
            if let Some(token) = token {
                request = request.header("authorization", format!("Bearer {token}"));
            }
            let response = app
                .oneshot(request.body(Body::from(body.to_owned())).unwrap())
                .await
                .unwrap();
            let status = response.status();
            let content_type = response
                .headers()
                .get("content-type")
                .map(|value| value.to_str().unwrap().to_owned())
                .unwrap_or_default();
            let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
            Reply {
                status,
                content_type,
                body: String::from_utf8(bytes.to_vec()).unwrap(),
            }
        }

        async fn create_plan(&self) -> serde_json::Value {
            // Include the legacy field here so authority tests exercise their
            // own boundary even before the missing-field regression is fixed.
            let response = self
                .call(
                    Some(&self.token),
                    "POST",
                    "/",
                    r#"{"tenant_id":"untrusted","prompt":"Test dynamic workflow prompt"}"#,
                )
                .await;
            assert_eq!(response.status, StatusCode::OK, "{}", response.body);
            let created = response.json();
            assert_eq!(created["plan"]["tenant_id"], "tenant-owned");
            assert_eq!(created["plan"]["status"], "awaiting_confirmation");
            assert_eq!(created["enqueued_jobs"], 0);
            created["plan"].clone()
        }
    }

    #[tokio::test]
    async fn generation_uses_signed_owner_and_persists_without_dispatch() {
        let mut f = Fixture::new().await;
        for body in [
            json!({"prompt":"Test dynamic workflow prompt"}),
            json!({"prompt":"Test dynamic workflow prompt", "tenant_id":"foreign"}),
        ] {
            let response = f.call(Some(&f.token), "POST", "/", &body.to_string()).await;
            assert_eq!(response.status, StatusCode::OK, "{}", response.body);
            let created = response.json();
            assert_eq!(created["plan"]["tenant_id"], "tenant-owned");
            assert_eq!(created["plan"]["status"], "awaiting_confirmation");
            assert_eq!(created["plan"]["requires_confirmation"], true);
            assert_eq!(created["enqueued_jobs"], 0);
            f.restart_manager();
            let response = f
                .call(
                    Some(&f.token),
                    "GET",
                    &format!("/{}", created["plan"]["id"].as_str().unwrap()),
                    "",
                )
                .await;
            assert_eq!(response.status, StatusCode::OK, "{}", response.body);
            assert_eq!(response.json(), created["plan"]);
        }
        assert!(f.queue.0.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn foreign_and_invalid_workflows_cannot_be_read_or_confirmed() {
        let mut f = Fixture::new().await;
        let foreign = Fixture::actor(&f.auth, "tenant-foreign", "ADMIN").await;
        let plan = f.create_plan().await;
        let id = plan["id"].as_str().unwrap();
        for cold in [false, true] {
            if cold {
                f.restart_manager();
            }
            for (token, rejected_id) in [
                (foreign.as_str(), id),
                (f.token.as_str(), "dwf-invalid"),
                (f.token.as_str(), "..%2Foutside"),
                (f.token.as_str(), "dwf-00000000-0000-0000-0000-000000000000"),
            ] {
                for (method, path) in [
                    ("GET", format!("/{rejected_id}")),
                    ("POST", format!("/{rejected_id}/confirm")),
                ] {
                    let response = f.call(Some(token), method, &path, "").await;
                    assert_eq!(response.status, StatusCode::NOT_FOUND, "{}", response.body);
                    assert_eq!(response.json(), json!({"error":"workflow not found"}));
                }
            }
            for (method, path) in [
                ("GET", format!("/{id}")),
                ("POST", format!("/{id}/confirm")),
            ] {
                assert_eq!(
                    f.call(None, method, &path, "").await.status,
                    StatusCode::UNAUTHORIZED
                );
            }
            let response = f.call(Some(&f.token), "GET", &format!("/{id}"), "").await;
            assert_eq!(response.status, StatusCode::OK);
            assert_eq!(response.json(), plan);
            assert!(f.queue.0.lock().unwrap().is_empty());
        }
    }

    #[tokio::test]
    async fn removed_owner_role_is_rechecked_before_request_validation_or_confirmation() {
        let f = Fixture::new().await;
        let plan = f.create_plan().await;
        let claims = f.auth.validate_token(&f.token).await.unwrap();
        assert!(claims.roles.iter().any(|role| role == "OWNER"));
        f.auth
            .update_user(
                &claims.sub,
                None,
                Some(vec!["MEMBER".into()]),
                None,
                "tenant-owned",
            )
            .await
            .unwrap();
        for (path, body) in [
            ("/".to_owned(), "{"),
            (
                "/".to_owned(),
                r#"{"tenant_id":"tenant-owned","prompt":"Test dynamic workflow prompt","confirm":true}"#,
            ),
            (format!("/{}/confirm", plan["id"].as_str().unwrap()), ""),
        ] {
            let response = f.call(Some(&f.token), "POST", &path, body).await;
            assert_eq!(response.status, StatusCode::FORBIDDEN, "{}", response.body);
            assert_eq!(
                response.json(),
                json!({"error":"current owner authority is required"})
            );
        }
        let response = f
            .call(
                Some(&f.token),
                "GET",
                &format!("/{}", plan["id"].as_str().unwrap()),
                "",
            )
            .await;
        assert_eq!(response.status, StatusCode::OK);
        assert_eq!(response.json(), plan);
        assert!(f.queue.0.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn revoked_bearer_cannot_create_read_or_confirm() {
        let f = Fixture::new().await;
        let plan = f.create_plan().await;
        let claims = f.auth.validate_token(&f.token).await.unwrap();
        f.auth
            .revoke_token(
                claims.jti,
                chrono::DateTime::from_timestamp(claims.exp, 0).unwrap(),
                "tenant-owned",
            )
            .await
            .unwrap();
        for (method, path, body) in [
            (
                "POST",
                "/".to_owned(),
                r#"{"tenant_id":"tenant-owned","prompt":"Test dynamic workflow prompt","confirm":true}"#,
            ),
            ("GET", format!("/{}", plan["id"].as_str().unwrap()), ""),
            (
                "POST",
                format!("/{}/confirm", plan["id"].as_str().unwrap()),
                "",
            ),
        ] {
            assert_eq!(
                f.call(Some(&f.token), method, &path, body).await.status,
                StatusCode::UNAUTHORIZED
            );
        }
        let stored = f
            .manager
            .get_workflow("tenant-owned", plan["id"].as_str().unwrap())
            .unwrap()
            .unwrap();
        assert_eq!(serde_json::to_value(stored).unwrap(), plan);
        assert!(f.queue.0.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn invalid_requests_return_structured_errors_without_dispatch() {
        let f = Fixture::new().await;
        for (body, status, message) in [
            ("{", StatusCode::BAD_REQUEST, "Invalid workflow request"),
            (
                r#"{"tenant_id":"untrusted","prompt":42}"#,
                StatusCode::UNPROCESSABLE_ENTITY,
                "Invalid workflow request",
            ),
            (
                r#"{"tenant_id":"untrusted"}"#,
                StatusCode::UNPROCESSABLE_ENTITY,
                "Invalid workflow request",
            ),
            (
                r#"{"tenant_id":"untrusted","prompt":" "}"#,
                StatusCode::BAD_REQUEST,
                "prompt is required",
            ),
        ] {
            let response = f.call(Some(&f.token), "POST", "/", body).await;
            assert_eq!(response.status, status, "{}", response.body);
            assert_eq!(response.json(), json!({"error":message}));
        }
        assert!(f.queue.0.lock().unwrap().is_empty());
    }
}
