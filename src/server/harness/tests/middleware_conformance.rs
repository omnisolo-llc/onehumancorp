use std::collections::BTreeSet;

use serde_json::Value;
use uuid::Uuid;

use server_harness::middleware::adapter::{
    AdapterError, OmniSoloEvent, OmniSoloHarnessAdapter, OmniSoloRunConfig,
};
use server_harness::middleware::capsule::{
    HandoffCoordinator, HandoffOperation, HandoffScope, LossEntry, LossReport, LossSeverity,
};
use server_harness::middleware::inference::{
    InferenceGateway, InferenceRequest, RuntimeWorker, WorkerState,
};
use server_harness::middleware::lease::FenceError;
use server_harness::middleware::local_services::{
    LocalServiceKind, LocalServiceRegistry, LocalServiceScopeContext,
};
use server_harness::middleware::types::{
    BindingAccessMode, BindingScope, BindingState, ModelDescriptor, ModelProvider,
    ModelRuntimeDescriptor, ModelRuntimeKind, RuntimeAutoscaling, RuntimeCapacity,
    RuntimePlacement, ServingEngine, SessionBinding,
};

fn fixture(name: &str) -> Value {
    let raw = match name {
        "session_task_run" => {
            include_str!("fixtures/harness_middleware/session_task_run.json")
        }
        "capsule_transfer" => {
            include_str!("fixtures/harness_middleware/capsule_transfer.json")
        }
        "model_runtime_oss" => {
            include_str!("fixtures/harness_middleware/model_runtime_oss.json")
        }
        "failure_injection" => {
            include_str!("fixtures/harness_middleware/failure_injection.json")
        }
        "handoff_loss" => include_str!("fixtures/harness_middleware/handoff_loss.json"),
        other => panic!("unknown fixture {other}"),
    };
    serde_json::from_str(raw).expect("fixture JSON")
}

#[test]
fn session_task_fixture_covers_durable_replay_and_transient_delivery() {
    let fixture = fixture("session_task_run");
    let mut run = OmniSoloHarnessAdapter::start(
        OmniSoloRunConfig::new(
            fixture["tenant_id"].as_str().unwrap(),
            fixture["objective"].as_str().unwrap(),
        )
        .with_turn(),
    )
    .expect("run start");

    run.record(OmniSoloEvent::TextChunk {
        content: "streamed but replay-optional".to_owned(),
    })
    .unwrap();
    run.record(OmniSoloEvent::ToolCall {
        name: "read_file".to_owned(),
        args_json: "{\"path\":\"README.md\"}".to_owned(),
        result: "contents".to_owned(),
        iteration: 1,
    })
    .unwrap();
    run.record(OmniSoloEvent::TaskComplete {
        content: "finished".to_owned(),
    })
    .unwrap();

    let durable_types = run
        .replay(1, 10)
        .into_iter()
        .map(|event| event.event_type)
        .collect::<Vec<_>>();
    let expected = fixture["durable_event_types"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(durable_types, expected);
    assert_eq!(run.events().len(), 4);
    assert_eq!(run.events()[1].durable_sequence, None);
    assert!(fixture["attempt_may_omit_turn"].as_bool().unwrap());
}

#[test]
fn capsule_fixture_transfers_content_and_redacts_secret_like_effects() {
    let fixture = fixture("capsule_transfer");
    let mut run = OmniSoloHarnessAdapter::start(
        OmniSoloRunConfig::new("tenant-fixture", "export a portable session").with_turn(),
    )
    .unwrap();
    run.record(OmniSoloEvent::ToolCall {
        name: "configure".to_owned(),
        args_json: "{\"password=\":\"secret\"}".to_owned(),
        result: "api_key=super-secret".to_owned(),
        iteration: 1,
    })
    .unwrap();
    run.record(OmniSoloEvent::TaskComplete {
        content: "portable result".to_owned(),
    })
    .unwrap();

    let target = fixture["target_harness_id"].as_str().unwrap();
    let capsule = run.export_capsule(target, Uuid::new_v4()).unwrap();
    let encoded = serde_json::to_string(&capsule).unwrap();
    assert!(!encoded.contains("super-secret"));
    assert!(
        capsule
            .loss_report
            .entries
            .iter()
            .any(|entry| entry.reason.contains("secret-like"))
    );
    assert!(fixture["native_records_are_excluded"].as_bool().unwrap());
    assert!(capsule.verify_integrity().is_ok());

    let imported =
        OmniSoloHarnessAdapter::import_capsule(&capsule, "tenant-fixture", target).expect("import");
    assert_eq!(imported.records.len(), capsule.records.len());
}

#[test]
fn stale_worker_fixture_rejects_old_fence_after_reassignment() {
    let fixture = fixture("failure_injection");
    assert!(
        fixture["cases"]
            .as_array()
            .unwrap()
            .iter()
            .any(|case| case == "stale_worker_after_reassignment")
    );

    let mut run =
        OmniSoloHarnessAdapter::start(OmniSoloRunConfig::new("tenant-1", "fenced run")).unwrap();
    let stale = run.fence_token();
    run.reassign_worker("worker-new").unwrap();
    assert!(matches!(
        run.record_with_fence(
            OmniSoloEvent::TextChunk {
                content: "late".to_owned(),
            },
            stale,
        ),
        Err(AdapterError::EventStore(
            server_harness::middleware::events::EventStoreError::Fence(FenceError::StaleGeneration)
        ))
    ));
}

#[test]
fn open_model_fixture_matches_capabilities_and_reconciles_uncertain_admission() {
    let fixture = fixture("model_runtime_oss");
    let model = ModelDescriptor {
        model_id: fixture["model_id"].as_str().unwrap().to_owned(),
        provider: ModelProvider::SelfHosted,
        revision: Some("rev-1".to_owned()),
        capabilities: fixture["required_capabilities"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap().to_owned())
            .collect(),
        ..Default::default()
    };
    let runtime = ModelRuntimeDescriptor {
        runtime_id: "runtime-k8s-vllm".to_owned(),
        model: model.clone(),
        kind: ModelRuntimeKind::KubernetesService,
        engine: Some(ServingEngine::Vllm),
        placement: Some(RuntimePlacement {
            cluster: Some(fixture["placement"]["cluster"].as_str().unwrap().to_owned()),
            namespace: Some(
                fixture["placement"]["namespace"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
            ),
            ..Default::default()
        }),
        capacity: RuntimeCapacity {
            max_concurrent_requests: 2,
            ..Default::default()
        },
        autoscaling: Some(RuntimeAutoscaling {
            min_replicas: fixture["autoscaling"]["min_replicas"].as_u64().unwrap() as u32,
            max_replicas: fixture["autoscaling"]["max_replicas"].as_u64().unwrap() as u32,
            scale_to_zero: fixture["autoscaling"]["scale_to_zero"].as_bool().unwrap(),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(runtime.kind, ModelRuntimeKind::KubernetesService);
    assert_eq!(runtime.engine, Some(ServingEngine::Vllm));

    let mut gateway = InferenceGateway::new();
    gateway
        .register_worker(RuntimeWorker {
            worker_id: "worker-model".to_owned(),
            tenant_id: Some("tenant-oss".to_owned()),
            runtime_id: runtime.runtime_id,
            model_revisions: BTreeSet::from(["rev-1".to_owned()]),
            capabilities: model.capabilities.clone(),
            max_context_tokens: 16_384,
            capacity_slots: 1,
            leased_slots: 0,
            state: WorkerState::Ready,
        })
        .unwrap();
    let request = InferenceRequest::new(
        Uuid::new_v4(),
        Uuid::new_v4(),
        model,
        "request-digest",
        BTreeSet::from(["tool_calling".to_owned(), "json_schema".to_owned()]),
        100,
        true,
    );
    let request_id = request.request_id;
    gateway.submit(request).unwrap();
    let admission = gateway.admit(request_id).unwrap();
    gateway
        .recover_after_restart(admission.admission_id)
        .unwrap();
    gateway
        .retry_uncertain(admission.admission_id, true)
        .unwrap();
    assert_eq!(
        gateway.admission(admission.admission_id).unwrap().state,
        server_harness::middleware::inference::InferenceState::Admitted
    );
}

#[test]
fn handoff_fixture_requires_loss_ack_and_fences_source_before_target() {
    let fixture = fixture("handoff_loss");
    assert!(fixture["requires_report_digest_ack"].as_bool().unwrap());
    assert!(
        fixture["source_binding_must_be_fenced_before_target"]
            .as_bool()
            .unwrap()
    );

    let session_id = Uuid::new_v4();
    let task_id = Uuid::new_v4();
    let source_id = Uuid::new_v4();
    let now = chrono::Utc::now();
    let binding = |binding_id: Uuid, harness_id: &str, state: BindingState| SessionBinding {
        binding_id,
        session_id,
        task_id: Some(task_id),
        harness_id: harness_id.to_owned(),
        scope: BindingScope::Task,
        owner_id: Uuid::new_v4(),
        workspace_mutation_scope_id: "workspace-1".to_owned(),
        access_mode: BindingAccessMode::ReadWrite,
        native_session_id: None,
        state,
        generation: 1,
        created_at: now,
        last_used_at: None,
        capability_snapshot_id: None,
        adapter_config_digest: None,
        last_imported_native_cursor: None,
        last_exported_durable_sequence: None,
        native_checkpoint_ref: None,
        worker_pool: None,
        state_locality: None,
        exact_resume_eligible: false,
        invalidation_reason: None,
        native_record_digest: None,
        extensions: Default::default(),
    };
    let mut coordinator = HandoffCoordinator::new();
    coordinator
        .register_binding(binding(source_id, "omnisolo", BindingState::Active))
        .unwrap();
    let report = LossReport::new(vec![LossEntry {
        source_path: "native.checkpoint".to_owned(),
        reason: "target cannot restore native checkpoint".to_owned(),
        severity: LossSeverity::Required,
        target_representation: Some("historical_data".to_owned()),
        acknowledgement_required: true,
        capability: None,
    }]);
    let operation_id = Uuid::new_v4();
    coordinator
        .begin(HandoffOperation::new(
            operation_id,
            session_id,
            Some(task_id),
            source_id,
            "future-harness".to_owned(),
            HandoffScope::Task { task_id },
            report.clone(),
        ))
        .unwrap();
    for (version, state) in [
        (
            0,
            server_harness::middleware::lifecycle::HandoffState::Fencing,
        ),
        (
            1,
            server_harness::middleware::lifecycle::HandoffState::Quiescing,
        ),
        (
            2,
            server_harness::middleware::lifecycle::HandoffState::Snapshotting,
        ),
        (
            3,
            server_harness::middleware::lifecycle::HandoffState::Compiling,
        ),
        (
            4,
            server_harness::middleware::lifecycle::HandoffState::AwaitingLossAck,
        ),
    ] {
        coordinator.advance(operation_id, version, state).unwrap();
    }
    assert!(matches!(
        coordinator.advance(
            operation_id,
            5,
            server_harness::middleware::lifecycle::HandoffState::TargetCreating,
        ),
        Err(server_harness::middleware::capsule::HandoffError::LossAcknowledgementRequired)
    ));
    coordinator
        .acknowledge_loss(operation_id, report.digest())
        .unwrap();
    coordinator
        .advance(
            operation_id,
            5,
            server_harness::middleware::lifecycle::HandoffState::TargetCreating,
        )
        .unwrap();
    let target_id = Uuid::new_v4();
    coordinator
        .install_target(
            operation_id,
            binding(target_id, "future-harness", BindingState::Inactive),
        )
        .unwrap();
    coordinator
        .advance(
            operation_id,
            6,
            server_harness::middleware::lifecycle::HandoffState::Activating,
        )
        .unwrap();
    coordinator.activate_target(operation_id).unwrap();
    coordinator
        .advance(
            operation_id,
            7,
            server_harness::middleware::lifecycle::HandoffState::Completed,
        )
        .unwrap();
    assert_eq!(
        coordinator.binding(source_id).unwrap().state,
        BindingState::Fenced
    );
    assert_eq!(
        coordinator.binding(target_id).unwrap().state,
        BindingState::Active
    );
}

#[test]
fn shared_service_bindings_remain_visible_across_harnesses_without_cross_workspace_authority() {
    let registry = LocalServiceRegistry::with_defaults();
    let session_id = Uuid::new_v4();
    let task_id = Uuid::new_v4();
    let attempt_id = Uuid::new_v4();
    let context = LocalServiceScopeContext::for_attempt(
        "tenant-shared-services",
        Some("project-shared"),
        Some("workspace-shared"),
        session_id,
        Some(task_id),
        Some(attempt_id),
    );

    let first_harness = registry.resolve(context.clone()).unwrap();
    let second_harness = registry.resolve(context.clone()).unwrap();
    registry.validate(&first_harness, &context).unwrap();
    registry.validate(&second_harness, &context).unwrap();
    assert_eq!(first_harness.bindings.len(), 8);
    assert_eq!(second_harness.bindings.len(), 8);

    for kind in [
        LocalServiceKind::Memory,
        LocalServiceKind::Workspace,
        LocalServiceKind::Artifact,
        LocalServiceKind::Mcp,
        LocalServiceKind::Browser,
        LocalServiceKind::Cache,
        LocalServiceKind::Integration,
        LocalServiceKind::ProviderFacade,
    ] {
        let first = first_harness.binding(kind).unwrap();
        let second = second_harness.binding(kind).unwrap();
        assert_ne!(first.binding_id, second.binding_id);
        assert_eq!(first.service_id, second.service_id);
        assert_eq!(first.scope, second.scope);
        assert_eq!(first.configuration_digest, second.configuration_digest);
        assert_eq!(first.tenant_id, "tenant-shared-services");
        assert_eq!(first.workspace_id.as_deref(), Some("workspace-shared"));
    }

    registry
        .authorize(
            first_harness.binding(LocalServiceKind::Memory).unwrap(),
            &context,
            "memory.write",
        )
        .unwrap();
    registry
        .authorize(
            second_harness.binding(LocalServiceKind::Artifact).unwrap(),
            &context,
            "artifact.read",
        )
        .unwrap();

    let mut other_workspace = context;
    other_workspace.workspace_id = Some("workspace-other".to_owned());
    assert!(registry
        .validate(&first_harness, &other_workspace)
        .is_err());
    let encoded = serde_json::to_string(&first_harness).unwrap();
    assert!(!encoded.contains("authority"));
    assert!(!encoded.contains("secret"));
}
