use std::time::Duration;

use axum::Router;
use axum::body::{Body, Bytes};
use axum::extract::State;
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::Response;
use axum::routing::{get, post};
use futures_util::StreamExt;
use reqwest::Url;
use serde_json::Value;
use tokio::net::TcpListener;
use tokio::sync::{oneshot, watch};
use tokio::task::JoinHandle;
use uuid::Uuid;

use super::types::ResolvedModelSelection;

const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Clone)]
struct ProviderFacadeState {
    client: reqwest::Client,
    upstream_base_url: Url,
    upstream_api_key: String,
    model_id: String,
    reasoning_effort: Option<Value>,
    revoked: watch::Receiver<bool>,
    token: String,
}

#[derive(Clone, PartialEq)]
pub struct ProviderFacadeConfig {
    pub upstream_url: String,
    pub upstream_api_key: String,
    pub selection: ResolvedModelSelection,
    pub request_timeout: Duration,
}

impl ProviderFacadeConfig {
    pub fn new(
        upstream_url: impl Into<String>,
        upstream_api_key: impl Into<String>,
        selection: ResolvedModelSelection,
    ) -> Self {
        Self {
            upstream_url: upstream_url.into(),
            upstream_api_key: upstream_api_key.into(),
            selection,
            request_timeout: DEFAULT_REQUEST_TIMEOUT,
        }
    }

    pub fn with_timeout(mut self, request_timeout: Duration) -> Self {
        self.request_timeout = request_timeout;
        self
    }
}

impl std::fmt::Debug for ProviderFacadeConfig {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ProviderFacadeConfig")
            .field("upstream_url", &"<configured>")
            .field("upstream_api_key", &"<configured>")
            .field("model_id", &self.selection.model_id)
            .field("request_timeout", &self.request_timeout)
            .finish()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct ProviderFacadeRoute {
    base_url: String,
    token: String,
}

impl std::fmt::Debug for ProviderFacadeRoute {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ProviderFacadeRoute")
            .field("base_url", &self.base_url)
            .field("token", &"<scoped>")
            .finish()
    }
}

impl ProviderFacadeRoute {
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}

#[derive(Debug)]
pub enum ProviderFacadeError {
    InvalidConfiguration(String),
    Bind(String),
    Server(String),
}

impl std::fmt::Display for ProviderFacadeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidConfiguration(message) => {
                write!(
                    formatter,
                    "invalid provider facade configuration: {message}"
                )
            }
            Self::Bind(message) => write!(formatter, "provider facade bind failed: {message}"),
            Self::Server(message) => write!(formatter, "provider facade server failed: {message}"),
        }
    }
}

impl std::error::Error for ProviderFacadeError {}

pub struct ProviderFacade {
    route: ProviderFacadeRoute,
    revoke: watch::Sender<bool>,
    shutdown: Option<oneshot::Sender<()>>,
    task: Option<JoinHandle<Result<(), std::io::Error>>>,
}

impl std::fmt::Debug for ProviderFacade {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ProviderFacade")
            .field("route", &self.route)
            .field("running", &self.task.is_some())
            .finish()
    }
}

impl ProviderFacade {
    pub async fn start(
        upstream_url: impl AsRef<str>,
        upstream_api_key: impl Into<String>,
        selection: ResolvedModelSelection,
    ) -> Result<Self, ProviderFacadeError> {
        Self::start_with_config(ProviderFacadeConfig::new(
            upstream_url.as_ref().to_owned(),
            upstream_api_key,
            selection,
        ))
        .await
    }

    pub async fn start_with_config(
        config: ProviderFacadeConfig,
    ) -> Result<Self, ProviderFacadeError> {
        let upstream_base_url = parse_upstream_url(&config.upstream_url)?;
        if config.upstream_api_key.trim().is_empty()
            || config.upstream_api_key.chars().any(char::is_control)
        {
            return Err(ProviderFacadeError::InvalidConfiguration(
                "upstream API key must be non-empty and contain no control characters".to_owned(),
            ));
        }
        if config.selection.model_id.trim().is_empty()
            || config.selection.model_id.chars().any(char::is_control)
        {
            return Err(ProviderFacadeError::InvalidConfiguration(
                "resolved model id must be non-empty and contain no control characters".to_owned(),
            ));
        }
        if config.request_timeout.is_zero() {
            return Err(ProviderFacadeError::InvalidConfiguration(
                "request timeout must be greater than zero".to_owned(),
            ));
        }
        let client = reqwest::Client::builder()
            .timeout(config.request_timeout)
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|error| {
                ProviderFacadeError::Server(redact(&error.to_string(), &config.upstream_api_key))
            })?;
        let token = Uuid::new_v4().to_string();
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .map_err(|error| ProviderFacadeError::Bind(error.to_string()))?;
        let address = listener
            .local_addr()
            .map_err(|error| ProviderFacadeError::Bind(error.to_string()))?;
        let route = ProviderFacadeRoute {
            base_url: format!("http://{address}/v1"),
            token: token.clone(),
        };
        let (revoke, revoked) = watch::channel(false);
        let state = ProviderFacadeState {
            client,
            upstream_base_url,
            upstream_api_key: config.upstream_api_key,
            model_id: config.selection.model_id,
            reasoning_effort: config.selection.reasoning_effort.map(|effort| {
                serde_json::to_value(effort).expect("reasoning effort serializes as a string")
            }),
            revoked,
            token,
        };
        let app = Router::new()
            .route("/v1/models", get(models))
            .route("/v1/responses", post(responses))
            .route("/v1/chat/completions", post(chat_completions))
            .with_state(state);
        let (shutdown, mut shutdown_receiver) = oneshot::channel();
        let task = tokio::spawn(async move {
            axum::serve(listener, app)
                .with_graceful_shutdown(async move {
                    let _ = (&mut shutdown_receiver).await;
                })
                .await
        });
        Ok(Self {
            route,
            revoke,
            shutdown: Some(shutdown),
            task: Some(task),
        })
    }

    pub fn route(&self) -> &ProviderFacadeRoute {
        &self.route
    }

    pub async fn shutdown(mut self) -> Result<(), ProviderFacadeError> {
        self.revoke.send_replace(true);
        if let Some(shutdown) = self.shutdown.take() {
            let _ = shutdown.send(());
        }
        if let Some(task) = self.task.take() {
            let result = task
                .await
                .map_err(|error| ProviderFacadeError::Server(error.to_string()))?;
            result.map_err(|error| ProviderFacadeError::Server(error.to_string()))?;
        }
        Ok(())
    }
}

impl Drop for ProviderFacade {
    fn drop(&mut self) {
        self.revoke.send_replace(true);
        if let Some(task) = &self.task {
            task.abort();
        }
    }
}

async fn models(State(state): State<ProviderFacadeState>, headers: HeaderMap) -> Response {
    if !authorized(&headers, &state.token) {
        return error_response(
            StatusCode::UNAUTHORIZED,
            "provider facade authorization failed",
        );
    }
    forward(&state, reqwest::Method::GET, "models", None).await
}

async fn responses(
    State(state): State<ProviderFacadeState>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    json_proxy(&state, headers, body, "responses").await
}

async fn chat_completions(
    State(state): State<ProviderFacadeState>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    json_proxy(&state, headers, body, "chat/completions").await
}

async fn json_proxy(
    state: &ProviderFacadeState,
    headers: HeaderMap,
    body: Bytes,
    path: &str,
) -> Response {
    if !authorized(&headers, &state.token) {
        return error_response(
            StatusCode::UNAUTHORIZED,
            "provider facade authorization failed",
        );
    }
    let mut payload: Value = match serde_json::from_slice(&body) {
        Ok(payload) => payload,
        Err(_) => return error_response(StatusCode::BAD_REQUEST, "request body must be JSON"),
    };
    let Some(model) = payload.get("model").and_then(Value::as_str) else {
        return error_response(StatusCode::BAD_REQUEST, "request model is required");
    };
    if model != state.model_id {
        return error_response(StatusCode::BAD_REQUEST, "request model is not admitted");
    }
    if let Err(message) = bind_reasoning(&mut payload, path, state.reasoning_effort.as_ref()) {
        return error_response(StatusCode::BAD_REQUEST, message);
    }
    let body = Bytes::from(serde_json::to_vec(&payload).expect("JSON value serializes"));
    forward(state, reqwest::Method::POST, path, Some(body)).await
}

fn authorized(headers: &HeaderMap, token: &str) -> bool {
    let expected = format!("Bearer {token}");
    headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value == expected)
}

async fn forward(
    state: &ProviderFacadeState,
    method: reqwest::Method,
    path: &str,
    body: Option<Bytes>,
) -> Response {
    let url = upstream_url(&state.upstream_base_url, path);
    let mut request = state.client.request(method, url).header(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {}", state.upstream_api_key),
    );
    if let Some(body) = body {
        request = request
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body);
    }
    let response = match tokio::select! {
        biased;
        _ = wait_for_revocation(state.revoked.clone()) => {
            return error_response(StatusCode::SERVICE_UNAVAILABLE, "provider facade route revoked");
        }
        response = request.send() => response,
    } {
        Ok(response) => response,
        Err(error) => {
            return error_response(
                StatusCode::BAD_GATEWAY,
                &redact(&error.to_string(), &state.upstream_api_key),
            );
        }
    };
    let status =
        StatusCode::from_u16(response.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .cloned();
    if !response.status().is_success() {
        let body = tokio::select! {
            biased;
            _ = wait_for_revocation(state.revoked.clone()) => {
                return error_response(StatusCode::SERVICE_UNAVAILABLE, "provider facade route revoked");
            }
            body = response.bytes() => body.unwrap_or_default(),
        };
        let body = redact(&String::from_utf8_lossy(&body), &state.upstream_api_key);
        return response_with_body(status, content_type.as_ref(), body.into_bytes());
    }
    let secret = state.upstream_api_key.clone();
    let stream = response
        .bytes_stream()
        .take_until(wait_for_revocation(state.revoked.clone()))
        .map(move |chunk| {
            chunk.map_err(|error| std::io::Error::other(redact(&error.to_string(), &secret)))
        });
    let mut output = Response::new(Body::from_stream(stream));
    *output.status_mut() = status;
    if let Some(content_type) = content_type
        && let Ok(content_type) = HeaderValue::from_bytes(content_type.as_bytes())
    {
        output
            .headers_mut()
            .insert(header::CONTENT_TYPE, content_type);
    }
    output
}

fn response_with_body(
    status: StatusCode,
    content_type: Option<&HeaderValue>,
    body: Vec<u8>,
) -> Response {
    let mut response = Response::new(Body::from(body));
    *response.status_mut() = status;
    if let Some(content_type) = content_type {
        response
            .headers_mut()
            .insert(header::CONTENT_TYPE, content_type.clone());
    }
    response
}

fn error_response(status: StatusCode, message: &str) -> Response {
    response_with_body(
        status,
        Some(&HeaderValue::from_static("text/plain; charset=utf-8")),
        message.as_bytes().to_vec(),
    )
}

fn parse_upstream_url(value: &str) -> Result<Url, ProviderFacadeError> {
    let value = value.trim().trim_end_matches('/');
    let url = Url::parse(value).map_err(|_| {
        ProviderFacadeError::InvalidConfiguration(
            "upstream URL must be an absolute HTTP(S) URL".to_owned(),
        )
    })?;
    if !matches!(url.scheme(), "http" | "https")
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err(ProviderFacadeError::InvalidConfiguration(
            "upstream URL must not contain credentials, query parameters, or fragments".to_owned(),
        ));
    }
    Ok(url)
}

fn upstream_url(base: &Url, path: &str) -> Url {
    let mut url = base.clone();
    let path = format!("{}/{}", base.path().trim_end_matches('/'), path);
    url.set_path(&path);
    url
}

fn redact(message: &str, secret: &str) -> String {
    message.replace(secret, "[REDACTED]")
}

// The receiver remains valid for every accepted request, including connections
// spawned by axum after the listener task has stopped.
async fn wait_for_revocation(mut revoked: watch::Receiver<bool>) {
    loop {
        if *revoked.borrow_and_update() {
            return;
        }
        if revoked.changed().await.is_err() {
            return;
        }
    }
}

fn bind_reasoning(
    payload: &mut Value,
    path: &str,
    effort: Option<&Value>,
) -> Result<(), &'static str> {
    let object = payload
        .as_object_mut()
        .ok_or("request body must be an object")?;
    let (field, other_field) = if path == "responses" {
        ("reasoning", "reasoning_effort")
    } else {
        ("reasoning_effort", "reasoning")
    };
    if object.contains_key(other_field) {
        return Err("reasoning field does not match request dialect");
    }
    if path == "responses" {
        if let Some(reasoning) = object.get(field) {
            let reasoning = reasoning.as_object().ok_or("reasoning must be an object")?;
            if let Some(requested) = reasoning.get("effort") {
                if Some(requested) != effort {
                    return Err("request reasoning effort is not admitted");
                }
            }
        }
        if let Some(effort) = effort {
            let reasoning = object.entry(field).or_insert_with(|| serde_json::json!({}));
            reasoning
                .as_object_mut()
                .expect("validated reasoning object")
                .insert("effort".to_owned(), effort.clone());
        }
    } else {
        if let Some(requested) = object.get(field) {
            if Some(requested) != effort {
                return Err("request reasoning effort is not admitted");
            }
        }
        if let Some(effort) = effort {
            object.insert(field.to_owned(), effort.clone());
        }
    }
    Ok(())
}
