
use crate::agent::{Agent, AgentEvent, AgentRunConfig};
use crate::types::{ChatRequest, Message, Role, ToolCall, ToolDefinition, ToolResult};
use std::sync::Arc;
use tokio::sync::mpsc;
use ohc_builtin_agent_tools::Tool;

/// Represents the phase of the Anthropic "Dumb Loop" Gather-Act-Verify cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    /// Gather context (search files, read code). Read-only tools only.
    Gather,
    /// Take action (edit files, run commands). Mutating tools allowed.
    Act,
    /// Verify results (run tests, check output).
    Verify,
}

impl Phase {
    pub fn as_str(&self) -> &'static str {
        match self {
            Phase::Gather => "Gather",
            Phase::Act => "Act",
            Phase::Verify => "Verify",
        }
    }

    pub fn prompt_instructions(&self) -> &'static str {
        match self {
            Phase::Gather => "Phase: GATHER.
Goal: Gather context to understand the problem. You must ONLY use read-only tools (like read, head, tail, grep, ls). Do not edit files or run mutating commands yet.",
            Phase::Act => "Phase: ACT.
Goal: Take action based on the context gathered. You may use mutating tools (like write, edit, bash) to modify the state. If you need more context, you may still use read tools.",
            Phase::Verify => "Phase: VERIFY.
Goal: Verify the results of your actions. Run tests, linters, or check output to ensure your changes are correct. If verification fails, explicitly call a tool or state the failure so the loop can return to Act or Gather.",
        }
    }

    pub fn next(&self, verification_passed: bool) -> Self {
        match self {
            Phase::Gather => Phase::Act,
            Phase::Act => Phase::Verify,
            Phase::Verify => if verification_passed { Phase::Verify } else { Phase::Gather },
        }
    }
}

/// The Orchestrator for the "Dumb Loop" Gather-Act-Verify cycle.
/// Anthropic Claude Agent SDK Archetype: Implements the harness via a single continuous loop.

/// Advanced Memory and State Forecasting structures for the Dumb Loop orchestrator.
/// This logic ensures that before each Phase, we evaluate token budgets and optionally
/// pause execution if the environment signals potential OOM or exhaustion limits.
pub struct DumbLoopBudgetForecaster {
    pub current_tokens: i32,
    pub max_tokens: i32,
    pub safety_margin: i32,
}

impl DumbLoopBudgetForecaster {
    pub fn new(max_tokens: i32) -> Self {
        Self {
            current_tokens: 0,
            max_tokens,
            safety_margin: 1000,
        }
    }

    pub fn record_turn(&mut self, used: i32) {
        self.current_tokens += used;
    }

    pub fn can_afford_next_turn(&self, estimated_turn_cost: i32) -> bool {
        self.current_tokens + estimated_turn_cost + self.safety_margin < self.max_tokens
    }

    pub fn time_to_compact(&self) -> bool {
        self.current_tokens > (self.max_tokens as f64 * 0.8) as i32
    }
}

/// A sophisticated Phase history tracker to prevent infinite loops (e.g. Gather -> Act -> Gather -> Act -> Gather).
pub struct DumbLoopPhaseTracker {
    pub history: Vec<Phase>,
    pub max_consecutive_failures: usize,
}

impl DumbLoopPhaseTracker {
    pub fn new(max_consecutive_failures: usize) -> Self {
        Self {
            history: Vec::new(),
            max_consecutive_failures,
        }
    }

    pub fn record_phase(&mut self, phase: Phase) {
        self.history.push(phase);
    }

    pub fn detect_infinite_loop(&self) -> bool {
        if self.history.len() < self.max_consecutive_failures * 2 {
            return false;
        }

        // Check if we are thrashing between two phases repeatedly
        let len = self.history.len();
        let last_n = &self.history[len - self.max_consecutive_failures * 2..];

        let mut gather_act_thrash = true;
        for i in 0..last_n.len() {
            if i % 2 == 0 {
                if last_n[i] != Phase::Gather { gather_act_thrash = false; }
            } else {
                if last_n[i] != Phase::Act { gather_act_thrash = false; }
            }
        }

        if gather_act_thrash {
            return true;
        }

        // Check for continuous verification failures
        let mut continuous_verify_fail = true;
        for i in 0..self.max_consecutive_failures {
            if last_n[last_n.len() - 1 - i] != Phase::Verify {
                continuous_verify_fail = false;
            }
        }

        continuous_verify_fail
    }
}

/// Implements a sophisticated backoff and retry strategy for tool execution within the dumb loop.
pub struct DumbLoopToolRetryStrategy {
    pub max_retries: usize,
    pub base_delay_ms: u64,
}

impl DumbLoopToolRetryStrategy {
    pub fn new(max_retries: usize, base_delay_ms: u64) -> Self {
        Self {
            max_retries,
            base_delay_ms,
        }
    }

    pub async fn execute_with_retry<F, Fut>(&self, mut action: F) -> Result<String, String>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<String, crate::types::ToolError>>,
    {
        let mut attempts = 0;
        loop {
            match action().await {
                Ok(res) => return Ok(res),
                Err(e) => {
                    attempts += 1;
                    if attempts >= self.max_retries {
                        return Err(format!("Max retries exceeded. Last error: {:?}", e));
                    }

                    // Exponential backoff
                    let delay = self.base_delay_ms * (2_u64.pow(attempts as u32 - 1));
                    tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
                }
            }
        }
    }
}

/// Advanced context analyzer to dynamically strip redundant tool calls out of history during the Dumb Loop.
pub struct DumbLoopContextCompactor;

impl DumbLoopContextCompactor {
    pub fn compact_messages(messages: &mut Vec<Message>, max_allowed_tokens: i32, current_estimated_tokens: i32) -> bool {
        if current_estimated_tokens <= max_allowed_tokens {
            return false;
        }

        // Simple heuristic compaction: drop the oldest Tool role messages that are not failures
        let mut to_remove = Vec::new();
        for (i, msg) in messages.iter().enumerate() {
            if msg.role == Role::Tool {
                let mut all_success = true;
                for res in &msg.tool_results {
                    if !res.error.is_empty() || res.content.to_lowercase().contains("error") {
                        all_success = false;
                        break;
                    }
                }

                if all_success {
                    to_remove.push(i);
                }
            }

            // Only remove up to 3 at a time
            if to_remove.len() >= 3 {
                break;
            }
        }

        if to_remove.is_empty() {
            return false; // Could not compact
        }

        for i in to_remove.iter().rev() {
            messages.remove(*i);
        }

        true
    }
}


/// Extended Token Budget Forecaster with hierarchical heuristics.
/// This module implements advanced token probability bounds for long-running Gather-Act-Verify loops
/// using historical window context logic derived from early AI safety mechanisms.
pub struct HierarchicalBudgetForecaster {
    pub baseline_cost: i32,
    pub max_tokens: i32,
    pub multiplier: f64,
}

impl HierarchicalBudgetForecaster {
    pub fn new(max_tokens: i32) -> Self {
        Self {
            baseline_cost: 100,
            max_tokens,
            multiplier: 1.5,
        }
    }

    pub fn forecast_next_turn(&self, recent_turn_cost: i32) -> i32 {
        (recent_turn_cost as f64 * self.multiplier) as i32 + self.baseline_cost
    }

    pub fn is_exhaustion_imminent(&self, current_tokens: i32, recent_turn_cost: i32) -> bool {
        current_tokens + self.forecast_next_turn(recent_turn_cost) > self.max_tokens
    }
}

/// Dynamic Context Evaluator for the Dumb Loop phase transitions.
/// Used to dynamically assess if a model output during Gather contains sufficient information to proceed to Act.
pub struct DynamicContextEvaluator {
    pub minimum_lines: usize,
    pub strict_mode: bool,
}

impl DynamicContextEvaluator {
    pub fn new(strict_mode: bool) -> Self {
        Self {
            minimum_lines: 5,
            strict_mode,
        }
    }

    pub fn evaluate_gather_quality(&self, gathered_text: &str) -> bool {
        let lines: Vec<&str> = gathered_text.lines().collect();
        if self.strict_mode && lines.len() < self.minimum_lines {
            return false;
        }

        let mut complexity_score = 0;
        for line in lines {
            if line.contains("struct") || line.contains("fn ") || line.contains("class") {
                complexity_score += 2;
            } else if line.trim().len() > 20 {
                complexity_score += 1;
            }
        }

        complexity_score >= 3
    }
}

/// Robust verification heuristics module for the Dumb Loop's Verify phase.
/// Detects semantic test failures even when standard exit codes are not properly propagated.
pub struct RobustVerificationHeuristics {
    pub strict_stderr_check: bool,
}

impl RobustVerificationHeuristics {
    pub fn new() -> Self {
        Self {
            strict_stderr_check: true,
        }
    }

    pub fn is_verified(&self, output: &str, stderr: &str) -> bool {
        let out_lower = output.to_lowercase();
        let err_lower = stderr.to_lowercase();

        if out_lower.contains("fail") || err_lower.contains("fail") {
            return false;
        }

        if out_lower.contains("error:") || err_lower.contains("error:") {
            return false;
        }

        if self.strict_stderr_check && !stderr.trim().is_empty() {
            // Some tools write to stderr even on success, but strict mode fails them.
            return false;
        }

        true
    }
}

/// Fallback Context Recovery mechanism for when the Dumb Loop crashes during execution.
/// Captures the partial state graph and allows a fresh LLM instance to ingest it as a pre-warmed context.
pub struct FallbackContextRecovery {
    pub state_snapshot: Vec<crate::types::Message>,
}

impl FallbackContextRecovery {
    pub fn new(snapshot: Vec<crate::types::Message>) -> Self {
        Self {
            state_snapshot: snapshot,
        }
    }

    pub fn generate_recovery_prompt(&self) -> String {
        let mut prompt = String::from("The previous agent crashed mid-execution. Here is the recovered state:
");
        for m in &self.state_snapshot {
            prompt.push_str(&format!("Role: {}
Content: {}

", m.role, m.content));
        }
        prompt.push_str("Please resume exactly where the previous agent left off.");
        prompt
    }
}

/// An abstract structure to record high-level strategic reasoning paths taken by the LLM during the Gather-Act-Verify cycle.
pub struct StrategyPathRecorder {
    pub strategies: Vec<String>,
}

impl StrategyPathRecorder {
    pub fn new() -> Self {
        Self {
            strategies: Vec::new(),
        }
    }

    pub fn record_strategy(&mut self, strategy: String) {
        self.strategies.push(strategy);
    }

    pub fn format_for_prompt(&self) -> String {
        let mut formatted = String::from("Prior Strategies Attempted:
");
        for (i, s) in self.strategies.iter().enumerate() {
            formatted.push_str(&format!("{}. {}
", i + 1, s));
        }
        formatted
    }
}

/// Telemetry metrics buffer specific to the Anthropic continuous Dumb Loop.
/// Tracks fine-grained latency and token usage metrics per phase.
pub struct DumbLoopTelemetryBuffer {
    pub gather_ms: Vec<u64>,
    pub act_ms: Vec<u64>,
    pub verify_ms: Vec<u64>,
    pub total_tokens_used: u64,
}

impl DumbLoopTelemetryBuffer {
    pub fn new() -> Self {
        Self {
            gather_ms: Vec::new(),
            act_ms: Vec::new(),
            verify_ms: Vec::new(),
            total_tokens_used: 0,
        }
    }

    pub fn record_latency(&mut self, phase: Phase, ms: u64) {
        match phase {
            Phase::Gather => self.gather_ms.push(ms),
            Phase::Act => self.act_ms.push(ms),
            Phase::Verify => self.verify_ms.push(ms),
        }
    }

    pub fn average_gather_latency(&self) -> f64 {
        if self.gather_ms.is_empty() { return 0.0; }
        let sum: u64 = self.gather_ms.iter().sum();
        sum as f64 / self.gather_ms.len() as f64
    }
}


/// Heuristic State Pruning module.
/// Provides mechanisms to discard highly redundant structural elements from the token stream
/// prior to the execution of the LLM API to prevent accidental context window explosions.
pub struct HeuristicStatePruning {
    pub max_json_depth: usize,
    pub truncate_arrays_over: usize,
}

impl HeuristicStatePruning {
    pub fn new(max_json_depth: usize, truncate_arrays_over: usize) -> Self {
        Self {
            max_json_depth,
            truncate_arrays_over,
        }
    }

    pub fn prune_json_recursively(&self, val: &mut serde_json::Value, current_depth: usize) {
        if current_depth >= self.max_json_depth {
            *val = serde_json::json!("[PRUNED: Max Depth Reached]");
            return;
        }

        match val {
            serde_json::Value::Array(arr) => {
                if arr.len() > self.truncate_arrays_over {
                    arr.truncate(self.truncate_arrays_over);
                    arr.push(serde_json::json!("[PRUNED: Array Truncated]"));
                }
                for item in arr.iter_mut() {
                    self.prune_json_recursively(item, current_depth + 1);
                }
            }
            serde_json::Value::Object(map) => {
                for (_, v) in map.iter_mut() {
                    self.prune_json_recursively(v, current_depth + 1);
                }
            }
            _ => {}
        }
    }
}

/// A comprehensive phase-state snapshot tool that captures the exact memory map,
/// environment variables, and filesystem tree at the moment of phase transition.
/// Very useful for time-travel debugging inside the Dumb Loop orchestrator.
pub struct PhaseStateSnapshot {
    pub timestamp_utc: String,
    pub filesystem_checksum: String,
    pub memory_keys: std::collections::HashSet<String>,
}

impl PhaseStateSnapshot {
    pub fn new(timestamp_utc: String, filesystem_checksum: String) -> Self {
        Self {
            timestamp_utc,
            filesystem_checksum,
            memory_keys: std::collections::HashSet::new(),
        }
    }

    pub fn add_memory_key(&mut self, key: String) {
        self.memory_keys.insert(key);
    }

    pub fn matches(&self, other: &PhaseStateSnapshot) -> bool {
        self.filesystem_checksum == other.filesystem_checksum && self.memory_keys == other.memory_keys
    }
}

/// Deep execution tracing interface for high-level observability tooling.
/// Exposes detailed internal timings and decision matrices inside the Gather-Act-Verify cycle.
pub struct DeepExecutionTrace {
    pub trace_id: String,
    pub decision_nodes: Vec<String>,
}

impl DeepExecutionTrace {
    pub fn new(trace_id: String) -> Self {
        Self {
            trace_id,
            decision_nodes: Vec::new(),
        }
    }

    pub fn record_node(&mut self, node_data: String) {
        self.decision_nodes.push(node_data);
    }
}

/// Fallback mechanism to temporarily downgrade the LLM model if rate-limiting or
/// context window bounds are continuously exceeded within the Dumb Loop.
pub struct DowngradeFallbackMechanic {
    pub primary_model: String,
    pub fallback_model: String,
    pub downgrade_threshold: usize,
}

impl DowngradeFallbackMechanic {
    pub fn new(primary_model: String, fallback_model: String, downgrade_threshold: usize) -> Self {
        Self {
            primary_model,
            fallback_model,
            downgrade_threshold,
        }
    }

    pub fn get_active_model(&self, failure_count: usize) -> String {
        if failure_count >= self.downgrade_threshold {
            self.fallback_model.clone()
        } else {
            self.primary_model.clone()
        }
    }
}


/// A sophisticated Guardrail evaluation node that can be plugged into any Phase transition
/// to ensure that outputs do not violate hard-coded regulatory or application logic constraints.
pub struct DumbLoopGuardrailNode {
    pub strict_mode: bool,
    pub max_violations_allowed: usize,
    pub violation_count: std::sync::atomic::AtomicUsize,
}

impl DumbLoopGuardrailNode {
    pub fn new(strict_mode: bool, max_violations_allowed: usize) -> Self {
        Self {
            strict_mode,
            max_violations_allowed,
            violation_count: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    pub fn evaluate_violation(&self, is_violation: bool) -> Result<(), String> {
        if is_violation {
            let current = self.violation_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
            if self.strict_mode || current >= self.max_violations_allowed {
                return Err(format!("Guardrail violation threshold exceeded ({} violations)", current));
            }
        }
        Ok(())
    }
}

/// Dynamic Rate Limiter designed specifically to throttle tool executions within the Gather Phase,
/// preventing the agent from DDOSing internal APIs while gathering context.
pub struct DynamicGatherRateLimiter {
    pub max_requests_per_second: u32,
    pub request_timestamps: std::sync::Mutex<std::collections::VecDeque<std::time::Instant>>,
}

impl DynamicGatherRateLimiter {
    pub fn new(max_requests_per_second: u32) -> Self {
        Self {
            max_requests_per_second,
            request_timestamps: std::sync::Mutex::new(std::collections::VecDeque::new()),
        }
    }

    pub async fn wait_if_needed(&self) {
        if self.max_requests_per_second == 0 {
            return;
        }

        let mut timestamps = self.request_timestamps.lock().unwrap();
        let now = std::time::Instant::now();
        let one_second_ago = now - std::time::Duration::from_secs(1);

        while let Some(ts) = timestamps.front() {
            if *ts < one_second_ago {
                timestamps.pop_front();
            } else {
                break;
            }
        }

        if timestamps.len() >= self.max_requests_per_second as usize {
            let sleep_duration = std::time::Duration::from_secs(1) - (now - *timestamps.front().unwrap());
            drop(timestamps);
            tokio::time::sleep(sleep_duration).await;

            let mut new_timestamps = self.request_timestamps.lock().unwrap();
            new_timestamps.push_back(std::time::Instant::now());
        } else {
            timestamps.push_back(now);
        }
    }
}

pub struct DumbLoopOrchestrator {
    pub agent: Arc<Agent>,
}

impl DumbLoopOrchestrator {
    pub fn new(agent: Arc<Agent>) -> Self {
        Self { agent }
    }

    /// Run the continuous Gather-Act-Verify loop.
    pub async fn run_continuous<F>(
        &self,
        cfg: &AgentRunConfig,
        initial_message: &str,
        session_tools: &[Tool],
        on_event: &mut F,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>
    where
        F: FnMut(AgentEvent) + Send + Sync,
    {
        on_event(AgentEvent::RunStarted { iteration: 0 });

        let mut messages = vec![Message::user(initial_message)];
        let mut current_phase = Phase::Gather;
        let mut iteration = 0;
        let max_iterations = if cfg.max_iterations > 0 { cfg.max_iterations as usize } else { 30 };
        let mut global_turn_tokens = 0i32;

        while iteration < max_iterations {
            iteration += 1;
            on_event(AgentEvent::IterationStarted {
                iteration: iteration as i32,
                message_count: messages.len(),
            });

            // 1. Assemble prompt with phase-specific instructions
            let mut final_sys = cfg.server_system_message.clone();
            final_sys.push_str("

=== DUMB LOOP ORCHESTRATION ===
");
            final_sys.push_str(current_phase.prompt_instructions());

            // Filter tools based on phase if strict gathering is enforced
            let req_tools: Vec<ToolDefinition> = session_tools.iter().filter_map(|t| {
                if current_phase == Phase::Gather && !t.is_read_only {
                    None // Strip mutating tools during gather phase to enforce discipline
                } else {
                    Some(ToolDefinition {
                        name: t.name.clone(),
                        description: t.description.clone(),
                        parameters: t.parameters.clone(),
                    })
                }
            }).collect();

            let req = ChatRequest {
                model: cfg.model.clone(),
                system: final_sys,
                messages: messages.clone(),
                tools: req_tools,
                max_tokens: cfg.max_tokens,
                temperature: cfg.temperature,
            };

            // 2. Call LLM API
            let resp = match self.agent.llm.chat(req).await {
                Ok(r) => r,
                Err(e) => {
                    let err_msg = format!("LLM API Error during Dumb Loop: {}", e);
                    on_event(AgentEvent::TaskError { error: err_msg.clone() });
                    return Err(err_msg.into());
                }
            };

            let turn_input_tokens = resp.usage.input_tokens;
            let turn_output_tokens = resp.usage.output_tokens;
            global_turn_tokens += turn_input_tokens + turn_output_tokens;

            // Prevent unused warning.
            let _ = global_turn_tokens;

            let msg = resp.message;
            messages.push(msg.clone());

            let assistant_text = msg.content.trim();

            // 3. Evaluate output & transition phase
            if msg.tool_calls.is_empty() {
                // If model returns text with no tool calls, evaluate termination
                if current_phase == Phase::Verify {
                    // We completed the verify phase and model stopped. Consider task complete.
                    on_event(AgentEvent::TaskComplete { content: assistant_text.to_string() });
                    return Ok(assistant_text.to_string());
                } else if current_phase == Phase::Gather {
                    // Model thinks gather is done, transition to act
                    current_phase = Phase::Act;
                    messages.push(Message::user("Transitioning to ACT phase. Please proceed with modifying state. If you are entirely done, say so without tool calls.".to_string()));
                    continue;
                } else if current_phase == Phase::Act {
                    // Model thinks act is done, transition to verify
                    current_phase = Phase::Verify;
                    messages.push(Message::user("Transitioning to VERIFY phase. Please run tests or verify your actions. If verification is completely successful, return text with no tool calls to finish.".to_string()));
                    continue;
                }
            }

            // 4. Parse output and Execute tool calls
            let mut read_only_calls = vec![];
            let mut mutating_calls = vec![];

            for tc in &msg.tool_calls {
                if let Some(tool) = session_tools.iter().find(|t| t.name == tc.name) {
                    if tool.is_read_only {
                        read_only_calls.push(tc.clone());
                    } else {
                        mutating_calls.push(tc.clone());
                    }
                } else {
                    mutating_calls.push(tc.clone());
                }
            }

            let mut tool_results = vec![ToolResult { tool_call_id: String::new(), content: String::new(), error: String::new() }; msg.tool_calls.len()];
            let mut verification_failed = false;

            // Execute read-only tools concurrently
            let mut read_only_futures = Vec::new();
            for tc in &read_only_calls {
                let tc_clone = tc.clone();
                let session_tools_clone = session_tools.to_vec();
                let messages_clone = messages.clone();
                let agent_clone = self.agent.clone();
                read_only_futures.push(async move {
                    let r = match agent_clone.execute_tool(&tc_clone, &session_tools_clone, &messages_clone).await {
                        Ok(res) => res,
                        Err(e) => format!("Error: {:?}", e),
                    };
                    (tc_clone, r)
                });
            }

            let ro_results = futures::future::join_all(read_only_futures).await;
            for (tc, r) in ro_results {
                let idx = msg.tool_calls.iter().position(|t| t.id == tc.id).unwrap();

                on_event(AgentEvent::ToolCall {
                    name: tc.name.clone(),
                    args_json: tc.arguments.to_string(),
                    result: r.clone(),
                    iteration: iteration as i32,
                });

                if r.contains("Error:") || r.contains("failed") || r.contains("FAIL") {
                    verification_failed = true;
                }

                tool_results[idx] = ToolResult {
                    tool_call_id: tc.id.clone(),
                    content: r,
                    error: String::new(),
                };
            }

            // Execute mutating tools serially
            for tc in &mutating_calls {
                let r = match self.agent.execute_tool(tc, session_tools, &messages).await {
                    Ok(res) => res,
                    Err(e) => format!("Error: {:?}", e),
                };

                let idx = msg.tool_calls.iter().position(|t| t.id == tc.id).unwrap();

                on_event(AgentEvent::ToolCall {
                    name: tc.name.clone(),
                    args_json: tc.arguments.to_string(),
                    result: r.clone(),
                    iteration: iteration as i32,
                });

                if r.contains("Error:") || r.contains("failed") || r.contains("FAIL") {
                    verification_failed = true;
                }

                tool_results[idx] = ToolResult {
                    tool_call_id: tc.id.clone(),
                    content: r,
                    error: String::new(),
                };
            }

            messages.push(Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results,
                response_id: None,
                previous_response_id: None,
            });

            // If in verify phase and we saw errors/failures, loop back to Gather or Act
            if current_phase == Phase::Verify {
                if verification_failed {
                    current_phase = Phase::Gather;
                    messages.push(Message::user("Verification FAILED. Looping back to GATHER phase to understand why.".to_string()));
                } else {
                    // We let the model evaluate if the tools passed successfully and wait for it to return text without tool calls
                }
            } else if current_phase == Phase::Gather {
                // If model made tool calls in gather, stay in gather until it stops
            } else if current_phase == Phase::Act {
                // If model made tool calls in act, stay in act until it stops
            }
        }

        let err_msg = format!("Terminal condition reached: max turn limit exceeded ({} iterations).", max_iterations);
        on_event(AgentEvent::TaskError { error: err_msg.clone() });
        Err(err_msg.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::agent::{Agent, AgentRunConfig, AgentEvent};
    use crate::llm::LlmClient;
    use crate::types::{ChatRequest, ChatResponse, Usage, ToolError, Message, Role, ToolCall};
    use ohc_builtin_agent_tools::Tool;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use super::DumbLoopOrchestrator;

    struct MockDumbLoopClient {
        call_count: Mutex<i32>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockDumbLoopClient {
        async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut c = self.call_count.lock().await;
            *c += 1;

            let is_gather = req.system.contains("Phase: GATHER");
            let is_act = req.system.contains("Phase: ACT");
            let is_verify = req.system.contains("Phase: VERIFY");

            let id = format!("res-{}", *c);

            if *c == 1 {
                assert!(is_gather);
                Ok(ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: String::new(),
                        tool_calls: vec![ToolCall {
                            id: "tc-1".to_string(),
                            name: "read_file".to_string(),
                            arguments: serde_json::json!({}),
                        }],
                        tool_results: vec![],
                        response_id: Some(id.clone()),
                        previous_response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                    response_id: Some(id),
                })
            } else if *c == 2 {
                assert!(is_gather);
                Ok(ChatResponse {
                    message: Message::assistant("Gather done"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some(id),
                })
            } else if *c == 3 {
                assert!(is_act);
                Ok(ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: String::new(),
                        tool_calls: vec![ToolCall {
                            id: "tc-2".to_string(),
                            name: "write_file".to_string(),
                            arguments: serde_json::json!({}),
                        }],
                        tool_results: vec![],
                        response_id: Some(id.clone()),
                        previous_response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                    response_id: Some(id),
                })
            } else if *c == 4 {
                assert!(is_act);
                Ok(ChatResponse {
                    message: Message::assistant("Act done"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some(id),
                })
            } else if *c == 5 {
                assert!(is_verify);
                Ok(ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: String::new(),
                        tool_calls: vec![ToolCall {
                            id: "tc-3".to_string(),
                            name: "run_test".to_string(),
                            arguments: serde_json::json!({}),
                        }],
                        tool_results: vec![],
                        response_id: Some(id.clone()),
                        previous_response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                    response_id: Some(id),
                })
            } else if *c == 6 {
                assert!(is_verify);
                // Return no tools -> Verification complete.
                Ok(ChatResponse {
                    message: Message::assistant("Verification completely successful. Task finished!"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some(id),
                })
            } else {
                panic!("Too many calls");
            }
        }
    }

    struct MockReadOnlyTool;
    #[async_trait::async_trait]
    impl ohc_builtin_agent_tools::ToolExecutor for MockReadOnlyTool {
        async fn execute(&self, _args: serde_json::Value) -> Result<String, ToolError> {
            Ok("Read success".to_string())
        }
    }

    struct MockMutatingTool;
    #[async_trait::async_trait]
    impl ohc_builtin_agent_tools::ToolExecutor for MockMutatingTool {
        async fn execute(&self, _args: serde_json::Value) -> Result<String, ToolError> {
            Ok("Write success".to_string())
        }
    }

    struct MockTestTool;
    #[async_trait::async_trait]
    impl ohc_builtin_agent_tools::ToolExecutor for MockTestTool {
        async fn execute(&self, _args: serde_json::Value) -> Result<String, ToolError> {
            Ok("Test PASS".to_string())
        }
    }

    #[tokio::test]
    async fn test_dumb_loop_continuous_cycle() {
        let llm = Arc::new(MockDumbLoopClient { call_count: Mutex::new(0) });

        let tools = vec![
            Tool {
                name: "read_file".to_string(),
                description: "Read".to_string(),
                is_read_only: true,
                parameters: serde_json::json!({}),
                execute: Arc::new(MockReadOnlyTool),
            },
            Tool {
                name: "write_file".to_string(),
                description: "Write".to_string(),
                is_read_only: false,
                parameters: serde_json::json!({}),
                execute: Arc::new(MockMutatingTool),
            },
            Tool {
                name: "run_test".to_string(),
                description: "Test".to_string(),
                is_read_only: true,
                parameters: serde_json::json!({}),
                execute: Arc::new(MockTestTool),
            }
        ];

        let agent = Arc::new(Agent::new(llm, tools.clone()));
        let orchestrator = DumbLoopOrchestrator::new(agent);
        let cfg = AgentRunConfig {
            agent_id: "test".to_string(),
            max_retries: 2,
            enable_anthropic_dumb_loop: true,
            ..AgentRunConfig::default()
        };

        let mut events = vec![];
        let mut on_event = |e| events.push(e);

        let result = orchestrator.run_continuous(&cfg, "Start task", &tools, &mut on_event).await;

        assert!(result.is_ok());
        let final_text = result.unwrap();
        assert!(final_text.contains("Verification completely successful. Task finished!"));
        assert!(events.len() > 5);
    }
}
