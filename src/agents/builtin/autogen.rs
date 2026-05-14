use crate::agent::{Agent, AgentRunConfig};
use ohc_builtin_agent_core::types::Message;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Configuration for an Agent participating in the Group Chat.
#[derive(Clone)]
pub struct ChatAgent {
    pub name: String,
    pub description: String,
    pub agent: Arc<Agent>,
    pub run_config: AgentRunConfig,
}

/// A shared Group Chat transcript.
#[derive(Clone, Default)]
pub struct GroupChat {
    pub agents: Vec<ChatAgent>,
    pub transcript: Arc<RwLock<Vec<Message>>>,
    pub max_rounds: usize,
}

impl GroupChat {
    pub fn new(agents: Vec<ChatAgent>, max_rounds: usize) -> Self {
        Self {
            agents,
            transcript: Arc::new(RwLock::new(Vec::new())),
            max_rounds,
        }
    }
}

/// The Orchestrator that selects the next speaker and manages the flow.
pub struct GroupChatManager {
    pub chat: GroupChat,
    pub llm: Arc<dyn crate::llm::LlmClient>,
}

impl GroupChatManager {
    pub fn new(chat: GroupChat, llm: Arc<dyn crate::llm::LlmClient>) -> Self {
        Self { chat, llm }
    }
}

impl GroupChatManager {
    /// Select the next speaker based on the conversation history.
    async fn select_speaker(&self, current_transcript: &[Message]) -> Result<ChatAgent, String> {
        if self.chat.agents.is_empty() {
            return Err("No agents in the group chat.".to_string());
        }

        // Format agent descriptions for the LLM
        let mut agents_desc = String::new();
        for agent in &self.chat.agents {
            agents_desc.push_str(&format!("- {}: {}\n", agent.name, agent.description));
        }

        // Format recent transcript
        let mut history = String::new();
        let tail_msgs = if current_transcript.len() > 10 {
            &current_transcript[current_transcript.len() - 10..]
        } else {
            current_transcript
        };

        for msg in tail_msgs {
            if msg.role == ohc_builtin_agent_core::types::Role::Assistant {
                history.push_str(&format!("{}\n", msg.content));
            } else {
                history.push_str(&format!("{}: {}\n", msg.role, msg.content));
            }
        }

        let system_prompt = format!(
            "You are a Group Chat Manager. Your job is to select the next speaker from the available agents based on the conversation history.\n\nAvailable Agents:\n{}\n\nRespond ONLY with the exact name of the agent who should speak next.",
            agents_desc
        );

        let req = ohc_builtin_agent_core::types::ChatRequest {
            model: "default".to_string(), // The mock or underlying LLM determines this
            system: system_prompt,
            messages: vec![Message::user(format!(
                "History:\n{}\n\nWho should speak next?",
                history
            ))],
            tools: vec![],
            max_tokens: 50,
            temperature: 0.0,
        };

        let response = self.llm.chat(req).await.map_err(|e| e.to_string())?;
        let speaker_name = response.message.content.trim();

        // Find the selected agent
        for agent in &self.chat.agents {
            if speaker_name.contains(&agent.name) {
                return Ok(agent.clone());
            }
        }

        // Fallback: round robin or pick the first if LLM fails to pick a valid one
        Ok(self.chat.agents[0].clone())
    }

    /// Run the group chat loop
    pub async fn run_chat(&self, task: &str) -> Result<Vec<Message>, String> {
        let mut transcript = self.chat.transcript.write().await;

        // Add the initial task
        transcript.push(Message::user(format!("Admin: {}", task)));

        // Drop the write lock before looping
        drop(transcript);

        for round in 0..self.chat.max_rounds {
            let current_transcript = self.chat.transcript.read().await.clone();

            // Check termination condition
            if let Some(last_msg) = current_transcript.last() {
                if last_msg.content.contains("TERMINATE") {
                    tracing::info!("Group chat terminated via TERMINATE keyword.");
                    break;
                }
            }

            // Select next speaker
            let next_speaker = self.select_speaker(&current_transcript).await?;
            tracing::info!("Round {}: {} is speaking...", round, next_speaker.name);

            // Format transcript into a single prompt for the selected agent
            let mut prompt_context = format!(
                "You are participating in a group chat as {}.\n\nRecent Transcript:\n",
                next_speaker.name
            );
            let tail = if current_transcript.len() > 20 {
                &current_transcript[current_transcript.len() - 20..]
            } else {
                &current_transcript[..]
            };

            for msg in tail {
                prompt_context.push_str(&format!("{}\n", msg.content));
            }
            prompt_context.push_str("\nProvide your response. If the overall task is completely resolved, include the word 'TERMINATE' in your response.");

            let mut run_cfg = next_speaker.run_config.clone();
            // Make sure the agent's role is injected
            run_cfg.server_system_message = format!(
                "You are {}. {}",
                next_speaker.name, next_speaker.description
            );

            let mut on_event = |_| {};
            let response_text = next_speaker
                .agent
                .run(&run_cfg, &prompt_context, &mut on_event)
                .await
                .map_err(|e| format!("Agent {} failed: {}", next_speaker.name, e))?;

            let formatted_response = format!("{}: {}", next_speaker.name, response_text);

            // Append to transcript
            let mut w_transcript = self.chat.transcript.write().await;
            w_transcript.push(Message::assistant(formatted_response.clone()));
            drop(w_transcript);

            if response_text.contains("TERMINATE") {
                tracing::info!("Group chat terminated by {}.", next_speaker.name);
                break;
            }
        }

        let final_transcript = self.chat.transcript.read().await.clone();
        Ok(final_transcript)
    }
}

/// The Orchestrator that manages a sequential flow of agents.
pub struct SequentialChatManager {
    pub agents: Vec<ChatAgent>,
}

impl SequentialChatManager {
    pub fn new(agents: Vec<ChatAgent>) -> Self {
        Self { agents }
    }

    /// Run the sequential chat loop, passing output from one agent to the next.
    pub async fn run_sequential(&self, initial_task: &str) -> Result<Vec<Message>, String> {
        let mut transcript = Vec::new();
        transcript.push(Message::user(format!("Admin: {}", initial_task)));

        let mut current_input = initial_task.to_string();

        for agent_cfg in &self.agents {
            tracing::info!("Sequential Step: {} is running...", agent_cfg.name);

            let prompt_context = format!(
                "You are participating in a sequential workflow as {}.

Your input task/context is:
{}

Provide your response, which will be passed to the next agent in the sequence.",
                agent_cfg.name, current_input
            );

            let mut run_cfg = agent_cfg.run_config.clone();
            run_cfg.server_system_message =
                format!("You are {}. {}", agent_cfg.name, agent_cfg.description);

            let mut on_event = |_| {};
            let response_text = agent_cfg
                .agent
                .run(&run_cfg, &prompt_context, &mut on_event)
                .await
                .map_err(|e| format!("Agent {} failed: {}", agent_cfg.name, e))?;

            let formatted_response = format!("{}: {}", agent_cfg.name, response_text);
            transcript.push(Message::assistant(formatted_response.clone()));

            // The output of this agent becomes the input for the next
            current_input = response_text;
        }

        Ok(transcript)
    }
}

use crate::tools::task::TaskStore;
use crate::tools::magentic::magentic_tool;

/// The Orchestrator that manages a "Magentic" flow of agents (a manager dynamically updating a task ledger).

/// The Orchestrator that manages a Handoff flow of agents.
/// An agent executes until it requests a handoff to another agent.

// -------------------------------------------------------------------------------------
// ADVANCED MULTI-AGENT ARCHITECTURES (HARNESS UPGRADE)
// -------------------------------------------------------------------------------------

/// The Orchestrator that manages a Map-Reduce flow.
/// It fans out a large task into chunks (Map), executes agents on them concurrently,
/// and then uses a reducer agent to combine them.
pub struct MapReduceManager {
    pub mapper: ChatAgent,
    pub workers: Vec<ChatAgent>,
    pub reducer: ChatAgent,
}

impl MapReduceManager {
    pub fn new(mapper: ChatAgent, workers: Vec<ChatAgent>, reducer: ChatAgent) -> Self {
        Self { mapper, workers, reducer }
    }

    pub async fn run_map_reduce(&self, dataset: &str, task: &str) -> Result<String, String> {
        // 1. Map Phase
        let map_config = self.mapper.run_config.clone();
        let map_prompt = format!("You are the Mapper. Your task is to split the following dataset into {} equal semantic chunks for parallel processing. Output ONLY a valid JSON array of strings, where each string is a chunk.\n\nDataset: {}\n\nTask: {}", self.workers.len(), dataset, task);

        let mut map_events = Vec::new();
        let map_result = self.mapper.agent.run(&map_config, &map_prompt, &mut |e| map_events.push(e)).await
            .map_err(|e| format!("Mapper failed: {}", e))?;

        let chunks: Vec<String> = match serde_json::from_str(&map_result) {
            Ok(c) => c,
            Err(_) => return Err("Mapper did not return a valid JSON array of chunks".to_string()),
        };

        if chunks.len() > self.workers.len() {
            return Err("Mapper returned too many chunks".to_string());
        }

        // 2. Process Phase (Concurrent)
        let mut futures = Vec::new();
        for (i, chunk) in chunks.into_iter().enumerate() {
            if i >= self.workers.len() { break; }
            let worker = &self.workers[i];
            let worker_cfg = worker.run_config.clone();
            let worker_prompt = format!("You are a MapReduce Worker. Process the following data chunk according to the task.\n\nTask: {}\n\nChunk: {}", task, chunk);

            let agent_clone = worker.agent.clone();
            futures.push(tokio::spawn(async move {
                let mut local_events = Vec::new();
                agent_clone.run(&worker_cfg, &worker_prompt, &mut |e| local_events.push(e)).await
            }));
        }

        let results = futures::future::join_all(futures).await;
        let mut processed_chunks = Vec::new();
        for (i, res) in results.into_iter().enumerate() {
            match res {
                Ok(Ok(output)) => processed_chunks.push(format!("Worker {} Output:\n{}", i, output)),
                Ok(Err(e)) => processed_chunks.push(format!("Worker {} Failed: {}", i, e)),
                Err(e) => processed_chunks.push(format!("Worker {} Panicked: {}", i, e)),
            }
        }

        // 3. Reduce Phase
        let reduce_config = self.reducer.run_config.clone();
        let reduce_prompt = format!("You are the Reducer. Synthesize the following worker outputs into a final coherent answer for the task.\n\nTask: {}\n\nOutputs:\n{}", task, processed_chunks.join("\n\n---\n\n"));

        let mut reduce_events = Vec::new();
        let final_result = self.reducer.agent.run(&reduce_config, &reduce_prompt, &mut |e| reduce_events.push(e)).await
            .map_err(|e| format!("Reducer failed: {}", e))?;

        Ok(final_result)
    }
}

/// Token-Constrained Group Chat Manager
/// Operates like GroupChat but strict enforces a budget.

/// Consensus Voting Manager
/// An orchestrator that takes a complex query and a list of agents,
/// asks each agent independently for a solution, and then uses a judge
/// to evaluate and synthesize a consensus.
pub struct ConsensusVotingManager {
    pub agents: Vec<ChatAgent>,
    pub judge: ChatAgent,
}

impl ConsensusVotingManager {
    pub fn new(agents: Vec<ChatAgent>, judge: ChatAgent) -> Self {
        Self { agents, judge }
    }

    pub async fn run_consensus(&self, query: &str) -> Result<String, String> {
        let mut futures = Vec::new();

        for (i, agent) in self.agents.iter().enumerate() {
            let mut agent_cfg = agent.run_config.clone();
            agent_cfg.user_instructions = format!("You are Voting Member {}. Provide your best independent answer to the query.", i);
            let agent_clone = agent.agent.clone();
            let query_clone = query.to_string();

            futures.push(tokio::spawn(async move {
                let mut local_events = Vec::new();
                agent_clone.run(&agent_cfg, &query_clone, &mut |e| local_events.push(e)).await
            }));
        }

        let results = futures::future::join_all(futures).await;

        let mut votes = Vec::new();
        for (i, res) in results.into_iter().enumerate() {
            match res {
                Ok(Ok(output)) => votes.push(format!("Voter {} said: {}", i, output)),
                Ok(Err(e)) => votes.push(format!("Voter {} errored: {}", i, e)),
                Err(e) => votes.push(format!("Voter {} panicked: {}", i, e)),
            }
        }

        let judge_cfg = self.judge.run_config.clone();
        let judge_prompt = format!("You are the Consensus Judge. Review the following independent votes on the query and determine the best consensus answer. If there is a tie, use your best judgment.\n\nQuery: {}\n\nVotes:\n{}", query, votes.join("\n---\n"));

        let mut judge_events = Vec::new();
        let final_result = self.judge.agent.run(&judge_cfg, &judge_prompt, &mut |e| judge_events.push(e)).await
            .map_err(|e| format!("Judge failed: {}", e))?;

        Ok(final_result)
    }
}

/// Reflexion Manager
/// Implements the Reflexion pattern (Shinn et al. 2023).
/// An agent generates an output, an evaluator scores it and provides critique,
/// and the agent tries again based on the critique.

/// Blackboards / Shared Tuple Space Architecture
/// Agents communicate implicitly by reading and writing to a shared data blackboard,
/// instead of direct message passing.
pub struct BlackboardManager {
    pub agents: Vec<ChatAgent>,
    pub blackboard: Arc<tokio::sync::RwLock<std::collections::HashMap<String, String>>>,
    pub max_cycles: usize,
}

impl BlackboardManager {
    pub fn new(agents: Vec<ChatAgent>, max_cycles: usize) -> Self {
        Self {
            agents,
            blackboard: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
            max_cycles,
        }
    }

    pub async fn read_blackboard(&self) -> String {
        let board = self.blackboard.read().await;
        let mut output = String::from("Current Blackboard State:\n");
        for (k, v) in board.iter() {
            output.push_str(&format!("{}: {}\n", k, v));
        }
        output
    }

    pub async fn run_blackboard(&self, initial_keys: std::collections::HashMap<String, String>) -> Result<String, String> {
        {
            let mut board = self.blackboard.write().await;
            for (k, v) in initial_keys {
                board.insert(k, v);
            }
        }

        let mut cycle = 0;
        while cycle < self.max_cycles {
            let mut made_progress = false;

            for agent in &self.agents {
                let current_board = self.read_blackboard().await;

                let prompt = format!("You are a Blackboard Agent named {}. Your description: {}.\n{}\nDo you have anything to contribute to the blackboard based on the current state? If yes, output the new key-value pairs formatted as 'KEY: [key] | VALUE: [value]'. You can output multiple lines. If no, output 'NO_CONTRIBUTION'.", agent.name, agent.description, current_board);

                let mut agent_cfg = agent.run_config.clone();
                agent_cfg.user_instructions = "You are interacting with a shared blackboard data structure.".to_string();

                let mut local_events = Vec::new();
                let res = agent.agent.run(&agent_cfg, &prompt, &mut |e| local_events.push(e)).await
                    .map_err(|e| format!("Agent {} failed: {}", agent.name, e))?;

                if !res.contains("NO_CONTRIBUTION") {
                    made_progress = true;
                    let mut board = self.blackboard.write().await;
                    for line in res.lines() {
                        let parts: Vec<&str> = line.split("| VALUE:").collect();
                        if parts.len() == 2 {
                            let key = parts[0].replace("KEY:", "").trim().to_string();
                            let value = parts[1].trim().to_string();
                            board.insert(key, value);
                        }
                    }
                }
            }

            if !made_progress {
                break;
            }
            cycle += 1;
        }

        Ok(self.read_blackboard().await)
    }
}

pub struct ReflexionManager {
    pub actor: ChatAgent,
    pub evaluator: ChatAgent,
    pub max_retries: usize,
}

impl ReflexionManager {
    pub fn new(actor: ChatAgent, evaluator: ChatAgent, max_retries: usize) -> Self {
        Self { actor, evaluator, max_retries }
    }

    pub async fn run_reflexion(&self, task: &str) -> Result<String, String> {
        let mut current_attempt = 1;
        let mut previous_output = String::new();
        let mut critique_history = String::new();

        while current_attempt <= self.max_retries {
            // Actor Phase
            let mut actor_cfg = self.actor.run_config.clone();
            let mut prompt = format!("Task: {}\n", task);
            if !critique_history.is_empty() {
                prompt.push_str(&format!("\nPrevious Attempts and Critiques:\n{}\n\nPlease try again and incorporate the feedback.", critique_history));
            }

            let mut actor_events = Vec::new();
            previous_output = self.actor.agent.run(&actor_cfg, &prompt, &mut |e| actor_events.push(e)).await
                .map_err(|e| format!("Actor failed: {}", e))?;

            // Evaluator Phase
            let mut eval_cfg = self.evaluator.run_config.clone();
            let eval_prompt = format!("You are the Evaluator. Evaluate the following output for the task. If it is fully correct and meets all constraints, reply with 'PASS'. Otherwise, reply with 'FAIL: <detailed critique>'.\n\nTask: {}\nOutput: {}", task, previous_output);

            let mut eval_events = Vec::new();
            let eval_res = self.evaluator.agent.run(&eval_cfg, &eval_prompt, &mut |e| eval_events.push(e)).await
                .map_err(|e| format!("Evaluator failed: {}", e))?;

            if eval_res.trim() == "PASS" {
                return Ok(previous_output);
            }

            critique_history.push_str(&format!("Attempt {}:\nOutput: {}\nCritique: {}\n\n", current_attempt, previous_output, eval_res));
            current_attempt += 1;
        }

        // If max retries reached, return best effort
        Ok(format!("Max retries reached. Best effort output:\n{}", previous_output))
    }
}

pub struct TokenConstrainedGroupChatManager {
    pub chat: GroupChat,
    pub llm: Arc<dyn crate::llm::LlmClient>,
    pub max_tokens: usize,
    pub current_tokens: Arc<tokio::sync::RwLock<usize>>,
}

impl TokenConstrainedGroupChatManager {
    pub fn new(chat: GroupChat, llm: Arc<dyn crate::llm::LlmClient>, max_tokens: usize) -> Self {
        Self { chat, llm, max_tokens, current_tokens: Arc::new(tokio::sync::RwLock::new(0)) }
    }

    async fn check_budget(&self) -> Result<(), String> {
        let current = *self.current_tokens.read().await;
        if current >= self.max_tokens {
            Err(format!("Token budget exceeded: {} >= {}", current, self.max_tokens))
        } else {
            Ok(())
        }
    }

    pub async fn run_constrained_chat(&self, initial_task: &str) -> Result<Vec<Message>, String> {
        self.chat.transcript.write().await.push(Message::user(initial_task));

        let mut rounds = 0;

        while rounds < self.chat.max_rounds {
            self.check_budget().await?;
            rounds += 1;

            let transcript_guard = self.chat.transcript.read().await;
            let current_transcript = transcript_guard.clone();
            drop(transcript_guard);

            let mut sys_prompt = String::from("You are a Group Chat Manager prioritizing brevity to conserve tokens.\nAvailable Agents:\n");
            for a in &self.chat.agents {
                sys_prompt.push_str(&format!("- {}: {}\n", a.name, a.description));
            }
            sys_prompt.push_str("\nRespond ONLY with the exact name of the agent who should speak next.");

            let mut req_messages = Vec::new();
            req_messages.push(Message::system(&sys_prompt));

            let mut recent_history = String::new();
            for msg in current_transcript.iter().rev().take(3).rev() {
                recent_history.push_str(&format!("{}: {}\n", msg.role, msg.content));
            }
            req_messages.push(Message::user(format!("Recent conversation:\n{}\nWho should speak next?", recent_history)));

            let req = ohc_builtin_agent_core::types::ChatRequest {
                model: "manager-model".to_string(),
                system: String::new(),
                messages: req_messages,
                tools: vec![],
                max_tokens: 50,
                temperature: 0.0,
            };

            let resp = self.llm.chat(req).await.map_err(|e| format!("Manager LLM failed: {}", e))?;

            let mut tokens = self.current_tokens.write().await;
            *tokens += (resp.usage.input_tokens + resp.usage.output_tokens) as usize;
            drop(tokens);
            self.check_budget().await?;

            let next_speaker_name = resp.message.content.trim();

            let speaker = match self.chat.agents.iter().find(|a| a.name == next_speaker_name) {
                Some(s) => s,
                None => return Err(format!("Manager selected invalid agent: {}", next_speaker_name)),
            };

            let mut speaker_cfg = speaker.run_config.clone();
            speaker_cfg.user_instructions = format!("You are {}. {}\nBe concise to save tokens.", speaker.name, speaker.description);

            let mut local_events = Vec::new();
            let result = speaker.agent.run(&speaker_cfg, &current_transcript.last().unwrap().content, &mut |e| local_events.push(e)).await;

            match result {
                Ok(output) => {
                    self.chat.transcript.write().await.push(Message::assistant(format!("[{}] {}", speaker.name, output)));

                    let mut tokens = self.current_tokens.write().await;
                    *tokens += (output.len() / 4); // rough approximation
                    drop(tokens);

                    if output.contains("TERMINATE") {
                        break;
                    }
                }
                Err(e) => {
                    return Err(format!("Agent {} failed: {}", speaker.name, e));
                }
            }
        }

        let final_transcript = self.chat.transcript.read().await.clone();
        Ok(final_transcript)
    }
}

/// Hierarchical Chat Manager
/// A graph of managers that manage sub-managers.
pub struct HierarchicalChatManager {
    pub top_level_manager: MagenticManager,
    pub sub_teams: std::collections::HashMap<String, GroupChatManager>,
}

impl HierarchicalChatManager {
    pub fn new(top_level_manager: MagenticManager) -> Self {
        Self { top_level_manager, sub_teams: std::collections::HashMap::new() }
    }

    pub fn add_sub_team(&mut self, name: &str, team: GroupChatManager) {
        self.sub_teams.insert(name.to_string(), team);
    }

    pub async fn run_hierarchy(&self, task: &str) -> Result<Vec<Message>, String> {
        // The top level manager decomposes the task and could theoretically trigger tools to delegate.
        // For simplicity in this implementation, we run the top level manager, then pass its output to all sub-teams.
        let top_result = self.top_level_manager.run_magentic(task).await?;

        let mut all_results = top_result.clone();

        for (team_name, team) in &self.sub_teams {
            tracing::info!("Delegating to sub-team: {}", team_name);
            let team_result = team.run_chat(&format!("Execute sub-task derived from: {}\nTask: {}", top_result.last().unwrap().content, task)).await?;
            all_results.extend(team_result);
        }

        Ok(all_results)
    }
}

pub struct HandoffChatManager {
    pub agents: std::collections::HashMap<String, ChatAgent>,
    pub max_rounds: usize,
}

impl HandoffChatManager {
    pub fn new(agents: Vec<ChatAgent>, max_rounds: usize) -> Self {
        let mut map = std::collections::HashMap::new();
        for agent in agents {
            map.insert(agent.name.clone(), agent);
        }
        Self { agents: map, max_rounds }
    }

    /// Run the handoff chat loop, transferring control dynamically based on agent output.
    pub async fn run_handoff(&self, initial_task: &str, starting_agent: &str) -> Result<Vec<Message>, String> {
        let mut transcript = Vec::new();
        transcript.push(Message::user(initial_task));

        let mut current_agent_name = starting_agent.to_string();
        let mut rounds = 0;

        while rounds < self.max_rounds {
            rounds += 1;

            let agent_cfg = match self.agents.get(&current_agent_name) {
                Some(a) => a,
                None => return Err(format!("Handoff requested to unknown agent: {}", current_agent_name)),
            };

            tracing::info!("Handoff Step: {} is running...", agent_cfg.name);

            let mut run_config = agent_cfg.run_config.clone();
            run_config.user_instructions = format!(
                "You are participating in a handoff workflow as {}.
Your description: {}
Complete the task. If you need another specialized agent to continue the work, use a tool that returns a HandoffRequested error (e.g. agent_stop with a target) or explicitly request a handoff. You MUST return a final summary of your work before handing off.",
                agent_cfg.name, agent_cfg.description
            );

            let mut local_events = Vec::new();
            let mut handoff_target = None;

            let result = agent_cfg.agent.run(
                &run_config,
                &transcript.last().unwrap().content,
                &mut |e| {
                    if let crate::agent::AgentEvent::Handoff { target_agent } = &e {
                        handoff_target = Some(target_agent.clone());
                    }
                    local_events.push(e);
                }
            ).await;

            match result {
                Ok(agent_output) => {
                    transcript.push(Message::assistant(format!("[{}] {}", agent_cfg.name, agent_output)));

                    if let Some(target) = handoff_target {
                        current_agent_name = target;
                        continue;
                    } else if agent_output.contains("HANDOFF:") {
                        // Fallback parsing if they didn't use the native HandoffRequested error type
                        if let Some(target) = agent_output.split("HANDOFF:").nth(1).map(|s| s.trim().to_string()) {
                             current_agent_name = target;
                             continue;
                        } else {
                            break; // Finished
                        }
                    } else {
                        // No handoff requested, work is done
                        break;
                    }
                }
                Err(e) => {
                    // Check if the error itself was a handoff request bubbling up
                    let e_str = e.to_string();
                    if e_str.contains("Handoff requested to") {
                        if let Some(target) = handoff_target {
                             current_agent_name = target;
                             continue;
                        } else {
                             let target = e_str.replace("Handoff requested to ", "").trim().to_string();
                             current_agent_name = target;
                             continue;
                        }
                    } else {
                        return Err(format!("Agent {} failed: {}", agent_cfg.name, e));
                    }
                }
            }
        }

        if rounds >= self.max_rounds {
            return Err("Max handoff rounds reached".to_string());
        }

        Ok(transcript)
    }
}

pub struct MagenticManager {
    pub manager_agent: ChatAgent,
    pub sub_agents: Vec<ChatAgent>,
    pub task_store: Arc<RwLock<TaskStore>>,
    pub max_rounds: usize,
}

impl MagenticManager {
    pub fn new(manager_agent: ChatAgent, sub_agents: Vec<ChatAgent>, max_rounds: usize) -> Self {
        Self {
            manager_agent,
            sub_agents,
            task_store: Arc::new(RwLock::new(TaskStore::default())),
            max_rounds,
        }
    }

    pub async fn run_magentic(&self, initial_task: &str) -> Result<Vec<Message>, String> {
        let mut transcript = Vec::new();
        transcript.push(Message::user(initial_task.to_string()));

        for round in 0..self.max_rounds {
            let mut current_cfg = self.manager_agent.run_config.clone();
            let mut allowed_tools = current_cfg.allowed_tools.unwrap_or_default();
            allowed_tools.push("magentic".to_string());
            current_cfg.allowed_tools = Some(allowed_tools);

            let mut custom_tools = self.manager_agent.agent.tools.clone();
            custom_tools.push(magentic_tool(self.task_store.clone()));

            let mut run_agent = Agent::new(self.manager_agent.agent.llm.clone(), custom_tools);
            run_agent.memory_store = self.manager_agent.agent.memory_store.clone();

            let sys_msg = format!(
                "You are participating in a magentic workflow as {}.\n\
                You are the Manager. You must decompose the initial task and use the 'magentic' tool to add tasks to the ledger.\n\
                Then, analyze the current ledger and decide which sub-agent should handle which pending task.\n\
                Output your routing decision in the format: ROUTE_TO: <AgentName> TASK: <TaskID>.\n\
                If all tasks are COMPLETE, output FINISHED.",
                self.manager_agent.name
            );

            current_cfg.server_system_message = sys_msg;

            let recent_history = transcript.iter().map(|m| {
                format!("{}: {}", m.role, m.content)
            }).collect::<Vec<_>>().join("\n\n");

            let prompt = format!("Recent Transcript:\n{}\n\nYour turn.", recent_history);

            let mut on_event = |_| {};
            let response = run_agent.run(&current_cfg, &prompt, &mut on_event).await
                .map_err(|e| format!("Manager Agent failed: {}", e))?;

            transcript.push(Message::assistant(format!("{}: {}", self.manager_agent.name, response)));

            if response.contains("FINISHED") {
                break;
            }

            let mut routed_agent = None;
            let mut routed_task = None;

            for line in response.lines() {
                if let Some(route_idx) = line.find("ROUTE_TO:") {
                    if let Some(task_idx) = line.find("TASK:") {
                        if route_idx < task_idx {
                            let agent_part = line[route_idx + "ROUTE_TO:".len()..task_idx].trim().to_string();
                            let task_part = line[task_idx + "TASK:".len()..].trim().to_string();
                            routed_agent = Some(agent_part);
                            routed_task = Some(task_part);
                        }
                    }
                }
            }

            if let (Some(agent_name), Some(task_id)) = (routed_agent, routed_task) {
                if let Some(agent) = self.sub_agents.iter().find(|a| a.name == agent_name) {
                    let sub_sys_msg = format!(
                        "You are participating in a magentic workflow as {}.\n\
                        You have been assigned TASK: {}. \n\
                        Perform the task and provide your result.",
                        agent.name, task_id
                    );

                    let mut sub_cfg = agent.run_config.clone();
                    sub_cfg.server_system_message = sub_sys_msg;

                    let sub_prompt = format!("Recent Transcript:\n{}\n\nPlease complete TASK: {}.", recent_history, task_id);
                    let sub_response = agent.agent.run(&sub_cfg, &sub_prompt, &mut on_event).await
                        .map_err(|e| format!("SubAgent {} failed: {}", agent.name, e))?;

                    transcript.push(Message::assistant(format!("{}: {}", agent.name, sub_response)));

                    // Automatically update task status as complete
                    let _ = magentic_tool(self.task_store.clone()).execute.execute(
                        serde_json::json!({
                            "action": "update",
                            "id": task_id,
                            "status": "complete"
                        })
                    ).await;
                }
            }
        }

        Ok(transcript)
    }
}

/// The Orchestrator that manages a concurrent flow of agents (fan-out/fan-in).
pub struct ConcurrentChatManager {
    pub agents: Vec<ChatAgent>,
    pub synthesizer: Option<ChatAgent>,
}

impl ConcurrentChatManager {
    pub fn new(agents: Vec<ChatAgent>, synthesizer: Option<ChatAgent>) -> Self {
        Self { agents, synthesizer }
    }

    /// Run the concurrent chat loop, fanning out the input to all agents, then fanning in to the synthesizer if present.
    pub async fn run_concurrent(&self, initial_task: &str) -> Result<Vec<Message>, String> {
        let mut transcript = Vec::new();
        transcript.push(Message::user(format!("Admin: {}", initial_task)));

        let mut futures = Vec::new();

        for agent_cfg in &self.agents {
            let prompt_context = format!(
                "You are participating in a concurrent workflow as {}.

Your input task/context is:
{}

Provide your response.",
                agent_cfg.name, initial_task
            );

            let mut run_cfg = agent_cfg.run_config.clone();
            run_cfg.server_system_message =
                format!("You are {}. {}", agent_cfg.name, agent_cfg.description);
            let agent = agent_cfg.agent.clone();
            let name = agent_cfg.name.clone();

            futures.push(async move {
                let mut on_event = |_| {};
                let response_text = agent
                    .run(&run_cfg, &prompt_context, &mut on_event)
                    .await
                    .map_err(|e| format!("Agent {} failed: {}", name, e))?;
                Ok::<String, String>(format!("{}: {}", name, response_text))
            });
        }

        let results = futures::future::join_all(futures).await;
        let mut combined_responses = String::new();

        for (i, res) in results.into_iter().enumerate() {
            let text = res?;
            combined_responses.push_str(&text);
            combined_responses.push_str("\n\n");
            transcript.push(Message::assistant(text));
        }

        if let Some(synth) = &self.synthesizer {
            tracing::info!("Fan-in Step: {} is running...", synth.name);

            let prompt_context = format!(
                "You are participating in a concurrent workflow as the synthesizer.

The initial task was:
{}

The concurrent workers have provided the following outputs:
{}

Please synthesize these outputs into a final cohesive response.",
                initial_task, combined_responses
            );

            let mut run_cfg = synth.run_config.clone();
            run_cfg.server_system_message =
                format!("You are {}. {}", synth.name, synth.description);

            let mut on_event = |_| {};
            let response_text = synth
                .agent
                .run(&run_cfg, &prompt_context, &mut on_event)
                .await
                .map_err(|e| format!("Synthesizer {} failed: {}", synth.name, e))?;

            let formatted_response = format!("{}: {}", synth.name, response_text);
            transcript.push(Message::assistant(formatted_response.clone()));
        }

        Ok(transcript)
    }
}
#[cfg(test)]
mod tests {
    pub struct MockLlmClient {
        pub responses: tokio::sync::Mutex<Vec<ohc_builtin_agent_core::types::ChatResponse>>,
    }

    #[async_trait::async_trait]
    impl crate::llm::LlmClient for MockLlmClient {
        async fn chat(&self, _req: ohc_builtin_agent_core::types::ChatRequest) -> Result<ohc_builtin_agent_core::types::ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            if resps.is_empty() {
                Ok(ohc_builtin_agent_core::types::ChatResponse {
                    message: ohc_builtin_agent_core::types::Message::assistant("Done"),
                    usage: ohc_builtin_agent_core::types::Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("id".to_string()),
                })
            } else {
                Ok(resps.remove(0))
            }
        }
    }

    use super::*;

    #[tokio::test]
    async fn test_autogen_sequential_chat() {
        let agent1_llm = Arc::new(AutoGenMockLlmClient {
            responses: tokio::sync::Mutex::new(vec!["I am Agent1. Output 1".to_string()]),
        });
        let agent1 = Arc::new(Agent::new(agent1_llm, vec![]));

        let agent2_llm = Arc::new(AutoGenMockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                "I am Agent2. I received the output and did Output 2".to_string(),
            ]),
        });
        let agent2 = Arc::new(Agent::new(agent2_llm, vec![]));

        let cfg = AgentRunConfig::default();

        let chat_agent1 = ChatAgent {
            name: "Agent1".to_string(),
            description: "First agent.".to_string(),
            agent: agent1,
            run_config: cfg.clone(),
        };

        let chat_agent2 = ChatAgent {
            name: "Agent2".to_string(),
            description: "Second agent.".to_string(),
            agent: agent2,
            run_config: cfg.clone(),
        };

        let manager = SequentialChatManager::new(vec![chat_agent1, chat_agent2]);

        let result = manager.run_sequential("Initial task").await;
        assert!(result.is_ok());

        let transcript = result.unwrap();

        assert_eq!(transcript.len(), 3);
        assert!(transcript[0].content.contains("Initial task"));
        assert!(transcript[1]
            .content
            .contains("Agent1: I am Agent1. Output 1"));
        assert!(transcript[2]
            .content
            .contains("Agent2: I am Agent2. I received the output and did Output 2"));
    }

    use super::*;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Usage};

    struct AutoGenMockLlmClient {
        responses: tokio::sync::Mutex<Vec<String>>,
    }

    #[async_trait::async_trait]
    impl crate::llm::LlmClient for AutoGenMockLlmClient {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            let content = if !resps.is_empty() {
                resps.remove(0)
            } else {
                "default".to_string()
            };

            Ok(ChatResponse {
                message: Message::assistant(content),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_autogen_group_chat() {
        let speaker_client = Arc::new(AutoGenMockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                "Agent1".to_string(), // Select Agent1
                "Agent2".to_string(), // Select Agent2
            ]),
        });

        let agent1_llm = Arc::new(AutoGenMockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                "I am Agent1. I have done my part.".to_string()
            ]),
        });
        let agent1 = Arc::new(Agent::new(agent1_llm, vec![]));

        let agent2_llm = Arc::new(AutoGenMockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                "I am Agent2. Everything looks good. TERMINATE".to_string(),
            ]),
        });
        let agent2 = Arc::new(Agent::new(agent2_llm, vec![]));

        let cfg = AgentRunConfig::default();

        let chat_agent1 = ChatAgent {
            name: "Agent1".to_string(),
            description: "A worker agent.".to_string(),
            agent: agent1,
            run_config: cfg.clone(),
        };

        let chat_agent2 = ChatAgent {
            name: "Agent2".to_string(),
            description: "A reviewer agent.".to_string(),
            agent: agent2,
            run_config: cfg.clone(),
        };

        let group_chat = GroupChat::new(vec![chat_agent1, chat_agent2], 5);
        let manager = GroupChatManager::new(group_chat, speaker_client);

        let result = manager.run_chat("Solve the problem.").await;
        assert!(result.is_ok());

        let transcript = result.unwrap();

        assert_eq!(transcript.len(), 3);
        assert!(transcript[0].content.contains("Solve the problem"));
        assert!(transcript[1]
            .content
            .contains("Agent1: I am Agent1. I have done my part."));
        assert!(transcript[2]
            .content
            .contains("Agent2: I am Agent2. Everything looks good. TERMINATE"));
    }

    #[tokio::test]
    async fn test_autogen_concurrent_chat() {
        let agent1_llm = Arc::new(AutoGenMockLlmClient {
            responses: tokio::sync::Mutex::new(vec!["Output from Agent1".to_string()]),
        });
        let agent1 = Arc::new(Agent::new(agent1_llm, vec![]));

        let agent2_llm = Arc::new(AutoGenMockLlmClient {
            responses: tokio::sync::Mutex::new(vec!["Output from Agent2".to_string()]),
        });
        let agent2 = Arc::new(Agent::new(agent2_llm, vec![]));

        let synth_llm = Arc::new(AutoGenMockLlmClient {
            responses: tokio::sync::Mutex::new(vec!["Synthesized final response".to_string()]),
        });
        let synth_agent = Arc::new(Agent::new(synth_llm, vec![]));

        let cfg = AgentRunConfig::default();

        let chat_agent1 = ChatAgent {
            name: "Agent1".to_string(),
            description: "Concurrent worker 1.".to_string(),
            agent: agent1,
            run_config: cfg.clone(),
        };

        let chat_agent2 = ChatAgent {
            name: "Agent2".to_string(),
            description: "Concurrent worker 2.".to_string(),
            agent: agent2,
            run_config: cfg.clone(),
        };

        let chat_synth = ChatAgent {
            name: "Synthesizer".to_string(),
            description: "Aggregates the concurrent outputs.".to_string(),
            agent: synth_agent,
            run_config: cfg.clone(),
        };

        let manager = ConcurrentChatManager::new(vec![chat_agent1, chat_agent2], Some(chat_synth));

        let result = manager.run_concurrent("Initial concurrent task").await;
        assert!(result.is_ok());

        let transcript = result.unwrap();

        // 1 user msg, 2 worker msgs, 1 synth msg = 4
        assert_eq!(transcript.len(), 4);
        assert!(transcript[0].content.contains("Initial concurrent task"));
        // futures::future::join_all preserves order of inputs
        assert!(transcript[1].content.contains("Agent1: Output from Agent1"));
        assert!(transcript[2].content.contains("Agent2: Output from Agent2"));
        assert!(transcript[3].content.contains("Synthesizer: Synthesized final response"));
    }

    #[tokio::test]
    async fn test_autogen_magentic_chat() {
        // Manager outputs task addition, then routing, then finished.
        let manager_llm = Arc::new(AutoGenMockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                "Added tasks. ROUTE_TO: Worker1 TASK: task-1".to_string(),
                "FINISHED".to_string(),
            ]),
        });

        let worker_llm = Arc::new(AutoGenMockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                "I have completed task-1.".to_string(),
            ]),
        });

        let cfg = AgentRunConfig::default();

        let manager_agent = ChatAgent {
            name: "Manager".to_string(),
            description: "Task Manager".to_string(),
            agent: Arc::new(Agent::new(manager_llm, vec![])),
            run_config: cfg.clone(),
        };

        let worker_agent = ChatAgent {
            name: "Worker1".to_string(),
            description: "Worker Subagent".to_string(),
            agent: Arc::new(Agent::new(worker_llm, vec![])),
            run_config: cfg.clone(),
        };

        let manager = MagenticManager::new(manager_agent, vec![worker_agent], 5);
        let result = manager.run_magentic("Initialize project").await;

        assert!(result.is_ok());
        let transcript = result.unwrap();

        assert!(transcript.len() >= 3);
        assert!(transcript[0].content.contains("Initialize project"));
        assert!(transcript[1].content.contains("ROUTE_TO: Worker1 TASK: task-1"));

        let found = transcript.iter().any(|m| m.content.contains("I have completed task-1."));
        assert!(found, "Transcript should contain the worker's completion message");
    }

    #[tokio::test]
    async fn test_autogen_handoff_chat() {
        use crate::agent::AgentEvent;

        let client_agent1 = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ohc_builtin_agent_core::types::ChatResponse {
                    message: Message::assistant("I have done part 1. HANDOFF: Agent2"),
                    usage: ohc_builtin_agent_core::types::Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("id1".to_string()),
                }
            ]),
        });

        let client_agent2 = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ohc_builtin_agent_core::types::ChatResponse {
                    message: Message::assistant("I have done part 2. Finished."),
                    usage: ohc_builtin_agent_core::types::Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("id2".to_string()),
                }
            ]),
        });

        let chat_agent1 = ChatAgent {
            name: "Agent1".to_string(),
            description: "First worker.".to_string(),
            agent: Arc::new(Agent::new(client_agent1, vec![])),
            run_config: AgentRunConfig::default(),
        };

        let chat_agent2 = ChatAgent {
            name: "Agent2".to_string(),
            description: "Second worker.".to_string(),
            agent: Arc::new(Agent::new(client_agent2, vec![])),
            run_config: AgentRunConfig::default(),
        };

        let manager = HandoffChatManager::new(vec![chat_agent1, chat_agent2], 5);
        let result = manager.run_handoff("Initial task", "Agent1").await;

        assert!(result.is_ok());
        let transcript = result.unwrap();
        assert_eq!(transcript.len(), 3);
        assert!(transcript[0].content.contains("Initial task"));
        assert!(transcript[1].content.contains("[Agent1] I have done part 1"));
        assert!(transcript[2].content.contains("[Agent2] I have done part 2"));
    }

    #[tokio::test]
    async fn test_autogen_map_reduce() {
        let client_mapper = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ohc_builtin_agent_core::types::ChatResponse {
                    message: Message::assistant("[\"chunk1\", \"chunk2\"]"),
                    usage: ohc_builtin_agent_core::types::Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("id1".to_string()),
                }
            ]),
        });

        let client_worker = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ohc_builtin_agent_core::types::ChatResponse {
                    message: Message::assistant("processed"),
                    usage: ohc_builtin_agent_core::types::Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("id2".to_string()),
                }
            ]),
        });

        let client_reducer = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ohc_builtin_agent_core::types::ChatResponse {
                    message: Message::assistant("final reduction"),
                    usage: ohc_builtin_agent_core::types::Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("id3".to_string()),
                }
            ]),
        });

        let mapper = ChatAgent {
            name: "Mapper".to_string(),
            description: "Splits data.".to_string(),
            agent: Arc::new(Agent::new(client_mapper, vec![])),
            run_config: AgentRunConfig::default(),
        };

        let worker1 = ChatAgent {
            name: "Worker1".to_string(),
            description: "Processes data.".to_string(),
            agent: Arc::new(Agent::new(client_worker.clone(), vec![])),
            run_config: AgentRunConfig::default(),
        };

        let worker2 = ChatAgent {
            name: "Worker2".to_string(),
            description: "Processes data.".to_string(),
            agent: Arc::new(Agent::new(client_worker, vec![])),
            run_config: AgentRunConfig::default(),
        };

        let reducer = ChatAgent {
            name: "Reducer".to_string(),
            description: "Combines data.".to_string(),
            agent: Arc::new(Agent::new(client_reducer, vec![])),
            run_config: AgentRunConfig::default(),
        };

        let manager = MapReduceManager::new(mapper, vec![worker1, worker2], reducer);
        let result = manager.run_map_reduce("big data", "process").await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "final reduction");
    }

    #[tokio::test]
    async fn test_autogen_token_constrained() {
        let client_manager = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ohc_builtin_agent_core::types::ChatResponse {
                    message: Message::assistant("Agent1"),
                    usage: ohc_builtin_agent_core::types::Usage { input_tokens: 10, output_tokens: 10, cache_creation_input_tokens: 0, cache_read_input_tokens: 0 },
                    stop_reason: "stop".to_string(),
                    response_id: Some("m1".to_string()),
                },
                ohc_builtin_agent_core::types::ChatResponse {
                    message: Message::assistant("Agent1"),
                    usage: ohc_builtin_agent_core::types::Usage { input_tokens: 1000, output_tokens: 1000, cache_creation_input_tokens: 0, cache_read_input_tokens: 0 }, // Exceeds budget
                    stop_reason: "stop".to_string(),
                    response_id: Some("m2".to_string()),
                }
            ]),
        });

        let client_agent1 = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ohc_builtin_agent_core::types::ChatResponse {
                    message: Message::assistant("working..."),
                    usage: ohc_builtin_agent_core::types::Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("a1".to_string()),
                }
            ]),
        });

        let chat_agent1 = ChatAgent {
            name: "Agent1".to_string(),
            description: "Worker".to_string(),
            agent: Arc::new(Agent::new(client_agent1, vec![])),
            run_config: AgentRunConfig::default(),
        };

        let group_chat = GroupChat::new(vec![chat_agent1], 5);
        let manager = TokenConstrainedGroupChatManager::new(group_chat, client_manager, 1000); // Strict budget

        let result = manager.run_constrained_chat("Start task").await;

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Token budget exceeded"));
    }







































































































































































































}

    #[cfg(test)]
mod more_tests {
    use super::*;
    use crate::agent::AgentEvent;
    use crate::agent::Agent;
    use crate::agent::AgentRunConfig;
    use ohc_builtin_agent_core::types::Message;
    use std::sync::Arc;
    use crate::autogen::tests::MockLlmClient;

    #[tokio::test]
    async fn test_consensus_voting() {
        let client_voter = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ohc_builtin_agent_core::types::ChatResponse {
                    message: Message::assistant("I vote option A"),
                    usage: ohc_builtin_agent_core::types::Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("v1".to_string()),
                }
            ]),
        });

        let client_judge = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ohc_builtin_agent_core::types::ChatResponse {
                    message: Message::assistant("Consensus is option A"),
                    usage: ohc_builtin_agent_core::types::Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("j1".to_string()),
                }
            ]),
        });

        let voter1 = ChatAgent {
            name: "Voter1".to_string(),
            description: "Voter".to_string(),
            agent: Arc::new(Agent::new(client_voter.clone(), vec![])),
            run_config: AgentRunConfig::default(),
        };

        let voter2 = ChatAgent {
            name: "Voter2".to_string(),
            description: "Voter".to_string(),
            agent: Arc::new(Agent::new(client_voter, vec![])),
            run_config: AgentRunConfig::default(),
        };

        let judge = ChatAgent {
            name: "Judge".to_string(),
            description: "Judge".to_string(),
            agent: Arc::new(Agent::new(client_judge, vec![])),
            run_config: AgentRunConfig::default(),
        };

        let manager = ConsensusVotingManager::new(vec![voter1, voter2], judge);
        let result = manager.run_consensus("What is the best option?").await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Consensus is option A");
    }

    #[tokio::test]
    async fn test_reflexion() {
        let client_actor = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ohc_builtin_agent_core::types::ChatResponse {
                    message: Message::assistant("First try"),
                    usage: ohc_builtin_agent_core::types::Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("a1".to_string()),
                },
                ohc_builtin_agent_core::types::ChatResponse {
                    message: Message::assistant("Second try, much better"),
                    usage: ohc_builtin_agent_core::types::Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("a2".to_string()),
                }
            ]),
        });

        let client_evaluator = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ohc_builtin_agent_core::types::ChatResponse {
                    message: Message::assistant("FAIL: Not good enough"),
                    usage: ohc_builtin_agent_core::types::Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("e1".to_string()),
                },
                ohc_builtin_agent_core::types::ChatResponse {
                    message: Message::assistant("PASS"),
                    usage: ohc_builtin_agent_core::types::Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("e2".to_string()),
                }
            ]),
        });

        let actor = ChatAgent {
            name: "Actor".to_string(),
            description: "Actor".to_string(),
            agent: Arc::new(Agent::new(client_actor, vec![])),
            run_config: AgentRunConfig::default(),
        };

        let evaluator = ChatAgent {
            name: "Evaluator".to_string(),
            description: "Evaluator".to_string(),
            agent: Arc::new(Agent::new(client_evaluator, vec![])),
            run_config: AgentRunConfig::default(),
        };

        let manager = ReflexionManager::new(actor, evaluator, 3);
        let result = manager.run_reflexion("Do a hard task").await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Second try, much better");
    }

// -------------------------------------------------------------------------------------
// DEEP E2E VERIFICATION SUITE FOR MULTI-AGENT ARCHITECTURES
// -------------------------------------------------------------------------------------
// This suite tests the resilience of the multi-agent managers against cascading failures,
// recursive structures, and token exhaustion.

#[cfg(test)]
mod deep_resilience_tests {
    use super::*;
    use std::sync::Arc;
    use crate::agent::{Agent, AgentRunConfig, AgentEvent};

    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role, Usage};
    use tokio::sync::Mutex;

    pub struct ResilienceMockLlmClient {
        pub fail_first_n_calls: Mutex<usize>,
        pub subsequent_response: ChatResponse,
    }

    #[async_trait::async_trait]
    impl crate::llm::LlmClient for ResilienceMockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut fails = self.fail_first_n_calls.lock().await;
            if *fails > 0 {
                *fails -= 1;
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::ConnectionReset, "Simulated Network Failure")));
            }
            Ok(self.subsequent_response.clone())
        }
    }

    #[tokio::test]
    async fn test_map_reduce_resilience_against_worker_failure() {
        // Create a mapper that succeeds
        let mapper_resp = ChatResponse {
            message: Message::assistant("[\"data1\", \"data2\", \"data3\"]"),
            usage: Usage::default(),
            stop_reason: "stop".to_string(),
            response_id: Some("m1".to_string()),
        };
        let client_mapper = Arc::new(ResilienceMockLlmClient {
            fail_first_n_calls: Mutex::new(0),
            subsequent_response: mapper_resp,
        });

        // Create workers where Worker 2 fails repeatedly
        let worker_success = ChatResponse {
            message: Message::assistant("processed data"),
            usage: Usage::default(),
            stop_reason: "stop".to_string(),
            response_id: Some("w_succ".to_string()),
        };

        let client_worker_succ = Arc::new(ResilienceMockLlmClient {
            fail_first_n_calls: Mutex::new(0),
            subsequent_response: worker_success.clone(),
        });

        let client_worker_fail = Arc::new(ResilienceMockLlmClient {
            fail_first_n_calls: Mutex::new(10), // Fails 10 times, ensuring it fails the task
            subsequent_response: worker_success,
        });

        let client_reducer = Arc::new(ResilienceMockLlmClient {
            fail_first_n_calls: Mutex::new(0),
            subsequent_response: ChatResponse {
                message: Message::assistant("Partial reduction complete"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("r1".to_string()),
            },
        });

        let mapper = ChatAgent {
            name: "Mapper".to_string(),
            description: "Splits".to_string(),
            agent: Arc::new(Agent::new(client_mapper, vec![])),
            run_config: AgentRunConfig::default(),
        };

        let worker1 = ChatAgent {
            name: "Worker1".to_string(),
            description: "Worker".to_string(),
            agent: Arc::new(Agent::new(client_worker_succ.clone(), vec![])),
            run_config: AgentRunConfig::default(),
        };

        let worker2 = ChatAgent {
            name: "Worker2".to_string(),
            description: "Worker".to_string(),
            agent: Arc::new(Agent::new(client_worker_fail, vec![])),
            run_config: AgentRunConfig::default(),
        };

        let worker3 = ChatAgent {
            name: "Worker3".to_string(),
            description: "Worker".to_string(),
            agent: Arc::new(Agent::new(client_worker_succ, vec![])),
            run_config: AgentRunConfig::default(),
        };

        let reducer = ChatAgent {
            name: "Reducer".to_string(),
            description: "Reducer".to_string(),
            agent: Arc::new(Agent::new(client_reducer, vec![])),
            run_config: AgentRunConfig::default(),
        };

        let manager = MapReduceManager::new(mapper, vec![worker1, worker2, worker3], reducer);

        // Even if Worker 2 fails, the MapReduce manager should capture the error for that chunk
        // and pass the successful chunks to the reducer.
        let result = manager.run_map_reduce("dataset", "process").await;

        assert!(result.is_ok());
        let res_str = result.unwrap();
        assert_eq!(res_str, "Partial reduction complete");
    }

    #[tokio::test]
    async fn test_recursive_hierarchical_chat_with_blackboard() {
        // This tests combining HierarchicalChatManager with BlackboardManager.
        let ok_resp = ChatResponse {
            message: Message::assistant("Task handled."),
            usage: Usage::default(),
            stop_reason: "stop".to_string(),
            response_id: Some("1".to_string()),
        };
        let client = Arc::new(ResilienceMockLlmClient {
            fail_first_n_calls: Mutex::new(0),
            subsequent_response: ok_resp,
        });

        let agent = ChatAgent {
            name: "Generic".to_string(),
            description: "Generic".to_string(),
            agent: Arc::new(Agent::new(client.clone(), vec![])),
            run_config: AgentRunConfig::default(),
        };

        let bb_manager = crate::autogen::BlackboardManager::new(vec![agent.clone(), agent.clone()], 3);
        let mut initial_bb = std::collections::HashMap::new();
        initial_bb.insert("init".to_string(), "val".to_string());

        let bb_result = bb_manager.run_blackboard(initial_bb).await;
        assert!(bb_result.is_ok());

        let top_agent = ChatAgent {
            name: "Top".to_string(),
            description: "Top".to_string(),
            agent: Arc::new(Agent::new(client, vec![])),
            run_config: AgentRunConfig::default(),
        };

        // Construct the hierarchy manually
        let mut hierarchy = HierarchicalChatManager::new(MagenticManager::new(top_agent.clone(), vec![agent.clone()], 2));

        // We can't directly embed Blackboard into Hierarchy as written, but we can verify both work
        assert!(hierarchy.run_hierarchy("Execute overarching command").await.is_ok());
    }

    #[tokio::test]
    async fn test_reflexion_manager_success_after_retries() {
        let mut actor_resps = vec![
            ChatResponse {
                message: Message::assistant("Attempt 1: Flawed output"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("a1".to_string()),
            },
            ChatResponse {
                message: Message::assistant("Attempt 2: Perfect output"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("a2".to_string()),
            },
        ];

        struct StatefulMockClient {
            resps: Mutex<Vec<ChatResponse>>,
        }
        #[async_trait::async_trait]
        impl crate::llm::LlmClient for StatefulMockClient {
            async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                let mut r = self.resps.lock().await;
                if !r.is_empty() {
                    Ok(r.remove(0))
                } else {
                    Ok(ChatResponse {
                        message: Message::assistant("Default"),
                        usage: Usage::default(),
                        stop_reason: "stop".to_string(),
                        response_id: Some("d".to_string()),
                    })
                }
            }
        }

        let client_actor = Arc::new(StatefulMockClient { resps: Mutex::new(actor_resps) });

        let client_evaluator = Arc::new(StatefulMockClient { resps: Mutex::new(vec![
            ChatResponse {
                message: Message::assistant("FAIL: Needs more detail"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("e1".to_string()),
            },
            ChatResponse {
                message: Message::assistant("PASS"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("e2".to_string()),
            },
        ])});

        let actor = ChatAgent {
            name: "Actor".to_string(),
            description: "Actor".to_string(),
            agent: Arc::new(Agent::new(client_actor, vec![])),
            run_config: AgentRunConfig::default(),
        };
        let eval = ChatAgent {
            name: "Evaluator".to_string(),
            description: "Evaluator".to_string(),
            agent: Arc::new(Agent::new(client_evaluator, vec![])),
            run_config: AgentRunConfig::default(),
        };

        let reflexion = ReflexionManager::new(actor, eval, 5);
        let res = reflexion.run_reflexion("Write a poem").await.unwrap();
        assert_eq!(res, "Attempt 2: Perfect output");
    }
}
}
