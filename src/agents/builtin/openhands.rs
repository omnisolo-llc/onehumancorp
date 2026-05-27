use crate::types::{Message, Role};
use std::sync::Arc;

/// OpenHands/OpenDevin: Python SDK + CLI sandbox interactive loop Mechanic
pub struct OpenHandsCli {
    llm: Arc<dyn crate::llm::LlmClient>,
    runner: Arc<crate::tools::runner::CommandRunner>,
}

impl OpenHandsCli {
    pub fn new(llm: Arc<dyn crate::llm::LlmClient>, runner: Arc<crate::tools::runner::CommandRunner>) -> Self {
        Self { llm, runner }
    }

    pub async fn run_loop(&self, initial_prompt: &str) -> Result<String, String> {
        let mut messages = vec![Message::user(initial_prompt)];

        for _ in 0..10 { // Max iterations
            let req = crate::types::ChatRequest {
                model: "default".to_string(),
                system: "You are an OpenHands CLI agent. Output shell commands inside <cmd> tags.".to_string(),
                messages: messages.clone(),
                tools: vec![],
                max_tokens: 1000,
                temperature: 0.0,
            };

            let resp = self.llm.chat(req).await.map_err(|e| e.to_string())?;
            let content = resp.message.content.clone();
            messages.push(Message::assistant(&content));

            // Parse <cmd> tags
            if let Some(cmd_start) = content.find("<cmd>") {
                if let Some(cmd_end) = content.find("</cmd>") {
                    let cmd = &content[cmd_start + 5..cmd_end].trim();
                    let output = self.runner.run("bash", &["-c", cmd], None, vec![]).await.map_err(|e| e.to_string())?;
                    messages.push(Message::user(format!("Command output:\n{}", String::from_utf8_lossy(&output.stdout))));
                    continue;
                }
            }

            // If no command, we assume it's done
            return Ok(content);
        }

        Err("Max iterations reached".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ChatResponse, Usage};

    struct MockLlm;
    #[async_trait::async_trait]
    impl crate::llm::LlmClient for MockLlm {
        async fn chat(&self, req: crate::types::ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let last_msg = req.messages.last().unwrap();
            let response_content = if last_msg.content.contains("Command output:") {
                "Task completed successfully."
            } else {
                "<cmd>echo 'hello'</cmd>"
            };

            Ok(ChatResponse {
                message: Message::assistant(response_content),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: None,
            })
        }
        async fn generate_embedding(&self, _text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
            Ok(vec![])
        }
    }

    #[tokio::test]
    async fn test_openhands_loop() {
        let llm = Arc::new(MockLlm);
        let runner = Arc::new(crate::tools::runner::CommandRunner::new());
        let cli = OpenHandsCli::new(llm, runner);

        let result = cli.run_loop("Say hello").await.unwrap();
        assert_eq!(result, "Task completed successfully.");
    }
}
