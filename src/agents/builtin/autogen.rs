use crate::agent::{Agent, AgentEvent, AgentRunConfig};
use ohc_builtin_agent_core::types::{Message, Role};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::json;

/// Configuration for an Agent participating in AutoGen workflows.
#[derive(Clone)]
pub struct ChatAgent {
    pub name: String,
    pub description: String,
    pub agent: Arc<Agent>,
    pub run_config: AgentRunConfig,
    pub system_message_override: Option<String>,
}

impl ChatAgent {
    pub fn new(name: &str, description: &str, agent: Arc<Agent>, run_config: AgentRunConfig) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            agent,
            run_config,
            system_message_override: None,
        }
    }

    pub fn with_system_message(mut self, msg: &str) -> Self {
        self.system_message_override = Some(msg.to_string());
        self
    }
}

/// A Team represents a collection of agents organized for a specific task.
#[derive(Clone)]
pub struct Team {
    pub name: String,
    pub members: Vec<ChatAgent>,
    pub lead: Option<ChatAgent>,
}

impl Team {
    pub fn new(name: &str, members: Vec<ChatAgent>) -> Self {
        Self {
            name: name.to_string(),
            members,
            lead: None,
        }
    }

    pub fn with_lead(mut self, lead: ChatAgent) -> Self {
        self.lead = Some(lead);
        self
    }

    pub fn get_member(&self, name: &str) -> Option<&ChatAgent> {
        self.members.iter().find(|m| m.name == name)
    }

    pub fn list_members(&self) -> String {
        self.members.iter()
            .map(|m| format!("{}: {}", m.name, m.description))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Metadata and state for a single AutoGen conversation instance.
#[derive(Clone)]
pub struct Conversation {
    pub id: String,
    pub task: String,
    pub team: Team,
    pub transcript: Arc<RwLock<Vec<Message>>>,
    pub metadata: std::collections::HashMap<String, String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Conversation {
    pub fn new(task: &str, team: Team) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            task: task.to_string(),
            team,
            transcript: Arc::new(RwLock::new(Vec::new())),
            metadata: std::collections::HashMap::new(),
            created_at: chrono::Utc::now(),
        }
    }

    pub async fn add_message(&self, msg: Message) {
        let mut t = self.transcript.write().await;
        t.push(msg);
    }

    pub async fn get_last_response(&self) -> Option<String> {
        let t = self.transcript.read().await;
        t.last().map(|m| m.content.clone())
    }
}

// ── Patterns ─────────────────────────────────────────────────────────────────

/// Result of an AutoGen orchestration run.
pub struct OrchestrationResult {
    pub transcript: Vec<Message>,
    pub final_response: String,
    pub rounds: usize,
    pub success: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeakerSelectionStrategy {
    Auto,
    RoundRobin,
    Random,
    Manual,
}

/// A shared Group Chat context.
#[derive(Clone)]
pub struct GroupChat {
    pub agents: Vec<ChatAgent>,
    pub transcript: Arc<RwLock<Vec<Message>>>,
    pub max_rounds: usize,
    pub speaker_selection_strategy: SpeakerSelectionStrategy,
    pub admin_name: String,
}

impl GroupChat {
    pub fn new(agents: Vec<ChatAgent>, max_rounds: usize) -> Self {
        Self {
            agents,
            transcript: Arc::new(RwLock::new(Vec::new())),
            max_rounds,
            speaker_selection_strategy: SpeakerSelectionStrategy::Auto,
            admin_name: "Admin".to_string(),
        }
    }

    pub fn with_strategy(mut self, strategy: SpeakerSelectionStrategy) -> Self {
        self.speaker_selection_strategy = strategy;
        self
    }
}

/// The Orchestrator for Group Chat patterns.
pub struct GroupChatManager {
    pub chat: GroupChat,
    pub manager_llm: Arc<dyn crate::llm::LlmClient>,
}

impl GroupChatManager {
    pub fn new(chat: GroupChat, manager_llm: Arc<dyn crate::llm::LlmClient>) -> Self {
        Self { chat, manager_llm }
    }

    async fn select_next_speaker(&self, current_round: usize, last_speaker: Option<&str>) -> Result<ChatAgent, String> {
        match self.chat.speaker_selection_strategy {
            SpeakerSelectionStrategy::RoundRobin => {
                let idx = current_round % self.chat.agents.len();
                Ok(self.chat.agents[idx].clone())
            }
            SpeakerSelectionStrategy::Random => {
                use rand::seq::SliceRandom;
                let mut rng = rand::thread_rng();
                Ok(self.chat.agents.choose(&mut rng).unwrap().clone())
            }
            SpeakerSelectionStrategy::Auto => {
                let transcript = self.chat.transcript.read().await;
                let agents_info = self.chat.agents.iter()
                    .map(|a| format!("{}: {}", a.name, a.description))
                    .collect::<Vec<_>>()
                    .join("\n");

                let mut history = String::new();
                for msg in transcript.iter().rev().take(10).rev() {
                    history.push_str(&format!("{}: {}\n", msg.role, msg.content));
                }

                let system_prompt = format!(
                    "You are a Group Chat Manager. Select the next speaker from the list below based on the conversation history.\n\n\
                    Agents:\n{}\n\n\
                    Respond ONLY with the exact name of the agent. Do not include any other text.\n\
                    If multiple agents are suitable, pick the one most likely to progress the task.\n\
                    If the task is done, you can still pick an agent to say TERMINATE.",
                    agents_info
                );

                let req = ohc_builtin_agent_core::types::ChatRequest {
                    model: "manager".to_string(),
                    system: system_prompt,
                    messages: vec![Message::user(format!("History:\n{}\n\nNext speaker?", history))],
                    tools: vec![],
                    max_tokens: 50,
                    temperature: 0.0,
                };

                let resp = self.manager_llm.chat(req).await.map_err(|e| e.to_string())?;
                let name = resp.message.content.trim();

                // Advanced Matching: Exact Match > Contains > Fallback
                if let Some(agent) = self.chat.agents.iter().find(|a| a.name == name) {
                    return Ok(agent.clone());
                }

                for agent in &self.chat.agents {
                    if name.contains(&agent.name) || agent.name.contains(name) {
                        return Ok(agent.clone());
                    }
                }

                // Fallback to Round Robin if LLM output is nonsensical
                let idx = current_round % self.chat.agents.len();
                Ok(self.chat.agents[idx].clone())
            }
            SpeakerSelectionStrategy::Manual => {
                // In a production app, this would wait for user input.
                // For the harness, we fallback to first agent.
                Ok(self.chat.agents[0].clone())
            }
        }
    }

    pub async fn run(&self, initial_task: &str) -> Result<OrchestrationResult, String> {
        {
            let mut t = self.chat.transcript.write().await;
            t.push(Message::user(format!("{}: {}", self.chat.admin_name, initial_task)));
        }

        let mut last_speaker = None;
        let mut total_rounds = 0;

        for round in 0..self.chat.max_rounds {
            total_rounds += 1;
            let next_agent = self.select_next_speaker(round, last_speaker.as_deref()).await?;
            tracing::info!("GroupChat Round {}: {} is speaking", round, next_agent.name);

            let current_transcript = {
                let t = self.chat.transcript.read().await;
                t.clone()
            };

            // Termination Check
            if let Some(last_msg) = current_transcript.last() {
                if last_msg.content.contains("TERMINATE") {
                    tracing::info!("Group Chat: Termination keyword detected in last message.");
                    break;
                }
            }

            let mut context = format!("You are participating in a group chat as {}.\nTask: {}\n\nRecent Transcript:\n", next_agent.name, initial_task);
            for msg in current_transcript.iter().rev().take(15).rev() {
                context.push_str(&format!("{}: {}\n", msg.role, msg.content));
            }
            context.push_str("\nProvide your unique contribution. If the task is completed, end your message with 'TERMINATE'.");

            let mut cfg = next_agent.run_config.clone();
            cfg.server_system_message = next_agent.system_message_override.clone().unwrap_or_else(|| {
                format!("You are {}. {}. You are in a collaborative group chat.", next_agent.name, next_agent.description)
            });

            // AutoGen Pattern Enhancement: Cross-context memory injection
            if let Some(ref ltm) = cfg.long_term_memory {
                 if let Ok(mems) = ltm.retrieve(initial_task, 3).await {
                     if !mems.is_empty() {
                         context.push_str("\n\nRelevant Context from Memory:\n");
                         for m in mems {
                             context.push_str(&format!("- {}\n", m));
                         }
                     }
                 }
            }

            // Use context compaction if the transcript gets too long
            if current_transcript.len() > 10 {
                cfg.enable_context_compaction = true;
                cfg.compaction_threshold_tokens = 4000;
            }

            let mut on_event = |_| {};
            let response = next_agent.agent.run(&cfg, &context, &mut on_event).await
                .map_err(|e| format!("Agent {} failed in group chat: {}", next_agent.name, e))?;

            let final_resp = format!("{}: {}", next_agent.name, response);
            {
                let mut t = self.chat.transcript.write().await;
                t.push(Message::assistant(final_resp));
            }

            if response.contains("TERMINATE") {
                tracing::info!("GroupChat: {} requested termination.", next_agent.name);
                break;
            }
            last_speaker = Some(next_agent.name.clone());
        }

        let final_transcript = self.chat.transcript.read().await.clone();
        let final_response = final_transcript.last().map(|m| m.content.clone()).unwrap_or_default();

        Ok(OrchestrationResult {
            transcript: final_transcript,
            final_response,
            rounds: total_rounds,
            success: true,
        })
    }
}

// ── Sequential ───────────────────────────────────────────────────────────────

pub struct SequentialFlow {
    pub agents: Vec<ChatAgent>,
}

impl SequentialFlow {
    pub fn new(agents: Vec<ChatAgent>) -> Self {
        Self { agents }
    }

    pub async fn run(&self, initial_task: &str) -> Result<OrchestrationResult, String> {
        let mut transcript = vec![Message::user(initial_task.to_string())];
        let mut current_input = initial_task.to_string();
        let mut steps = 0;

        for (i, agent) in self.agents.iter().enumerate() {
            steps += 1;
            tracing::info!("SequentialFlow Step {}: {}", i + 1, agent.name);

            let mut cfg = agent.run_config.clone();
            cfg.server_system_message = agent.system_message_override.clone().unwrap_or_else(|| {
                format!("You are {}. {}. You are part of a sequential pipeline.", agent.name, agent.description)
            });

            let prompt = format!(
                "OVERALL GOAL: {}\n\n\
                STAGE {}/{}: {}\n\n\
                INPUT FROM PREVIOUS STAGE:\n\
                --------------------------\n\
                {}\n\
                --------------------------\n\n\
                Process this input and provide your refined output for the next stage.",
                initial_task, i + 1, self.agents.len(), agent.name, current_input
            );

            let mut on_event = |_| {};
            let response = agent.agent.run(&cfg, &prompt, &mut on_event).await
                .map_err(|e| format!("Agent {} in sequence failed: {}", agent.name, e))?;

            transcript.push(Message::assistant(format!("{}: {}", agent.name, response)));
            current_input = response;
        }

        Ok(OrchestrationResult {
            transcript,
            final_response: current_input,
            rounds: steps,
            success: true,
        })
    }
}

// ── Concurrent ───────────────────────────────────────────────────────────────

pub struct ConcurrentFlow {
    pub workers: Vec<ChatAgent>,
    pub synthesizer: Option<ChatAgent>,
}

impl ConcurrentFlow {
    pub fn new(workers: Vec<ChatAgent>, synthesizer: Option<ChatAgent>) -> Self {
        Self { workers, synthesizer }
    }

    pub async fn run(&self, task: &str) -> Result<OrchestrationResult, String> {
        let mut transcript = vec![Message::user(task.to_string())];
        let mut futures = Vec::new();

        for agent in &self.workers {
            let agent_c = agent.clone();
            let task_c = task.to_string();
            futures.push(tokio::spawn(async move {
                let mut on_event = |_| {};
                let mut cfg = agent_c.run_config.clone();
                cfg.server_system_message = agent_c.system_message_override.clone().unwrap_or_else(|| {
                    format!("You are {}. {}. Work independently on the assigned task.", agent_c.name, agent_c.description)
                });

                agent_c.agent.run(&cfg, &task_c, &mut on_event).await
                    .map(|res| (agent_c.name, res))
            }));
        }

        let mut worker_outputs = Vec::new();
        for f in futures {
            let res = f.await.map_err(|e| e.to_string())?;
            match res {
                Ok((name, out)) => {
                    transcript.push(Message::assistant(format!("{}: {}", name, out)));
                    worker_outputs.push(format!("{}: {}", name, out));
                }
                Err(e) => return Err(format!("Concurrent worker failed: {}", e)),
            }
        }

        let mut final_response = worker_outputs.join("\n\n");

        if let Some(synth) = &self.synthesizer {
            tracing::info!("ConcurrentFlow: Synthesis step starting...");
            let combined = worker_outputs.iter()
                .map(|out| format!("--- OUTPUT FROM WORKER ---\n{}\n-------------------------", out))
                .collect::<Vec<_>>()
                .join("\n\n");

            let synth_prompt = format!(
                "TASK TO CONSOLIDATE: '{}'\n\n\
                WORKER CONTRIBUTIONS:\n\
                {}\n\n\
                Please synthesize these independent contributions into a single, cohesive, high-quality final response.",
                task, combined
            );

            let mut cfg = synth.run_config.clone();
            cfg.server_system_message = synth.system_message_override.clone().unwrap_or_else(|| {
                format!("You are {}. {}. You are the Lead Synthesizer.", synth.name, synth.description)
            });

            let mut on_event = |_| {};
            let response = synth.agent.run(&cfg, &synth_prompt, &mut on_event).await
                .map_err(|e| format!("Synthesizer failed: {}", e))?;

            let synth_out = format!("{}: {}", synth.name, response);
            transcript.push(Message::assistant(synth_out.clone()));
            final_response = response;
        }

        Ok(OrchestrationResult {
            transcript,
            final_response,
            rounds: 1, // Concurrent step counts as 1 super-round
            success: true,
        })
    }
}

// ── Handoff ──────────────────────────────────────────────────────────────────

pub struct HandoffManager {
    pub agents: Vec<ChatAgent>,
}

impl HandoffManager {
    pub fn new(agents: Vec<ChatAgent>) -> Self {
        Self { agents }
    }

    pub async fn run(&self, initial_task: &str, start_agent_name: &str) -> Result<OrchestrationResult, String> {
        let mut transcript = vec![Message::user(initial_task.to_string())];
        let mut current_agent = self.agents.iter().find(|a| a.name == start_agent_name)
            .ok_or_else(|| format!("Start agent {} not found", start_agent_name))?.clone();

        let mut current_task = initial_task.to_string();
        let max_handoffs = 10;
        let mut handoff_count = 0;
        let mut final_response = String::new();

        while handoff_count < max_handoffs {
            handoff_count += 1;
            let mut cfg = current_agent.run_config.clone();
            let mut tools = current_agent.agent.tools.clone();

            // Inject Handoff Tool
            struct HandoffExecutor {
                agents: Vec<String>,
            }
            #[async_trait::async_trait]
            impl crate::tools::ToolExecutor for HandoffExecutor {
                async fn execute(&self, args: serde_json::Value) -> Result<String, ohc_builtin_agent_core::types::ToolError> {
                    let to = args["target_agent"].as_str().unwrap_or("");
                    let reason = args["reason"].as_str().unwrap_or("");
                    if self.agents.contains(&to.to_string()) {
                        Ok(format!("HANDOFF_TO: {} REASON: {}", to, reason))
                    } else {
                        Err(ohc_builtin_agent_core::types::ToolError::LlmRecoverable(format!("Agent {} not found for handoff", to)))
                    }
                }
            }

            tools.push(crate::tools::Tool {
                name: "handoff".to_string(),
                description: "Transfer control to another specialized agent. Use this if the task requires expertise you don't have.".to_string(),
                is_read_only: false,
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "target_agent": { "type": "string", "enum": self.agents.iter().map(|a| a.name.clone()).collect::<Vec<_>>() },
                        "reason": { "type": "string", "description": "Why are you handing off?" }
                    },
                    "required": ["target_agent"]
                }),
                execute: Arc::new(HandoffExecutor { agents: self.agents.iter().map(|a| a.name.clone()).collect() }),
            });

            let mut run_agent = Agent::new(current_agent.agent.llm.clone(), tools);
            run_agent.memory_store = current_agent.agent.memory_store.clone();

            cfg.server_system_message = current_agent.system_message_override.clone().unwrap_or_else(|| {
                format!(
                    "You are {}. {}. \n\
                    You are part of a specialized handoff network. \n\
                    1. If you can complete the task, do so and end with 'FINISHED'.\n\
                    2. If you need someone else, use the 'handoff' tool.\n\
                    3. Do not loop forever. Be decisive.",
                    current_agent.name, current_agent.description
                )
            });

            let mut on_event = |_| {};
            let response = run_agent.run(&cfg, &current_task, &mut on_event).await
                .map_err(|e| format!("Agent {} failed during handoff flow: {}", current_agent.name, e))?;

            transcript.push(Message::assistant(format!("{}: {}", current_agent.name, response)));
            final_response = response.clone();

            if response.contains("FINISHED") {
                tracing::info!("Handoff Flow: Completed by {}.", current_agent.name);
                break;
            }

            if let Some(handoff_idx) = response.find("HANDOFF_TO:") {
                let part = &response[handoff_idx + "HANDOFF_TO:".len()..];
                let target_name = part.split_whitespace().next().unwrap_or("").trim_matches(|c: char| !c.is_alphanumeric());

                if let Some(next) = self.agents.iter().find(|a| a.name == target_name) {
                    tracing::info!("Handoff [{}/{}]: {} -> {}", handoff_count, max_handoffs, current_agent.name, target_name);

                    let prev_name = current_agent.name.clone();
                    current_agent = next.clone();
                    current_task = format!(
                        "TRANSFERRED TASK FROM: {}\n\n\
                        CONTEXT OF HANDOFF:\n\
                        {}\n\n\
                        Please take over and continue the work.",
                        prev_name, response
                    );
                } else {
                    tracing::warn!("Handoff failed: Agent '{}' not found in registry.", target_name);
                    break;
                }
            } else {
                // If no tool called and no FINISHED, we assume it's stuck or done but forgot the keyword.
                break;
            }
        }

        Ok(OrchestrationResult {
            transcript,
            final_response,
            rounds: handoff_count,
            success: true,
        })
    }
}

// ── Magentic ─────────────────────────────────────────────────────────────────

use crate::tools::task::TaskStore;
use crate::tools::magentic::magentic_tool;

pub struct MagenticManager {
    pub manager: ChatAgent,
    pub sub_agents: Vec<ChatAgent>,
    pub task_store: Arc<RwLock<TaskStore>>,
    pub max_rounds: usize,
}

impl MagenticManager {
    pub fn new(manager: ChatAgent, sub_agents: Vec<ChatAgent>, max_rounds: usize) -> Self {
        Self {
            manager,
            sub_agents,
            task_store: Arc::new(RwLock::new(TaskStore::default())),
            max_rounds,
        }
    }

    pub async fn run(&self, initial_task: &str) -> Result<OrchestrationResult, String> {
        let mut transcript = vec![Message::user(initial_task.to_string())];
        let mut total_rounds = 0;
        let mut final_response = String::new();

        for round in 0..self.max_rounds {
            total_rounds += 1;
            tracing::info!("Magentic Round {}: Manager evaluating ledger", round);

            let mut mgr_cfg = self.manager.run_config.clone();
            let mut mgr_tools = self.manager.agent.tools.clone();
            mgr_tools.push(magentic_tool(self.task_store.clone()));

            let mut mgr_agent = Agent::new(self.manager.agent.llm.clone(), mgr_tools);
            mgr_agent.memory_store = self.manager.agent.memory_store.clone();

            mgr_cfg.server_system_message = self.manager.system_message_override.clone().unwrap_or_else(|| {
                format!(
                    "You are {}. {}. \n\
                    You are the Orchestrator. \n\
                    1. Use 'MagenticLedger' tool (action: 'add') to decompose the task into atomic sub-tasks.\n\
                    2. Use 'MagenticLedger' tool (action: 'list') to see pending tasks.\n\
                    3. Assign a task to an agent from: {}.\n\
                    4. FORMAT FOR ASSIGNMENT: 'ASSIGN: <AgentName> TASK_ID: <ID>'.\n\
                    5. Once all sub-tasks are COMPLETED, output 'FINISHED'.",
                    self.manager.name, self.manager.description,
                    self.sub_agents.iter().map(|a| a.name.clone()).collect::<Vec<_>>().join(", ")
                )
            });

            let mut on_event = |_| {};
            let response = mgr_agent.run(&mgr_cfg, initial_task, &mut on_event).await
                .map_err(|e| format!("Magentic Manager failed: {}", e))?;

            transcript.push(Message::assistant(format!("{}: {}", self.manager.name, response)));
            final_response = response.clone();

            if response.contains("FINISHED") {
                tracing::info!("Magentic Flow: Manager signaled FINISHED.");
                break;
            }

            // Enhanced Assignment Parsing
            let mut assignments = Vec::new();
            for line in response.lines() {
                if let Some(assign_idx) = line.find("ASSIGN:") {
                    let part = &line[assign_idx + "ASSIGN:".len()..];
                    let mut words = part.split_whitespace();
                    let agent_name = words.next().unwrap_or("").trim_matches(|c: char| !c.is_alphanumeric());

                    if let Some(task_word) = words.find(|w| w.contains("TASK_ID:")) {
                        let task_id = task_word.split(':').last().unwrap_or("").trim();
                        assignments.push((agent_name.to_string(), task_id.to_string()));
                    }
                }
            }

            if assignments.is_empty() {
                // If the manager didn't assign anything but didn't finish, we give it one more round
                // or assume it's just planning.
                continue;
            }

            // Execute assignments (Concurrent execution of sub-tasks in a round)
            let mut subtask_futures = Vec::new();
            for (agent_name, task_id) in assignments {
                if let Some(sub_agent) = self.sub_agents.iter().find(|a| a.name == agent_name) {
                    let sub_agent_c = sub_agent.clone();
                    let task_id_c = task_id.clone();
                    let manager_resp_c = response.clone();
                    let task_store_c = self.task_store.clone();

                    subtask_futures.push(tokio::spawn(async move {
                        tracing::info!("Magentic: Subagent {} starting task {}", agent_name, task_id_c);

                        let sub_prompt = format!(
                            "ASSIGNED TASK ID: {}\n\
                            INSTRUCTIONS FROM MANAGER: {}\n\n\
                            Please execute this task and provide the result.",
                            task_id_c, manager_resp_c
                        );

                        let mut sub_cfg = sub_agent_c.run_config.clone();
                        sub_cfg.server_system_message = sub_agent_c.system_message_override.clone().unwrap_or_else(|| {
                             format!("You are {}. {}. Complete the assigned task.", sub_agent_c.name, sub_agent_c.description)
                        });

                        let mut sub_on_event = |_| {};
                        let sub_resp = sub_agent_c.agent.run(&sub_cfg, &sub_prompt, &mut sub_on_event).await;

                        match sub_resp {
                            Ok(res) => {
                                // Update ledger
                                let _ = magentic_tool(task_store_c).execute.execute(json!({
                                    "action": "update",
                                    "id": task_id_c,
                                    "status": "completed",
                                    "result": res.clone()
                                })).await;
                                Ok((sub_agent_c.name, res))
                            }
                            Err(e) => Err(format!("Subagent {} failed: {}", sub_agent_c.name, e)),
                        }
                    }));
                }
            }

            for f in subtask_futures {
                match f.await.map_err(|e| e.to_string())? {
                    Ok((name, res)) => {
                        transcript.push(Message::assistant(format!("{}: {}", name, res)));
                    }
                    Err(e) => {
                        tracing::error!("Magentic Subtask Error: {}", e);
                        transcript.push(Message::assistant(format!("SYSTEM: {}", e)));
                    }
                }
            }
        }

        Ok(OrchestrationResult {
            transcript,
            final_response,
            rounds: total_rounds,
            success: true,
        })
    }
}

// ── Orchestrator ─────────────────────────────────────────────────────────────

pub enum AutoGenPattern {
    Sequential(SequentialFlow),
    Concurrent(ConcurrentFlow),
    GroupChat(GroupChatManager),
    Handoff(HandoffManager),
    Magentic(MagenticManager),
}

pub struct AutoGenOrchestrator {
    pub pattern: AutoGenPattern,
    pub team: Option<Team>,
}

impl AutoGenOrchestrator {
    pub fn new(pattern: AutoGenPattern) -> Self {
        Self { pattern, team: None }
    }

    pub fn with_team(mut self, team: Team) -> Self {
        self.team = Some(team);
        self
    }

    pub async fn run(&self, task: &str) -> Result<OrchestrationResult, String> {
        tracing::info!("AutoGenOrchestrator: Starting pattern execution for task: {}", task);
        let result = match &self.pattern {
            AutoGenPattern::Sequential(f) => f.run(task).await,
            AutoGenPattern::Concurrent(f) => f.run(task).await,
            AutoGenPattern::GroupChat(f) => f.run(task).await,
            AutoGenPattern::Handoff(f) => {
                let start_agent = &f.agents[0].name;
                f.run(task, start_agent).await
            },
            AutoGenPattern::Magentic(f) => f.run(task).await,
        };

        if let Ok(ref res) = result {
             tracing::info!("AutoGenOrchestrator: Pattern execution completed successfully with {} messages.", res.transcript.len());
        }

        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Usage};

    struct MockLlm {
        responses: tokio::sync::Mutex<Vec<String>>,
    }

    #[async_trait::async_trait]
    impl crate::llm::LlmClient for MockLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut r = self.responses.lock().await;
            let content = if r.is_empty() { "TERMINATE".to_string() } else { r.remove(0) };
            Ok(ChatResponse {
                message: Message::assistant(content),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id".to_string()),
            })
        }
    }

    fn create_mock_agent(responses: Vec<String>) -> Arc<Agent> {
        Arc::new(Agent::new(Arc::new(MockLlm { responses: tokio::sync::Mutex::new(responses) }), vec![]))
    }

    #[tokio::test]
    async fn test_sequential_flow() {
        let a1 = ChatAgent::new("A1", "Step 1", create_mock_agent(vec!["Result 1".into()]), AgentRunConfig::default());
        let a2 = ChatAgent::new("A2", "Step 2", create_mock_agent(vec!["Result 2".into()]), AgentRunConfig::default());

        let flow = SequentialFlow::new(vec![a1, a2]);
        let res = flow.run("Initial").await.unwrap();
        assert_eq!(res.transcript.len(), 3);
        assert!(res.transcript[1].content.contains("Result 1"));
        assert!(res.transcript[2].content.contains("Result 2"));
        assert_eq!(res.final_response, "Result 2");
    }

    #[tokio::test]
    async fn test_concurrent_flow() {
        let w1 = ChatAgent::new("W1", "Worker 1", create_mock_agent(vec!["Out 1".into()]), AgentRunConfig::default());
        let w2 = ChatAgent::new("W2", "Worker 2", create_mock_agent(vec!["Out 2".into()]), AgentRunConfig::default());
        let s = ChatAgent::new("S", "Synth", create_mock_agent(vec!["Combined".into()]), AgentRunConfig::default());

        let flow = ConcurrentFlow::new(vec![w1, w2], Some(s));
        let res = flow.run("Task").await.unwrap();
        assert_eq!(res.transcript.len(), 4);
        assert!(res.final_response.contains("Combined"));
    }

    #[tokio::test]
    async fn test_handoff_flow() {
        let a1 = ChatAgent::new("A1", "Hander", create_mock_agent(vec!["HANDOFF_TO: A2".into()]), AgentRunConfig::default());
        let a2 = ChatAgent::new("A2", "Receiver", create_mock_agent(vec!["FINISHED".into()]), AgentRunConfig::default());

        let flow = HandoffManager::new(vec![a1, a2]);
        let res = flow.run("Task", "A1").await.unwrap();
        assert!(res.transcript.iter().any(|m| m.content.contains("A1: HANDOFF_TO: A2")));
        assert!(res.transcript.iter().any(|m| m.content.contains("A2: FINISHED")));
        assert!(res.final_response.contains("FINISHED"));
    }

    #[tokio::test]
    async fn test_group_chat_round_robin() {
        let a1 = ChatAgent::new("A1", "First", create_mock_agent(vec!["A1 says hi".into()]), AgentRunConfig::default());
        let a2 = ChatAgent::new("A2", "Second", create_mock_agent(vec!["A2 says hi".into()]), AgentRunConfig::default());

        let chat = GroupChat::new(vec![a1, a2], 2).with_strategy(SpeakerSelectionStrategy::RoundRobin);
        let mgr = GroupChatManager::new(chat, Arc::new(MockLlm { responses: tokio::sync::Mutex::new(vec![]) }));

        let res = mgr.run("Task").await.unwrap();
        assert_eq!(res.transcript.len(), 3); // Admin + A1 + A2
        assert!(res.transcript[1].content.contains("A1: A1 says hi"));
        assert!(res.transcript[2].content.contains("A2: A2 says hi"));
    }

    #[tokio::test]
    async fn test_magentic_ledger_integration() {
        let manager_llm = Arc::new(MockLlm {
            responses: tokio::sync::Mutex::new(vec![
                "ASSIGN: Worker1 TASK_ID: task-1".into(),
                "FINISHED".into()
            ])
        });

        let worker_llm = Arc::new(MockLlm {
            responses: tokio::sync::Mutex::new(vec!["Done task 1".into()])
        });

        let mgr_agent = ChatAgent::new("Manager", "Mgr", Arc::new(Agent::new(manager_llm, vec![])), AgentRunConfig::default());
        let worker_agent = ChatAgent::new("Worker1", "Work", Arc::new(Agent::new(worker_llm, vec![])), AgentRunConfig::default());

        let mgr = MagenticManager::new(mgr_agent, vec![worker_agent], 5);
        let res = mgr.run("Initial").await.unwrap();

        assert!(res.transcript.iter().any(|m| m.content.contains("Manager: ASSIGN: Worker1 TASK_ID: task-1")));
        assert!(res.transcript.iter().any(|m| m.content.contains("Worker1: Done task 1")));
    }

    #[tokio::test]
    async fn test_autogen_tool() {
        let a1 = ChatAgent::new("A1", "Specialist", create_mock_agent(vec!["Subtask Result".into()]), AgentRunConfig::default());
        let flow = SequentialFlow::new(vec![a1]);
        let orchestrator = Arc::new(AutoGenOrchestrator::new(AutoGenPattern::Sequential(flow)));

        let tool = autogen_tool(orchestrator);
        let res = tool.execute.execute(json!({"sub_task": "Go"})).await.unwrap();
        assert!(res.contains("Subtask Result"));
    }
}
