use crate::agent::{Agent, AgentRunConfig};
use ohc_builtin_agent_core::types::Message;
use std::sync::Arc;
use tokio::sync::RwLock;

/// AutoGen: Architecture consists of Core, AgentChat, and Extensions. Implements 5 mechanical patterns: sequential, concurrent (fan-out/fan-in), group chat, handoff, and magentic (manager agent dynamically updating a task ledger).

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

        for _round in 0..self.max_rounds {
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

        for (_i, res) in results.into_iter().enumerate() {
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

/// The Orchestrator that manages a handoff flow between agents.
pub struct HandoffManager {
    pub agents: Vec<ChatAgent>,
    pub max_rounds: usize,
}

impl HandoffManager {
    pub fn new(agents: Vec<ChatAgent>, max_rounds: usize) -> Self {
        Self { agents, max_rounds }
    }

    /// Run the handoff chat loop.
    pub async fn run_handoff(&self, initial_task: &str, start_agent_name: &str) -> Result<Vec<Message>, String> {
        let mut transcript = Vec::new();
        transcript.push(Message::user(format!("Admin: {}", initial_task)));

        let mut current_agent_name = start_agent_name.to_string();
        let mut prompt_context = format!("Your task is to handle the following: {}. You may ask to handoff to another agent if needed.", initial_task);

        for _round in 0..self.max_rounds {
            let agent_cfg = self.agents.iter().find(|a| a.name == current_agent_name)
                .ok_or_else(|| format!("Agent {} not found", current_agent_name))?;

            tracing::info!("Handoff Step: {} is running...", agent_cfg.name);

            let mut run_cfg = agent_cfg.run_config.clone();
            run_cfg.server_system_message = format!("You are {}. {}", agent_cfg.name, agent_cfg.description);

            let mut handoff_target = None;
            let mut on_event = |e: crate::agent::AgentEvent| {
                if let crate::agent::AgentEvent::Handoff { target_agent } = e {
                    handoff_target = Some(target_agent);
                }
            };
            let result = agent_cfg.agent.run(&run_cfg, &prompt_context, &mut on_event).await;

            match result {
                Ok(response_text) => {
                    // Check if handoff was requested via AgentEvent
                    if let Some(target) = handoff_target {
                        // Add to transcript
                        transcript.push(Message::assistant(format!("{}: [Handoff requested to {}]", agent_cfg.name, target)));

                        // Update current agent and prompt context
                        current_agent_name = target.clone();
                        let recent_history = transcript.iter().map(|m| format!("{}: {}", m.role, m.content)).collect::<Vec<_>>().join("\n\n");
                        prompt_context = format!("You have received a handoff. Recent Transcript:\n{}\n\nPlease continue the task.", recent_history);
                    } else {
                        transcript.push(Message::assistant(format!("{}: {}", agent_cfg.name, response_text)));
                        // Completed successfully without a handoff.
                        return Ok(transcript);
                    }
                }
                Err(e) => {
                    return Err(format!("Agent {} failed: {}", current_agent_name, e));
                }
            }
        }

        Err(format!("Handoff flow reached max rounds ({}) without completing.", self.max_rounds))
    }
}

#[cfg(test)]
mod tests {
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
        let _agent1_llm = Arc::new(AutoGenMockLlmClient {
            responses: tokio::sync::Mutex::new(vec!["Handoff requested to Agent2".to_string()]),
        });

        let agent2_llm = Arc::new(AutoGenMockLlmClient {
            responses: tokio::sync::Mutex::new(vec!["I will handle this now.".to_string()]),
        });

        // We need the mock to return an error of type ToolError::Unexpected("HandoffRequested,
        // but AutoGenMockLlmClient returns Ok(ChatResponse).
        // To test HandoffManager, we need an agent that triggers the handoff error.
        // We can create a mock tool that requests handoff.

        struct MockHandoffTool;
        #[async_trait::async_trait]
        impl crate::tools::ToolExecutor for MockHandoffTool {
            async fn execute(&self, _args: serde_json::Value) -> Result<String, crate::types::ToolError> {
                Err(crate::types::ToolError::Unexpected("HandoffRequested: Agent2".to_string()))
            }
        }

        let _agent1_llm_tool = Arc::new(AutoGenMockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                "I need to call the handoff tool".to_string() // Won't be used if we just mock the tool directly, wait we need to trigger it.
            ]),
        });

        // Let's use a simpler approach. We mock the LlmClient to just return a message with a tool call to the handoff tool.
        struct HandoffLlmClient;
        #[async_trait::async_trait]
        impl crate::llm::LlmClient for HandoffLlmClient {
            async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                Ok(ChatResponse {
                    message: ohc_builtin_agent_core::types::Message {
                        role: ohc_builtin_agent_core::types::Role::Assistant,
                        content: "".to_string(),
                        tool_calls: vec![ohc_builtin_agent_core::types::ToolCall {
                            id: "call_1".to_string(),
                            name: "handoff_tool".to_string(),
                            arguments: serde_json::json!({}),
                        }],
                        tool_results: vec![],
                        response_id: None,
                        previous_response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                    response_id: Some("id1".to_string()),
                })
            }
        }

        let agent1 = Arc::new(Agent::new(Arc::new(HandoffLlmClient), vec![
            crate::tools::Tool {
                name: "handoff_tool".to_string(),
                description: "handoff".to_string(),
                is_read_only: false,
                parameters: serde_json::json!({}),
                execute: Arc::new(MockHandoffTool),
            }
        ]));

        let agent2 = Arc::new(Agent::new(agent2_llm, vec![]));

        let cfg = AgentRunConfig::default();

        let chat_agent1 = ChatAgent {
            name: "Agent1".to_string(),
            description: "Agent 1".to_string(),
            agent: agent1,
            run_config: cfg.clone(),
        };

        let chat_agent2 = ChatAgent {
            name: "Agent2".to_string(),
            description: "Agent 2".to_string(),
            agent: agent2,
            run_config: cfg.clone(),
        };

        let manager = HandoffManager::new(vec![chat_agent1, chat_agent2], 5);
        let result = manager.run_handoff("Start task", "Agent1").await;

        assert!(result.is_ok(), "run_handoff failed: {:?}", result.unwrap_err());
        let transcript = result.unwrap();

        assert!(transcript.len() >= 3, "Transcript too short: {:?}", transcript);
        assert!(transcript[0].content.contains("Start task"));
        assert!(transcript[1].content.contains("Agent1: [Handoff requested to Agent2]"));
        assert!(transcript[2].content.contains("Agent2: I will handle this now."));
    }
}
