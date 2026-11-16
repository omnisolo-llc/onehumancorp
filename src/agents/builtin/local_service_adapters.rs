//! Adapters preserve the agent's existing memory backend selection and storage roots.
use async_trait::async_trait;
use serde_json::{Value, json};
use server_harness::middleware::local_service_gateway::*;
use server_harness::middleware::local_services::LocalServiceKind;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::sync::Arc;

pub(crate) fn scope_key(namespace: &str) -> String {
    format!("{:x}", Sha256::digest(namespace.as_bytes()))
}

pub struct SelectedMemoryBackend {
    selected: Arc<dyn crate::memory_store::LongTermMemory>,
}
impl SelectedMemoryBackend {
    pub fn new(selected: Arc<dyn crate::memory_store::LongTermMemory>) -> Self {
        Self { selected }
    }
}
#[async_trait]
impl LocalServiceBackend for SelectedMemoryBackend {
    fn capabilities(&self) -> BTreeSet<String> {
        if self.selected.supports_scoped_services() {
            ["memory.write", "memory.search"]
                .into_iter()
                .map(str::to_owned)
                .collect()
        } else {
            BTreeSet::new()
        }
    }
    async fn execute(
        &self,
        namespace: &ServiceNamespace,
        operation: LocalServiceOperation,
    ) -> Result<Value, LocalServiceGatewayError> {
        let scope = namespace.storage_key();
        match operation {
            LocalServiceOperation::MemoryWrite { content } => {
                if content.is_empty() || content.len() > 1_048_576 {
                    return Err(LocalServiceGatewayError::InvalidOperation);
                }
                self.selected
                    .store_scoped(&scope, &content)
                    .await
                    .map_err(|_| LocalServiceGatewayError::BackendFailure)?;
                Ok(json!({"digest":format!("sha256:{:x}",Sha256::digest(content.as_bytes()))}))
            }
            LocalServiceOperation::MemorySearch { query, limit } => {
                if limit == 0 || limit > 1000 || query.len() > 4096 {
                    return Err(LocalServiceGatewayError::InvalidOperation);
                }
                let items = self
                    .selected
                    .retrieve_scoped(&scope, &query, limit)
                    .await
                    .map_err(|_| LocalServiceGatewayError::BackendFailure)?;
                Ok(json!({"items":items}))
            }
            _ => Err(LocalServiceGatewayError::InvalidOperation),
        }
    }
}

/// Workspace, artifact and cache bytes use the configured OmniSoloMemory provider.
/// Keys are content-independent opaque identifiers, never caller-controlled paths.
pub struct ExistingBlobBackend {
    selected: Arc<dyn crate::memory_store::OmniSoloMemory>,
    kind: LocalServiceKind,
}
impl ExistingBlobBackend {
    pub fn new(
        selected: Arc<dyn crate::memory_store::OmniSoloMemory>,
        kind: LocalServiceKind,
    ) -> Self {
        Self { selected, kind }
    }
}
#[async_trait]
impl LocalServiceBackend for ExistingBlobBackend {
    fn capabilities(&self) -> BTreeSet<String> {
        let prefix = match self.kind {
            LocalServiceKind::Artifact => "artifact",
            LocalServiceKind::Workspace => "workspace",
            LocalServiceKind::Cache => "cache",
            _ => return BTreeSet::new(),
        };
        [format!("{prefix}.read"), format!("{prefix}.write")]
            .into_iter()
            .collect()
    }
    async fn execute(
        &self,
        namespace: &ServiceNamespace,
        operation: LocalServiceOperation,
    ) -> Result<Value, LocalServiceGatewayError> {
        if operation.capability().0 != self.kind {
            return Err(LocalServiceGatewayError::InvalidOperation);
        }
        let (key, content) = match operation {
            LocalServiceOperation::ArtifactWrite { key, content }
            | LocalServiceOperation::WorkspaceWrite { key, content }
            | LocalServiceOperation::CacheWrite { key, content } => (key, Some(content)),
            LocalServiceOperation::ArtifactRead { key }
            | LocalServiceOperation::WorkspaceRead { key }
            | LocalServiceOperation::CacheRead { key } => (key, None),
            _ => return Err(LocalServiceGatewayError::InvalidOperation),
        };
        if key.is_empty() || key.len() > 4096 {
            return Err(LocalServiceGatewayError::InvalidOperation);
        }
        let storage_key = scope_key(&key);
        let storage_scope = namespace.storage_key();
        if let Some(content) = content {
            if content.len() > 16 * 1024 * 1024 {
                return Err(LocalServiceGatewayError::InvalidOperation);
            }
            self.selected
                .write(&storage_scope, &storage_key, &content)
                .await
                .map_err(|_| LocalServiceGatewayError::BackendFailure)?;
            let digest = format!("sha256:{:x}", Sha256::digest(&content));
            Ok(
                json!({"key":key,"digest":digest,"snapshot_id":uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_OID,digest.as_bytes())}),
            )
        } else {
            let content = self
                .selected
                .read(&storage_scope, &storage_key)
                .await
                .map_err(|_| LocalServiceGatewayError::BackendFailure)?;
            let digest = format!("sha256:{:x}", Sha256::digest(&content));
            Ok(
                json!({"key":key,"content":content,"digest":digest,"snapshot_id":uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_OID,digest.as_bytes())}),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use server_harness::middleware::local_services::*;
    use uuid::Uuid;

    struct SelectedEmbeddings(std::sync::atomic::AtomicUsize);
    #[async_trait]
    impl crate::llm::LlmClient for SelectedEmbeddings {
        async fn chat(
            &self,
            _: crate::types::ChatRequest,
        ) -> Result<crate::types::ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            unreachable!()
        }
        async fn generate_embedding(
            &self,
            _: &str,
        ) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
            self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Ok(vec![0.5, 0.25])
        }
    }
    #[tokio::test]
    async fn integration_metadata_and_explicit_invocation_use_selected_tool_catalog() {
        let root = std::env::temp_dir().join(format!("integration-service-{}", Uuid::new_v4()));
        tokio::fs::create_dir_all(&root).await.unwrap();
        tokio::fs::write(root.join("selected.txt"), "selected integration value")
            .await
            .unwrap();
        let scope = LocalServiceScopeContext::for_attempt(
            "tenant",
            Some("project"),
            Some("workspace"),
            Uuid::new_v4(),
            Some(Uuid::new_v4()),
            Some(Uuid::new_v4()),
        );
        let policy = crate::agent::AgentRunConfig {
            allowed_tools: Some(vec!["Read".into()]),
            ..Default::default()
        };
        let tool = crate::tools::read::read_tool(Some(root.clone()));
        let mut gateway = LocalServiceGateway::new(LocalServiceRegistry::with_defaults());
        gateway.register(
            LocalServiceKind::Integration,
            Arc::new(ExistingToolBackend::new(
                LocalServiceKind::Integration,
                &[tool],
                &policy,
                &scope,
            )),
        );
        let bundle = gateway.resolve(scope.clone()).unwrap();
        let metadata = gateway
            .execute(
                &bundle.bindings[0],
                &scope,
                LocalServiceOperation::IntegrationRead { key: "Read".into() },
            )
            .await
            .unwrap();
        assert_eq!(metadata["name"], "Read");
        assert_eq!(metadata["inputSchema"]["required"], json!(["path"]));
        let output = gateway
            .execute(
                &bundle.bindings[0],
                &scope,
                LocalServiceOperation::IntegrationInvoke {
                    action: "Read".into(),
                    arguments: json!({"path":"selected.txt"}),
                },
            )
            .await
            .unwrap();
        assert!(output.to_string().contains("selected integration value"));
        assert!(
            gateway
                .execute(
                    &bundle.bindings[0],
                    &scope,
                    LocalServiceOperation::IntegrationRead { key: "Bash".into() }
                )
                .await
                .is_err()
        );
        tokio::fs::remove_dir_all(root).await.unwrap();
    }

    #[tokio::test]
    async fn configured_vector_memory_reuses_existing_database_and_selected_embeddings() {
        let root = std::env::temp_dir().join(format!("selected-vector-{}.db", Uuid::new_v4()));
        let options = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(&root)
            .create_if_missing(true);
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect_with(options)
            .await
            .unwrap();
        sqlx::query("CREATE TABLE consolidated_memory(id TEXT PRIMARY KEY,tenant_id TEXT NOT NULL,agent_id TEXT,content TEXT NOT NULL,embedding TEXT,source_type TEXT NOT NULL,created_at TEXT,last_referenced_at TEXT,reference_count INTEGER,reliability_score INTEGER,owner_override BOOLEAN,metadata TEXT)").execute(&pool).await.unwrap();
        let config = ServiceMemoryConfiguration::Vector {
            url: format!("sqlite://{}", root.display()),
            tenant_id: "tenant".into(),
            agent_id: "agent".into(),
        };
        assert!(selected_memory_backend(&config, None).await.is_err());
        let embeddings = Arc::new(SelectedEmbeddings(std::sync::atomic::AtomicUsize::new(0)));
        let (backend, identity) = selected_memory_backend(&config, Some(embeddings.clone()))
            .await
            .unwrap();
        let mut gateway = LocalServiceGateway::new(
            LocalServiceRegistry::with_defaults()
                .with_backend_configuration(&identity)
                .unwrap(),
        );
        gateway.register(LocalServiceKind::Memory, backend);
        let scope = LocalServiceScopeContext::for_attempt(
            "tenant",
            Some("project"),
            Some("workspace"),
            Uuid::new_v4(),
            Some(Uuid::new_v4()),
            Some(Uuid::new_v4()),
        );
        let bundle = gateway.resolve(scope.clone()).unwrap();
        gateway
            .execute(
                &bundle.bindings[0],
                &scope,
                LocalServiceOperation::MemoryWrite {
                    content: "vector-service-sentinel".into(),
                },
            )
            .await
            .unwrap();
        assert_eq!(embeddings.0.load(std::sync::atomic::Ordering::SeqCst), 1);
        let rows: Vec<(String, String)> =
            sqlx::query_as("SELECT tenant_id,content FROM consolidated_memory")
                .fetch_all(&pool)
                .await
                .unwrap();
        assert_eq!(
            rows,
            vec![("tenant".into(), "vector-service-sentinel".into())]
        );
        let result = gateway
            .execute(
                &bundle.bindings[0],
                &scope,
                LocalServiceOperation::MemorySearch {
                    query: "sentinel".into(),
                    limit: 5,
                },
            )
            .await
            .unwrap();
        assert_eq!(result, json!({"items":["vector-service-sentinel"]}));
        pool.close().await;
        drop(gateway);
        tokio::fs::remove_file(root).await.unwrap();
    }

    #[tokio::test]
    async fn configured_service_memory_uses_existing_json_and_anthropic_backends() {
        let root = std::env::temp_dir().join(format!("selected-service-{}", Uuid::new_v4()));
        for config in [
            ServiceMemoryConfiguration::Json {
                root: root.join("json"),
            },
            ServiceMemoryConfiguration::Anthropic {
                root: root.join("anthropic"),
            },
        ] {
            let (backend, identity) = selected_memory_backend(&config, None).await.unwrap();
            let registry = LocalServiceRegistry::with_defaults()
                .with_backend_configuration(&identity)
                .unwrap();
            let mut gateway = LocalServiceGateway::new(registry);
            gateway.register(LocalServiceKind::Memory, backend);
            let scope = LocalServiceScopeContext::for_attempt(
                "tenant",
                Some("project"),
                Some("workspace"),
                Uuid::new_v4(),
                Some(Uuid::new_v4()),
                Some(Uuid::new_v4()),
            );
            let bundle = gateway.resolve(scope.clone()).unwrap();
            let binding = &bundle.bindings[0];
            gateway
                .execute(
                    binding,
                    &scope,
                    LocalServiceOperation::MemoryWrite {
                        content: "configured-helper-sentinel".into(),
                    },
                )
                .await
                .unwrap();
            let result = gateway
                .execute(
                    binding,
                    &scope,
                    LocalServiceOperation::MemorySearch {
                        query: "helper-sentinel".into(),
                        limit: 5,
                    },
                )
                .await
                .unwrap();
            assert!(result.to_string().contains("configured-helper-sentinel"));
        }
        tokio::fs::remove_dir_all(root).await.unwrap();
    }

    #[test]
    fn selected_backend_configuration_changes_portable_digests() {
        let scope = LocalServiceScopeContext::for_attempt(
            "tenant",
            Some("project"),
            Some("workspace"),
            Uuid::new_v4(),
            Some(Uuid::new_v4()),
            Some(Uuid::new_v4()),
        );
        let build = |memory_root: &str, blob_root: &str| {
            let run = crate::agent::AgentRunConfig {
                long_term_memory: Some(Arc::new(crate::json_store::NamespaceJsonStore::new(
                    memory_root,
                ))),
                ..Default::default()
            };
            gateway_for_agent_run(
                &run,
                LocalServiceRegistry::with_defaults(),
                Some(Arc::new(crate::memory_store::FileBasedMemory::new(
                    blob_root,
                ))),
            )
        };
        let first = build("/selected/memory-a", "/selected/blobs");
        let same = build("/selected/memory-a", "/selected/blobs");
        let changed_memory = build("/selected/memory-b", "/selected/blobs");
        let changed_blob = build("/selected/memory-a", "/selected/other-blobs");
        let refs = first.resolve(scope.clone()).unwrap();
        let digest = |gateway: &LocalServiceGateway| {
            gateway.resolve(scope.clone()).unwrap().bindings[0]
                .configuration_digest
                .clone()
        };
        assert_eq!(digest(&first), digest(&same));
        assert_ne!(digest(&first), digest(&changed_memory));
        assert_ne!(digest(&first), digest(&changed_blob));
        assert!(changed_memory.registry().rebind(&refs, scope).is_err());
        assert!(!serde_json::to_string(&refs).unwrap().contains("/selected/"));
    }

    #[tokio::test]
    async fn configured_json_and_file_stores_share_data_without_cross_workspace_access() {
        let root = std::env::temp_dir().join(format!("local-services-{}", Uuid::new_v4()));
        let memory = Arc::new(crate::json_store::NamespaceJsonStore::new(
            root.join("memory"),
        ));
        let blobs = Arc::new(crate::memory_store::FileBasedMemory::new(
            root.join("artifacts"),
        ));
        let registry = LocalServiceRegistry::with_defaults();
        let mut gateway = LocalServiceGateway::new(registry.clone());
        gateway.register(
            LocalServiceKind::Memory,
            Arc::new(SelectedMemoryBackend::new(memory)),
        );
        gateway.register(
            LocalServiceKind::Artifact,
            Arc::new(ExistingBlobBackend::new(blobs, LocalServiceKind::Artifact)),
        );
        let scope = LocalServiceScopeContext::for_attempt(
            "tenant",
            Some("project"),
            Some("workspace"),
            Uuid::new_v4(),
            Some(Uuid::new_v4()),
            Some(Uuid::new_v4()),
        );
        let writer = gateway.resolve(scope.clone()).unwrap();
        let reader = gateway.resolve(scope.clone()).unwrap();
        gateway
            .execute(
                writer.binding(LocalServiceKind::Memory).unwrap(),
                &scope,
                LocalServiceOperation::MemoryWrite {
                    content: "real-json-sentinel".into(),
                },
            )
            .await
            .unwrap();
        assert_eq!(
            gateway
                .execute(
                    reader.binding(LocalServiceKind::Memory).unwrap(),
                    &scope,
                    LocalServiceOperation::MemorySearch {
                        query: "sentinel".into(),
                        limit: 10
                    }
                )
                .await
                .unwrap(),
            serde_json::json!({"items":["real-json-sentinel"]})
        );
        gateway
            .execute(
                writer.binding(LocalServiceKind::Artifact).unwrap(),
                &scope,
                LocalServiceOperation::ArtifactWrite {
                    key: "report".into(),
                    content: b"real-file-sentinel".to_vec(),
                },
            )
            .await
            .unwrap();
        let read = gateway
            .execute(
                reader.binding(LocalServiceKind::Artifact).unwrap(),
                &scope,
                LocalServiceOperation::ArtifactRead {
                    key: "report".into(),
                },
            )
            .await
            .unwrap();
        assert_eq!(
            read["content"],
            serde_json::json!(b"real-file-sentinel".to_vec())
        );
        let mut other = scope.clone();
        other.workspace_id = Some("other".into());
        let foreign = gateway.resolve(other.clone()).unwrap();
        assert_eq!(
            gateway
                .execute(
                    foreign.binding(LocalServiceKind::Memory).unwrap(),
                    &other,
                    LocalServiceOperation::MemorySearch {
                        query: "sentinel".into(),
                        limit: 10
                    }
                )
                .await
                .unwrap(),
            serde_json::json!({"items":[]})
        );
        assert!(
            gateway
                .execute(
                    foreign.binding(LocalServiceKind::Artifact).unwrap(),
                    &other,
                    LocalServiceOperation::ArtifactRead {
                        key: "report".into()
                    }
                )
                .await
                .is_err()
        );
        tokio::fs::remove_dir_all(root).await.unwrap();
    }
}

/// Register the backend already selected by AgentRunConfig; never repeat env
/// precedence here or construct a replacement database.
pub fn gateway_for_agent_run(
    run: &crate::agent::AgentRunConfig,
    registry: server_harness::middleware::local_services::LocalServiceRegistry,
    blobs: Option<Arc<dyn crate::memory_store::OmniSoloMemory>>,
) -> LocalServiceGateway {
    let identity = serde_json::json!({
        "memory": run.long_term_memory.as_ref().map(|memory|memory.service_configuration_identity()),
        "blobs": blobs.as_ref().map(|blobs|blobs.service_configuration_identity()),
        "workspace": run.workspace_path,
        "allowed_tools": run.allowed_tools,
        "project_trusted": run.project_trusted,
        "high_risk_tools": run.high_risk_tools,
        "human_in_loop": run.hil_spectrum,
        "permission_architecture": run.permission_architecture,
    }).to_string();
    let registry = registry
        .with_backend_configuration(identity.as_bytes())
        .expect("builtin service registry must be configured before issuing leases");
    let mut gateway = LocalServiceGateway::new(registry);
    if let Some(memory) = &run.long_term_memory {
        gateway.register(
            LocalServiceKind::Memory,
            Arc::new(SelectedMemoryBackend::new(memory.clone())),
        );
    }
    if let Some(blobs) = blobs {
        for kind in [
            LocalServiceKind::Workspace,
            LocalServiceKind::Artifact,
            LocalServiceKind::Cache,
        ] {
            gateway.register(
                kind,
                Arc::new(ExistingBlobBackend::new(blobs.clone(), kind)),
            );
        }
    }
    gateway
}

struct ScopedToolState {
    bundle: server_harness::middleware::local_services::LocalServiceBundle,
    scope: server_harness::middleware::local_services::LocalServiceScopeContext,
}
struct ScopedToolLease {
    gateway: Arc<LocalServiceGateway>,
    state: std::sync::Mutex<ScopedToolState>,
}
#[derive(Clone)]
pub struct GatewayToolLease(Arc<ScopedToolLease>);
impl GatewayToolLease {
    pub fn revoke_current(&self) {
        let state = self
            .0
            .state
            .lock()
            .expect("local service tool state poisoned");
        if let Some(attempt) = state.scope.attempt_id {
            self.0
                .gateway
                .registry()
                .revoke_attempt(&state.scope.tenant_id, attempt);
        }
    }
    pub fn rebind_attempt(
        &self,
        attempt: uuid::Uuid,
    ) -> Result<
        server_harness::middleware::local_services::LocalServiceBundle,
        server_harness::middleware::local_services::LocalServiceError,
    > {
        let mut state = self
            .0
            .state
            .lock()
            .expect("local service tool state poisoned");
        let mut scope = state.scope.clone();
        scope.attempt_id = Some(attempt);
        let bundle = self
            .0
            .gateway
            .registry()
            .rebind(&state.bundle, scope.clone())?;
        state.scope = scope;
        state.bundle = bundle.clone();
        Ok(bundle)
    }
}
impl Drop for ScopedToolLease {
    fn drop(&mut self) {
        let state = self
            .state
            .lock()
            .expect("local service tool state poisoned");
        if let Some(attempt) = state.scope.attempt_id {
            self.gateway
                .registry()
                .revoke_attempt(&state.scope.tenant_id, attempt);
        }
        let gateway = self.gateway.clone();
        let bundle = state.bundle.clone();
        if let Ok(runtime) = tokio::runtime::Handle::try_current() {
            runtime.spawn(async move {
                gateway.release(&bundle).await;
            });
        }
    }
}
struct ScopedGatewayTool {
    lease: Arc<ScopedToolLease>,
}
#[async_trait]
impl crate::tools::ToolExecutor for ScopedGatewayTool {
    async fn execute(&self, args: Value) -> Result<String, crate::types::ToolError> {
        let operation: LocalServiceOperation = serde_json::from_value(args).map_err(|_| {
            crate::types::ToolError::LlmRecoverable("invalid local service operation".to_owned())
        })?;
        let (binding, scope) = {
            let state = self
                .lease
                .state
                .lock()
                .expect("local service tool state poisoned");
            let binding = state
                .bundle
                .binding(operation.capability().0)
                .cloned()
                .ok_or_else(|| {
                    crate::types::ToolError::LlmRecoverable(
                        "local service is unavailable for this attempt".to_owned(),
                    )
                })?;
            (binding, state.scope.clone())
        };
        let result = self
            .lease
            .gateway
            .execute(&binding, &scope, operation)
            .await
            .map_err(|error| crate::types::ToolError::LlmRecoverable(error.to_string()))?;
        Ok(result.to_string())
    }
}

pub fn gateway_tool(
    gateway: Arc<LocalServiceGateway>,
    bundle: server_harness::middleware::local_services::LocalServiceBundle,
    scope: server_harness::middleware::local_services::LocalServiceScopeContext,
) -> (crate::tools::Tool, GatewayToolLease) {
    let capabilities = bundle
        .bindings
        .iter()
        .flat_map(|binding| binding.granted_capabilities.iter().cloned())
        .collect::<Vec<_>>();
    let lease = Arc::new(ScopedToolLease {
        gateway,
        state: std::sync::Mutex::new(ScopedToolState { bundle, scope }),
    });
    (
        crate::tools::Tool {
            name: "local_services".to_owned(),
            description: format!(
                "Use scoped local services. Available capabilities: {}",
                capabilities.join(", ")
            ),
            is_read_only: false,
            parameters: json!({
                "type": "object",
                "properties": {
                    "operation": {"type": "string"},
                    "content": {},
                    "key": {"type": "string"},
                    "query": {"type": "string"},
                    "limit": {"type": "integer"},
                    "tool": {"type": "string"},
                    "arguments": {"type": "object"},
                    "action": {"type": "string"},
                    "target": {"type": "string"},
                    "value": {"type": "string"},
                    "url": {"type": "string"}
                },
                "required": ["operation"]
            }),
            execute: Arc::new(ScopedGatewayTool {
                lease: lease.clone(),
            }),
        },
        GatewayToolLease(lease),
    )
}

/// Reuse the configured tool executors behind the same approved-tool policy.
/// Catalog entries contain schemas only; executable closures remain service-owned.
pub struct ExistingToolBackend {
    kind: LocalServiceKind,
    tools: std::collections::BTreeMap<String, crate::tools::Tool>,
    policy: crate::agent::AgentRunConfig,
    tenant_id: String,
    project_id: Option<String>,
}
impl ExistingToolBackend {
    pub fn new(
        kind: LocalServiceKind,
        tools: &[crate::tools::Tool],
        run: &crate::agent::AgentRunConfig,
        scope: &server_harness::middleware::local_services::LocalServiceScopeContext,
    ) -> Self {
        let tools = tools
            .iter()
            .filter(|tool| {
                run.allowed_tools
                    .as_ref()
                    .is_none_or(|allowed| allowed.contains(&tool.name))
            })
            .map(|tool| (tool.name.clone(), tool.clone()))
            .collect();
        Self {
            kind,
            tools,
            policy: run.clone(),
            tenant_id: scope.tenant_id.clone(),
            project_id: scope.project_id.clone(),
        }
    }
}
#[async_trait]
impl LocalServiceBackend for ExistingToolBackend {
    fn capabilities(&self) -> BTreeSet<String> {
        if self.tools.is_empty() {
            return BTreeSet::new();
        }
        match self.kind {
            LocalServiceKind::Mcp => ["mcp.catalog", "mcp.invoke"]
                .into_iter()
                .map(str::to_owned)
                .collect(),
            LocalServiceKind::Integration => {
                let mut capabilities = BTreeSet::from(["integration.invoke".to_owned()]);
                if self.tools.values().any(|tool| tool.is_read_only) {
                    capabilities.insert("integration.read".to_owned());
                }
                capabilities
            }
            _ => BTreeSet::new(),
        }
    }
    async fn execute(
        &self,
        namespace: &ServiceNamespace,
        operation: LocalServiceOperation,
    ) -> Result<Value, LocalServiceGatewayError> {
        if namespace.tenant_id != self.tenant_id || namespace.project_id != self.project_id {
            return Err(
                server_harness::middleware::local_services::LocalServiceError::TenantMismatch
                    .into(),
            );
        }
        if operation.capability().0 != self.kind {
            return Err(LocalServiceGatewayError::InvalidOperation);
        }
        let requires_read_only =
            matches!(&operation, LocalServiceOperation::IntegrationRead { .. });
        let (name, args) = match operation {
            LocalServiceOperation::McpCatalog => {
                return Ok(
                    json!({"tools":self.tools.values().map(|tool|json!({"name":tool.name,"description":tool.description,"inputSchema":tool.parameters})).collect::<Vec<_>>()}),
                );
            }
            LocalServiceOperation::McpInvoke { tool, arguments } => (tool, arguments),
            LocalServiceOperation::IntegrationInvoke { action, arguments } => (action, arguments),
            LocalServiceOperation::IntegrationRead { key } => (key, json!({})),
            _ => return Err(LocalServiceGatewayError::InvalidOperation),
        };
        let tool = self.tools.get(&name).ok_or_else(|| {
            LocalServiceGatewayError::UnsupportedCapability("configured tool".to_owned())
        })?;
        if requires_read_only && !tool.is_read_only {
            return Err(
                server_harness::middleware::local_services::LocalServiceError::CapabilityDenied(
                    "integration.invoke is required".to_owned(),
                )
                .into(),
            );
        }
        let call_id = format!(
            "local-service-{}",
            scope_key(&format!("{}:{}:{}", namespace.storage_key(), name, args))
        );
        let call = crate::types::ToolCall {
            id: call_id.clone(),
            name: name.clone(),
            arguments: args.clone(),
        };
        crate::tools_gating::ToolGater::check_gating(&call, tool.is_read_only, &self.policy)
            .map_err(|_| {
                LocalServiceGatewayError::Authorization(
                    server_harness::middleware::local_services::LocalServiceError::CapabilityDenied(
                        format!("tool policy rejected call {call_id}"),
                    ),
                )
            })?;
        if requires_read_only {
            return Ok(
                json!({"name":tool.name,"description":tool.description,"inputSchema":tool.parameters,"read_only":tool.is_read_only}),
            );
        }
        let output = tool
            .execute
            .execute(args)
            .await
            .map_err(|_| LocalServiceGatewayError::BackendFailure)?;
        Ok(json!({"content":output,"call_id":call_id}))
    }
}

/// Browser navigation and interaction use the configured `agent-browser` controller.
/// Each issued binding owns its own isolated session (`--session <lease_id>`).
pub struct AgentBrowserBackend {
    controller: crate::tools::browser::AgentBrowserController,
    policy: crate::agent::AgentRunConfig,
    tenant_id: String,
    project_id: Option<String>,
    navigated_leases: tokio::sync::Mutex<BTreeSet<uuid::Uuid>>,
    operation_lock: tokio::sync::Mutex<()>,
}
impl AgentBrowserBackend {
    pub fn new(
        controller: crate::tools::browser::AgentBrowserController,
        scope: &server_harness::middleware::local_services::LocalServiceScopeContext,
        policy: &crate::agent::AgentRunConfig,
    ) -> Self {
        Self {
            controller,
            policy: policy.clone(),
            tenant_id: scope.tenant_id.clone(),
            project_id: scope.project_id.clone(),
            navigated_leases: tokio::sync::Mutex::new(BTreeSet::new()),
            operation_lock: tokio::sync::Mutex::new(()),
        }
    }

    fn allows_tool(&self, tool_name: &str) -> bool {
        self.policy
            .allowed_tools
            .as_ref()
            .is_none_or(|tools| tools.iter().any(|tool| tool == tool_name))
    }

    fn check_browser_gating(
        &self,
        lease: uuid::Uuid,
        suffix: &str,
        tool_name: &str,
        is_read_only: bool,
        arguments: Value,
    ) -> Result<(), LocalServiceGatewayError> {
        let call = crate::types::ToolCall {
            id: format!("browser-{lease}-{suffix}"),
            name: tool_name.to_owned(),
            arguments,
        };
        crate::tools_gating::ToolGater::check_gating(&call, is_read_only, &self.policy).map_err(
            |_| {
                LocalServiceGatewayError::Authorization(
                    server_harness::middleware::local_services::LocalServiceError::CapabilityDenied(
                        "browser tool policy rejected".to_owned(),
                    ),
                )
            },
        )
    }

    async fn validate_navigation_target(url: &str) -> Result<url::Url, LocalServiceGatewayError> {
        let trimmed = url.trim();
        if trimmed != url || url.chars().any(char::is_control) {
            return Err(LocalServiceGatewayError::InvalidOperation);
        }
        #[cfg(test)]
        {
            let parsed =
                url::Url::parse(trimmed).map_err(|_| LocalServiceGatewayError::InvalidOperation)?;
            crate::tools::network_policy::validate_url(
                &parsed,
                crate::tools::network_policy::private_network_allowed(),
            )
            .map_err(|_| LocalServiceGatewayError::InvalidOperation)?;
            Ok(parsed)
        }
        #[cfg(not(test))]
        {
            crate::tools::browser::AgentBrowserController::validate_navigation_target(trimmed)
                .await
                .map_err(|_| LocalServiceGatewayError::InvalidOperation)
        }
    }

    fn build_browser_act_command(
        action: &str,
        target: Option<&str>,
        value: Option<&str>,
    ) -> Result<(Vec<String>, bool), LocalServiceGatewayError> {
        let require_target = |raw: Option<&str>| -> Result<String, LocalServiceGatewayError> {
            raw.map(str::trim)
                .filter(|s| !s.is_empty() && !s.starts_with('-'))
                .map(str::to_owned)
                .ok_or(LocalServiceGatewayError::InvalidOperation)
        };
        let action = action.trim().to_ascii_lowercase();
        let cmd = match action.as_str() {
            "back" | "forward" | "reload" => vec![action.clone()],
            "click" | "hover" | "press" | "wait" => {
                vec![action.clone(), require_target(target)?]
            }
            "fill" | "select" => {
                let target = require_target(target)?;
                let value = value.ok_or(LocalServiceGatewayError::InvalidOperation)?;
                vec![action.clone(), target, value.to_owned()]
            }
            "scroll" => {
                let direction = target.unwrap_or("down").trim();
                if !matches!(direction, "up" | "down" | "left" | "right") {
                    return Err(LocalServiceGatewayError::InvalidOperation);
                }
                let mut cmd = vec!["scroll".to_owned(), direction.to_owned()];
                if let Some(amount) = value.map(str::trim).filter(|v| !v.is_empty()) {
                    if amount.starts_with('-') {
                        return Err(LocalServiceGatewayError::InvalidOperation);
                    }
                    cmd.push(amount.to_owned());
                }
                cmd
            }
            _ => return Err(LocalServiceGatewayError::InvalidOperation),
        };
        let is_mutating = action != "wait";
        Ok((cmd, is_mutating))
    }
}
#[async_trait]
impl LocalServiceBackend for AgentBrowserBackend {
    fn capabilities(&self) -> BTreeSet<String> {
        if !self.allows_tool("BrowserObserve") {
            return BTreeSet::new();
        }
        let mut capabilities =
            BTreeSet::from(["browser.navigate".to_owned(), "browser.snapshot".to_owned()]);
        if self.allows_tool("BrowserAct") {
            capabilities.insert("browser.act".to_owned());
        }
        capabilities
    }
    async fn execute(
        &self,
        namespace: &ServiceNamespace,
        operation: LocalServiceOperation,
    ) -> Result<Value, LocalServiceGatewayError> {
        if namespace.tenant_id != self.tenant_id || namespace.project_id != self.project_id {
            return Err(
                server_harness::middleware::local_services::LocalServiceError::TenantMismatch
                    .into(),
            );
        }
        let _operation = self.operation_lock.lock().await;
        let lease = namespace
            .lease_id
            .ok_or(LocalServiceGatewayError::InvalidOperation)?;
        match operation {
            LocalServiceOperation::BrowserNavigate { url } => {
                Self::validate_navigation_target(&url).await?;
                self.check_browser_gating(
                    lease,
                    "navigate",
                    "BrowserObserve",
                    true,
                    json!({"action": "open", "url": url}),
                )?;
                self.controller
                    .run_browser_command(
                        &lease.to_string(),
                        &["open".to_owned(), url.clone()],
                        false,
                        false,
                    )
                    .await
                    .map_err(|err| {
                        tracing::warn!("agent-browser navigate failed: {err}");
                        LocalServiceGatewayError::BackendFailure
                    })?;
                self.navigated_leases.lock().await.insert(lease);
                Ok(json!({"navigated": true, "url": url}))
            }
            LocalServiceOperation::BrowserSnapshot => {
                self.check_browser_gating(
                    lease,
                    "snapshot",
                    "BrowserObserve",
                    true,
                    json!({"action": "snapshot"}),
                )?;
                if !self.navigated_leases.lock().await.contains(&lease) {
                    return Err(LocalServiceGatewayError::InvalidOperation);
                }
                let content = self
                    .controller
                    .run_browser_command(
                        &lease.to_string(),
                        &["snapshot".to_owned(), "-i".to_owned()],
                        false,
                        true,
                    )
                    .await
                    .map_err(|err| {
                        tracing::warn!("agent-browser snapshot failed: {err}");
                        LocalServiceGatewayError::BackendFailure
                    })?;
                if content.len() > 16 * 1024 * 1024 {
                    return Err(LocalServiceGatewayError::BackendFailure);
                }
                let digest = format!("sha256:{:x}", Sha256::digest(content.as_bytes()));
                Ok(json!({
                    "content": content,
                    "content_type": "application/json",
                    "digest": digest,
                }))
            }
            LocalServiceOperation::BrowserAct {
                action,
                target,
                value,
            } => {
                self.check_browser_gating(
                    lease,
                    "act",
                    "BrowserAct",
                    false,
                    json!({"action": action, "target": target, "value": value}),
                )?;
                if !self.navigated_leases.lock().await.contains(&lease) {
                    return Err(LocalServiceGatewayError::InvalidOperation);
                }
                let (cmd_args, is_mutating) =
                    Self::build_browser_act_command(&action, target.as_deref(), value.as_deref())?;
                let out = self
                    .controller
                    .run_browser_command(&lease.to_string(), &cmd_args, is_mutating, false)
                    .await
                    .map_err(|err| {
                        tracing::warn!("agent-browser act failed: {err}");
                        LocalServiceGatewayError::BackendFailure
                    })?;
                Ok(json!({"result": out}))
            }
            _ => Err(LocalServiceGatewayError::InvalidOperation),
        }
    }
    async fn release(&self, namespace: &ServiceNamespace) {
        let _operation = self.operation_lock.lock().await;
        if let Some(lease) = namespace.lease_id {
            self.navigated_leases.lock().await.remove(&lease);
            self.controller.close_session(&lease.to_string()).await;
        }
    }
}

#[cfg(test)]
mod scoped_backend_tests {
    use super::*;
    use crate::memory_store::LongTermMemory;
    #[tokio::test]
    async fn selected_non_sqlite_backends_keep_namespace_isolation() {
        let root = std::env::temp_dir().join(format!("scoped-memory-{}", uuid::Uuid::new_v4()));
        let stores: Vec<Arc<dyn LongTermMemory>> = vec![
            Arc::new(crate::json_store::NamespaceJsonStore::new(
                root.join("json"),
            )),
            Arc::new(crate::in_memory_store::InMemoryNamespaceStore::new()),
            Arc::new(
                crate::memory_store::Anthropic3TierMemoryStore::new(root.join("anthropic"))
                    .unwrap(),
            ),
        ];
        for store in stores {
            assert!(store.supports_scoped_services());
            store
                .store_scoped("workspace-a", "scope-sentinel")
                .await
                .unwrap();
            let contents = store
                .retrieve_scoped("workspace-a", "sentinel", 10)
                .await
                .unwrap();
            assert_eq!(contents.len(), 1);
            assert!(contents[0].contains("scope-sentinel"));
            assert!(
                store
                    .retrieve_scoped("workspace-b", "sentinel", 10)
                    .await
                    .unwrap()
                    .is_empty()
            );
        }
        tokio::fs::remove_dir_all(root).await.unwrap();
    }

    #[tokio::test]
    async fn tool_leases_rebind_recovery_and_fence_old_attempts() {
        use server_harness::middleware::local_services::*;
        let registry = LocalServiceRegistry::with_defaults();
        let mut gateway = LocalServiceGateway::new(registry.clone());
        gateway.register(
            LocalServiceKind::Memory,
            Arc::new(SelectedMemoryBackend::new(Arc::new(
                crate::in_memory_store::InMemoryNamespaceStore::new(),
            ))),
        );
        let scope = LocalServiceScopeContext::for_attempt(
            "tenant",
            Some("project"),
            Some("workspace"),
            uuid::Uuid::new_v4(),
            Some(uuid::Uuid::new_v4()),
            Some(uuid::Uuid::new_v4()),
        );
        let original = gateway.resolve(scope.clone()).unwrap();
        let (tool, lease) = gateway_tool(Arc::new(gateway), original.clone(), scope.clone());
        tool.execute
            .execute(json!({"operation":"memory_write","content":"survives-recovery"}))
            .await
            .unwrap();
        lease.revoke_current();
        assert!(
            tool.execute
                .execute(json!({"operation":"memory_search","query":"recovery","limit":10}))
                .await
                .is_err()
        );
        let rebound = lease.rebind_attempt(uuid::Uuid::new_v4()).unwrap();
        assert_eq!(rebound.bindings[0].generation, 2);
        let output = tool
            .execute
            .execute(json!({"operation":"memory_search","query":"recovery","limit":10}))
            .await
            .unwrap();
        assert!(output.contains("survives-recovery"));
        assert!(
            registry
                .authorize(&original.bindings[0], &scope, "memory.search")
                .is_err()
        );
    }
}

#[cfg(all(test, unix))]
mod browser_backend_tests {
    use super::*;
    use server_harness::middleware::local_services::*;
    use std::collections::VecDeque;
    use std::os::unix::fs::PermissionsExt;
    use std::os::unix::process::ExitStatusExt;

    struct MockBrowserRunner {
        calls: std::sync::Mutex<Vec<(String, Vec<String>)>>,
        responses: std::sync::Mutex<VecDeque<std::io::Result<std::process::Output>>>,
    }

    impl MockBrowserRunner {
        fn new() -> Self {
            Self {
                calls: std::sync::Mutex::new(Vec::new()),
                responses: std::sync::Mutex::new(VecDeque::new()),
            }
        }

        fn push_stdout(&self, stdout: &str) {
            self.responses
                .lock()
                .unwrap()
                .push_back(Ok(std::process::Output {
                    status: std::process::ExitStatus::from_raw(0),
                    stdout: stdout.as_bytes().to_vec(),
                    stderr: Vec::new(),
                }));
        }

        fn calls(&self) -> Vec<(String, Vec<String>)> {
            self.calls.lock().unwrap().clone()
        }
    }

    #[async_trait]
    impl crate::tools::runner::CommandRunner for MockBrowserRunner {
        async fn run(
            &self,
            program: &str,
            args: &[&str],
            _root: Option<&std::path::Path>,
            _envs: Vec<(String, String)>,
        ) -> std::io::Result<std::process::Output> {
            self.calls.lock().unwrap().push((
                program.to_owned(),
                args.iter().map(|s| (*s).to_owned()).collect(),
            ));
            self.responses
                .lock()
                .unwrap()
                .pop_front()
                .unwrap_or_else(|| {
                    Ok(std::process::Output {
                        status: std::process::ExitStatus::from_raw(0),
                        stdout: br#"{"success":true}"#.to_vec(),
                        stderr: Vec::new(),
                    })
                })
        }
    }

    #[test]
    fn agent_browser_backend_isolates_leases_executes_flow_and_cleans_up_on_release() {
        let dir = tempfile::tempdir().unwrap();
        let fake_bin = dir.path().join("fake-agent-browser");
        std::fs::write(&fake_bin, b"#!/bin/sh\nexit 0\n").unwrap();
        std::fs::set_permissions(&fake_bin, std::fs::Permissions::from_mode(0o755)).unwrap();

        let scope = LocalServiceScopeContext::for_attempt(
            "tenant",
            Some("project"),
            Some("workspace"),
            uuid::Uuid::new_v4(),
            Some(uuid::Uuid::new_v4()),
            Some(uuid::Uuid::new_v4()),
        );
        let policy = crate::agent::AgentRunConfig {
            hil_spectrum: crate::types::HumanInLoopSpectrum::Autonomous,
            permission_architecture: crate::types::PermissionArchitecture::Permissive,
            ..Default::default()
        };

        temp_env::with_vars(
            [
                (
                    "OMNISOLO_AGENT_BROWSER_BIN",
                    Some(fake_bin.to_str().unwrap()),
                ),
                ("OMNISOLO_AGENT_ALLOW_PRIVATE_NETWORK", Some("false")),
            ],
            || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                rt.block_on(async {
                    let runner = Arc::new(MockBrowserRunner::new());
                    let controller = crate::tools::browser::AgentBrowserController::new(
                        Some(dir.path().to_path_buf()),
                        runner.clone(),
                    );

                    let mut excluded_policy = policy.clone();
                    excluded_policy.allowed_tools = Some(vec!["Read".into()]);
                    assert!(
                        AgentBrowserBackend::new(controller.clone(), &scope, &excluded_policy)
                            .capabilities()
                            .is_empty()
                    );

                    let mut observe_only_policy = policy.clone();
                    observe_only_policy.allowed_tools = Some(vec!["BrowserObserve".into()]);
                    assert_eq!(
                        AgentBrowserBackend::new(controller.clone(), &scope, &observe_only_policy)
                            .capabilities(),
                        BTreeSet::from([
                            "browser.navigate".to_owned(),
                            "browser.snapshot".to_owned()
                        ])
                    );

                    let mut gateway =
                        LocalServiceGateway::new(LocalServiceRegistry::with_defaults());
                    gateway.register(
                        LocalServiceKind::Browser,
                        Arc::new(AgentBrowserBackend::new(
                            controller.clone(),
                            &scope,
                            &policy,
                        )),
                    );
                    let first = gateway.resolve(scope.clone()).unwrap();
                    let second = gateway.resolve(scope.clone()).unwrap();
                    let first_binding = first.binding(LocalServiceKind::Browser).unwrap();
                    let second_binding = second.binding(LocalServiceKind::Browser).unwrap();
                    let first_lease = first_binding.binding_id.to_string();

                    let (tool, _lease) =
                        gateway_tool(Arc::new(gateway.clone()), first.clone(), scope.clone());
                    assert!(tool.parameters["properties"].get("target").is_some());
                    assert!(tool.parameters["properties"].get("value").is_some());

                    // SSRF and invalid schemes must be rejected before spawning agent-browser.
                    for bad_url in [
                        "http://127.0.0.1:8080/admin",
                        "file:///etc/passwd",
                        "https://user:secret@example.test/",
                    ] {
                        assert!(
                            gateway
                                .execute(
                                    first_binding,
                                    &scope,
                                    LocalServiceOperation::BrowserNavigate {
                                        url: bad_url.into(),
                                    },
                                )
                                .await
                                .is_err(),
                            "expected rejection for {bad_url}"
                        );
                    }

                    // 1. Navigate first lease
                    runner.push_stdout(
                        r#"{"success":true,"data":{"url":"https://example.test/first"}}"#,
                    );
                    let nav = gateway
                        .execute(
                            first_binding,
                            &scope,
                            LocalServiceOperation::BrowserNavigate {
                                url: "https://example.test/first".into(),
                            },
                        )
                        .await
                        .unwrap();
                    assert_eq!(
                        nav,
                        json!({"navigated": true, "url": "https://example.test/first"})
                    );

                    // Foreign / unnavigated lease cannot snapshot or act.
                    assert!(
                        gateway
                            .execute(
                                second_binding,
                                &scope,
                                LocalServiceOperation::BrowserSnapshot
                            )
                            .await
                            .is_err()
                    );
                    assert!(
                        gateway
                            .execute(
                                second_binding,
                                &scope,
                                LocalServiceOperation::BrowserAct {
                                    action: "click".into(),
                                    target: Some("@e1".into()),
                                    value: None,
                                },
                            )
                            .await
                            .is_err()
                    );

                    // 2. Snapshot first lease
                    let snapshot_payload =
                        r#"{"success":true,"data":{"tree":"@e1 button Submit"}}"#;
                    runner.push_stdout(snapshot_payload);
                    let snapshot = gateway
                        .execute(
                            first_binding,
                            &scope,
                            LocalServiceOperation::BrowserSnapshot,
                        )
                        .await
                        .unwrap();
                    let expected_digest =
                        format!("sha256:{:x}", Sha256::digest(snapshot_payload.as_bytes()));
                    assert_eq!(snapshot["content"], json!(snapshot_payload));
                    assert_eq!(snapshot["content_type"], json!("application/json"));
                    assert_eq!(snapshot["digest"], json!(expected_digest));

                    // 3. Act on first lease (click + fill + wait; reject disallowed action)
                    runner.push_stdout(r#"{"success":true,"clicked":"@e1"}"#);
                    let click = gateway
                        .execute(
                            first_binding,
                            &scope,
                            LocalServiceOperation::BrowserAct {
                                action: "click".into(),
                                target: Some("@e1".into()),
                                value: None,
                            },
                        )
                        .await
                        .unwrap();
                    assert_eq!(
                        click["result"],
                        json!(r#"{"success":true,"clicked":"@e1"}"#)
                    );

                    runner.push_stdout(r#"{"success":true,"filled":"@e2"}"#);
                    let fill = gateway
                        .execute(
                            first_binding,
                            &scope,
                            LocalServiceOperation::BrowserAct {
                                action: "fill".into(),
                                target: Some("@e2".into()),
                                value: Some("hello".into()),
                            },
                        )
                        .await
                        .unwrap();
                    assert_eq!(fill["result"], json!(r#"{"success":true,"filled":"@e2"}"#));

                    runner.push_stdout(r#"{"success":true,"waited":"50"}"#);
                    let wait = gateway
                        .execute(
                            first_binding,
                            &scope,
                            LocalServiceOperation::BrowserAct {
                                action: "wait".into(),
                                target: Some("50".into()),
                                value: None,
                            },
                        )
                        .await
                        .unwrap();
                    assert_eq!(wait["result"], json!(r#"{"success":true,"waited":"50"}"#));

                    assert!(
                        gateway
                            .execute(
                                first_binding,
                                &scope,
                                LocalServiceOperation::BrowserAct {
                                    action: "close".into(),
                                    target: None,
                                    value: None,
                                },
                            )
                            .await
                            .is_err()
                    );

                    for bad_act in [
                        LocalServiceOperation::BrowserAct {
                            action: "click".into(),
                            target: Some("--headed".into()),
                            value: None,
                        },
                        LocalServiceOperation::BrowserAct {
                            action: "fill".into(),
                            target: Some("-s".into()),
                            value: Some("hello".into()),
                        },
                        LocalServiceOperation::BrowserAct {
                            action: "scroll".into(),
                            target: Some("down".into()),
                            value: Some("--headed".into()),
                        },
                    ] {
                        assert!(matches!(
                            gateway.execute(first_binding, &scope, bad_act).await,
                            Err(LocalServiceGatewayError::InvalidOperation)
                        ));
                    }

                    let session_profile = dir
                        .path()
                        .join(".omnisolo-browser-sessions")
                        .join(&first_lease);
                    assert!(session_profile.exists());
                    assert!(controller.session_metrics(&first_lease).is_some());

                    // 4. Release first lease and verify session close + cleanup
                    gateway.release(&first).await;
                    assert!(!session_profile.exists());
                    assert!(controller.session_metrics(&first_lease).is_none());
                    assert!(
                        gateway
                            .execute(
                                first_binding,
                                &scope,
                                LocalServiceOperation::BrowserSnapshot
                            )
                            .await
                            .is_err()
                    );

                    let calls = runner.calls();
                    assert!(calls.iter().any(|(prog, args)| {
                        prog == fake_bin.to_str().unwrap()
                            && args
                                .windows(2)
                                .any(|w| w == ["--session", first_lease.as_str()])
                            && args.ends_with(&[
                                "open".to_owned(),
                                "https://example.test/first".to_owned(),
                            ])
                    }));
                    assert!(calls.iter().any(|(_, args)| {
                        args.ends_with(&["snapshot".to_owned(), "-i".to_owned()])
                    }));
                    assert!(calls.iter().any(|(_, args)| {
                        args.ends_with(&["click".to_owned(), "@e1".to_owned()])
                    }));
                    assert!(calls.iter().any(|(_, args)| {
                        args.ends_with(&["fill".to_owned(), "@e2".to_owned(), "hello".to_owned()])
                    }));
                    assert!(calls.iter().any(|(_, args)| {
                        args.ends_with(&["wait".to_owned(), "50".to_owned()])
                    }));
                    assert!(calls.iter().any(|(_, args)| {
                        args == &vec![
                            "--json".to_owned(),
                            "--session".to_owned(),
                            first_lease.clone(),
                            "close".to_owned(),
                        ]
                    }));
                });
            },
        );
    }

    #[test]
    fn agent_browser_backend_gates_mutating_act_under_restrictive_policy() {
        let dir = tempfile::tempdir().unwrap();
        let fake_bin = dir.path().join("fake-agent-browser");
        std::fs::write(&fake_bin, b"#!/bin/sh\nexit 0\n").unwrap();
        std::fs::set_permissions(&fake_bin, std::fs::Permissions::from_mode(0o755)).unwrap();

        let scope = LocalServiceScopeContext::for_attempt(
            "tenant",
            Some("project"),
            Some("workspace"),
            uuid::Uuid::new_v4(),
            Some(uuid::Uuid::new_v4()),
            Some(uuid::Uuid::new_v4()),
        );
        let policy = crate::agent::AgentRunConfig {
            hil_spectrum: crate::types::HumanInLoopSpectrum::Autonomous,
            permission_architecture: crate::types::PermissionArchitecture::Restrictive,
            ..Default::default()
        };

        temp_env::with_vars(
            [
                (
                    "OMNISOLO_AGENT_BROWSER_BIN",
                    Some(fake_bin.to_str().unwrap()),
                ),
                ("OMNISOLO_AGENT_ALLOW_PRIVATE_NETWORK", Some("false")),
            ],
            || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                rt.block_on(async {
                    let runner = Arc::new(MockBrowserRunner::new());
                    let controller = crate::tools::browser::AgentBrowserController::new(
                        Some(dir.path().to_path_buf()),
                        runner,
                    );
                    let mut gateway =
                        LocalServiceGateway::new(LocalServiceRegistry::with_defaults());
                    gateway.register(
                        LocalServiceKind::Browser,
                        Arc::new(AgentBrowserBackend::new(controller, &scope, &policy)),
                    );
                    let bundle = gateway.resolve(scope.clone()).unwrap();
                    let binding = bundle.binding(LocalServiceKind::Browser).unwrap();

                    gateway
                        .execute(
                            binding,
                            &scope,
                            LocalServiceOperation::BrowserNavigate {
                                url: "https://example.test/restrictive".into(),
                            },
                        )
                        .await
                        .unwrap();
                    assert!(
                        gateway
                            .execute(
                                binding,
                                &scope,
                                LocalServiceOperation::BrowserAct {
                                    action: "click".into(),
                                    target: Some("@e1".into()),
                                    value: None,
                                },
                            )
                            .await
                            .is_err()
                    );
                    gateway.release(&bundle).await;
                });
            },
        );
    }
}

/// Explicit service-owned selection. This does not read or reorder agent env settings.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "backend", rename_all = "snake_case", deny_unknown_fields)]
pub enum ServiceMemoryConfiguration {
    Json {
        root: std::path::PathBuf,
    },
    Anthropic {
        root: std::path::PathBuf,
    },
    Redis {
        url: String,
        namespace: String,
    },
    Vector {
        url: String,
        tenant_id: String,
        agent_id: String,
    },
}

/// Open a configured existing backend. Vector writes retain the selected real
/// embedding client; database schema is validated and never replaced here.
pub async fn selected_memory_backend(
    configuration: &ServiceMemoryConfiguration,
    embeddings: Option<Arc<dyn crate::llm::LlmClient>>,
) -> Result<(Arc<dyn LocalServiceBackend>, Vec<u8>), String> {
    use crate::memory_store::{LongTermMemory, PersistentMemoryStore, VectorRepository};
    let selected: Arc<dyn LongTermMemory> = match configuration {
        ServiceMemoryConfiguration::Json { root } => {
            if !root.is_absolute() {
                return Err("selected JSON memory root must be absolute".into());
            }
            tokio::fs::create_dir_all(root)
                .await
                .map_err(|_| "selected JSON memory is unavailable")?;
            Arc::new(crate::json_store::NamespaceJsonStore::new(root))
        }
        ServiceMemoryConfiguration::Anthropic { root } => {
            if !root.is_absolute() {
                return Err("selected Anthropic memory root must be absolute".into());
            }
            Arc::new(
                crate::memory_store::Anthropic3TierMemoryStore::new(root)
                    .map_err(|_| "selected Anthropic memory is unavailable")?,
            )
        }
        ServiceMemoryConfiguration::Redis { url, namespace } => {
            if namespace.trim().is_empty() {
                return Err("selected Redis namespace is required".into());
            }
            let store = crate::memory_store::RedisMemoryStore::new(url, namespace)
                .map_err(|_| "invalid selected Redis memory")?;
            // Verify availability without creating or changing any stored data.
            store
                .retrieve_scoped("service-bootstrap", "", 1)
                .await
                .map_err(|_| "selected Redis memory is unavailable")?;
            Arc::new(store)
        }
        ServiceMemoryConfiguration::Vector {
            url,
            tenant_id,
            agent_id,
        } => {
            if tenant_id.trim().is_empty() || agent_id.trim().is_empty() {
                return Err("selected vector memory requires tenant and agent identity".into());
            }
            let llm = embeddings
                .ok_or("selected vector memory requires the configured embedding client")?;
            let repo = if url.starts_with("sqlite:") {
                use std::str::FromStr;
                if url.contains(":memory:") {
                    return Err("selected vector database must be persistent".into());
                }
                let options = sqlx::sqlite::SqliteConnectOptions::from_str(url)
                    .map_err(|_| "invalid selected vector database")?
                    .create_if_missing(false);
                let pool = sqlx::sqlite::SqlitePoolOptions::new()
                    .max_connections(4)
                    .connect_with(options)
                    .await
                    .map_err(|_| "selected vector database is unavailable")?;
                sqlx::query("SELECT tenant_id,content,metadata FROM consolidated_memory LIMIT 0")
                    .execute(&pool)
                    .await
                    .map_err(|_| "selected vector database schema is unavailable")?;
                VectorRepository::new_sqlite(pool)
            } else if url.starts_with("postgres:") || url.starts_with("postgresql:") {
                let pool = sqlx::postgres::PgPoolOptions::new()
                    .max_connections(4)
                    .connect(url)
                    .await
                    .map_err(|_| "selected vector database is unavailable")?;
                sqlx::query("SELECT tenant_id,content,metadata FROM consolidated_memory LIMIT 0")
                    .execute(&pool)
                    .await
                    .map_err(|_| "selected vector database schema is unavailable")?;
                VectorRepository::new(pool)
            } else {
                return Err("invalid selected vector database".into());
            };
            Arc::new(PersistentMemoryStore {
                repo: Arc::new(repo),
                tenant_id: tenant_id.clone(),
                agent_id: agent_id.clone(),
                llm,
            })
        }
    };
    let identity = Sha256::digest(selected.service_configuration_identity().as_bytes()).to_vec();
    Ok((Arc::new(SelectedMemoryBackend::new(selected)), identity))
}
