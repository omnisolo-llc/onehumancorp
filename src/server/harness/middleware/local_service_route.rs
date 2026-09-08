//! Loopback service surface for external harnesses. The listener owns the scope
//! and opaque route credential; request bodies never supply authority.
use super::local_service_gateway::{
    LocalServiceGateway, LocalServiceGatewayError, LocalServiceOperation,
};
use super::local_services::{LocalServiceBundle, LocalServiceScopeContext};
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

#[derive(Clone)]
pub struct LocalServiceRoute {
    base_url: String,
    token: String,
}
impl std::fmt::Debug for LocalServiceRoute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalServiceRoute")
            .field("base_url", &self.base_url)
            .field("token", &"<scoped>")
            .finish()
    }
}
impl LocalServiceRoute {
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
    pub fn token(&self) -> &str {
        &self.token
    }
}
#[derive(Clone)]
struct RouteState {
    gateway: Arc<LocalServiceGateway>,
    bundle: LocalServiceBundle,
    scope: LocalServiceScopeContext,
    token: String,
}
pub struct LocalServiceListener {
    route: LocalServiceRoute,
    state: RouteState,
    shutdown: Option<oneshot::Sender<()>>,
    task: Option<JoinHandle<std::io::Result<()>>>,
}
impl LocalServiceListener {
    pub async fn start(
        gateway: Arc<LocalServiceGateway>,
        bundle: LocalServiceBundle,
        scope: LocalServiceScopeContext,
    ) -> std::io::Result<Self> {
        gateway
            .registry()
            .validate_issued(&bundle, &scope)
            .map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    "local service bindings were not admitted",
                )
            })?;
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let route = LocalServiceRoute {
            base_url: format!("http://{}/v1", listener.local_addr()?),
            token: uuid::Uuid::new_v4().to_string(),
        };
        let state = RouteState {
            gateway,
            bundle,
            scope,
            token: route.token.clone(),
        };
        let app = Router::new()
            .route("/v1/operations", axum::routing::post(operation))
            .with_state(state.clone());
        let (shutdown, receive) = oneshot::channel();
        let task = tokio::spawn(async move {
            axum::serve(listener, app)
                .with_graceful_shutdown(async move {
                    let _ = receive.await;
                })
                .await
        });
        Ok(Self {
            route,
            state,
            shutdown: Some(shutdown),
            task: Some(task),
        })
    }
    pub fn route(&self) -> &LocalServiceRoute {
        &self.route
    }
    pub async fn shutdown(mut self) -> std::io::Result<()> {
        self.state.gateway.release(&self.state.bundle).await;
        if let Some(shutdown) = self.shutdown.take() {
            let _ = shutdown.send(());
        }
        if let Some(mut task) = self.task.take() {
            match tokio::time::timeout(std::time::Duration::from_secs(5), &mut task).await {
                Ok(result) => {
                    result.map_err(|_| std::io::Error::other("local service listener failed"))??
                }
                Err(_) => {
                    task.abort();
                    let _ = task.await;
                }
            }
        }
        Ok(())
    }
}
impl Drop for LocalServiceListener {
    fn drop(&mut self) {
        if let Some(attempt) = self.state.scope.attempt_id {
            self.state
                .gateway
                .registry()
                .revoke_attempt(&self.state.scope.tenant_id, attempt);
        }
        if let Some(shutdown) = self.shutdown.take() {
            let _ = shutdown.send(());
        }
        if let Some(task) = self.task.take() {
            task.abort();
        }
        if let Ok(runtime) = tokio::runtime::Handle::try_current() {
            let gateway = self.state.gateway.clone();
            let bundle = self.state.bundle.clone();
            runtime.spawn(async move {
                gateway.release(&bundle).await;
            });
        }
    }
}
async fn operation(
    State(state): State<RouteState>,
    headers: HeaderMap,
    Json(operation): Json<LocalServiceOperation>,
) -> Response {
    let expected = format!("Bearer {}", state.token);
    if headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        != Some(expected.as_str())
    {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error":"local service route authorization rejected"})),
        )
            .into_response();
    }
    let Some(binding) = state.bundle.binding(operation.capability().0) else {
        return (
            StatusCode::NOT_IMPLEMENTED,
            Json(json!({"error":"local service is unavailable"})),
        )
            .into_response();
    };
    match state
        .gateway
        .execute(binding, &state.scope, operation)
        .await
    {
        Ok(result) => Json(result).into_response(),
        Err(error) => {
            let status = match error {
                LocalServiceGatewayError::Authorization(_) => StatusCode::FORBIDDEN,
                LocalServiceGatewayError::UnsupportedCapability(_) => StatusCode::NOT_IMPLEMENTED,
                LocalServiceGatewayError::InvalidOperation => StatusCode::BAD_REQUEST,
                LocalServiceGatewayError::BackendFailure => StatusCode::BAD_GATEWAY,
            };
            (status, Json(json!({"error":error.to_string()}))).into_response()
        }
    }
}
