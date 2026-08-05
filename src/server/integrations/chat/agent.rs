use std::sync::Arc;
use tokio::sync::mpsc;
use crate::models::Message;
use ohc_builtin_agent::agent::{Agent, AgentRunConfig, AgentEvent};
use ohc_builtin_agent_llm::LlmClient;

/// The Ambassador: AI auto-responder for the omnichannel chat system.
pub struct AmbassadorAgent {
    pub llm: Arc<dyn LlmClient>,
    pub receiver: mpsc::Receiver<Message>,
}

impl AmbassadorAgent {
    pub async fn run(mut self) {
        while let Some(msg) = self.receiver.recv().await {
            // In a real application, we would retrieve context from the DB
            let context = format!("New message from contact in conversation {}: {}", msg.conversation_id, msg.content);

            let config = AgentRunConfig {
                model: "default-model".to_string(),
                developer_instructions: "You are The Ambassador, an AI auto-responder for an omnichannel chat system. Draft a helpful, concise response to the customer's message.".to_string(),
                user_instructions: context.clone(),
                ..Default::default()
            };

            let agent = Agent::new(self.llm.clone(), vec![]);

            // Channel to receive events from the agent
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

            let agent_task = tokio::spawn(async move {
                let mut tx_clone = |event: AgentEvent| {
                    let _ = tx.send(event);
                };
                let _ = agent.run(&config, &context, &mut tx_clone).await;
            });

            // Process agent output to generate a draft reply
            let mut final_draft = String::new();
            while let Some(event) = rx.recv().await {
                if let AgentEvent::TextChunk { content } = event {
                    final_draft.push_str(&content);
                } else if let AgentEvent::TaskComplete { content } = event {
                    final_draft = content;
                }
            }

            let _ = agent_task.await;

            // Here we would normally save the draft to the database using `msg.conversation_id`
            // and emit an event to the frontend via WebSocket.
            tracing::info!("Ambassador drafted response for message {}: {}", msg.id, final_draft);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Usage, Role, Message as CoreMessage};


    struct MockLlmClient {
        response_text: String,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let msg = CoreMessage {
                role: Role::Assistant,
                content: self.response_text.clone(),
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            };
            Ok(ChatResponse {
                response_id: Some("test".to_string()),
                stop_reason: "stop".to_string(),
                message: msg,
                usage: Usage::default(),
            })
        }
    }

    #[tokio::test]
    async fn test_ambassador_drafts_response() {
        let (tx, rx) = mpsc::channel(100);
        let llm = Arc::new(MockLlmClient {
            response_text: "Thank you for reaching out! A human will be with you shortly.".to_string(),
        });

        let ambassador = AmbassadorAgent {
            llm,
            receiver: rx,
        };

        // Send a message
        let msg = Message {
            id: "msg123".to_string(),
            tenant_id: "tenant1".to_string(),
            conversation_id: "conv1".to_string(),
            sender_type: "contact".to_string(),
            sender_id: None,
            content: "Hi, I need help with my order.".to_string(),
            created_at: chrono::Utc::now(),
        };

        let ambassador_task = tokio::spawn(ambassador.run());

        tx.send(msg).await.unwrap();
        // Drop tx so the receiver loop exits
        drop(tx);

        let _ = ambassador_task.await;
    }
}
