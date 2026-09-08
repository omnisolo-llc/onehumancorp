use server_harness::middleware::local_service_gateway::*;
use server_harness::middleware::local_services::*;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::test]
async fn configured_sqlite_memory_is_shared_across_native_harness_bindings() {
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
    let registry = LocalServiceRegistry::with_defaults();
    let mut gateway = LocalServiceGateway::new(registry.clone());
    gateway.register(
        LocalServiceKind::Memory,
        Arc::new(SqliteAgentMemoryBackend::new(pool.clone())),
    );
    let scope = LocalServiceScopeContext::for_attempt(
        "tenant",
        Some("project"),
        Some("workspace"),
        Uuid::new_v4(),
        Some(Uuid::new_v4()),
        Some(Uuid::new_v4()),
    );
    let writer = registry.resolve(scope.clone()).unwrap();
    gateway
        .execute(
            writer.binding(LocalServiceKind::Memory).unwrap(),
            &scope,
            LocalServiceOperation::MemoryWrite {
                content: "shared-sentinel".into(),
            },
        )
        .await
        .unwrap();
    // Every native adapter uses a separately issued lease over the configured store.
    for harness in [
        "omnisolo",
        "codex",
        "opencode",
        "deepseek",
        "pi",
        "kimi",
        "openhands",
        "openharness",
    ] {
        let reader = registry.resolve(scope.clone()).unwrap();
        let result = gateway
            .execute(
                reader.binding(LocalServiceKind::Memory).unwrap(),
                &scope,
                LocalServiceOperation::MemorySearch {
                    query: "sentinel".into(),
                    limit: 10,
                },
            )
            .await
            .unwrap();
        assert_eq!(
            result,
            serde_json::json!({"items":["shared-sentinel"]}),
            "{harness}"
        );
    }
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM agent_memory")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count.0, 1, "the existing selected database owns the write");
    let mut other = scope.clone();
    other.workspace_id = Some("other-workspace".into());
    let foreign = registry.resolve(other.clone()).unwrap();
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
                writer.binding(LocalServiceKind::Memory).unwrap(),
                &other,
                LocalServiceOperation::MemorySearch {
                    query: "sentinel".into(),
                    limit: 10
                }
            )
            .await
            .is_err()
    );
    assert!(
        gateway
            .execute(
                writer.binding(LocalServiceKind::Memory).unwrap(),
                &scope,
                LocalServiceOperation::BrowserSnapshot
            )
            .await
            .is_err()
    );
    registry.revoke_attempt(&scope.tenant_id, scope.attempt_id.unwrap());
    assert!(
        gateway
            .execute(
                writer.binding(LocalServiceKind::Memory).unwrap(),
                &scope,
                LocalServiceOperation::MemorySearch {
                    query: "sentinel".into(),
                    limit: 10
                }
            )
            .await
            .is_err()
    );
}

#[tokio::test]
async fn gateway_issues_only_available_capabilities_and_revokes_inflight_operations() {
    use async_trait::async_trait;
    use std::collections::BTreeSet;
    struct Browser {
        entered: tokio::sync::Notify,
        released: std::sync::atomic::AtomicBool,
    }
    #[async_trait]
    impl LocalServiceBackend for Browser {
        fn capabilities(&self) -> BTreeSet<String> {
            ["browser.snapshot".to_owned()].into_iter().collect()
        }
        async fn execute(
            &self,
            _: &ServiceNamespace,
            _: LocalServiceOperation,
        ) -> Result<serde_json::Value, LocalServiceGatewayError> {
            self.entered.notify_one();
            std::future::pending().await
        }
        async fn release(&self, ns: &ServiceNamespace) {
            assert!(ns.lease_id.is_some());
            self.released
                .store(true, std::sync::atomic::Ordering::SeqCst);
        }
    }
    let registry = LocalServiceRegistry::with_defaults();
    let browser = Arc::new(Browser {
        entered: tokio::sync::Notify::new(),
        released: std::sync::atomic::AtomicBool::new(false),
    });
    let mut gateway = LocalServiceGateway::new(registry.clone());
    gateway.register(LocalServiceKind::Browser, browser.clone());
    let scope = LocalServiceScopeContext::for_attempt(
        "tenant",
        None,
        None,
        Uuid::new_v4(),
        Some(Uuid::new_v4()),
        Some(Uuid::new_v4()),
    );
    let bundle = gateway.resolve(scope.clone()).unwrap();
    assert_eq!(bundle.bindings.len(), 1);
    assert_eq!(bundle.bindings[0].granted_capabilities.len(), 1);
    let other = gateway.resolve(scope.clone()).unwrap();
    assert_ne!(
        ServiceNamespace::from_binding(&bundle.bindings[0]).storage_key(),
        ServiceNamespace::from_binding(&other.bindings[0]).storage_key()
    );
    let task_gateway = gateway.clone();
    let binding = bundle.bindings[0].clone();
    let task = tokio::spawn(async move {
        task_gateway
            .execute(&binding, &scope, LocalServiceOperation::BrowserSnapshot)
            .await
    });
    browser.entered.notified().await;
    gateway.release(&bundle).await;
    assert!(
        tokio::time::timeout(std::time::Duration::from_secs(1), task)
            .await
            .unwrap()
            .unwrap()
            .is_err()
    );
    assert!(browser.released.load(std::sync::atomic::Ordering::SeqCst));
}

#[tokio::test]
async fn provider_service_calls_use_the_attempt_route_and_are_revoked() {
    use server_harness::middleware::provider_facade::ProviderFacade;
    use server_harness::middleware::types::{ModelApiDialect, ResolvedModelSelection};
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}/v1", listener.local_addr().unwrap());
    let app = axum::Router::new().route(
        "/v1/models",
        axum::routing::get(|headers: axum::http::HeaderMap| async move {
            assert_eq!(
                headers.get("authorization").unwrap(),
                "Bearer provider-secret"
            );
            axum::Json(serde_json::json!({"data":[{"id":"model"}]}))
        }),
    );
    let task = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    let facade = ProviderFacade::start(
        url,
        "provider-secret",
        ResolvedModelSelection {
            provider_route: "openai".into(),
            model_id: "model".into(),
            reasoning_effort: None,
            api_dialect: ModelApiDialect::OpenAiResponses,
            context_window: None,
            max_output_tokens: None,
            capabilities: Default::default(),
            binding_revision: "test".into(),
            binding_digest: "test".into(),
            metadata: Default::default(),
        },
    )
    .await
    .unwrap();
    let scope = LocalServiceScopeContext::for_attempt(
        "tenant",
        None,
        None,
        Uuid::new_v4(),
        Some(Uuid::new_v4()),
        Some(Uuid::new_v4()),
    );
    let backend = Arc::new(ProviderRouteBackend::default());
    backend.bind(
        &scope.tenant_id,
        scope.attempt_id.unwrap(),
        facade.route().clone(),
    );
    let mut gateway = LocalServiceGateway::new(LocalServiceRegistry::with_defaults());
    gateway.register(LocalServiceKind::ProviderFacade, backend);
    let bundle = gateway.resolve(scope.clone()).unwrap();
    assert_eq!(
        gateway
            .execute(
                &bundle.bindings[0],
                &scope,
                LocalServiceOperation::ProviderModels
            )
            .await
            .unwrap(),
        serde_json::json!({"data":[{"id":"model"}]})
    );
    gateway.release(&bundle).await;
    assert!(
        gateway
            .execute(
                &bundle.bindings[0],
                &scope,
                LocalServiceOperation::ProviderModels
            )
            .await
            .is_err()
    );
    facade.shutdown().await.unwrap();
    task.abort();
}
