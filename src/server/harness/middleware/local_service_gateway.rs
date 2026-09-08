//! Typed, lease-fenced operations over configured local service implementations.
//! Portable bindings identify services; the issuing registry supplies authority.
use super::local_services::*;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "operation", rename_all = "snake_case")]
pub enum LocalServiceOperation {
    ProviderModels,
    ProviderResponses { body: Value },
    ProviderChatCompletions { body: Value },
    MemoryWrite { content: String },
    MemorySearch { query: String, limit: usize },
    ArtifactWrite { key: String, content: Vec<u8> },
    ArtifactRead { key: String },
    WorkspaceWrite { key: String, content: Vec<u8> },
    WorkspaceRead { key: String },
    McpCatalog,
    McpInvoke { tool: String, arguments: Value },
    BrowserNavigate { url: String },
    BrowserSnapshot,
    CacheWrite { key: String, content: Vec<u8> },
    CacheRead { key: String },
    IntegrationRead { key: String },
    IntegrationInvoke { action: String, arguments: Value },
}
impl LocalServiceOperation {
    pub fn capability(&self) -> (LocalServiceKind, &'static str) {
        use LocalServiceKind::*;
        match self {
            Self::ProviderModels => (ProviderFacade, "provider.models"),
            Self::ProviderResponses { .. } => (ProviderFacade, "provider.responses"),
            Self::ProviderChatCompletions { .. } => (ProviderFacade, "provider.chat_completions"),
            Self::MemoryWrite { .. } => (Memory, "memory.write"),
            Self::MemorySearch { .. } => (Memory, "memory.search"),
            Self::ArtifactWrite { .. } => (Artifact, "artifact.write"),
            Self::ArtifactRead { .. } => (Artifact, "artifact.read"),
            Self::WorkspaceWrite { .. } => (Workspace, "workspace.write"),
            Self::WorkspaceRead { .. } => (Workspace, "workspace.read"),
            Self::McpCatalog => (Mcp, "mcp.catalog"),
            Self::McpInvoke { .. } => (Mcp, "mcp.invoke"),
            Self::BrowserNavigate { .. } => (Browser, "browser.navigate"),
            Self::BrowserSnapshot => (Browser, "browser.snapshot"),
            Self::CacheWrite { .. } => (Cache, "cache.write"),
            Self::CacheRead { .. } => (Cache, "cache.read"),
            Self::IntegrationRead { .. } => (Integration, "integration.read"),
            Self::IntegrationInvoke { .. } => (Integration, "integration.invoke"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ServiceNamespace {
    #[serde(skip)]
    pub session_id: uuid::Uuid,
    #[serde(skip)]
    pub task_id: Option<uuid::Uuid>,
    #[serde(skip)]
    pub attempt_id: Option<uuid::Uuid>,
    pub tenant_id: String,
    pub project_id: Option<String>,
    pub workspace_id: Option<String>,
    pub kind: LocalServiceKind,
    // Browser resources belong to this issued lease, never the shared task namespace.
    pub lease_id: Option<uuid::Uuid>,
}
impl ServiceNamespace {
    pub fn from_binding(binding: &LocalServiceBinding) -> Self {
        Self {
            session_id: binding.session_id,
            task_id: binding.task_id,
            attempt_id: binding.attempt_id,
            tenant_id: binding.tenant_id.clone(),
            project_id: binding.project_id.clone(),
            workspace_id: matches!(binding.scope, LocalServiceScope::Workspace)
                .then(|| binding.workspace_id.clone())
                .flatten(),
            kind: binding.kind,
            lease_id: (binding.kind == LocalServiceKind::Browser).then_some(binding.binding_id),
        }
    }
    pub fn storage_key(&self) -> String {
        format!(
            "{:x}",
            Sha256::digest(serde_json::to_vec(self).expect("namespace serializes"))
        )
    }
}

#[derive(Debug)]
pub enum LocalServiceGatewayError {
    Authorization(LocalServiceError),
    UnsupportedCapability(String),
    InvalidOperation,
    BackendFailure,
}
impl std::fmt::Display for LocalServiceGatewayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authorization(e) => write!(f, "{e}"),
            Self::UnsupportedCapability(c) => {
                write!(f, "local service capability unavailable: {c}")
            }
            Self::InvalidOperation => f.write_str("invalid local service operation"),
            Self::BackendFailure => f.write_str("local service backend failed"),
        }
    }
}
impl std::error::Error for LocalServiceGatewayError {}
impl From<LocalServiceError> for LocalServiceGatewayError {
    fn from(error: LocalServiceError) -> Self {
        Self::Authorization(error)
    }
}

/// Implementations hold their own database clients, credentials and process leases.
/// They must honor the supplied namespace and return portable operation results.
#[async_trait]
pub trait LocalServiceBackend: Send + Sync {
    fn capabilities(&self) -> BTreeSet<String>;
    async fn execute(
        &self,
        namespace: &ServiceNamespace,
        operation: LocalServiceOperation,
    ) -> Result<Value, LocalServiceGatewayError>;
    async fn release(&self, _namespace: &ServiceNamespace) {}
}

#[derive(Clone)]
pub struct LocalServiceGateway {
    registry: LocalServiceRegistry,
    backends: BTreeMap<LocalServiceKind, Arc<dyn LocalServiceBackend>>,
}
impl LocalServiceGateway {
    pub fn new(registry: LocalServiceRegistry) -> Self {
        Self {
            registry,
            backends: BTreeMap::new(),
        }
    }
    pub fn registry(&self) -> &LocalServiceRegistry {
        &self.registry
    }
    pub fn register(&mut self, kind: LocalServiceKind, backend: Arc<dyn LocalServiceBackend>) {
        self.backends.insert(kind, backend);
    }
    pub fn available_capabilities(&self) -> BTreeMap<LocalServiceKind, BTreeSet<String>> {
        self.backends
            .iter()
            .map(|(kind, backend)| (*kind, backend.capabilities()))
            .collect()
    }
    pub fn resolve(
        &self,
        scope: LocalServiceScopeContext,
    ) -> Result<LocalServiceBundle, LocalServiceError> {
        self.registry
            .resolve_capabilities(scope, &self.available_capabilities())
    }
    pub async fn execute(
        &self,
        binding: &LocalServiceBinding,
        scope: &LocalServiceScopeContext,
        operation: LocalServiceOperation,
    ) -> Result<Value, LocalServiceGatewayError> {
        let (kind, capability) = operation.capability();
        if binding.kind != kind {
            return Err(LocalServiceError::CapabilityDenied(capability.to_owned()).into());
        }
        self.registry.authorize(binding, scope, capability)?;
        let backend = self.backends.get(&kind).ok_or_else(|| {
            LocalServiceGatewayError::UnsupportedCapability(capability.to_owned())
        })?;
        if !backend.capabilities().contains(capability) {
            return Err(LocalServiceGatewayError::UnsupportedCapability(
                capability.to_owned(),
            ));
        }
        let mut revoked = self.registry.revocation(binding)?;
        let namespace = ServiceNamespace::from_binding(binding);
        tokio::select! {
            biased;
            _ = async {
                while !*revoked.borrow_and_update() {
                    if revoked.changed().await.is_err() { break; }
                }
            } => Err(LocalServiceError::BindingRevoked.into()),
            result=backend.execute(&namespace,operation)=>result,
        }
    }
    pub async fn release(&self, bundle: &LocalServiceBundle) {
        for binding in &bundle.bindings {
            if let Some(attempt) = binding.attempt_id {
                self.registry.revoke_attempt(&binding.tenant_id, attempt);
            }
        }
        for binding in &bundle.bindings {
            if let Some(backend) = self.backends.get(&binding.kind) {
                backend
                    .release(&ServiceNamespace::from_binding(binding))
                    .await;
            }
            self.registry.retire_revoked(binding.binding_id);
        }
    }
}

/// Adapter over the existing SqliteMemoryStore agent_memory FTS5 table. The
/// caller supplies its selected pool; this adapter creates no database or schema.
pub struct SqliteAgentMemoryBackend {
    pool: sqlx::SqlitePool,
}
impl SqliteAgentMemoryBackend {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}
#[async_trait]
impl LocalServiceBackend for SqliteAgentMemoryBackend {
    fn capabilities(&self) -> BTreeSet<String> {
        ["memory.write", "memory.search"]
            .into_iter()
            .map(str::to_owned)
            .collect()
    }
    async fn execute(
        &self,
        namespace: &ServiceNamespace,
        operation: LocalServiceOperation,
    ) -> Result<Value, LocalServiceGatewayError> {
        let namespace = namespace.storage_key();
        match operation {
            LocalServiceOperation::MemoryWrite { content } => {
                if content.is_empty() || content.len() > 1_048_576 {
                    return Err(LocalServiceGatewayError::InvalidOperation);
                }
                let tags = serde_json::to_string(&vec![format!("omnisolo.scope:{namespace}")])
                    .expect("tags serialize");
                sqlx::query("INSERT INTO agent_memory(content,tags,created_at) VALUES(?,?,?)")
                    .bind(&content)
                    .bind(tags)
                    .bind(chrono::Utc::now().timestamp())
                    .execute(&self.pool)
                    .await
                    .map_err(|_| LocalServiceGatewayError::BackendFailure)?;
                Ok(json!({"digest":format!("sha256:{:x}",Sha256::digest(content.as_bytes()))}))
            }
            LocalServiceOperation::MemorySearch { query, limit } => {
                if limit == 0 || limit > 1000 || query.len() > 4096 {
                    return Err(LocalServiceGatewayError::InvalidOperation);
                }
                let tag = format!("omnisolo.scope:{namespace}");
                // Filter namespaces in SQL before applying the result limit.
                let rows:Vec<(String,)> = sqlx::query_as("SELECT content FROM agent_memory WHERE EXISTS (SELECT 1 FROM json_each(agent_memory.tags) WHERE value=?) AND instr(lower(content),lower(?))>0 ORDER BY rowid DESC LIMIT ?")
                    .bind(tag).bind(query).bind(limit as i64).fetch_all(&self.pool).await
                    .map_err(|_|LocalServiceGatewayError::BackendFailure)?;
                Ok(json!({"items":rows.into_iter().map(|row|row.0).collect::<Vec<_>>()}))
            }
            _ => Err(LocalServiceGatewayError::InvalidOperation),
        }
    }
}

/// Backend for worker-owned provider routes. Register/revoke routes from the
/// provider lifecycle; callers supply only an operation and their issued lease.
#[derive(Clone, Default)]
pub struct ProviderRouteBackend {
    routes: Arc<
        std::sync::Mutex<
            BTreeMap<(String, uuid::Uuid), super::provider_facade::ProviderFacadeRoute>,
        >,
    >,
}
impl ProviderRouteBackend {
    pub fn bind(
        &self,
        tenant: &str,
        attempt: uuid::Uuid,
        route: super::provider_facade::ProviderFacadeRoute,
    ) {
        self.routes
            .lock()
            .expect("provider route state poisoned")
            .insert((tenant.to_owned(), attempt), route);
    }
    pub fn revoke(&self, tenant: &str, attempt: uuid::Uuid) {
        self.routes
            .lock()
            .expect("provider route state poisoned")
            .remove(&(tenant.to_owned(), attempt));
    }
}
#[async_trait]
impl LocalServiceBackend for ProviderRouteBackend {
    fn capabilities(&self) -> BTreeSet<String> {
        [
            "provider.models",
            "provider.responses",
            "provider.chat_completions",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect()
    }
    async fn execute(
        &self,
        namespace: &ServiceNamespace,
        operation: LocalServiceOperation,
    ) -> Result<Value, LocalServiceGatewayError> {
        let attempt = namespace
            .attempt_id
            .ok_or(LocalServiceGatewayError::InvalidOperation)?;
        let route = self
            .routes
            .lock()
            .expect("provider route state poisoned")
            .get(&(namespace.tenant_id.clone(), attempt))
            .cloned()
            .ok_or(LocalServiceGatewayError::Authorization(
                LocalServiceError::BindingRevoked,
            ))?;
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|_| LocalServiceGatewayError::BackendFailure)?;
        let request = match operation {
            LocalServiceOperation::ProviderModels => {
                client.get(format!("{}/models", route.base_url()))
            }
            LocalServiceOperation::ProviderResponses { mut body } => {
                if let Some(object) = body.as_object_mut() {
                    object.insert("stream".into(), Value::Bool(false));
                }
                client
                    .post(format!("{}/responses", route.base_url()))
                    .json(&body)
            }
            LocalServiceOperation::ProviderChatCompletions { mut body } => {
                if let Some(object) = body.as_object_mut() {
                    object.insert("stream".into(), Value::Bool(false));
                }
                client
                    .post(format!("{}/chat/completions", route.base_url()))
                    .json(&body)
            }
            _ => return Err(LocalServiceGatewayError::InvalidOperation),
        };
        let response = request
            .bearer_auth(route.token())
            .send()
            .await
            .map_err(|_| LocalServiceGatewayError::BackendFailure)?;
        if !response.status().is_success() {
            return Err(LocalServiceGatewayError::BackendFailure);
        }
        response
            .json()
            .await
            .map_err(|_| LocalServiceGatewayError::BackendFailure)
    }
    async fn release(&self, namespace: &ServiceNamespace) {
        if let Some(attempt) = namespace.attempt_id {
            self.revoke(&namespace.tenant_id, attempt);
        }
    }
}
