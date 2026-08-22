use std::sync::{Arc, Mutex};
use std::time::Duration;

use server_harness::middleware::adapter::{
    OmniSoloEvent, OmniSoloHarnessAdapter, OmniSoloRunConfig,
};
use server_harness::middleware::capsule::{PortableRecord, SessionCapsule};
use server_lib::interop::protocol::{HarnessCapsuleOperation, InteropProtocol};
use server_lib::msgbus::{Bus, MemoryBus, Message};

fn test_capsule() -> SessionCapsule {
    let mut adapter = OmniSoloHarnessAdapter::start(
        OmniSoloRunConfig::new("tenant_interop", "transfer this task").with_turn(),
    )
    .unwrap();
    adapter
        .record(OmniSoloEvent::ToolCall {
            name: "read_file".to_owned(),
            args_json: r#"{"path":"README.md"}"#.to_owned(),
            result: "portable result".to_owned(),
            iteration: 1,
        })
        .unwrap();
    adapter
        .export_capsule("opencode", uuid::Uuid::new_v4())
        .unwrap()
}

#[tokio::test]
async fn typed_capsule_handoff_and_resume_preserve_session_task_and_content() {
    let bus = Arc::new(MemoryBus::new());
    let protocol = InteropProtocol::new(bus.clone(), bus.clone(), "interop-source".to_owned());
    let capsule = test_capsule();
    let received = Arc::new(Mutex::new(Vec::<HarnessCapsuleOperation>::new()));
    let received_clone = received.clone();

    let _cancel = protocol
        .listen_for_capsule_operations(Box::new(move |operation| {
            received_clone.lock().unwrap().push(operation);
        }))
        .await
        .unwrap();

    protocol
        .handoff_capsule(&capsule, 3, "session-fence-3")
        .await
        .unwrap();
    protocol
        .resume_capsule(&capsule, 4, "session-fence-4")
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;

    let operations = received.lock().unwrap();
    assert_eq!(operations.len(), 2);
    assert_eq!(operations[0].envelope.kind, "handoff");
    assert_eq!(operations[1].envelope.kind, "resume");
    for operation in operations.iter() {
        assert_eq!(operation.envelope.tenant_id, capsule.manifest.tenant_id);
        assert_eq!(
            operation.envelope.session_id,
            capsule.manifest.session_id.to_string()
        );
        assert_eq!(
            operation.envelope.operation_id,
            capsule.manifest.handoff_id.to_string()
        );
        assert_eq!(operation.envelope.task_id, portable_task_id(&capsule));
        assert_eq!(operation.capsule, capsule);
        assert!(
            operation
                .capsule
                .records
                .iter()
                .any(|record| matches!(record, PortableRecord::ToolResult(_)))
        );
    }
}

#[tokio::test]
async fn typed_capsule_transport_is_idempotent_and_drops_invalid_operations() {
    let bus = Arc::new(MemoryBus::new());
    let protocol = InteropProtocol::new(bus.clone(), bus.clone(), "interop-source".to_owned());
    let duplicate_protocol =
        InteropProtocol::new(bus.clone(), bus.clone(), "interop-target".to_owned());
    let capsule = test_capsule();
    let received = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let received_clone = received.clone();

    let _cancel = protocol
        .listen_for_capsule_operations(Box::new(move |_operation| {
            received_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }))
        .await
        .unwrap();

    protocol
        .handoff_capsule(&capsule, 5, "session-fence-5")
        .await
        .unwrap();
    duplicate_protocol
        .handoff_capsule(&capsule, 5, "session-fence-5")
        .await
        .unwrap();
    assert!(
        protocol
            .handoff_capsule(&capsule, 0, "session-fence-0")
            .await
            .unwrap_err()
            .contains("operation generation")
    );
    assert!(
        protocol
            .handoff_capsule(&capsule, 6, "")
            .await
            .unwrap_err()
            .contains("fencing token")
    );

    let mut tampered = capsule.clone();
    tampered.manifest.target_harness_id = "tampered".to_owned();
    assert!(
        protocol
            .handoff_capsule(&tampered, 6, "session-fence-6")
            .await
            .unwrap_err()
            .contains("integrity")
    );

    bus.publish(Message {
        topic: "system:harness_session_operation".to_owned(),
        payload: vec![255, 255, 255],
    })
    .await
    .unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_eq!(received.load(std::sync::atomic::Ordering::SeqCst), 1);
}

fn portable_task_id(capsule: &SessionCapsule) -> String {
    capsule
        .records
        .iter()
        .find_map(|record| match record {
            PortableRecord::Task(task) => Some(task.task_id.to_string()),
            _ => None,
        })
        .unwrap()
}
