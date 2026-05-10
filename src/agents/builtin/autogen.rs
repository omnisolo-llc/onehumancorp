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

/// The Orchestrator that manages a concurrent fan-out/fan-in flow of agents.
pub struct ConcurrentChatManager {
    pub workers: Vec<ChatAgent>,
    pub aggregator: ChatAgent,
}

impl ConcurrentChatManager {
    pub fn new(workers: Vec<ChatAgent>, aggregator: ChatAgent) -> Self {
        Self { workers, aggregator }
    }

    /// Run the concurrent chat loop, fanning out to workers and fanning in to the aggregator.
    pub async fn run_concurrent(&self, initial_task: &str) -> Result<Vec<Message>, String> {
        let mut transcript = Vec::new();
        transcript.push(Message::user(format!("Admin: {}", initial_task)));

        let mut futures = Vec::new();
        for worker in &self.workers {
            let worker_cfg = worker.clone();
            let task = initial_task.to_string();
            futures.push(tokio::spawn(async move {
                tracing::info!("Concurrent Step: {} is running...", worker_cfg.name);
                let prompt_context = format!(
                    "You are participating in a concurrent workflow as {}.\n\nYour input task/context is:\n{}\n\nProvide your response.",
                    worker_cfg.name, task
                );
                let mut run_cfg = worker_cfg.run_config.clone();
                run_cfg.server_system_message = format!("You are {}. {}", worker_cfg.name, worker_cfg.description);
                let mut on_event = |_| {};
                let response_text = worker_cfg.agent.run(&run_cfg, &prompt_context, &mut on_event).await
                    .map_err(|e| format!("Agent {} failed: {}", worker_cfg.name, e))?;
                Ok::<String, String>(format!("{}: {}", worker_cfg.name, response_text))
            }));
        }

        let join_results = futures::future::join_all(futures).await;
        let mut results = Vec::new();
        for join_res in join_results {
            match join_res {
                Ok(Ok(res)) => {
                    transcript.push(Message::assistant(res.clone()));
                    results.push(res);
                }
                Ok(Err(e)) => return Err(e),
                Err(e) => return Err(format!("Task panicked or cancelled: {}", e)),
            }
        }

        tracing::info!("Concurrent Step: Aggregator {} is running...", self.aggregator.name);
        let aggregated_input = results.join("\n\n---\n\n");
        let prompt_context = format!("You are participating in a concurrent workflow as an aggregator ({}).\n\nYour input task/context is:\n{}\n\nHere are the results from the concurrent workers:\n{}\n\nProvide your final aggregated response.", self.aggregator.name, initial_task, aggregated_input);
        let mut run_cfg = self.aggregator.run_config.clone();
        run_cfg.server_system_message = format!("You are {}. {}", self.aggregator.name, self.aggregator.description);
        let mut on_event = |_| {};
        let response_text = self.aggregator.agent.run(&run_cfg, &prompt_context, &mut on_event).await.map_err(|e| format!("Aggregator {} failed: {}", self.aggregator.name, e))?;
        transcript.push(Message::assistant(format!("{}: {}", self.aggregator.name, response_text)));

        Ok(transcript)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_autogen_concurrent_chat() {
        let agent1_llm = Arc::new(AutoGenMockLlmClient {
            responses: tokio::sync::Mutex::new(vec!["Worker1 output".to_string()]),
        });
        let agent1 = Arc::new(Agent::new(agent1_llm, vec![]));

        let agent2_llm = Arc::new(AutoGenMockLlmClient {
            responses: tokio::sync::Mutex::new(vec!["Worker2 output".to_string()]),
        });
        let agent2 = Arc::new(Agent::new(agent2_llm, vec![]));

        let aggregator_llm = Arc::new(AutoGenMockLlmClient {
            responses: tokio::sync::Mutex::new(vec!["Aggregated output".to_string()]),
        });
        let aggregator_agent = Arc::new(Agent::new(aggregator_llm, vec![]));

        let cfg = AgentRunConfig::default();

        let worker1 = ChatAgent {
            name: "Worker1".to_string(),
            description: "First concurrent worker.".to_string(),
            agent: agent1,
            run_config: cfg.clone(),
        };

        let worker2 = ChatAgent {
            name: "Worker2".to_string(),
            description: "Second concurrent worker.".to_string(),
            agent: agent2,
            run_config: cfg.clone(),
        };

        let aggregator = ChatAgent {
            name: "Aggregator".to_string(),
            description: "Aggregates the concurrent results.".to_string(),
            agent: aggregator_agent,
            run_config: cfg.clone(),
        };

        let manager = ConcurrentChatManager::new(vec![worker1, worker2], aggregator);

        let result = manager.run_concurrent("Initial concurrent task").await;
        assert!(result.is_ok());

        let transcript = result.unwrap();

        assert_eq!(transcript.len(), 4);
        assert!(transcript[0].content.contains("Initial concurrent task"));
        assert!(transcript[1].content.contains("Worker1: Worker1 output") || transcript[1].content.contains("Worker2: Worker2 output"));
        assert!(transcript[2].content.contains("Worker1: Worker1 output") || transcript[2].content.contains("Worker2: Worker2 output"));
        assert!(transcript[3].content.contains("Aggregator: Aggregated output"));
    }

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
}
