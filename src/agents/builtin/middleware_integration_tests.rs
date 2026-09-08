use crate::agent::AgentEvent;
use crate::middleware::record_agent_event;
use server_harness::middleware::adapter::{OmniSoloHarnessAdapter, OmniSoloRunConfig};

#[test]
fn omnisolo_run_preserves_legacy_projection_while_recording_canonical_data() {
    let mut adapter = OmniSoloHarnessAdapter::start(
        OmniSoloRunConfig::new("tenant-1", "inspect the workspace")
            .with_worker("omnisolo-worker")
            .with_turn(),
    )
    .expect("adapter run");

    let tool_event = record_agent_event(
        &mut adapter,
        &AgentEvent::ToolCall {
            name: "read_file".into(),
            args_json: "{\"path\":\"README.md\"}".into(),
            result: "contents".into(),
            iteration: 1,
        },
    )
    .expect("tool projection");
    let complete_event = record_agent_event(
        &mut adapter,
        &AgentEvent::TaskComplete {
            content: "finished".into(),
        },
    )
    .expect("completion projection");

    assert_eq!(
        tool_event.r#type,
        crate::proto::agent_service::EventType::ToolCall as i32
    );
    assert_eq!(tool_event.tool_name, "read_file");
    assert_eq!(tool_event.task_id, adapter.task().task_id.to_string());
    assert_eq!(tool_event.middleware_event_type, "tool.call_settled");
    assert_eq!(
        complete_event.r#type,
        crate::proto::agent_service::EventType::TaskComplete as i32
    );
    assert_eq!(
        adapter.task().state,
        server_harness::middleware::types::TaskState::Completed
    );
    assert_eq!(
        adapter.attempt().state,
        server_harness::middleware::types::AttemptState::Succeeded
    );
    assert_eq!(adapter.replay(1, 10).len(), 3);
}

#[test]
fn task_level_attempts_remain_valid_for_non_interactive_omnisolo_work() {
    let adapter = OmniSoloHarnessAdapter::start(
        OmniSoloRunConfig::new("tenant-1", "background indexing").without_turn(),
    )
    .expect("adapter run");

    assert!(adapter.turn().is_none());
    assert!(adapter.attempt().turn_id.is_none());
    assert_eq!(
        adapter.task().kind,
        server_harness::middleware::types::TaskKind::UserObjective
    );
}
