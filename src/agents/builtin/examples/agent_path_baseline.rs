use ohc_builtin_agent::agent::{Agent, AgentRunConfig};
use ohc_builtin_agent::llm::LlmClient;
use ohc_builtin_agent::tools::{Tool, ToolExecutor};
use ohc_builtin_agent::types::{ChatRequest, ChatResponse, Message, Role, ToolError, Usage};
use ohc_builtin_agent_core::request_profile::{RequestProfile, profile_request};
use serde::Serialize;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};
use std::time::Instant;

const WARMUP_TURNS: usize = 20;
const MEASURED_TURNS: usize = 200;
const INPUT: &str = "What is six times seven?";
const EXPECTED_ANSWER: &str = "The verified answer is 42.";

#[derive(Default)]
struct DeterministicLlmClient {
    calls: AtomicUsize,
    last_request: Mutex<Option<ChatRequest>>,
}

#[async_trait::async_trait]
impl LlmClient for DeterministicLlmClient {
    async fn chat(
        &self,
        request: ChatRequest,
    ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        self.calls.fetch_add(1, Ordering::Relaxed);
        *self.last_request.lock().expect("request mutex poisoned") = Some(request);
        Ok(ChatResponse {
            message: Message::assistant(EXPECTED_ANSWER),
            usage: Usage {
                input_tokens: 120,
                output_tokens: 8,
                ..Default::default()
            },
            stop_reason: "stop".to_string(),
            response_id: Some("fixture-response".to_string()),
        })
    }
}

struct NeverExecutor {
    executions: Arc<AtomicUsize>,
}

#[async_trait::async_trait]
impl ToolExecutor for NeverExecutor {
    async fn execute(&self, _args: serde_json::Value) -> Result<String, ToolError> {
        self.executions.fetch_add(1, Ordering::Relaxed);
        Err(ToolError::Unexpected("not executed".into()))
    }
}

#[derive(Debug, Serialize)]
struct BenchmarkResult {
    schema_version: u8,
    iterations: usize,
    median_micros: u128,
    p95_micros: u128,
    request_profile: RequestProfile,
    llm_calls_per_turn: f64,
    quality_passed: bool,
}

fn fixture_tool() -> (Tool, Arc<AtomicUsize>) {
    let executions = Arc::new(AtomicUsize::new(0));
    let tool = Tool {
        name: "Lookup".into(),
        description: "Lookup authoritative facts".into(),
        is_read_only: true,
        parameters: serde_json::json!({
            "type": "object",
            "properties": {"id": {"type": "string"}}
        }),
        execute: Arc::new(NeverExecutor {
            executions: executions.clone(),
        }),
    };
    (tool, executions)
}

fn validate_request(request: &ChatRequest) -> Result<RequestProfile, &'static str> {
    if request.tools.len() != 1 {
        return Err("tool count invariant changed");
    }
    if request.tools[0].name != "Lookup" {
        return Err("tool name invariant changed");
    }
    if request.tools[0].description != "Lookup authoritative facts" {
        return Err("tool description invariant changed");
    }
    if request.tools[0].parameters
        != serde_json::json!({
            "type": "object",
            "properties": {"id": {"type": "string"}}
        })
    {
        return Err("tool parameter schema invariant changed");
    }

    let user_messages = request
        .messages
        .iter()
        .filter(|message| message.role == Role::User)
        .collect::<Vec<_>>();
    if user_messages.len() != 1 || user_messages[0].content != INPUT {
        return Err("exact user-message invariant changed");
    }

    Ok(profile_request(request))
}

async fn run_turn(
    agent: &Agent,
    llm: &DeterministicLlmClient,
    config: &AgentRunConfig,
    tool: &Tool,
    tool_executions: &AtomicUsize,
) -> (RequestProfile, std::time::Duration) {
    let calls_before_turn = llm.calls.load(Ordering::Relaxed);
    let started = Instant::now();
    let turn_result = agent
        .run_tao_orchestration_loop(config, INPUT, std::slice::from_ref(tool), &mut |_| {})
        .await;
    let elapsed = started.elapsed();

    let answer = turn_result.expect("deterministic production turn failed");
    assert_eq!(answer, EXPECTED_ANSWER, "answer invariant changed");
    assert_eq!(
        llm.calls.load(Ordering::Relaxed) - calls_before_turn,
        1,
        "LLM-call count invariant changed"
    );
    assert_eq!(
        tool_executions.load(Ordering::Relaxed),
        0,
        "the production fixture must not execute its native tool"
    );

    let request = llm
        .last_request
        .lock()
        .expect("request mutex poisoned")
        .take()
        .expect("the production turn did not call the LLM");
    let profile = validate_request(&request).expect("request invariant changed");
    (profile, elapsed)
}

fn percentile(samples: &mut [u128], percent: usize) -> u128 {
    assert!(!samples.is_empty(), "percentile needs at least one sample");
    assert!((1..=100).contains(&percent), "percentile must be 1..=100");
    samples.sort_unstable();
    let rank = (samples.len() * percent).div_ceil(100);
    samples[rank - 1]
}

async fn run_benchmark() -> (BenchmarkResult, usize) {
    let llm = Arc::new(DeterministicLlmClient::default());
    let agent = Agent::new(llm.clone(), vec![]);
    let config = AgentRunConfig {
        max_iterations: 1,
        enable_lost_in_the_middle_prevention: false,
        ..Default::default()
    };
    let (tool, tool_executions) = fixture_tool();

    for _ in 0..WARMUP_TURNS {
        let _ = run_turn(&agent, &llm, &config, &tool, &tool_executions).await;
    }

    let calls_before_measurement = llm.calls.load(Ordering::Relaxed);
    let mut durations = Vec::with_capacity(MEASURED_TURNS);
    let mut measured_profile = None;
    for _ in 0..MEASURED_TURNS {
        let (profile, elapsed) = run_turn(&agent, &llm, &config, &tool, &tool_executions).await;
        durations.push(elapsed.as_micros());
        if let Some(expected) = &measured_profile {
            assert_eq!(expected, &profile, "request profile changed between turns");
        } else {
            measured_profile = Some(profile);
        }
    }
    let measured_calls = llm.calls.load(Ordering::Relaxed) - calls_before_measurement;

    let mut median_samples = durations.clone();
    let median_micros = percentile(&mut median_samples, 50);
    let p95_micros = percentile(&mut durations, 95);
    (
        BenchmarkResult {
            schema_version: 1,
            iterations: MEASURED_TURNS,
            median_micros,
            p95_micros,
            request_profile: measured_profile.expect("measured turns cannot be empty"),
            llm_calls_per_turn: measured_calls as f64 / MEASURED_TURNS as f64,
            quality_passed: true,
        },
        tool_executions.load(Ordering::Relaxed),
    )
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (result, tool_executions) = run_benchmark().await;
    assert_eq!(tool_executions, 0, "tool execution invariant changed");
    println!(
        "{}",
        serde_json::to_string(&result).expect("benchmark result must serialize")
    );
}

#[cfg(test)]
mod tests {
    use super::{MEASURED_TURNS, WARMUP_TURNS, percentile, run_benchmark, validate_request};
    use ohc_builtin_agent::types::{ChatRequest, Message, ToolDefinition};

    fn request_with_messages(messages: Vec<Message>) -> ChatRequest {
        ChatRequest {
            model: "fixture-model".to_string(),
            system: String::new(),
            messages,
            tools: vec![ToolDefinition {
                name: "Lookup".to_string(),
                description: "Lookup authoritative facts".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {"id": {"type": "string"}}
                }),
            }],
            max_tokens: 256,
            temperature: 0.0,
        }
    }

    #[test]
    fn benchmark_sample_counts_are_fixed() {
        assert_eq!(WARMUP_TURNS, 20);
        assert_eq!(MEASURED_TURNS, 200);
    }

    #[test]
    fn percentile_uses_the_nearest_rank() {
        let mut samples = (1_u128..=200).collect::<Vec<_>>();

        assert_eq!(percentile(&mut samples, 50), 100);
        assert_eq!(percentile(&mut samples, 95), 190);
    }

    #[test]
    fn request_requires_exactly_one_exact_fixture_user_message() {
        let valid = request_with_messages(vec![Message::user(super::INPUT)]);
        assert!(validate_request(&valid).is_ok());

        let appended = request_with_messages(vec![Message::user(format!(
            "{} Please elaborate.",
            super::INPUT
        ))]);
        assert!(validate_request(&appended).is_err());

        let duplicated = request_with_messages(vec![
            Message::user(super::INPUT),
            Message::user(super::INPUT),
        ]);
        assert!(validate_request(&duplicated).is_err());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn benchmark_preserves_the_production_fixture_contract() {
        let (result, tool_executions) = run_benchmark().await;

        assert_eq!(result.schema_version, 1);
        assert_eq!(result.iterations, 200);
        assert_eq!(result.llm_calls_per_turn, 1.0);
        assert!(result.quality_passed);
        assert_eq!(result.request_profile.tool_count, 1);
        assert_eq!(result.request_profile.history_chars, super::INPUT.len());
        assert_eq!(tool_executions, 0);

        let serialized = serde_json::to_value(result).unwrap();
        let keys = serialized.as_object().unwrap().keys().collect::<Vec<_>>();
        assert_eq!(
            keys,
            vec![
                "iterations",
                "llm_calls_per_turn",
                "median_micros",
                "p95_micros",
                "quality_passed",
                "request_profile",
                "schema_version",
            ]
        );
    }
}
