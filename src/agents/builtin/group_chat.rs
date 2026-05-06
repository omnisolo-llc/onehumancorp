use crate::agent::{Agent, AgentRunConfig};
use ohc_builtin_agent_core::types::{ChatRequest, Message};
use std::sync::Arc;
use crate::llm::LlmClient;

/// AutoGen Mechanic: Group Chat Participant
#[derive(Clone)]
pub struct GroupChatParticipant {
    pub name: String,
    pub description: String,
    pub system_message: String,
    pub agent: Arc<Agent>,
    pub config: AgentRunConfig,
}

impl GroupChatParticipant {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        system_message: impl Into<String>,
        agent: Arc<Agent>,
        config: AgentRunConfig,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            system_message: system_message.into(),
            agent,
            config,
        }
    }
}

/// AutoGen Mechanic: Group Chat Manager
/// Oversees a group of agents, dynamically selecting the next speaker based on the conversation history.
pub struct GroupChat {
    pub participants: Vec<GroupChatParticipant>,
    pub manager_llm: Arc<dyn LlmClient>,
    pub history: Vec<Message>,
    pub max_turns: usize,
}

impl GroupChat {
    pub fn new(participants: Vec<GroupChatParticipant>, manager_llm: Arc<dyn LlmClient>, max_turns: usize) -> Self {
        Self {
            participants,
            manager_llm,
            history: Vec::new(),
            max_turns,
        }
    }

    /// Run the group chat loop
    pub async fn run(&mut self, initial_task: &str) -> Result<String, String> {
        self.history.push(Message::user(format!("Task: {}", initial_task)));

        let mut turn = 0;

        while turn < self.max_turns {
            // 1. Select Next Speaker
            let speaker_index = self.select_next_speaker().await?;
            let speaker = &self.participants[speaker_index];

            tracing::info!("Group Chat Turn {}: Selected Speaker: {}", turn, speaker.name);

            // 2. Prepare Context for Speaker
            let mut prompt_context = String::new();
            prompt_context.push_str("You are in a group chat. The history of the chat is:\n");
            for msg in &self.history {
                let prefix = match msg.role {
                    ohc_builtin_agent_core::types::Role::User => "Human",
                    ohc_builtin_agent_core::types::Role::Assistant => "Assistant/Speaker",
                    ohc_builtin_agent_core::types::Role::System => "System",
                    ohc_builtin_agent_core::types::Role::Tool => "Tool",
                };
                prompt_context.push_str(&format!("[{}]: {}\n", prefix, msg.content));
            }
            prompt_context.push_str(&format!("\nYour role is: {}\nYour task: Continue the group chat and provide your contribution.", speaker.name));

            let mut run_cfg = speaker.config.clone();
            run_cfg.server_system_message = speaker.system_message.clone();

            let mut on_event = |_| {};

            // 3. Execute Speaker Agent
            let result = speaker.agent.run(&run_cfg, &prompt_context, &mut on_event).await
                .map_err(|e| format!("Agent {} failed: {}", speaker.name, e))?;

            // 4. Record Response
            let formatted_response = format!("{}: {}", speaker.name, result);
            self.history.push(Message::assistant(formatted_response));

            // Stop condition check
            if result.to_lowercase().contains("task complete") || result.to_lowercase().contains("terminate") {
                tracing::info!("Group Chat terminated by speaker.");
                break;
            }

            turn += 1;
        }

        if let Some(last_msg) = self.history.last() {
            Ok(last_msg.content.clone())
        } else {
            Err("No history generated.".to_string())
        }
    }

    /// Ask the LLM to pick the next speaker based on the current history.
    async fn select_next_speaker(&self) -> Result<usize, String> {
        let mut system_prompt = String::from(
            "You are the manager of a group chat. Your job is to select the next speaker based on the conversation history. \
             You MUST respond with exactly the NAME of the next speaker, and nothing else. \n\nAvailable speakers:\n"
        );

        for p in &self.participants {
            system_prompt.push_str(&format!("- {} (Role: {})\n", p.name, p.description));
        }

        let mut history_str = String::new();
        for msg in &self.history {
            history_str.push_str(&format!("{}: {}\n", msg.role, msg.content));
        }

        let prompt = format!("Chat history:\n{}\n\nWho should speak next? Respond with exactly one name from the available speakers list.", history_str);

        let req = ChatRequest {
            model: "manager-model".to_string(), // In production, we might read from a config
            system: system_prompt,
            messages: vec![Message::user(prompt)],
            tools: vec![],
            max_tokens: 50,
            temperature: 0.0,
        };

        let resp = self.manager_llm.chat(req).await.map_err(|e| format!("Manager LLM error: {}", e))?;
        let selection = resp.message.content.trim();

        for (i, p) in self.participants.iter().enumerate() {
            // Case-insensitive exact match or contains check
            if selection.to_lowercase().contains(&p.name.to_lowercase()) {
                return Ok(i);
            }
        }

        // Fallback to round-robin if the manager hallucinated
        let next_idx = if self.participants.is_empty() { return Err("No participants in group chat".to_string()); } else { self.history.iter().filter(|m| m.role == ohc_builtin_agent_core::types::Role::Assistant).count() % self.participants.len() };
        tracing::warn!("Manager selected unknown speaker '{}'. Falling back to {}", selection, self.participants[next_idx].name);
        Ok(next_idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmClient;
    use ohc_builtin_agent_core::types::{ChatResponse, Usage};
    use tokio::sync::Mutex;

    struct MockManagerLlm {
        responses: Mutex<Vec<String>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockManagerLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            let content = if !resps.is_empty() {
                resps.remove(0)
            } else {
                "Alice".to_string() // Fallback
            };

            Ok(ChatResponse {
                message: Message::assistant(content),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    struct MockAgentLlm {
        content: String,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockAgentLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant(self.content.clone()),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_autogen_group_chat() {
        let manager_llm = Arc::new(MockManagerLlm {
            responses: Mutex::new(vec![
                "Alice".to_string(), // Turn 1: Alice
                "Bob".to_string(),   // Turn 2: Bob
                "Alice".to_string(), // Turn 3: Alice
            ]),
        });

        let alice_llm = Arc::new(MockAgentLlm { content: "I am Alice".to_string() });
        let bob_llm = Arc::new(MockAgentLlm { content: "I am Bob, TERMINATE".to_string() });

        let alice_agent = Arc::new(Agent::new(alice_llm, vec![]));
        let bob_agent = Arc::new(Agent::new(bob_llm, vec![]));

        let p1 = GroupChatParticipant::new("Alice", "SWE", "You are a SWE", alice_agent, AgentRunConfig::default());
        let p2 = GroupChatParticipant::new("Bob", "QA", "You are QA", bob_agent, AgentRunConfig::default());

        let mut chat = GroupChat::new(vec![p1, p2], manager_llm, 10);
        let result = chat.run("Fix the bug").await.unwrap();

        assert!(result.contains("Bob"));
        assert!(result.contains("TERMINATE"));
        // History should contain: 1 User task, 1 Alice response, 1 Bob response (which terminates)
        assert_eq!(chat.history.len(), 3);
    }
}
