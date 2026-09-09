//! Service-owned bootstrap: storage selection and admitted sessions come from a
//! mounted control-plane file, never from a native harness's request or process.
use server_harness::middleware::{
    grpc::HarnessWorkerGrpcService,
    local_service_gateway::{LocalServiceGateway, SqliteAgentMemoryBackend},
    local_services::{LocalServiceKind, LocalServiceRegistry, LocalServiceScopeContext},
};
use std::{path::Path, sync::Arc};

pub(crate) struct Bootstrap {
    scopes: Vec<LocalServiceScopeContext>,
    sqlite_url: Option<String>,
    memory: Option<Value>,
    embedding: Option<Value>,
    blob_root: Option<String>,
    identity: Vec<u8>,
    tool_root: Option<String>,
    allowed_tools: Vec<String>,
    browser: bool,
}
impl Bootstrap {
    fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() > 65536 {
            return Err("local service configuration is too large");
        }
        let value: serde_json::Value =
            serde_json::from_slice(bytes).map_err(|_| "invalid local service configuration")?;
        if value.get("version").and_then(serde_json::Value::as_u64) != Some(1) {
            return Err("unsupported local service configuration");
        }
        let scopes: Vec<LocalServiceScopeContext> = serde_json::from_value(
            value
                .get("scopes")
                .cloned()
                .ok_or("admitted scopes are required")?,
        )
        .map_err(|_| "invalid admitted scopes")?;
        let mut sessions = std::collections::BTreeSet::new();
        if scopes.is_empty()
            || scopes.len() > 256
            || scopes.iter().any(|scope| {
                scope.tenant_id.trim().is_empty()
                    || scope
                        .project_id
                        .as_deref()
                        .is_none_or(|id| id.trim().is_empty())
                    || scope
                        .workspace_id
                        .as_deref()
                        .is_none_or(|id| id.trim().is_empty())
                    || scope.session_id.is_nil()
                    || scope.task_id.is_some_and(|id| id.is_nil())
                    || !sessions.insert(scope.session_id)
                    || scope.attempt_id.is_some()
            })
        {
            return Err("configuration requires distinct admitted session namespaces");
        }
        let sqlite_url = value
            .get("sqlite_url")
            .filter(|v| !v.is_null())
            .map(|v| {
                v.as_str()
                    .filter(|url| url.starts_with("sqlite:") && !url.contains(":memory:"))
                    .map(str::to_owned)
                    .ok_or("invalid selected memory database")
            })
            .transpose()?;
        let blob_root = value
            .get("blob_root")
            .filter(|v| !v.is_null())
            .map(|v| {
                v.as_str()
                    .filter(|root| Path::new(root).is_absolute())
                    .map(str::to_owned)
                    .ok_or("selected blob root must be absolute")
            })
            .transpose()?;
        let tool_root = value
            .get("tool_root")
            .filter(|v| !v.is_null())
            .map(|v| {
                v.as_str()
                    .filter(|root| Path::new(root).is_absolute())
                    .map(str::to_owned)
                    .ok_or("selected tool root must be absolute")
            })
            .transpose()?;
        let allowed_tools: Vec<String> = serde_json::from_value(
            value
                .get("allowed_tools")
                .cloned()
                .unwrap_or(serde_json::json!([])),
        )
        .map_err(|_| "invalid configured tool catalog")?;
        let browser = value
            .get("browser")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);
        if (browser || !allowed_tools.is_empty()) && tool_root.is_none() {
            return Err("configured tools require a service-owned root");
        }
        let memory = value
            .get("memory")
            .filter(|value| !value.is_null())
            .cloned();
        let embedding = value
            .get("embedding")
            .filter(|value| !value.is_null())
            .cloned();
        if memory.is_some() && sqlite_url.is_some() {
            return Err("select exactly one memory backend");
        }
        let identity = serde_json::to_vec(&serde_json::json!({"memory":memory,"embedding":embedding,"sqlite_url":sqlite_url,"blob_root":blob_root,"tool_root":tool_root,"allowed_tools":allowed_tools,"browser":browser})).map_err(|_| "invalid backend identity")?;
        let identity = Sha256::digest(identity).to_vec();
        Ok(Self {
            scopes,
            sqlite_url,
            memory,
            embedding,
            blob_root,
            identity,
            tool_root,
            allowed_tools,
            browser,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bootstrap_requires_explicit_non_nil_control_plane_scopes() {
        for value in [
            serde_json::json!({"version":1,"scopes":[]}),
            serde_json::json!({"version":1,"scopes":[{"tenant_id":"t","project_id":"p","workspace_id":"w","session_id":"00000000-0000-0000-0000-000000000000","task_id":null,"attempt_id":null}]}),
        ] {
            assert!(Bootstrap::parse(&serde_json::to_vec(&value).unwrap()).is_err());
        }
    }
    #[test]
    fn bootstrap_preserves_selected_database_and_storage() {
        let value = serde_json::json!({"version":1,"sqlite_url":"sqlite:///selected/memory.db","blob_root":"/selected/blobs","scopes":[{"tenant_id":"t","project_id":"p","workspace_id":"w","session_id":"aaaaaaaa-aaaa-4aaa-aaaa-aaaaaaaaaaaa","task_id":null,"attempt_id":null}]});
        let config = Bootstrap::parse(&serde_json::to_vec(&value).unwrap())
            .ok()
            .unwrap();
        assert_eq!(
            config.sqlite_url.as_deref(),
            Some("sqlite:///selected/memory.db")
        );
        assert_eq!(config.blob_root.as_deref(), Some("/selected/blobs"));
        assert_eq!(config.scopes[0].tenant_id, "t");
    }
}

use async_trait::async_trait;
use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Query, State},
    http::{HeaderMap, StatusCode},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use server_harness::middleware::local_service_gateway::{
    LocalServiceBackend, LocalServiceGatewayError, LocalServiceOperation, ServiceNamespace,
};
use server_harness::middleware::local_services::LocalServiceBundle;
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
    time::Duration,
};
use tokio::sync::Mutex;
use uuid::Uuid;

type Error = Box<dyn std::error::Error + Send + Sync>;
type ProjectKey = (String, Option<String>);

struct ProjectBackends(BTreeMap<ProjectKey, Arc<dyn LocalServiceBackend>>);
#[async_trait]
impl LocalServiceBackend for ProjectBackends {
    fn capabilities(&self) -> BTreeSet<String> {
        self.0
            .values()
            .next()
            .map(|backend| backend.capabilities())
            .unwrap_or_default()
    }
    async fn execute(
        &self,
        namespace: &ServiceNamespace,
        operation: LocalServiceOperation,
    ) -> Result<Value, LocalServiceGatewayError> {
        self.0
            .get(&(namespace.tenant_id.clone(), namespace.project_id.clone()))
            .ok_or(LocalServiceGatewayError::InvalidOperation)?
            .execute(namespace, operation)
            .await
    }
    async fn release(&self, namespace: &ServiceNamespace) {
        if let Some(backend) = self
            .0
            .get(&(namespace.tenant_id.clone(), namespace.project_id.clone()))
        {
            backend.release(namespace).await;
        }
    }
}

#[derive(Clone)]
struct DaemonState {
    gateway: Arc<LocalServiceGateway>,
    token: Arc<String>,
    scopes: Arc<Vec<LocalServiceScopeContext>>,
    identity: Arc<Vec<u8>>,
    leases: Arc<Mutex<BTreeMap<Uuid, LocalServiceBundle>>>,
    audit: Arc<Mutex<BTreeMap<Uuid, BTreeSet<String>>>>,
}
#[derive(Serialize, Deserialize)]
struct Configuration {
    scopes: Vec<LocalServiceScopeContext>,
    identity: Vec<u8>,
    capabilities: BTreeMap<LocalServiceKind, BTreeSet<String>>,
}
#[derive(Serialize, Deserialize)]
struct OperationCall {
    scope: LocalServiceScopeContext,
    operation: LocalServiceOperation,
}
#[derive(Serialize, Deserialize)]
struct RevokeCall {
    tenant_id: String,
    attempt_id: Uuid,
}

fn authorized(headers: &HeaderMap, token: &str) -> bool {
    let Some(value) = headers
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
    else {
        return false;
    };
    value.len() == token.len()
        && value
            .bytes()
            .zip(token.bytes())
            .fold(0_u8, |difference, (a, b)| difference | (a ^ b))
            == 0
}
fn scope_is_admitted(
    scopes: &[LocalServiceScopeContext],
    scope: &LocalServiceScopeContext,
) -> bool {
    !scope.attempt_id.is_none_or(|id| id.is_nil())
        && scopes.iter().any(|allowed| {
            allowed.tenant_id == scope.tenant_id
                && allowed.project_id == scope.project_id
                && allowed.workspace_id == scope.workspace_id
                && allowed.session_id == scope.session_id
                && (allowed.task_id.is_none() || allowed.task_id == scope.task_id)
        })
}
async fn configuration(
    State(state): State<DaemonState>,
    headers: HeaderMap,
) -> Result<Json<Configuration>, StatusCode> {
    if !authorized(&headers, &state.token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(Json(Configuration {
        scopes: state.scopes.as_ref().clone(),
        identity: state.identity.as_ref().clone(),
        capabilities: state.gateway.available_capabilities(),
    }))
}
async fn operation(
    State(state): State<DaemonState>,
    headers: HeaderMap,
    Json(call): Json<OperationCall>,
) -> Result<Json<Value>, StatusCode> {
    if !authorized(&headers, &state.token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    if !scope_is_admitted(&state.scopes, &call.scope) {
        return Err(StatusCode::FORBIDDEN);
    }
    let attempt = call.scope.attempt_id.ok_or(StatusCode::BAD_REQUEST)?;
    let bundle = {
        let mut leases = state.leases.lock().await;
        if let Some(bundle) = leases.get(&attempt) {
            state
                .gateway
                .registry()
                .validate_issued(bundle, &call.scope)
                .map_err(|_| StatusCode::FORBIDDEN)?;
            bundle.clone()
        } else {
            let bundle = state
                .gateway
                .resolve(call.scope.clone())
                .map_err(|_| StatusCode::FORBIDDEN)?;
            leases.insert(attempt, bundle.clone());
            bundle
        }
    };
    let (kind, capability) = call.operation.capability();
    let binding = bundle
        .bindings
        .iter()
        .find(|binding| binding.kind == kind)
        .ok_or(StatusCode::NOT_IMPLEMENTED)?;
    let result = state
        .gateway
        .execute(binding, &call.scope, call.operation)
        .await
        .map_err(|error| match error {
            LocalServiceGatewayError::Authorization(_) => StatusCode::FORBIDDEN,
            LocalServiceGatewayError::UnsupportedCapability(_) => StatusCode::NOT_IMPLEMENTED,
            LocalServiceGatewayError::InvalidOperation => StatusCode::BAD_REQUEST,
            LocalServiceGatewayError::BackendFailure => StatusCode::BAD_GATEWAY,
        })?;
    state
        .audit
        .lock()
        .await
        .entry(attempt)
        .or_default()
        .insert(capability.to_owned());
    Ok(Json(result))
}
#[derive(Deserialize)]
struct AuditQuery {
    attempt_id: Uuid,
}
async fn audit(
    State(state): State<DaemonState>,
    headers: HeaderMap,
    Query(query): Query<AuditQuery>,
) -> Result<Json<Value>, StatusCode> {
    if !authorized(&headers, &state.token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let operations = state
        .audit
        .lock()
        .await
        .get(&query.attempt_id)
        .cloned()
        .unwrap_or_default();
    Ok(Json(
        json!({"attempt_id":query.attempt_id,"operations":operations}),
    ))
}
async fn revoke(
    State(state): State<DaemonState>,
    headers: HeaderMap,
    Json(call): Json<RevokeCall>,
) -> Result<StatusCode, StatusCode> {
    if !authorized(&headers, &state.token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let mut leases = state.leases.lock().await;
    if let Some(bundle) = leases.get(&call.attempt_id)
        && bundle
            .bindings
            .iter()
            .any(|binding| binding.tenant_id != call.tenant_id)
    {
        return Err(StatusCode::FORBIDDEN);
    }
    state
        .gateway
        .registry()
        .revoke_attempt(&call.tenant_id, call.attempt_id);
    if let Some(bundle) = leases.remove(&call.attempt_id) {
        state.gateway.release(&bundle).await;
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn selected_gateway(config: &Bootstrap) -> Result<LocalServiceGateway, Error> {
    use omnisolo_builtin_agent::{
        agent::AgentRunConfig,
        local_service_adapters::{
            ExistingBlobBackend, ExistingToolBackend, ScreenshotBrowserBackend,
        },
        memory_store::FileBasedMemory,
    };
    let registry =
        LocalServiceRegistry::with_defaults().with_backend_configuration(&config.identity)?;
    let mut gateway = LocalServiceGateway::new(registry);
    if let Some(url) = &config.sqlite_url {
        let options = sqlx::sqlite::SqliteConnectOptions::from_str(url)?.create_if_missing(false);
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(4)
            .connect_with(options)
            .await?;
        // The selected canonical database must already have the agent's schema.
        sqlx::query("SELECT content,tags,created_at FROM agent_memory LIMIT 0")
            .execute(&pool)
            .await?;
        gateway.register(
            LocalServiceKind::Memory,
            Arc::new(SqliteAgentMemoryBackend::new(pool)),
        );
    }
    if let Some(memory) = &config.memory {
        use omnisolo_builtin_agent::local_service_adapters::{
            ServiceMemoryConfiguration, selected_memory_backend,
        };
        let selected: ServiceMemoryConfiguration = serde_json::from_value(memory.clone())
            .map_err(|_| "invalid selected memory configuration")?;
        if let ServiceMemoryConfiguration::Vector { tenant_id, .. } = &selected
            && config
                .scopes
                .iter()
                .any(|scope| &scope.tenant_id != tenant_id)
        {
            return Err("vector memory tenant must match admitted scopes".into());
        }
        let embedding: Option<Arc<dyn omnisolo_builtin_agent::llm::LlmClient>> = if let Some(embedding) =
            &config.embedding
        {
            let base = embedding
                .get("base_url")
                .and_then(Value::as_str)
                .ok_or("embedding base URL is required")?;
            let url = reqwest::Url::parse(base).map_err(|_| "invalid embedding base URL")?;
            if !matches!(url.scheme(), "https" | "http")
                || !url.username().is_empty()
                || url.password().is_some()
                || url.query().is_some()
                || url.fragment().is_some()
            {
                return Err("invalid embedding base URL".into());
            }
            let model = embedding
                .get("model")
                .and_then(Value::as_str)
                .filter(|model| !model.trim().is_empty())
                .ok_or("embedding model is required")?;
            let key_env = embedding
                .get("api_key_env")
                .and_then(Value::as_str)
                .ok_or("embedding credential variable is required")?;
            let key = std::env::var(key_env).map_err(|_| "embedding credential is unavailable")?;
            if key.trim().is_empty() || key.chars().any(char::is_control) {
                return Err("invalid embedding credential".into());
            }
            let mut selected =
                omnisolo_builtin_agent::llm::openai::OpenAIClientConfig::openai_compatible(
                    key, base, None,
                );
            selected.embedding_model = model.to_owned();
            Some(Arc::new(
                omnisolo_builtin_agent::llm::openai::OpenAIClient::from_config(selected),
            ))
        } else {
            None
        };
        let (backend, _identity) = selected_memory_backend(&selected, embedding)
            .await
            .map_err(std::io::Error::other)?;
        gateway.register(LocalServiceKind::Memory, backend);
    }
    if let Some(root) = &config.blob_root {
        let root = tokio::fs::canonicalize(root).await?;
        let selected = Arc::new(FileBasedMemory::new(root));
        for kind in [
            LocalServiceKind::Artifact,
            LocalServiceKind::Workspace,
            LocalServiceKind::Cache,
        ] {
            gateway.register(
                kind,
                Arc::new(ExistingBlobBackend::new(selected.clone(), kind)),
            );
        }
    }
    if let Some(root) = &config.tool_root {
        let root = tokio::fs::canonicalize(root).await?;
        let mut catalogs = BTreeMap::new();
        let mut integrations = BTreeMap::new();
        let mut browsers = BTreeMap::new();
        for scope in &config.scopes {
            let key = (scope.tenant_id.clone(), scope.project_id.clone());
            if catalogs.contains_key(&key) {
                continue;
            }
            let project = format!("{:x}", Sha256::digest(serde_json::to_vec(&key)?));
            let directory = root.join(project);
            tokio::fs::create_dir_all(&directory).await?;
            let service = omnisolo_builtin_agent::service::AgentServiceImpl::new_for_tenant(
                "local-service-catalog",
                Default::default(),
                omnisolo_builtin_agent::auth::AuthMode::Spiffe {
                    allowed_id: "spiffe://omnisolo/local-service-daemon".into(),
                },
                omnisolo_builtin_agent::tools::tenant::TenantContext::new(&scope.tenant_id)
                    .map_err(std::io::Error::other)?,
            );
            let mut names = config.allowed_tools.clone();
            if config.browser && !names.iter().any(|name| name == "Screenshot") {
                names.push("Screenshot".into());
            }
            let tools = service.local_service_tools(directory.clone(), &names).await;
            if names
                .iter()
                .any(|name| !tools.iter().any(|tool| &tool.name == name))
            {
                return Err("a configured service tool is unavailable".into());
            }
            let run = AgentRunConfig {
                allowed_tools: Some(names),
                workspace_path: Some(directory.to_string_lossy().into_owned()),
                ..Default::default()
            };
            let selected: Vec<_> = tools
                .iter()
                .filter(|tool| config.allowed_tools.contains(&tool.name))
                .cloned()
                .collect();
            catalogs.insert(
                key.clone(),
                Arc::new(ExistingToolBackend::new(
                    LocalServiceKind::Mcp,
                    &selected,
                    &run,
                    scope,
                )) as Arc<dyn LocalServiceBackend>,
            );
            integrations.insert(
                key.clone(),
                Arc::new(ExistingToolBackend::new(
                    LocalServiceKind::Integration,
                    &selected,
                    &run,
                    scope,
                )) as Arc<dyn LocalServiceBackend>,
            );
            if config.browser {
                let screenshot = tools
                    .iter()
                    .find(|tool| tool.name == "Screenshot")
                    .ok_or("configured browser tool is unavailable")?;
                browsers.insert(
                    key,
                    Arc::new(ScreenshotBrowserBackend::new(
                        screenshot.clone(),
                        directory,
                        scope,
                        &run,
                    )) as Arc<dyn LocalServiceBackend>,
                );
            }
        }
        if !config.allowed_tools.is_empty() {
            gateway.register(LocalServiceKind::Mcp, Arc::new(ProjectBackends(catalogs)));
            gateway.register(
                LocalServiceKind::Integration,
                Arc::new(ProjectBackends(integrations)),
            );
        }
        if config.browser {
            gateway.register(
                LocalServiceKind::Browser,
                Arc::new(ProjectBackends(browsers)),
            );
        }
    }
    Ok(gateway)
}

/// Run only in a service container. This mode never starts a native harness.
pub async fn run_daemon() -> Result<(), Error> {
    use tokio::io::AsyncReadExt;
    let path = std::env::var("OMNISOLO_LOCAL_SERVICE_CONFIG_FILE")
        .map_err(|_| "service configuration file is required")?;
    let token = std::env::var("OMNISOLO_LOCAL_SERVICE_CONTROL_TOKEN")
        .map_err(|_| "service control credential is required")?;
    if token.len() < 32 || token.len() > 2048 || !token.bytes().all(|byte| byte.is_ascii_graphic())
    {
        return Err("invalid service control credential".into());
    }
    let mut bytes = Vec::new();
    tokio::fs::File::open(path)
        .await?
        .take(65537)
        .read_to_end(&mut bytes)
        .await?;
    let config = Bootstrap::parse(&bytes).map_err(std::io::Error::other)?;
    let state = DaemonState {
        gateway: Arc::new(selected_gateway(&config).await?),
        token: Arc::new(token),
        scopes: Arc::new(config.scopes),
        identity: Arc::new(config.identity),
        leases: Arc::new(Mutex::new(BTreeMap::new())),
        audit: Arc::new(Mutex::new(BTreeMap::new())),
    };
    let app = Router::new()
        .route("/v1/configuration", get(configuration))
        .route("/v1/operations", post(operation))
        .route("/v1/revoke", post(revoke))
        .route("/v1/audit", get(audit))
        .layer(DefaultBodyLimit::max(20 * 1024 * 1024))
        .with_state(state);
    let address: std::net::SocketAddr = std::env::var("OMNISOLO_LOCAL_SERVICE_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8095".into())
        .parse()?;
    if !address.ip().is_loopback() {
        return Err("local service listener must bind loopback".into());
    }
    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(crate::shutdown_signal())
        .await?;
    Ok(())
}

struct RemoteBackend {
    client: reqwest::Client,
    endpoint: String,
    token: Arc<String>,
    scopes: Arc<Vec<LocalServiceScopeContext>>,
    capabilities: BTreeSet<String>,
}
#[async_trait]
impl LocalServiceBackend for RemoteBackend {
    fn capabilities(&self) -> BTreeSet<String> {
        self.capabilities.clone()
    }
    async fn execute(
        &self,
        namespace: &ServiceNamespace,
        operation: LocalServiceOperation,
    ) -> Result<Value, LocalServiceGatewayError> {
        let mut scope = self
            .scopes
            .iter()
            .find(|scope| {
                scope.tenant_id == namespace.tenant_id
                    && scope.project_id == namespace.project_id
                    && scope.session_id == namespace.session_id
            })
            .cloned()
            .ok_or(LocalServiceGatewayError::InvalidOperation)?;
        scope.attempt_id = Some(
            namespace
                .attempt_id
                .ok_or(LocalServiceGatewayError::InvalidOperation)?,
        );
        scope.task_id = namespace.task_id;
        let response = self
            .client
            .post(format!("{}/v1/operations", self.endpoint))
            .bearer_auth(self.token.as_str())
            .json(&OperationCall { scope, operation })
            .send()
            .await
            .map_err(|_| LocalServiceGatewayError::BackendFailure)?;
        match response.status().as_u16() {
            200..=299 => {}
            401 | 403 => {
                return Err(LocalServiceGatewayError::Authorization(
                    server_harness::middleware::local_services::LocalServiceError::BindingRevoked,
                ));
            }
            400 => return Err(LocalServiceGatewayError::InvalidOperation),
            501 => {
                return Err(LocalServiceGatewayError::UnsupportedCapability(
                    "configured service operation".into(),
                ));
            }
            _ => return Err(LocalServiceGatewayError::BackendFailure),
        }
        let bytes = bounded_response(response)
            .await
            .map_err(|_| LocalServiceGatewayError::BackendFailure)?;
        serde_json::from_slice(&bytes).map_err(|_| LocalServiceGatewayError::BackendFailure)
    }
    async fn release(&self, namespace: &ServiceNamespace) {
        let Some(attempt_id) = namespace.attempt_id else {
            return;
        };
        let _ = self
            .client
            .post(format!("{}/v1/revoke", self.endpoint))
            .bearer_auth(self.token.as_str())
            .json(&RevokeCall {
                tenant_id: namespace.tenant_id.clone(),
                attempt_id,
            })
            .send()
            .await;
    }
}
async fn bounded_response(mut response: reqwest::Response) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| "local service response failed")?
    {
        if bytes.len().saturating_add(chunk.len()) > 20 * 1024 * 1024 {
            return Err("local service response exceeds limit".into());
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
}

async fn bootstrap_response(
    client: &reqwest::Client,
    endpoint: &str,
    token: &str,
) -> Result<reqwest::Response, Error> {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(60);
    loop {
        let response = client
            .get(format!("{endpoint}/v1/configuration"))
            .bearer_auth(token)
            .timeout(Duration::from_secs(5))
            .send()
            .await;
        match response {
            Ok(response) if response.status().is_success() => return Ok(response),
            Ok(response) if !response.status().is_server_error() => {
                return Err("local service bootstrap rejected".into());
            }
            _ if tokio::time::Instant::now() >= deadline => {
                return Err("local service bootstrap unavailable".into());
            }
            _ => tokio::time::sleep(Duration::from_secs(1)).await,
        }
    }
}

pub(crate) async fn configure_worker(
    mut service: HarnessWorkerGrpcService,
    provider_configured: bool,
) -> Result<(HarnessWorkerGrpcService, Option<Arc<String>>), Error> {
    let endpoint = std::env::var("OMNISOLO_LOCAL_SERVICE_ENDPOINT").ok();
    let token = std::env::var("OMNISOLO_LOCAL_SERVICE_CONTROL_TOKEN").ok();
    let (endpoint, token) = match (endpoint, token) {
        (None, None) => return Ok((service, None)),
        (Some(endpoint), Some(token)) => (endpoint, token),
        _ => return Err("local service endpoint and control credential are both required".into()),
    };
    let url = reqwest::Url::parse(&endpoint).map_err(|_| "invalid local service endpoint")?;
    if url.scheme() != "http"
        || !matches!(url.host_str(), Some("127.0.0.1" | "[::1]"))
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || url.path() != "/"
    {
        return Err("local service endpoint must be a loopback HTTP origin".into());
    }
    if token.len() < 32 || token.len() > 2048 || !token.bytes().all(|byte| byte.is_ascii_graphic())
    {
        return Err("invalid local service control credential".into());
    }
    crate::protected_process::protect_credentials()?;
    let token = Arc::new(token);
    let endpoint = endpoint.trim_end_matches('/').to_owned();
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(Duration::from_secs(120))
        .build()?;
    let response = bootstrap_response(&client, &endpoint, &token).await?;
    let configuration: Configuration = serde_json::from_slice(&bounded_response(response).await?)
        .map_err(|_| "invalid local service bootstrap response")?;
    let registry = LocalServiceRegistry::with_defaults()
        .with_backend_configuration(&configuration.identity)?;
    let mut gateway = LocalServiceGateway::new(registry);
    let scopes = Arc::new(configuration.scopes);
    for (kind, capabilities) in configuration.capabilities {
        if kind == LocalServiceKind::ProviderFacade {
            continue;
        }
        gateway.register(
            kind,
            Arc::new(RemoteBackend {
                client: client.clone(),
                endpoint: endpoint.clone(),
                token: token.clone(),
                scopes: scopes.clone(),
                capabilities,
            }),
        );
    }
    if provider_configured {
        gateway.register(
            LocalServiceKind::ProviderFacade,
            service.provider_service_backend(),
        );
    }
    service = service.with_local_service_gateway(Arc::new(gateway));
    for scope in scopes.iter() {
        service = service.with_trusted_service_scope(scope.clone());
    }
    Ok((service, Some(token)))
}

#[cfg(test)]
mod daemon_tests {
    use super::*;
    fn scope() -> LocalServiceScopeContext {
        LocalServiceScopeContext {
            tenant_id: "tenant-service-test".into(),
            project_id: Some("project".into()),
            workspace_id: Some("workspace".into()),
            session_id: Uuid::new_v4(),
            task_id: None,
            attempt_id: None,
        }
    }
    fn headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "authorization",
            format!("Bearer {}", "test-control-credential-0123456789")
                .parse()
                .unwrap(),
        );
        headers
    }
    #[tokio::test]
    async fn daemon_enforces_admission_records_success_and_fences_revoked_attempt() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::query(
            "CREATE VIRTUAL TABLE agent_memory USING fts5(content, tags, created_at UNINDEXED)",
        )
        .execute(&pool)
        .await
        .unwrap();
        let mut gateway = LocalServiceGateway::new(
            LocalServiceRegistry::with_defaults()
                .with_backend_configuration(b"selected-memory-test")
                .unwrap(),
        );
        gateway.register(
            LocalServiceKind::Memory,
            Arc::new(SqliteAgentMemoryBackend::new(pool)),
        );
        let admitted = scope();
        let mut attempt = admitted.clone();
        attempt.attempt_id = Some(Uuid::new_v4());
        let state = DaemonState {
            gateway: Arc::new(gateway),
            token: Arc::new("test-control-credential-0123456789".into()),
            scopes: Arc::new(vec![admitted]),
            identity: Arc::new(vec![1]),
            leases: Arc::new(Mutex::new(BTreeMap::new())),
            audit: Arc::new(Mutex::new(BTreeMap::new())),
        };
        let write = || OperationCall {
            scope: attempt.clone(),
            operation: LocalServiceOperation::MemoryWrite {
                content: "service sentinel".into(),
            },
        };
        assert_eq!(
            operation(State(state.clone()), HeaderMap::new(), Json(write()))
                .await
                .unwrap_err(),
            StatusCode::UNAUTHORIZED
        );
        let mut foreign = write();
        foreign.scope.project_id = Some("foreign".into());
        assert_eq!(
            operation(State(state.clone()), headers(), Json(foreign))
                .await
                .unwrap_err(),
            StatusCode::FORBIDDEN
        );
        let _ = operation(State(state.clone()), headers(), Json(write()))
            .await
            .unwrap();
        let receipt = audit(
            State(state.clone()),
            headers(),
            Query(AuditQuery {
                attempt_id: attempt.attempt_id.unwrap(),
            }),
        )
        .await
        .unwrap()
        .0;
        assert_eq!(receipt["operations"], json!(["memory.write"]));
        revoke(
            State(state.clone()),
            headers(),
            Json(RevokeCall {
                tenant_id: attempt.tenant_id.clone(),
                attempt_id: attempt.attempt_id.unwrap(),
            }),
        )
        .await
        .unwrap();
        assert_eq!(
            operation(State(state.clone()), headers(), Json(write()))
                .await
                .unwrap_err(),
            StatusCode::FORBIDDEN
        );
        let mut reader = attempt.clone();
        reader.attempt_id = Some(Uuid::new_v4());
        let result = operation(
            State(state.clone()),
            headers(),
            Json(OperationCall {
                scope: reader.clone(),
                operation: LocalServiceOperation::MemorySearch {
                    query: "sentinel".into(),
                    limit: 5,
                },
            }),
        )
        .await
        .unwrap()
        .0;
        assert!(result.to_string().contains("service sentinel"));

        // Exercise the actual worker-to-daemon HTTP boundary, including revocation.
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let app = Router::new()
            .route("/v1/operations", post(operation))
            .route("/v1/revoke", post(revoke))
            .with_state(state.clone());
        let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let remote = RemoteBackend {
            client: reqwest::Client::new(),
            endpoint,
            token: state.token.clone(),
            scopes: state.scopes.clone(),
            capabilities: BTreeSet::from(["memory.search".into()]),
        };
        let mut namespace = ServiceNamespace {
            session_id: reader.session_id,
            task_id: reader.task_id,
            attempt_id: reader.attempt_id,
            tenant_id: reader.tenant_id,
            project_id: reader.project_id,
            workspace_id: reader.workspace_id,
            kind: LocalServiceKind::Memory,
            lease_id: None,
        };
        let search = || LocalServiceOperation::MemorySearch {
            query: "sentinel".into(),
            limit: 5,
        };
        assert!(
            remote
                .execute(&namespace, search())
                .await
                .unwrap()
                .to_string()
                .contains("service sentinel")
        );
        remote.release(&namespace).await;
        assert!(matches!(
            remote.execute(&namespace, search()).await,
            Err(LocalServiceGatewayError::Authorization(_))
        ));
        namespace.session_id = Uuid::new_v4();
        assert!(matches!(
            remote.execute(&namespace, search()).await,
            Err(LocalServiceGatewayError::InvalidOperation)
        ));
        server.abort();
    }
    #[tokio::test]
    async fn bootstrap_retries_startup_failures_but_rejects_invalid_credentials() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        let calls = Arc::new(AtomicUsize::new(0));
        let app = Router::new()
            .route(
                "/v1/configuration",
                get(
                    |State(calls): State<Arc<AtomicUsize>>, headers: HeaderMap| async move {
                        if !authorized(&headers, "valid-control") {
                            return StatusCode::UNAUTHORIZED;
                        }
                        if calls.fetch_add(1, Ordering::SeqCst) == 0 {
                            StatusCode::SERVICE_UNAVAILABLE
                        } else {
                            StatusCode::OK
                        }
                    },
                ),
            )
            .with_state(calls.clone());
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let task = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let client = reqwest::Client::new();
        assert!(
            bootstrap_response(&client, &endpoint, "valid-control")
                .await
                .unwrap()
                .status()
                .is_success()
        );
        assert_eq!(calls.load(Ordering::SeqCst), 2);
        assert!(
            bootstrap_response(&client, &endpoint, "invalid-control")
                .await
                .is_err()
        );
        assert_eq!(calls.load(Ordering::SeqCst), 2);
        task.abort();
    }

    #[test]
    fn backend_identity_changes_with_selected_storage_but_not_admitted_session() {
        let mut value = json!({"version":1,"scopes":[scope()],"blob_root":"/selected/one"});
        let first = Bootstrap::parse(&serde_json::to_vec(&value).unwrap())
            .unwrap()
            .identity;
        value["scopes"] = json!([scope()]);
        assert_eq!(
            first,
            Bootstrap::parse(&serde_json::to_vec(&value).unwrap())
                .unwrap()
                .identity
        );
        value["blob_root"] = json!("/selected/two");
        assert_ne!(
            first,
            Bootstrap::parse(&serde_json::to_vec(&value).unwrap())
                .unwrap()
                .identity
        );
        assert_eq!(first.len(), 32);
    }
}
