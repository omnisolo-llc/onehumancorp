use ohc_builtin_agent_core::types::ToolError;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
use opentelemetry::{global, KeyValue};

use crate::guardrails::GuardrailConfig;

/// Events emitted by the agent run loop.
#[derive(Debug, Clone)]
pub enum AgentEvent {
    RunStarted { iteration: i32 },
    TextChunk { content: String },
    ToolCall { name: String, args_json: String, result: String, iteration: i32 },
    TaskComplete { content: String },
    TaskError { error: String },
    UserInterventionRequired { error: String },
    IterationStarted { iteration: i32, message_count: usize },
    CheckpointSaved { iteration: i32, path: String },
    Handoff { target_agent: String },
    RewindOccurred { iteration: i32, checkpoint_id: String, reason: String },
}

/// Configuration for a single agent run.
#[derive(Debug, Clone)]
pub struct AgentRunConfig {
    pub agent_id: String,
    /// Error Handling (Compounding Error Prevention): Stripe limits retries to exactly 2.
    pub max_retries: usize,
    pub model: String,
    pub server_system_message: String,
    pub developer_instructions: String,
    pub user_instructions: String,
    pub max_tokens: i32,
    pub temperature: f32,
    pub max_iterations: i32,
    pub max_task_tokens: i32, // budget for token tracking
    pub confidence_threshold: f32,
        pub enable_harness_thickness_optimization: bool,
pub enable_llmcompiler_plan_and_execute: bool,
    pub enable_acon_context_strategy: bool,
    pub enable_observation_masking: bool,
    pub observation_masking_threshold: usize,
    pub observation_masking_size_limit: usize,
    pub enable_lost_in_the_middle_prevention: bool,
    pub enable_context_compaction: bool,
    pub compaction_threshold_tokens: i32,
    pub enable_llm_judge: bool,
    pub enable_computational_guides: bool,
    pub computational_guide_command: String,
    pub enable_visual_verification: bool,
    pub visual_verification_command: String,
    pub guardrails: Option<GuardrailConfig>,
    pub enable_state_checkpointing: bool,
    pub enable_git_checkpointing: bool,
    pub state_scratchpad_path: Option<String>,
    pub workspace_path: Option<String>,
    pub project_trusted: bool,
    pub injected_context: Option<Vec<ohc_builtin_agent_core::types::Message>>,
    pub allowed_tools: Option<Vec<String>>,
    pub high_risk_tools: Vec<String>,
    pub approved_tool_calls: Vec<String>,
    pub thread_id: Option<String>,
    pub resume_from_checkpoint_id: Option<String>,
    pub enable_single_agent_maximization: bool,
    pub enable_vercel_tool_scoping_metric: bool,
    pub enable_lazy_tool_loading: bool,
    pub enable_langgraph_mechanic: bool,
    pub enable_time_travel_rewind: bool,
    pub max_rewind_attempts: usize,
    pub long_term_memory: Option<Arc<dyn crate::memory_store::LongTermMemory>>,
}

impl Default for AgentRunConfig {
    fn default() -> Self {
        Self {
            agent_id: "default-agent".to_string(),
            max_retries: 2,
            model: String::new(),
            server_system_message: String::new(),
            developer_instructions: String::new(),
            user_instructions: String::new(),
            max_tokens: 2048,
            temperature: 0.0,
            max_iterations: 100,
            max_task_tokens: 100_000,
            confidence_threshold: 0.0,
                        enable_harness_thickness_optimization: false,
enable_llmcompiler_plan_and_execute: false,
            enable_acon_context_strategy: false,
            enable_observation_masking: true,
            observation_masking_threshold: 3,
            observation_masking_size_limit: 512,
            enable_lost_in_the_middle_prevention: true,
            enable_context_compaction: true,
            compaction_threshold_tokens: 60_000,
            enable_llm_judge: false,
            enable_computational_guides: false,
            computational_guide_command: String::new(),
            enable_visual_verification: false,
            visual_verification_command: String::new(),
            guardrails: None,
            enable_state_checkpointing: false,
            enable_git_checkpointing: false,
            state_scratchpad_path: None,
            workspace_path: None,
            project_trusted: true,
            injected_context: None,
            allowed_tools: None,
            high_risk_tools: vec![],
            approved_tool_calls: vec![],
            thread_id: None,
            resume_from_checkpoint_id: None,
            enable_single_agent_maximization: false,
            enable_vercel_tool_scoping_metric: false,
            enable_lazy_tool_loading: false,
            enable_langgraph_mechanic: false,
            enable_time_travel_rewind: false,
            max_rewind_attempts: 3,
            long_term_memory: None,
        }
    }
}

/// Progress metrics for a running agent task.
#[derive(Default)]
pub struct AgentProgress {
    tool_use_count: AtomicU64,
    token_count: AtomicI64,
}

impl AgentProgress {
    pub fn record_tool_use(&self) {
        self.tool_use_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn add_tokens(&self, n: i64) {
        self.token_count.fetch_add(n, Ordering::Relaxed);
    }

    pub fn tool_use_count(&self) -> u64 {
        self.tool_use_count.load(Ordering::Relaxed)
    }

    pub fn token_count(&self) -> i64 {
        self.token_count.load(Ordering::Relaxed)
    }
}

// Prompt Construction: OpenAI Codex Mechanic
// 1. Server-controlled System Message (Highest Priority)
// 2. Tool Definitions
// 3. Developer Instructions
// 4. User Instructions (cascading AGENTS.md files, capped at 32 KiB)
// 5. Conversation History (happens at run loop)
