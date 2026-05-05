use crate::agent::{Agent, AgentRunConfig};
use ohc_builtin_agent_core::types::{ChatRequest, Message, Role};
use crate::llm::LlmClient;
use std::sync::Arc;

/// A participating agent in the Group Chat.
#[derive(Debug, Clone)]
pub struct GroupChatParticipant {
    pub name: String,
    pub description: String,
    pub system_message: String,
}

/// The GroupChat holds the participants and conversation history,
/// and implements the speaker selection mechanic using an LLM.
pub struct GroupChat {
    pub participants: Vec<GroupChatParticipant>,
    pub history: Vec<Message>,
    pub max_rounds: usize,
    llm: Arc<dyn LlmClient>,
    model: String,
}

impl GroupChat {
    pub fn new(participants: Vec<GroupChatParticipant>, max_rounds: usize, llm: Arc<dyn LlmClient>, model: String) -> Self {
        Self {
            participants,
            history: Vec::new(),
            max_rounds,
            llm,
            model,
        }
    }

    /// Speaker Selection: asks the LLM who should speak next based on the history.
    pub async fn select_next_speaker(&self) -> Result<String, String> {
        let mut prompt = String::from("You are a speaker selector in a group chat. Based on the conversation history, select the next speaker from the following list:\n");
        for p in &self.participants {
            prompt.push_str(&format!("- {}: {}\n", p.name, p.description));
        }
        prompt.push_str("\nOutput ONLY the exact name of the selected speaker, with no extra text or markdown formatting.\n\nConversation History:\n");

        for msg in &self.history {
            prompt.push_str(&format!("{}: {}\n", msg.role, msg.content));
        }

        let req = ChatRequest {
            model: self.model.clone(),
            system: "You are an expert speaker selector for a group chat.".to_string(),
            messages: vec![Message::user(prompt)],
            tools: vec![],
            max_tokens: 50,
            temperature: 0.0,
        };

        match self.llm.chat(req).await {
            Ok(resp) => {
                let name = resp.message.content.trim().to_string();
                if self.participants.iter().any(|p| p.name == name) {
                    Ok(name)
                } else {
                    // Fallback to the first participant if the LLM hallucinated a name
                    Ok(self.participants[0].name.clone())
                }
            }
            Err(e) => Err(format!("Speaker selection failed: {}", e)),
        }
    }
}

/// The GroupChatManager coordinates the execution of the group chat.
pub struct GroupChatManager {
    pub chat: GroupChat,
    pub agent_runner: Arc<Agent>,
    pub base_config: AgentRunConfig,
}

impl GroupChatManager {
    pub fn new(chat: GroupChat, agent_runner: Arc<Agent>, base_config: AgentRunConfig) -> Self {
        Self {
            chat,
            agent_runner,
            base_config,
        }
    }

    pub async fn run(&mut self, initial_message: &str) -> Result<Vec<Message>, String> {
        self.chat.history.push(Message::user(initial_message));
        let mut rounds = 0;

        while rounds < self.chat.max_rounds {
            let next_speaker_name = self.chat.select_next_speaker().await?;
            let speaker = self.chat.participants.iter().find(|p| p.name == next_speaker_name).unwrap().clone();

            let mut run_cfg = self.base_config.clone();
            run_cfg.server_system_message = speaker.system_message.clone();

            let mut prompt = String::from("Conversation History:\n");
            for msg in &self.chat.history {
                prompt.push_str(&format!("{}: {}\n", msg.role, msg.content));
            }
            prompt.push_str(&format!("\nNow it is your turn to speak, {}. Respond to the conversation.", speaker.name));

            let mut on_event = |_| {};
            let result = self.agent_runner.run(&run_cfg, &prompt, &mut on_event).await
                .map_err(|e| format!("Agent run failed: {}", e))?;

            let mut msg = Message::assistant(result);
            // Prefix the content with the speaker's name so it's clear in the history
            msg.content = format!("{}: {}", speaker.name, msg.content);
            self.chat.history.push(msg.clone());

            rounds += 1;
        }

        Ok(self.chat.history.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatResponse, Usage};
    use tokio::sync::Mutex;

    struct MockLlmClientAutogen {
        responses: Mutex<Vec<String>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClientAutogen {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
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
    async fn test_autogen_group_chat_speaker_selection() {
        let client = Arc::new(MockLlmClientAutogen {
            // First time it returns "Bob", second time it returns something invalid.
            responses: Mutex::new(vec![
                "Bob".to_string(),
                "InvalidName".to_string()
            ]),
        });

        let p1 = GroupChatParticipant {
            name: "Alice".to_string(),
            description: "Engineer".to_string(),
            system_message: "".to_string(),
        };
        let p2 = GroupChatParticipant {
            name: "Bob".to_string(),
            description: "Designer".to_string(),
            system_message: "".to_string(),
        };

        let chat = GroupChat::new(vec![p1, p2], 2, client, "test".to_string());

        let speaker1 = chat.select_next_speaker().await.unwrap();
        assert_eq!(speaker1, "Bob");

        // The fallback mechanism should select the first participant (Alice) since "InvalidName" is not in the list.
        let speaker2 = chat.select_next_speaker().await.unwrap();
        assert_eq!(speaker2, "Alice");
    }

    #[tokio::test]
    async fn test_autogen_group_chat_manager_run() {
        let client = Arc::new(MockLlmClientAutogen {
            // Interleaves: select speaker, agent response, select speaker, agent response
            responses: Mutex::new(vec![
                "Bob".to_string(),
                "Hello from Bob".to_string(),
                "Alice".to_string(),
                "Hello from Alice".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client.clone(), vec![]));
        let cfg = AgentRunConfig::default();

        let p1 = GroupChatParticipant {
            name: "Alice".to_string(),
            description: "Engineer".to_string(),
            system_message: "You are Alice.".to_string(),
        };
        let p2 = GroupChatParticipant {
            name: "Bob".to_string(),
            description: "Designer".to_string(),
            system_message: "You are Bob.".to_string(),
        };

        let chat = GroupChat::new(vec![p1, p2], 2, client.clone(), "test".to_string());
        let mut manager = GroupChatManager::new(chat, agent, cfg);

        let history = manager.run("Let's talk!").await.unwrap();

        // Initial msg + 2 rounds
        assert_eq!(history.len(), 3);
        assert_eq!(history[0].role, Role::User);
        assert_eq!(history[0].content, "Let's talk!");

        assert_eq!(history[1].role, Role::Assistant);
        assert!(history[1].content.contains("Bob: Hello from Bob"));

        assert_eq!(history[2].role, Role::Assistant);
        assert!(history[2].content.contains("Alice: Hello from Alice"));
    }
}
