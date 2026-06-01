use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceAgentConfig {
    pub language: String,
    pub instructions: String,
}

pub struct VoiceAgent {
    pub config: VoiceAgentConfig,
}

impl VoiceAgent {
    pub fn new(config: VoiceAgentConfig) -> Self {
        Self { config }
    }

    pub async fn process_utterance(&self, text: &str) -> String {
        let text_lower = text.to_lowercase();

        if text_lower.contains("book") || text_lower.contains("appointment") {
            // Simulated tool call
            self.check_availability().await;
            "I can help with that. What day works best for you?".to_string()
        } else if text_lower.contains("quote") || text_lower.contains("price") {
            // Simulated tool call
            self.create_lead().await;
            "Our prices start at $50. I can text you a detailed quote link if you'd like.".to_string()
        } else {
            "I'm sorry, I can have someone call you back about that. Let me text you a link to our services.".to_string()
        }
    }

    async fn check_availability(&self) {
        // Mock tool call
    }

    async fn create_lead(&self) {
        // Mock tool call
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_process_utterance_booking() {
        let agent = VoiceAgent::new(VoiceAgentConfig {
            language: "en".to_string(),
            instructions: "Be helpful".to_string(),
        });

        let response = agent.process_utterance("I would like to book an appointment").await;
        assert!(response.contains("What day works best"));
    }

    #[tokio::test]
    async fn test_process_utterance_quote() {
        let agent = VoiceAgent::new(VoiceAgentConfig {
            language: "en".to_string(),
            instructions: "Be helpful".to_string(),
        });

        let response = agent.process_utterance("What is the price for plumbing?").await;
        assert!(response.contains("prices start at $50"));
    }

    #[tokio::test]
    async fn test_process_utterance_fallback() {
        let agent = VoiceAgent::new(VoiceAgentConfig {
            language: "en".to_string(),
            instructions: "Be helpful".to_string(),
        });

        let response = agent.process_utterance("Do you repair spaceships?").await;
        assert!(response.contains("have someone call you back"));
    }
}
