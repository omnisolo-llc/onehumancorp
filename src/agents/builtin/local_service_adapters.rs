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

/// Workspace, artifact and cache bytes use the configured OHCMemory provider.
/// Keys are content-independent opaque identifiers, never caller-controlled paths.
pub struct ExistingBlobBackend {
    selected: Arc<dyn crate::memory_store::OHCMemory>,
    kind: LocalServiceKind,
}
impl ExistingBlobBackend {
    pub fn new(selected: Arc<dyn crate::memory_store::OHCMemory>, kind: LocalServiceKind) -> Self {
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
            let mut run = crate::agent::AgentRunConfig::default();
            run.long_term_memory = Some(Arc::new(crate::json_store::NamespaceJsonStore::new(
                memory_root,
            )));
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
    blobs: Option<Arc<dyn crate::memory_store::OHCMemory>>,
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
            parameters: json!({"type":"object","properties":{"operation":{"type":"string"},"content":{},"key":{"type":"string"},"query":{"type":"string"},"limit":{"type":"integer"},"tool":{"type":"string"},"arguments":{"type":"object"},"action":{"type":"string"},"url":{"type":"string"}},"required":["operation"]}),
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

/// Browser navigation uses the configured Playwright Screenshot executor. Each
/// issued binding owns its own captured page; cookies and process handles never
/// cross the gateway. The existing sandbox runner owns the browser process.
pub struct ScreenshotBrowserBackend {
    screenshot: crate::tools::Tool,
    policy: crate::agent::AgentRunConfig,
    root: std::path::PathBuf,
    tenant_id: String,
    project_id: Option<String>,
    pages: tokio::sync::Mutex<std::collections::BTreeMap<uuid::Uuid, Vec<u8>>>,
    operation_lock: tokio::sync::Mutex<()>,
}
impl ScreenshotBrowserBackend {
    pub fn new(
        screenshot: crate::tools::Tool,
        root: std::path::PathBuf,
        scope: &server_harness::middleware::local_services::LocalServiceScopeContext,
        policy: &crate::agent::AgentRunConfig,
    ) -> Self {
        Self {
            screenshot,
            policy: policy.clone(),
            root,
            tenant_id: scope.tenant_id.clone(),
            project_id: scope.project_id.clone(),
            pages: tokio::sync::Mutex::new(std::collections::BTreeMap::new()),
            operation_lock: tokio::sync::Mutex::new(()),
        }
    }
}
#[async_trait]
impl LocalServiceBackend for ScreenshotBrowserBackend {
    fn capabilities(&self) -> BTreeSet<String> {
        if self
            .policy
            .allowed_tools
            .as_ref()
            .is_some_and(|tools| !tools.contains(&self.screenshot.name))
        {
            return BTreeSet::new();
        }
        ["browser.navigate", "browser.snapshot"]
            .into_iter()
            .map(str::to_owned)
            .collect()
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
                let parsed = reqwest::Url::parse(&url)
                    .map_err(|_| LocalServiceGatewayError::InvalidOperation)?;
                if !matches!(parsed.scheme(), "http" | "https")
                    || !parsed.username().is_empty()
                    || parsed.password().is_some()
                {
                    return Err(LocalServiceGatewayError::InvalidOperation);
                }
                let file = format!("omnisolo-browser-{lease}.png");
                let call = crate::types::ToolCall {
                    id: format!("browser-{lease}"),
                    name: self.screenshot.name.clone(),
                    arguments: json!({"url":url,"path":file}),
                };
                crate::tools_gating::ToolGater::check_gating(&call,self.screenshot.is_read_only,&self.policy)
                    .map_err(|_|LocalServiceGatewayError::Authorization(server_harness::middleware::local_services::LocalServiceError::CapabilityDenied("browser tool policy rejected".to_owned())))?;
                self.screenshot
                    .execute
                    .execute(json!({"url":url,"path":file}))
                    .await
                    .map_err(|_| LocalServiceGatewayError::BackendFailure)?;
                let path = self.root.join(&file);
                let content = tokio::fs::read(&path)
                    .await
                    .map_err(|_| LocalServiceGatewayError::BackendFailure)?;
                let _ = tokio::fs::remove_file(path).await;
                if content.len() > 16 * 1024 * 1024 {
                    return Err(LocalServiceGatewayError::BackendFailure);
                }
                let digest = format!("sha256:{:x}", Sha256::digest(&content));
                self.pages.lock().await.insert(lease, content);
                Ok(json!({"snapshot_digest":digest}))
            }
            LocalServiceOperation::BrowserSnapshot => {
                let pages = self.pages.lock().await;
                let content = pages
                    .get(&lease)
                    .ok_or(LocalServiceGatewayError::InvalidOperation)?;
                Ok(
                    json!({"content":content,"content_type":"image/png","digest":format!("sha256:{:x}",Sha256::digest(content))}),
                )
            }
            _ => Err(LocalServiceGatewayError::InvalidOperation),
        }
    }
    async fn release(&self, namespace: &ServiceNamespace) {
        let _operation = self.operation_lock.lock().await;
        if let Some(lease) = namespace.lease_id {
            self.pages.lock().await.remove(&lease);
            let _ = tokio::fs::remove_file(self.root.join(format!("omnisolo-browser-{lease}.png")))
                .await;
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
    use std::os::unix::process::ExitStatusExt;
    struct CaptureRunner;
    #[async_trait]
    impl crate::tools::runner::CommandRunner for CaptureRunner {
        async fn run(
            &self,
            program: &str,
            args: &[&str],
            root: Option<&std::path::Path>,
            _: Vec<(String, String)>,
        ) -> std::io::Result<std::process::Output> {
            assert_eq!(program, "npx");
            assert_eq!(&args[..2], &["playwright", "screenshot"]);
            tokio::fs::write(root.unwrap().join(args[3]), args[2].as_bytes()).await?;
            Ok(std::process::Output {
                status: std::process::ExitStatus::from_raw(0),
                stdout: vec![],
                stderr: vec![],
            })
        }
    }
    #[tokio::test]
    async fn configured_browser_executor_keeps_snapshots_in_distinct_service_leases() {
        let root = std::env::temp_dir().join(format!("browser-service-{}", uuid::Uuid::new_v4()));
        tokio::fs::create_dir_all(&root).await.unwrap();
        let scope = LocalServiceScopeContext::for_attempt(
            "tenant",
            Some("project"),
            Some("workspace"),
            uuid::Uuid::new_v4(),
            Some(uuid::Uuid::new_v4()),
            Some(uuid::Uuid::new_v4()),
        );
        let mut policy = crate::agent::AgentRunConfig::default();
        policy.hil_spectrum = crate::types::HumanInLoopSpectrum::Autonomous;
        policy.permission_architecture = crate::types::PermissionArchitecture::Permissive;
        let tool =
            crate::tools::screenshot::screenshot_tool(Some(root.clone()), Arc::new(CaptureRunner));
        let mut excluded_policy = policy.clone();
        excluded_policy.allowed_tools = Some(vec!["Read".into()]);
        assert!(
            ScreenshotBrowserBackend::new(tool.clone(), root.clone(), &scope, &excluded_policy)
                .capabilities()
                .is_empty()
        );
        let mut gateway = LocalServiceGateway::new(LocalServiceRegistry::with_defaults());
        gateway.register(
            LocalServiceKind::Browser,
            Arc::new(ScreenshotBrowserBackend::new(
                tool,
                root.clone(),
                &scope,
                &policy,
            )),
        );
        let first = gateway.resolve(scope.clone()).unwrap();
        let second = gateway.resolve(scope.clone()).unwrap();
        gateway
            .execute(
                &first.bindings[0],
                &scope,
                LocalServiceOperation::BrowserNavigate {
                    url: "https://example.test/first".into(),
                },
            )
            .await
            .unwrap();
        let snapshot = gateway
            .execute(
                &first.bindings[0],
                &scope,
                LocalServiceOperation::BrowserSnapshot,
            )
            .await
            .unwrap();
        assert_eq!(
            snapshot["content"],
            json!(b"https://example.test/first".to_vec())
        );
        assert!(
            gateway
                .execute(
                    &second.bindings[0],
                    &scope,
                    LocalServiceOperation::BrowserSnapshot
                )
                .await
                .is_err()
        );
        gateway.release(&first).await;
        assert!(
            gateway
                .execute(
                    &first.bindings[0],
                    &scope,
                    LocalServiceOperation::BrowserSnapshot
                )
                .await
                .is_err()
        );
        assert!(
            tokio::fs::read_dir(&root)
                .await
                .unwrap()
                .next_entry()
                .await
                .unwrap()
                .is_none()
        );
        tokio::fs::remove_dir_all(root).await.unwrap();
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
