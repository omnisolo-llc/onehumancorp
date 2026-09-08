use crate::agent::AgentEvent;
use crate::proto::agent_service::{EventType, RunTaskEvent};
use server_harness::middleware::adapter::{AdapterError, OmniSoloEvent, OmniSoloHarnessAdapter};
use server_harness::middleware::capsule::{PortableRecord, SessionCapsule};
use server_harness::middleware::events::AppendResult;
use server_harness::middleware::types::{ContentPart, EventEnvelope, MessageRole};

pub fn canonical_event(event: &AgentEvent) -> OmniSoloEvent {
    match event {
        AgentEvent::RunStarted { iteration } => OmniSoloEvent::RunStarted {
            iteration: *iteration,
        },
        AgentEvent::TextChunk { content } => OmniSoloEvent::TextChunk {
            content: content.clone(),
        },
        AgentEvent::ToolCall {
            name,
            args_json,
            result,
            iteration,
        } => OmniSoloEvent::ToolCall {
            name: name.clone(),
            args_json: args_json.clone(),
            result: result.clone(),
            iteration: *iteration,
        },
        AgentEvent::TaskComplete { content } => OmniSoloEvent::TaskComplete {
            content: content.clone(),
        },
        AgentEvent::TaskError { error } => OmniSoloEvent::TaskError {
            error: error.clone(),
        },
        AgentEvent::UserInterventionRequired { error } => OmniSoloEvent::UserInterventionRequired {
            error: error.clone(),
        },
        AgentEvent::IterationStarted {
            iteration,
            message_count,
        } => OmniSoloEvent::IterationStarted {
            iteration: *iteration,
            message_count: *message_count,
        },
        AgentEvent::CheckpointSaved { iteration, path } => OmniSoloEvent::CheckpointSaved {
            iteration: *iteration,
            path: path.clone(),
        },
        AgentEvent::Handoff { target_agent } => OmniSoloEvent::Handoff {
            target_agent: target_agent.clone(),
        },
        AgentEvent::RewindOccurred {
            iteration,
            checkpoint_id,
            reason,
        } => OmniSoloEvent::RewindOccurred {
            iteration: *iteration,
            checkpoint_id: checkpoint_id.clone(),
            reason: reason.clone(),
        },
        AgentEvent::GuardrailTripped { reason } => OmniSoloEvent::GuardrailTripped {
            reason: reason.clone(),
        },
        AgentEvent::CostUpdate { total_cost_usd } => OmniSoloEvent::CostUpdate {
            total_cost_usd: *total_cost_usd,
        },
    }
}

pub fn legacy_projection(event: &AgentEvent) -> RunTaskEvent {
    match event {
        AgentEvent::RunStarted { iteration } => RunTaskEvent {
            r#type: EventType::RunStarted as i32,
            iteration: *iteration,
            ..Default::default()
        },
        AgentEvent::IterationStarted {
            iteration,
            message_count,
        } => RunTaskEvent {
            r#type: EventType::IterationStarted as i32,
            iteration: *iteration,
            message_count: *message_count as i32,
            ..Default::default()
        },
        AgentEvent::CheckpointSaved { iteration, path } => RunTaskEvent {
            r#type: EventType::TextChunk as i32,
            content: format!(
                "[Checkpoint Saved: Iteration {}, Path: {}]\n",
                iteration, path
            ),
            ..Default::default()
        },
        AgentEvent::TextChunk { content } => RunTaskEvent {
            r#type: EventType::TextChunk as i32,
            content: content.clone(),
            ..Default::default()
        },
        AgentEvent::CostUpdate { total_cost_usd } => RunTaskEvent {
            r#type: EventType::TextChunk as i32,
            content: format!("[Cost Updated] Session Cost: ${:.6}\n", total_cost_usd),
            ..Default::default()
        },
        AgentEvent::ToolCall {
            name,
            args_json,
            result,
            iteration,
        } => RunTaskEvent {
            r#type: EventType::ToolCall as i32,
            tool_name: name.clone(),
            tool_args_json: args_json.clone(),
            tool_result: result.clone(),
            iteration: *iteration,
            ..Default::default()
        },
        AgentEvent::TaskComplete { content } => RunTaskEvent {
            r#type: EventType::TaskComplete as i32,
            content: content.clone(),
            ..Default::default()
        },
        AgentEvent::TaskError { error } => RunTaskEvent {
            r#type: EventType::TaskError as i32,
            error: error.clone(),
            ..Default::default()
        },
        AgentEvent::UserInterventionRequired { error } => RunTaskEvent {
            r#type: EventType::TaskError as i32,
            error: format!("USER INTERVENTION REQUIRED: {}", error),
            ..Default::default()
        },
        AgentEvent::Handoff { target_agent } => RunTaskEvent {
            r#type: EventType::Handoff as i32,
            content: format!("HANDOFF REQUESTED TO: {}", target_agent),
            ..Default::default()
        },
        AgentEvent::RewindOccurred {
            iteration,
            checkpoint_id,
            reason,
        } => RunTaskEvent {
            r#type: EventType::TextChunk as i32,
            content: format!(
                "[Rewind Occurred at Iteration {}: Checkpoint {}, Reason: {}]\n",
                iteration, checkpoint_id, reason
            ),
            ..Default::default()
        },
        AgentEvent::GuardrailTripped { reason } => RunTaskEvent {
            r#type: EventType::TaskError as i32,
            content: format!("Guardrail Tripped: {}", reason),
            ..Default::default()
        },
    }
}

pub fn enrich_legacy_projection(
    mut legacy: RunTaskEvent,
    canonical: &EventEnvelope,
) -> RunTaskEvent {
    legacy.session_id = canonical.session_id.to_string();
    legacy.task_id = canonical
        .task_id
        .map(|id| id.to_string())
        .unwrap_or_default();
    legacy.turn_id = canonical
        .turn_id
        .map(|id| id.to_string())
        .unwrap_or_default();
    legacy.attempt_id = canonical
        .ingest_attempt_id
        .map(|id| id.to_string())
        .unwrap_or_default();
    legacy.event_id = canonical.event_id.to_string();
    legacy.durable_sequence = canonical.durable_sequence.unwrap_or_default();
    legacy.delivery_sequence = canonical.delivery_sequence.unwrap_or_default();
    legacy.middleware_event_type = canonical.event_type.clone();
    legacy.source_attempt_id = canonical
        .source_attempt_id
        .map(|id| id.to_string())
        .unwrap_or_default();
    legacy.ingest_attempt_id = canonical
        .ingest_attempt_id
        .map(|id| id.to_string())
        .unwrap_or_default();
    legacy.payload_json = serde_json::to_string(&canonical.payload).unwrap_or_default();
    legacy.harness_id = canonical.harness_id.clone().unwrap_or_default();
    legacy
}

pub fn record_agent_event(
    adapter: &mut OmniSoloHarnessAdapter,
    event: &AgentEvent,
) -> Result<RunTaskEvent, AdapterError> {
    let canonical = canonical_event(event);
    let receipt: AppendResult = if matches!(event, AgentEvent::RunStarted { .. }) {
        AppendResult {
            event: adapter
                .events()
                .first()
                .cloned()
                .ok_or(AdapterError::NoDurableHead)?,
            duplicate: true,
        }
    } else {
        adapter.record(canonical)?
    };
    Ok(enrich_legacy_projection(
        legacy_projection(event),
        &receipt.event,
    ))
}

/// Projects the portable capsule's safe message and tool-result records into
/// the existing OmniSolo context format. Native records and unsupported data
/// never enter this projection.
pub fn portable_messages(capsule: &SessionCapsule) -> Vec<ohc_builtin_agent_core::types::Message> {
    capsule
        .records
        .iter()
        .filter_map(|record| match record {
            PortableRecord::Message(message) => Some(core_message(
                role_from_portable(&message.role),
                text_from_parts(&message.content),
            )),
            PortableRecord::ToolResult(result) => Some(core_message(
                ohc_builtin_agent_core::types::Role::Tool,
                text_from_parts(&result.content),
            )),
            _ => None,
        })
        .collect()
}

fn role_from_portable(role: &MessageRole) -> ohc_builtin_agent_core::types::Role {
    match role {
        MessageRole::System | MessageRole::Developer => ohc_builtin_agent_core::types::Role::System,
        MessageRole::User => ohc_builtin_agent_core::types::Role::User,
        MessageRole::Assistant => ohc_builtin_agent_core::types::Role::Assistant,
        MessageRole::Tool => ohc_builtin_agent_core::types::Role::Tool,
        MessageRole::Other => ohc_builtin_agent_core::types::Role::Assistant,
    }
}

fn text_from_parts(parts: &[ContentPart]) -> String {
    let mut texts = Vec::new();
    for part in parts {
        match part {
            ContentPart::Text { text, .. } | ContentPart::ReasoningSummary { text, .. } => {
                texts.push(text.clone());
            }
            ContentPart::StructuredJson { value, .. } => {
                if let Ok(text) = serde_json::to_string(value) {
                    texts.push(text);
                }
            }
            _ => {}
        }
    }
    texts.join("\n")
}

fn core_message(
    role: ohc_builtin_agent_core::types::Role,
    content: String,
) -> ohc_builtin_agent_core::types::Message {
    ohc_builtin_agent_core::types::Message {
        role,
        content,
        tool_calls: Vec::new(),
        tool_results: Vec::new(),
        response_id: None,
        previous_response_id: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::AgentEvent;
    use chrono::Utc;
    use server_harness::middleware::adapter::OmniSoloRunConfig;
    use server_harness::middleware::capsule::{PortableMessage, PortableToolResult};
    use server_harness::middleware::types::MessageStatus;

    #[test]
    fn all_omnisolo_events_have_canonical_and_legacy_projections() {
        let events = vec![
            AgentEvent::RunStarted { iteration: 1 },
            AgentEvent::TextChunk {
                content: "text".into(),
            },
            AgentEvent::ToolCall {
                name: "read".into(),
                args_json: "{}".into(),
                result: "ok".into(),
                iteration: 1,
            },
            AgentEvent::TaskComplete {
                content: "done".into(),
            },
            AgentEvent::TaskError {
                error: "failed".into(),
            },
            AgentEvent::UserInterventionRequired {
                error: "approve".into(),
            },
            AgentEvent::IterationStarted {
                iteration: 1,
                message_count: 2,
            },
            AgentEvent::CheckpointSaved {
                iteration: 1,
                path: "checkpoint".into(),
            },
            AgentEvent::Handoff {
                target_agent: "research".into(),
            },
            AgentEvent::RewindOccurred {
                iteration: 1,
                checkpoint_id: "cp".into(),
                reason: "retry".into(),
            },
            AgentEvent::GuardrailTripped {
                reason: "blocked".into(),
            },
            AgentEvent::CostUpdate {
                total_cost_usd: 0.12,
            },
        ];

        for event in &events {
            let canonical = canonical_event(event);
            let legacy = legacy_projection(event);
            assert_ne!(legacy.r#type, 0);
            assert!(!serde_json::to_string(&canonical).unwrap().is_empty());
        }
    }

    #[test]
    fn projection_contains_canonical_ids_and_event_payload() {
        let mut adapter =
            OmniSoloHarnessAdapter::start(OmniSoloRunConfig::new("tenant-1", "task").with_turn())
                .unwrap();
        let projected = record_agent_event(
            &mut adapter,
            &AgentEvent::ToolCall {
                name: "read".into(),
                args_json: "{\"path\":\"x\"}".into(),
                result: "contents".into(),
                iteration: 2,
            },
        )
        .unwrap();

        assert_eq!(
            projected.session_id,
            adapter.session().session_id.to_string()
        );
        assert_eq!(projected.task_id, adapter.task().task_id.to_string());
        assert_eq!(
            projected.turn_id,
            adapter.turn().unwrap().turn_id.to_string()
        );
        assert_eq!(projected.middleware_event_type, "tool.call_settled");
        assert!(projected.payload_json.contains("contents"));
        assert_eq!(projected.durable_sequence, 2);
    }

    #[test]
    fn portable_capsule_records_become_safe_omnisolo_context_messages() {
        let mut adapter =
            OmniSoloHarnessAdapter::start(OmniSoloRunConfig::new("tenant-1", "task").with_turn())
                .unwrap();
        record_agent_event(
            &mut adapter,
            &AgentEvent::ToolCall {
                name: "read".into(),
                args_json: "{}".into(),
                result: "tool output".into(),
                iteration: 1,
            },
        )
        .unwrap();
        record_agent_event(
            &mut adapter,
            &AgentEvent::TaskComplete {
                content: "assistant output".into(),
            },
        )
        .unwrap();
        let capsule = adapter
            .export_capsule("omnisolo", uuid::Uuid::new_v4())
            .unwrap();
        let messages = portable_messages(&capsule);

        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].role, ohc_builtin_agent_core::types::Role::Tool);
        assert_eq!(messages[0].content, "tool output");
        assert_eq!(
            messages[1].role,
            ohc_builtin_agent_core::types::Role::Assistant
        );
        assert_eq!(messages[1].content, "assistant output");
    }

    #[test]
    fn portable_projection_maps_roles_and_safe_content_parts() {
        let adapter =
            OmniSoloHarnessAdapter::start(OmniSoloRunConfig::new("tenant-1", "task").with_turn())
                .unwrap();
        let mut capsule = adapter
            .export_capsule("omnisolo", uuid::Uuid::new_v4())
            .unwrap();
        let session_id = capsule.manifest.session_id;
        let now = Utc::now();
        let message = |role| {
            PortableRecord::Message(PortableMessage {
                message_id: uuid::Uuid::new_v4(),
                session_id,
                task_id: None,
                turn_id: None,
                role,
                actor: None,
                origin: None,
                phase: None,
                parent_message_id: None,
                correlation_id: None,
                status: MessageStatus::Settled,
                content: vec![
                    ContentPart::Text {
                        text: "text".to_owned(),
                        annotations: Vec::new(),
                    },
                    ContentPart::ReasoningSummary {
                        text: "summary".to_owned(),
                        annotations: Vec::new(),
                    },
                    ContentPart::StructuredJson {
                        value: serde_json::json!({"key": "value"}),
                        schema_ref: None,
                    },
                    ContentPart::ToolCallRef {
                        tool_call_id: uuid::Uuid::new_v4(),
                    },
                ],
                visible_to_user: true,
                redaction_state: None,
                created_at: now,
                settled_at: Some(now),
            })
        };
        capsule.records = vec![
            message(MessageRole::System),
            message(MessageRole::Developer),
            message(MessageRole::User),
            message(MessageRole::Assistant),
            message(MessageRole::Tool),
            message(MessageRole::Other),
            PortableRecord::ToolResult(PortableToolResult {
                tool_call_id: uuid::Uuid::new_v4(),
                content: vec![ContentPart::Text {
                    text: "tool".to_owned(),
                    annotations: Vec::new(),
                }],
                metadata: Default::default(),
            }),
        ];

        let messages = portable_messages(&capsule);
        assert_eq!(messages.len(), 7);
        assert_eq!(
            messages[0].role,
            ohc_builtin_agent_core::types::Role::System
        );
        assert_eq!(
            messages[1].role,
            ohc_builtin_agent_core::types::Role::System
        );
        assert_eq!(messages[2].role, ohc_builtin_agent_core::types::Role::User);
        assert_eq!(
            messages[3].role,
            ohc_builtin_agent_core::types::Role::Assistant
        );
        assert_eq!(messages[4].role, ohc_builtin_agent_core::types::Role::Tool);
        assert_eq!(
            messages[5].role,
            ohc_builtin_agent_core::types::Role::Assistant
        );
        assert_eq!(messages[0].content, "text\nsummary\n{\"key\":\"value\"}");
        assert_eq!(messages[6].content, "tool");
    }

    #[test]
    fn record_projection_handles_initial_event_and_terminal_rejection() {
        let mut adapter =
            OmniSoloHarnessAdapter::start(OmniSoloRunConfig::new("tenant-1", "task").with_turn())
                .unwrap();
        let initial = record_agent_event(&mut adapter, &AgentEvent::RunStarted { iteration: 0 })
            .expect("initial projection");
        assert_eq!(initial.middleware_event_type, "run.started");

        record_agent_event(
            &mut adapter,
            &AgentEvent::TaskComplete {
                content: "done".to_owned(),
            },
        )
        .unwrap();
        assert!(matches!(
            record_agent_event(
                &mut adapter,
                &AgentEvent::TextChunk {
                    content: "late".to_owned(),
                },
            ),
            Err(AdapterError::TerminalAttempt)
        ));
    }
}
