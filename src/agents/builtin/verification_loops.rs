<<<<<<< HEAD
use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message};
=======
use ohc_builtin_agent_core::types::{ChatRequest,  Message};
>>>>>>> 8f4cf653 (💰 Miser: Add miser verification)
use crate::llm::LlmClient;
use std::sync::Arc;

/// A feedforward verification loop using linters, type-checkers, or unit tests.
#[async_trait::async_trait]
pub trait ComputationalGuide: Send + Sync {
    async fn verify(&self, code: &str, context: &str) -> Result<(), String>;
}

/// A feedback verification loop using visual checks (screenshots via Playwright and/or Desktop/Mobile UI tests).
#[async_trait::async_trait]
pub trait VisualVerifier: Send + Sync {
    async fn verify_visual(&self, ui_state_path: &str) -> Result<(), String>;
}

/// A feedback verification loop using a separate LLM-as-judge subagent.
#[async_trait::async_trait]
pub trait InferentialSensor: Send + Sync {
    async fn verify_inferential(&self, output: &str, task: &str) -> Result<(), String>;
}

/// A manager that coordinates the 3 distinct verification loops.
pub struct VerificationManager {
    computational: Vec<Arc<dyn ComputationalGuide>>,
    visual: Vec<Arc<dyn VisualVerifier>>,
    inferential: Vec<Arc<dyn InferentialSensor>>,
}

impl VerificationManager {
    pub fn new() -> Self {
        Self {
            computational: Vec::new(),
            visual: Vec::new(),
            inferential: Vec::new(),
        }
    }

    pub fn add_computational(&mut self, guide: Arc<dyn ComputationalGuide>) {
        self.computational.push(guide);
    }

    pub fn add_visual(&mut self, verifier: Arc<dyn VisualVerifier>) {
        self.visual.push(verifier);
    }

    pub fn add_inferential(&mut self, sensor: Arc<dyn InferentialSensor>) {
        self.inferential.push(sensor);
    }

    pub async fn run_computational_guides(&self, code: &str, context: &str) -> Result<(), String> {
        for guide in &self.computational {
            guide.verify(code, context).await?;
        }
        Ok(())
    }

    pub async fn run_visual_verifiers(&self, ui_state_path: &str) -> Result<(), String> {
        for verifier in &self.visual {
            verifier.verify_visual(ui_state_path).await?;
        }
        Ok(())
    }

    pub async fn run_inferential_sensors(&self, output: &str, task: &str) -> Result<(), String> {
        for sensor in &self.inferential {
            sensor.verify_inferential(output, task).await?;
        }
        Ok(())
    }
}

/// An InferentialSensor that uses an LlmClient to act as a judge.
pub struct LlmJudgeSensor {
    pub llm: Arc<dyn LlmClient>,
}

#[async_trait::async_trait]
impl InferentialSensor for LlmJudgeSensor {
    async fn verify_inferential(&self, output: &str, task: &str) -> Result<(), String> {
        let system_prompt = "You are a harsh but fair judge. Evaluate the output against the task. If the output solves the task, reply only with 'PASS'. If it fails, reply with 'FAIL: ' followed by the reason.";
        let user_prompt = format!("Task: {}\nOutput: {}", task, output);

        let req = ChatRequest {
            model: "judge-model".to_string(),
            system: system_prompt.to_string(),
            messages: vec![Message::user(user_prompt)],
            tools: vec![],
            max_tokens: 500,
            temperature: 0.0,
        };

        match self.llm.chat(req).await {
            Ok(resp) => {
                let text = resp.message.content.trim();
                if text == "PASS" {
                    Ok(())
                } else if text.starts_with("FAIL:") {
                    Err(text.to_string())
                } else {
                    Err(format!("Judge returned unexpected response: {}", text))
                }
            }
            Err(e) => Err(format!("LLM Error: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::Usage;

    struct MockComputationalGuide {
        should_fail: bool,
    }

    #[async_trait::async_trait]
    impl ComputationalGuide for MockComputationalGuide {
        async fn verify(&self, code: &str, _context: &str) -> Result<(), String> {
            if self.should_fail {
                Err(format!("Linter failed on code: {}", code))
            } else {
                Ok(())
            }
        }
    }

    struct MockVisualVerifier {
        should_fail: bool,
    }

    #[async_trait::async_trait]
    impl VisualVerifier for MockVisualVerifier {
        async fn verify_visual(&self, ui_state_path: &str) -> Result<(), String> {
            if self.should_fail {
                Err(format!("Visual check failed for path: {}", ui_state_path))
            } else {
                Ok(())
            }
        }
    }

    struct MockJudgeLlm {
        response_text: String,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockJudgeLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant(&self.response_text),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("judge-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_computational_guide() {
        let mut manager = VerificationManager::new();
        manager.add_computational(Arc::new(MockComputationalGuide { should_fail: false }));

        let result = manager.run_computational_guides("let x = 1;", "context").await;
        assert!(result.is_ok());

        let mut manager_fail = VerificationManager::new();
        manager_fail.add_computational(Arc::new(MockComputationalGuide { should_fail: true }));

        let result_fail = manager_fail.run_computational_guides("let x = 1;", "context").await;
        assert!(result_fail.is_err());
        assert_eq!(result_fail.unwrap_err(), "Linter failed on code: let x = 1;");
    }

    #[tokio::test]
    async fn test_visual_verifier() {
        let mut manager = VerificationManager::new();
        manager.add_visual(Arc::new(MockVisualVerifier { should_fail: false }));

        let result = manager.run_visual_verifiers("/tmp/screen.png").await;
        assert!(result.is_ok());

        let mut manager_fail = VerificationManager::new();
        manager_fail.add_visual(Arc::new(MockVisualVerifier { should_fail: true }));

        let result_fail = manager_fail.run_visual_verifiers("/tmp/screen.png").await;
        assert!(result_fail.is_err());
        assert_eq!(result_fail.unwrap_err(), "Visual check failed for path: /tmp/screen.png");
    }

    #[tokio::test]
    async fn test_inferential_sensor() {
        let pass_llm = Arc::new(MockJudgeLlm { response_text: "PASS".to_string() });
        let sensor_pass = Arc::new(LlmJudgeSensor { llm: pass_llm });

        let mut manager = VerificationManager::new();
        manager.add_inferential(sensor_pass);

        let result = manager.run_inferential_sensors("output", "task").await;
        assert!(result.is_ok());

        let fail_llm = Arc::new(MockJudgeLlm { response_text: "FAIL: missing requirement".to_string() });
        let sensor_fail = Arc::new(LlmJudgeSensor { llm: fail_llm });

        let mut manager_fail = VerificationManager::new();
        manager_fail.add_inferential(sensor_fail);

        let result_fail = manager_fail.run_inferential_sensors("output", "task").await;
        assert!(result_fail.is_err());
        assert_eq!(result_fail.unwrap_err(), "FAIL: missing requirement");

        let weird_llm = Arc::new(MockJudgeLlm { response_text: "It looks good to me!".to_string() });
        let sensor_weird = Arc::new(LlmJudgeSensor { llm: weird_llm });

        let mut manager_weird = VerificationManager::new();
        manager_weird.add_inferential(sensor_weird);

        let result_weird = manager_weird.run_inferential_sensors("output", "task").await;
        assert!(result_weird.is_err());
        assert!(result_weird.unwrap_err().contains("unexpected response: It looks good to me!"));
    }
}
