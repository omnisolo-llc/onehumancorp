use server_harness::middleware::local_services::{
    LocalServiceKind, LocalServiceRegistry, LocalServiceScopeContext,
};
use uuid::Uuid;

#[test]
fn default_bundle_contains_all_shared_local_service_kinds_without_authority() {
    let registry = LocalServiceRegistry::with_defaults();
    let context = LocalServiceScopeContext::for_attempt(
        "tenant-a",
        Some("project-a"),
        Some("workspace-a"),
        Uuid::from_u128(1),
        Some(Uuid::from_u128(2)),
        Some(Uuid::from_u128(3)),
    );
    let bundle = registry.resolve(context).unwrap();

    assert_eq!(bundle.bindings.len(), 8);
    for kind in [
        LocalServiceKind::Memory,
        LocalServiceKind::Mcp,
        LocalServiceKind::Workspace,
        LocalServiceKind::Artifact,
        LocalServiceKind::Browser,
        LocalServiceKind::Cache,
        LocalServiceKind::Integration,
        LocalServiceKind::ProviderFacade,
    ] {
        assert!(bundle.binding(kind).is_some(), "missing service kind {kind:?}");
    }

    let encoded = serde_json::to_string(&bundle).unwrap().to_ascii_lowercase();
    for forbidden in ["token", "secret", "authorization", "password", "cookie"] {
        assert!(!encoded.contains(forbidden), "portable bundle contains {forbidden}");
    }
}

#[test]
fn local_service_authorization_is_tenant_and_capability_scoped() {
    let registry = LocalServiceRegistry::with_defaults();
    let context = LocalServiceScopeContext::for_attempt(
        "tenant-a",
        Some("project-a"),
        Some("workspace-a"),
        Uuid::from_u128(1),
        Some(Uuid::from_u128(2)),
        Some(Uuid::from_u128(3)),
    );
    let bundle = registry.resolve(context.clone()).unwrap();
    let memory = bundle.binding(LocalServiceKind::Memory).unwrap();

    registry.authorize(memory, &context, "memory.read").unwrap();

    let other_tenant = LocalServiceScopeContext::for_attempt(
        "tenant-b",
        Some("project-a"),
        Some("workspace-a"),
        context.session_id,
        context.task_id,
        context.attempt_id,
    );
    assert!(registry.authorize(memory, &other_tenant, "memory.read").is_err());
    assert!(registry
        .authorize(memory, &context, "browser.navigate")
        .is_err());
}

#[test]
fn workspace_memory_and_service_bindings_are_shared_across_harnesses() {
    let registry = LocalServiceRegistry::with_defaults();
    let context = LocalServiceScopeContext::for_attempt(
        "tenant-shared",
        Some("project-shared"),
        Some("workspace-shared"),
        Uuid::from_u128(11),
        Some(Uuid::from_u128(12)),
        Some(Uuid::from_u128(13)),
    );
    let harness_a = registry.resolve(context.clone()).unwrap();
    let harness_b = registry.resolve(context.clone()).unwrap();

    for kind in [
        LocalServiceKind::Memory,
        LocalServiceKind::Mcp,
        LocalServiceKind::Workspace,
        LocalServiceKind::Artifact,
        LocalServiceKind::Browser,
        LocalServiceKind::Cache,
        LocalServiceKind::Integration,
        LocalServiceKind::ProviderFacade,
    ] {
        let first = harness_a.binding(kind).unwrap();
        let second = harness_b.binding(kind).unwrap();
        assert_ne!(first.binding_id, second.binding_id);
        assert_eq!(first.service_id, second.service_id);
        assert_eq!(first.scope, second.scope);
        assert_eq!(first.configuration_digest, second.configuration_digest);
        registry
            .validate(&harness_a, &context)
            .expect("harness A binding must remain valid");
        registry
            .validate(&harness_b, &context)
            .expect("harness B binding must remain valid");
    }

    let memory = harness_b.binding(LocalServiceKind::Memory).unwrap();
    assert_eq!(memory.service_id, "omnisolo.memory");
    assert_eq!(memory.scope, server_harness::middleware::local_services::LocalServiceScope::Workspace);
    registry.authorize(memory, &context, "memory.write").unwrap();
}
