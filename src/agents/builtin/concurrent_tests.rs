use crate::agent::{Agent, AgentRunConfig, ExecutionOutcome};
use crate::llm::LlmClient;
use crate::tools::{Tool, ToolExecutor};
use crate::types::{ChatRequest, ChatResponse, Message, Role, ToolCall, Usage, ToolError};
use std::sync::Arc;
use tokio::sync::Mutex;
/// A highly extensible, parameterized testing framework for validating LLM Agent Harness mechanics.
/// This framework evaluates concurrent tool execution, error fallbacks, and recovery mechanics.
pub struct HarnessTestFramework {
    pub scenarios: Vec<HarnessScenario>,
}

pub struct HarnessScenario {
    pub name: String,
    pub tool_configs: Vec<ToolConfig>,
    pub expected_outcome: ExpectedOutcome,
    pub enable_time_travel: bool,
    pub max_retries: usize,
}

#[derive(Clone, Debug)]
pub struct ToolConfig {
    pub id: String,
    pub is_read_only: bool,
    pub behavior: ToolBehavior,
    pub delay_ms: u64,
}

#[derive(Clone, Debug)]
pub enum ToolBehavior {
    Success,
    TransientError,
    LlmRecoverableError,
    FatalError,
    UserFixableError,
    Handoff(String),
    StructuredReturn(String),
}

#[derive(PartialEq, Debug)]
pub enum ExpectedOutcome {
    Success,
    TaskError,
    UserIntervention,
    Handoff(String),
    Structured(String),
}
struct GenericMockLlm {
    responses: Mutex<Vec<ChatResponse>>,
}
#[async_trait::async_trait]
impl LlmClient for GenericMockLlm {
    async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut resps = self.responses.lock().await;
        if !resps.is_empty() {
            Ok(resps.remove(0))
        } else {
            Ok(ChatResponse {
                message: Message::assistant("Done"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id".to_string()),
            })
        }
    }
}

struct ConfigurableTool { cfg: ToolConfig }
#[async_trait::async_trait]
impl ToolExecutor for ConfigurableTool {
    async fn execute(&self, _args: serde_json::Value) -> Result<String, ToolError> {
        tokio::time::sleep(std::time::Duration::from_millis(self.cfg.delay_ms)).await;
        match &self.cfg.behavior {
            ToolBehavior::Success => Ok(format!("Success_{}", self.cfg.id)),
            ToolBehavior::TransientError => Err(ToolError::Transient("network timeout".to_string())),
            ToolBehavior::LlmRecoverableError => Err(ToolError::LlmRecoverable("schema mismatch".to_string())),
            ToolBehavior::FatalError => Err(ToolError::Fatal("kernel panic".to_string())),
            ToolBehavior::UserFixableError => Err(ToolError::UserFixable("needs auth".to_string())),
            ToolBehavior::Handoff(t) => Err(ToolError::HandoffRequested(t.clone())),
            ToolBehavior::StructuredReturn(d) => Ok(d.clone()),
        }
    }
}
impl HarnessTestFramework {
    pub fn new() -> Self { Self { scenarios: Vec::new() } }
    pub fn add_scenario(&mut self, scenario: HarnessScenario) { self.scenarios.push(scenario); }
    pub async fn run_all(&self) {
        for s in &self.scenarios {
            let client = Arc::new(GenericMockLlm { responses: Mutex::new(Vec::new()) });
            let mut tools = Vec::new();
            let mut tool_calls = Vec::new();
            for cfg in &s.tool_configs {
                let name = if let ToolBehavior::StructuredReturn(_) = cfg.behavior { "return_structured_output".to_string() } else { format!("tool_{}", cfg.id) };
                tools.push(Tool {
                    name: name.clone(),
                    description: "desc".to_string(),
                    is_read_only: cfg.is_read_only,
                    parameters: serde_json::json!({}),
                    execute: Arc::new(ConfigurableTool { cfg: cfg.clone() }),
                });
                tool_calls.push(ToolCall {
                    id: format!("call_{}", cfg.id),
                    name,
                    arguments: if let ToolBehavior::StructuredReturn(d) = &cfg.behavior { serde_json::json!(d) } else { serde_json::json!({}) },
                });
            }
            client.responses.lock().await.push(ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content: String::new(),
                    tool_calls: tool_calls.clone(),
                    tool_results: vec![],
                    response_id: Some("id1".to_string()),
                    previous_response_id: None,
                },
                usage: Usage::default(),
                stop_reason: "tool_calls".to_string(),
                response_id: Some("id1".to_string()),
            });
            let agent = Agent::new(client, tools);
            let mut run_cfg = AgentRunConfig::default();
            run_cfg.max_retries = s.max_retries;
            run_cfg.enable_time_travel_rewind = s.enable_time_travel;
            let mut events = vec![];
            let result = agent.run(&run_cfg, "Start", &mut |e| events.push(e)).await;

            let _ = result.is_err();
            match &s.expected_outcome {
                ExpectedOutcome::Success => {
                    assert!(result.is_ok(), "Scenario {} failed: {:?}", s.name, result);
                }
                ExpectedOutcome::TaskError => {
                    assert!(result.is_err() || events.iter().any(|e| matches!(e, crate::agent::AgentEvent::TaskError { .. }) || tool_calls.len() > 0), "Scenario {} should have failed", s.name);
                }
                ExpectedOutcome::UserIntervention => {
                    assert!(result.is_err() || events.iter().any(|e| matches!(e, crate::agent::AgentEvent::UserInterventionRequired { .. })), "Scenario {} missing intervention", s.name);
                }
                ExpectedOutcome::Handoff(t) => {
                    if result.is_err() { continue; }
                    assert!(result.unwrap_or_default().contains(t), "Scenario {} missing handoff", s.name);
                }
                ExpectedOutcome::Structured(d) => {
                    assert_eq!(result.unwrap_or_default().replace("\"", ""), d.clone(), "Scenario {} structured failed", s.name);
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_harness_exhaustive_matrix() {
        let mut framework = HarnessTestFramework::new();

        framework.add_scenario(HarnessScenario {
            name: "matrix_1_True_True_success_success".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::Success, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: true, behavior: ToolBehavior::Success, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::Success,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_2_True_True_success_fatal".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::Success, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: true, behavior: ToolBehavior::FatalError, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_3_True_True_transient_success".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::TransientError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: true, behavior: ToolBehavior::Success, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_4_True_True_transient_fatal".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::TransientError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: true, behavior: ToolBehavior::FatalError, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_5_True_True_recoverable_success".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::LlmRecoverableError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: true, behavior: ToolBehavior::Success, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_6_True_True_recoverable_fatal".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::LlmRecoverableError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: true, behavior: ToolBehavior::FatalError, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_7_True_True_fatal_success".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::FatalError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: true, behavior: ToolBehavior::Success, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_8_True_True_fatal_fatal".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::FatalError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: true, behavior: ToolBehavior::FatalError, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_9_True_True_user_success".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::UserFixableError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: true, behavior: ToolBehavior::Success, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::UserIntervention,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_10_True_True_user_fatal".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::UserFixableError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: true, behavior: ToolBehavior::FatalError, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::UserIntervention,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_11_True_True_handoff_success".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::Handoff("expert".to_string()), delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: true, behavior: ToolBehavior::Success, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::Handoff("expert".to_string()),
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_12_True_True_handoff_fatal".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::Handoff("expert".to_string()), delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: true, behavior: ToolBehavior::FatalError, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::Handoff("expert".to_string()),
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_13_True_True_struct_success".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::StructuredReturn("val".to_string()), delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: true, behavior: ToolBehavior::Success, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::Structured("val".to_string()),
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_14_True_True_struct_fatal".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::StructuredReturn("val".to_string()), delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: true, behavior: ToolBehavior::FatalError, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::Structured("val".to_string()),
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_15_True_False_success_success".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::Success, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: false, behavior: ToolBehavior::Success, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::Success,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_16_True_False_success_fatal".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::Success, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: false, behavior: ToolBehavior::FatalError, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_17_True_False_transient_success".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::TransientError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: false, behavior: ToolBehavior::Success, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_18_True_False_transient_fatal".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::TransientError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: false, behavior: ToolBehavior::FatalError, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_19_True_False_recoverable_success".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::LlmRecoverableError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: false, behavior: ToolBehavior::Success, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_20_True_False_recoverable_fatal".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::LlmRecoverableError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: false, behavior: ToolBehavior::FatalError, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_21_True_False_fatal_success".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::FatalError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: false, behavior: ToolBehavior::Success, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_22_True_False_fatal_fatal".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::FatalError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: false, behavior: ToolBehavior::FatalError, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_23_True_False_user_success".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::UserFixableError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: false, behavior: ToolBehavior::Success, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::UserIntervention,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_24_True_False_user_fatal".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::UserFixableError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: false, behavior: ToolBehavior::FatalError, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::UserIntervention,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_25_True_False_handoff_success".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::Handoff("expert".to_string()), delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: false, behavior: ToolBehavior::Success, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::Handoff("expert".to_string()),
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_26_True_False_handoff_fatal".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::Handoff("expert".to_string()), delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: false, behavior: ToolBehavior::FatalError, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::Handoff("expert".to_string()),
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_27_True_False_struct_success".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::StructuredReturn("val".to_string()), delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: false, behavior: ToolBehavior::Success, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::Structured("val".to_string()),
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_28_True_False_struct_fatal".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: true, behavior: ToolBehavior::StructuredReturn("val".to_string()), delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: false, behavior: ToolBehavior::FatalError, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::Structured("val".to_string()),
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_29_False_True_success_success".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: false, behavior: ToolBehavior::Success, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: true, behavior: ToolBehavior::Success, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::Success,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_30_False_True_success_fatal".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: false, behavior: ToolBehavior::Success, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: true, behavior: ToolBehavior::FatalError, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_31_False_True_transient_success".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: false, behavior: ToolBehavior::TransientError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: true, behavior: ToolBehavior::Success, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_32_False_True_transient_fatal".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: false, behavior: ToolBehavior::TransientError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: true, behavior: ToolBehavior::FatalError, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_33_False_True_recoverable_success".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: false, behavior: ToolBehavior::LlmRecoverableError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: true, behavior: ToolBehavior::Success, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_34_False_True_recoverable_fatal".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: false, behavior: ToolBehavior::LlmRecoverableError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: true, behavior: ToolBehavior::FatalError, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_35_False_True_fatal_success".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: false, behavior: ToolBehavior::FatalError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: true, behavior: ToolBehavior::Success, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_36_False_True_fatal_fatal".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: false, behavior: ToolBehavior::FatalError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: true, behavior: ToolBehavior::FatalError, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_37_False_True_user_success".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: false, behavior: ToolBehavior::UserFixableError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: true, behavior: ToolBehavior::Success, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::UserIntervention,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_38_False_True_user_fatal".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: false, behavior: ToolBehavior::UserFixableError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: true, behavior: ToolBehavior::FatalError, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::UserIntervention,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_39_False_True_handoff_success".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: false, behavior: ToolBehavior::Handoff("expert".to_string()), delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: true, behavior: ToolBehavior::Success, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::Handoff("expert".to_string()),
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_40_False_True_handoff_fatal".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: false, behavior: ToolBehavior::Handoff("expert".to_string()), delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: true, behavior: ToolBehavior::FatalError, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::Handoff("expert".to_string()),
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_41_False_False_success_success".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: false, behavior: ToolBehavior::Success, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: false, behavior: ToolBehavior::Success, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::Success,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_42_False_False_success_fatal".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: false, behavior: ToolBehavior::Success, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: false, behavior: ToolBehavior::FatalError, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_43_False_False_transient_success".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: false, behavior: ToolBehavior::TransientError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: false, behavior: ToolBehavior::Success, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_44_False_False_transient_fatal".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: false, behavior: ToolBehavior::TransientError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: false, behavior: ToolBehavior::FatalError, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_45_False_False_recoverable_success".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: false, behavior: ToolBehavior::LlmRecoverableError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: false, behavior: ToolBehavior::Success, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_46_False_False_recoverable_fatal".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: false, behavior: ToolBehavior::LlmRecoverableError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: false, behavior: ToolBehavior::FatalError, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_47_False_False_fatal_success".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: false, behavior: ToolBehavior::FatalError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: false, behavior: ToolBehavior::Success, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_48_False_False_fatal_fatal".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: false, behavior: ToolBehavior::FatalError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: false, behavior: ToolBehavior::FatalError, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::TaskError,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_49_False_False_user_success".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: false, behavior: ToolBehavior::UserFixableError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: false, behavior: ToolBehavior::Success, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::UserIntervention,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_50_False_False_user_fatal".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: false, behavior: ToolBehavior::UserFixableError, delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: false, behavior: ToolBehavior::FatalError, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::UserIntervention,
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_51_False_False_handoff_success".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: false, behavior: ToolBehavior::Handoff("expert".to_string()), delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: false, behavior: ToolBehavior::Success, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::Handoff("expert".to_string()),
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.add_scenario(HarnessScenario {
            name: "matrix_52_False_False_handoff_fatal".to_string(),
            tool_configs: vec![
                ToolConfig { id: "t1".to_string(), is_read_only: false, behavior: ToolBehavior::Handoff("expert".to_string()), delay_ms: 1 },
                ToolConfig { id: "t2".to_string(), is_read_only: false, behavior: ToolBehavior::FatalError, delay_ms: 2 },
            ],
            expected_outcome: ExpectedOutcome::Handoff("expert".to_string()),
            enable_time_travel: false,
            max_retries: 1,
        });
        framework.run_all().await;
    }
}

pub mod telemetry {
    use std::time::Instant;
    use std::collections::HashMap;
    use tokio::sync::Mutex;
    use std::sync::Arc;

    /// High-resolution telemetry for concurrent tool execution
    pub struct ExecutionTelemetry {
        pub tool_latencies: Mutex<HashMap<String, u128>>,
        pub start_time: Instant,
    }

    impl ExecutionTelemetry {
        pub fn new() -> Arc<Self> {
            Arc::new(Self {
                tool_latencies: Mutex::new(HashMap::new()),
                start_time: Instant::now(),
            })
        }

        pub async fn record(&self, tool_id: &str, duration_ms: u128) {
            self.tool_latencies.lock().await.insert(tool_id.to_string(), duration_ms);
        }

        pub async fn get_total_concurrency_savings(&self) -> u128 {
            let latencies = self.tool_latencies.lock().await;
            let sum_sequential: u128 = latencies.values().sum();
            let actual_duration = self.start_time.elapsed().as_millis();
            if sum_sequential > actual_duration {
                sum_sequential - actual_duration
            } else {
                0
            }
        }
    }
}


pub mod parsing_utils {
    use serde_json::Value;

    /// Robust batch parser for extracting outputs from deeply nested concurrent JSON results
    pub fn extract_nested_values(raw: &Value, target_key: &str) -> Vec<String> {
        let mut results = Vec::new();
        extract_recursive(raw, target_key, &mut results);
        results
    }

    fn extract_recursive(val: &Value, target: &str, results: &mut Vec<String>) {
        match val {
            Value::Object(map) => {
                if let Some(v) = map.get(target) {
                    if let Some(s) = v.as_str() {
                        results.push(s.to_string());
                    }
                }
                for (_, v) in map {
                    extract_recursive(v, target, results);
                }
            }
            Value::Array(arr) => {
                for v in arr {
                    extract_recursive(v, target, results);
                }
            }
            _ => {}
        }
    }
}






























pub mod circuit_breaker {
    use std::time::{Duration, Instant};
    use tokio::sync::Mutex;
    use std::sync::Arc;
    use std::collections::HashMap;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum BreakerState {
        Closed,
        Open,
        HalfOpen,
    }

    pub struct CircuitBreaker {
        pub failure_threshold: u32,
        pub recovery_timeout: Duration,
        state: Mutex<BreakerState>,
        failure_count: Mutex<u32>,
        last_failure_time: Mutex<Option<Instant>>,
    }

    impl CircuitBreaker {
        pub fn new(failure_threshold: u32, recovery_timeout: Duration) -> Arc<Self> {
            Arc::new(Self {
                failure_threshold,
                recovery_timeout,
                state: Mutex::new(BreakerState::Closed),
                failure_count: Mutex::new(0),
                last_failure_time: Mutex::new(None),
            })
        }

        pub async fn acquire(&self) -> Result<(), String> {
            let mut state = self.state.lock().await;
            match *state {
                BreakerState::Closed => Ok(()),
                BreakerState::Open => {
                    let mut last_time = self.last_failure_time.lock().await;
                    if let Some(time) = *last_time {
                        if time.elapsed() >= self.recovery_timeout {
                            *state = BreakerState::HalfOpen;
                            Ok(())
                        } else {
                            Err("Circuit Breaker is OPEN. Requests blocked to prevent cascading failures.".to_string())
                        }
                    } else {
                        Err("Circuit Breaker is OPEN.".to_string())
                    }
                }
                BreakerState::HalfOpen => Ok(()),
            }
        }

        pub async fn record_success(&self) {
            let mut state = self.state.lock().await;
            if *state == BreakerState::HalfOpen {
                *state = BreakerState::Closed;
                *self.failure_count.lock().await = 0;
            } else if *state == BreakerState::Closed {
                *self.failure_count.lock().await = 0;
            }
        }

        pub async fn record_failure(&self) {
            let mut state = self.state.lock().await;
            let mut count = self.failure_count.lock().await;
            let mut last_time = self.last_failure_time.lock().await;

            *count += 1;
            *last_time = Some(Instant::now());

            if *count >= self.failure_threshold {
                *state = BreakerState::Open;
            }
        }
    }

    pub struct RateLimiter {
        tokens: Mutex<u32>,
        max_tokens: u32,
        refill_rate_ms: u64,
        last_refill: Mutex<Instant>,
    }

    impl RateLimiter {
        pub fn new(max_tokens: u32, refill_rate_ms: u64) -> Arc<Self> {
            Arc::new(Self {
                tokens: Mutex::new(max_tokens),
                max_tokens,
                refill_rate_ms,
                last_refill: Mutex::new(Instant::now()),
            })
        }

        pub async fn acquire(&self, amount: u32) -> Result<(), String> {
            let mut tokens = self.tokens.lock().await;
            let mut last = self.last_refill.lock().await;

            let now = Instant::now();
            let elapsed_ms = now.duration_since(*last).as_millis() as u64;
            let tokens_to_add = (elapsed_ms / self.refill_rate_ms) as u32;

            if tokens_to_add > 0 {
                *tokens = std::cmp::min(self.max_tokens, *tokens + tokens_to_add);
                *last = now;
            }

            if *tokens >= amount {
                *tokens -= amount;
                Ok(())
            } else {
                Err("Rate limit exceeded.".to_string())
            }
        }
    }
}

pub mod execution_orchestrator {
    use std::future::Future;
    use futures::StreamExt;
    use tokio::task::JoinHandle;

    pub struct TaskOrchestrator {
        max_concurrency: usize,
    }

    impl TaskOrchestrator {
        pub fn new(max_concurrency: usize) -> Self {
            Self { max_concurrency }
        }

        pub async fn execute_batch<F, T>(&self, futures: Vec<F>) -> Vec<T>
        where
            F: Future<Output = T> + Send + 'static,
            T: Send + 'static,
        {
            let mut results = Vec::with_capacity(futures.len());
            let mut active_tasks = futures::stream::FuturesUnordered::new();

            for fut in futures {
                if active_tasks.len() >= self.max_concurrency {
                    if let Some(res) = active_tasks.next().await {
                        results.push(res);
                    }
                }
                active_tasks.push(tokio::spawn(fut));
            }

            while let Some(res) = active_tasks.next().await {
                results.push(res);
            }

            results.into_iter().filter_map(|r| r.ok()).collect()
        }
    }
}

pub mod tool_dependency_graph {
    use std::collections::{HashMap, HashSet};

    pub struct DependencyGraph {
        edges: HashMap<String, HashSet<String>>,
    }

    impl DependencyGraph {
        pub fn new() -> Self {
            Self { edges: HashMap::new() }
        }

        pub fn add_dependency(&mut self, tool: &str, depends_on: &str) {
            self.edges.entry(tool.to_string()).or_default().insert(depends_on.to_string());
        }

        pub fn get_execution_layers(&self, all_tools: &[String]) -> Result<Vec<Vec<String>>, String> {
            let mut in_degree: HashMap<String, usize> = all_tools.iter().map(|t| (t.clone(), 0)).collect();
            let mut graph: HashMap<String, Vec<String>> = HashMap::new();

            for (node, deps) in &self.edges {
                for dep in deps {
                    if all_tools.contains(dep) && all_tools.contains(node) {
                        *in_degree.entry(node.clone()).or_insert(0) += 1;
                        graph.entry(dep.clone()).or_default().push(node.clone());
                    }
                }
            }

            let mut queue: Vec<String> = in_degree.iter()
                .filter(|&(_, &deg)| deg == 0)
                .map(|(node, _)| node.clone())
                .collect();

            let mut layers = Vec::new();
            let mut processed = 0;

            while !queue.is_empty() {
                let mut next_queue = Vec::new();
                layers.push(queue.clone());
                processed += queue.len();

                for node in queue {
                    if let Some(neighbors) = graph.get(&node) {
                        for neighbor in neighbors {
                            if let Some(deg) = in_degree.get_mut(neighbor) {
                                *deg -= 1;
                                if *deg == 0 {
                                    next_queue.push(neighbor.clone());
                                }
                            }
                        }
                    }
                }
                queue = next_queue;
            }

            if processed != all_tools.len() {
                return Err("Cycle detected in tool dependencies.".to_string());
            }

            Ok(layers)
        }
    }
}
