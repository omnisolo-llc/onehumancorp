use std::sync::Arc;
use tokio::process::Command;
use crate::types::{ChatRequest, ChatResponse, Message, Role, ToolCall};
use crate::llm::LlmClient;
use serde::{Deserialize, Serialize};
use std::process::Stdio;

/// Represents the result of a single verification loop.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VerificationStatus {
    Passed,
    Failed(String),
}

pub trait VerificationLoop: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}

/// Verification Loop 1: Computational/Guides
/// Feedforward linters, type-checkers, compilers.
/// Used to catch errors *before* committing to a final action or after writing code.
pub struct ComputationalGuide {
    pub command: String,
    pub working_dir: String,
}

#[async_trait::async_trait]
pub trait GuideExecutor: Send + Sync {
    async fn execute_guide(&self, guide: &ComputationalGuide, recent_files: &[String]) -> Result<VerificationStatus, String>;
}

pub struct DefaultGuideExecutor;

#[async_trait::async_trait]
impl GuideExecutor for DefaultGuideExecutor {
    async fn execute_guide(&self, guide: &ComputationalGuide, _recent_files: &[String]) -> Result<VerificationStatus, String> {
        let mut cmd = if cfg!(target_os = "windows") {
            let mut c = Command::new("cmd");
            c.arg("/C").arg(&guide.command);
            c
        } else {
            let mut c = Command::new("sh");
            c.arg("-c").arg(&guide.command);
            c
        };

        cmd.current_dir(&guide.working_dir)
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());

        match cmd.output().await {
            Ok(output) => {
                if output.status.success() {
                    Ok(VerificationStatus::Passed)
                } else {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Ok(VerificationStatus::Failed(format!("Computational guide failed:\nStdout:\n{}\nStderr:\n{}", stdout, stderr)))
                }
            }
            Err(e) => Err(format!("Failed to execute computational guide: {}", e)),
        }
    }
}

/// Verification Loop 2: Visual Sensors
/// Screenshots via Playwright and/or Desktop/Mobile UI tests.
pub struct VisualSensor {
    pub verification_command: String,
    pub working_dir: String,
}

#[async_trait::async_trait]
pub trait SensorExecutor: Send + Sync {
    async fn execute_sensor(&self, sensor: &VisualSensor) -> Result<VerificationStatus, String>;
}

pub struct DefaultSensorExecutor;

#[async_trait::async_trait]
impl SensorExecutor for DefaultSensorExecutor {
    async fn execute_sensor(&self, sensor: &VisualSensor) -> Result<VerificationStatus, String> {
        let mut cmd = if cfg!(target_os = "windows") {
            let mut c = Command::new("cmd");
            c.arg("/C").arg(&sensor.verification_command);
            c
        } else {
            let mut c = Command::new("sh");
            c.arg("-c").arg(&sensor.verification_command);
            c
        };

        cmd.current_dir(&sensor.working_dir)
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());

        match cmd.output().await {
            Ok(output) => {
                if output.status.success() {
                    Ok(VerificationStatus::Passed)
                } else {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Ok(VerificationStatus::Failed(format!("Visual verification rejected the output:\nStdout:\n{}\nStderr:\n{}", stdout, stderr)))
                }
            }
            Err(e) => Err(format!("Failed to execute visual verification: {}", e)),
        }
    }
}

/// Verification Loop 3: Inferential / Sensors
/// LLM-as-judge subagent evaluates the output.
pub struct InferentialJudge {
    pub llm: Arc<dyn LlmClient>,
    pub model: String,
    pub custom_prompt: Option<String>,
}

impl InferentialJudge {
    pub fn new(llm: Arc<dyn LlmClient>, model: String) -> Self {
        Self { llm, model, custom_prompt: None }
    }

    pub fn with_prompt(mut self, prompt: String) -> Self {
        self.custom_prompt = Some(prompt);
        self
    }

    pub async fn evaluate(&self, output_to_evaluate: &str, original_request: &str) -> Result<VerificationStatus, String> {
        let system_prompt = self.custom_prompt.clone().unwrap_or_else(|| {
            "You are an expert judge and QA engineer. Evaluate the following output against the original request. Output ONLY 'APPROVE' if it correctly fulfills the request, or 'REJECT: <reason>' if it fails, is incomplete, or violates constraints.".to_string()
        });

        let req = ChatRequest {
            model: self.model.clone(),
            system: system_prompt,
            messages: vec![Message::user(format!(
                "Original Request:\n{}\n\nOutput to Evaluate:\n{}",
                original_request, output_to_evaluate
            ))],
            tools: vec![],
            max_tokens: 500,
            temperature: 0.0,
        };

        match self.llm.chat(req).await {
            Ok(resp) => {
                let text = resp.message.content.trim();
                if text.starts_with("REJECT:") {
                    let reason = text.strip_prefix("REJECT:").unwrap_or(text).trim();
                    Ok(VerificationStatus::Failed(reason.to_string()))
                } else {
                    Ok(VerificationStatus::Passed)
                }
            }
            Err(e) => Err(format!("LLM Judge error: {}", e)),
        }
    }
}

/// The Verification Coordinator orchestrates all three loops.
pub struct VerificationCoordinator {
    pub computational_guide: Option<ComputationalGuide>,
    pub visual_sensor: Option<VisualSensor>,
    pub inferential_judge: Option<InferentialJudge>,
    pub guide_executor: Arc<dyn GuideExecutor>,
    pub sensor_executor: Arc<dyn SensorExecutor>,
}

impl VerificationCoordinator {
    pub fn new() -> Self {
        Self {
            computational_guide: None,
            visual_sensor: None,
            inferential_judge: None,
            guide_executor: Arc::new(DefaultGuideExecutor),
            sensor_executor: Arc::new(DefaultSensorExecutor),
        }
    }

    pub fn with_computational_guide(mut self, cmd: String, dir: String) -> Self {
        if !cmd.is_empty() {
            self.computational_guide = Some(ComputationalGuide { command: cmd, working_dir: dir });
        }
        self
    }

    pub fn with_visual_sensor(mut self, cmd: String, dir: String) -> Self {
        if !cmd.is_empty() {
            self.visual_sensor = Some(VisualSensor { verification_command: cmd, working_dir: dir });
        }
        self
    }

    pub fn with_inferential_judge(mut self, llm: Arc<dyn LlmClient>, model: String, custom_prompt: Option<String>) -> Self {
        let mut judge = InferentialJudge::new(llm, model);
        if let Some(p) = custom_prompt {
            judge = judge.with_prompt(p);
        }
        self.inferential_judge = Some(judge);
        self
    }

    pub fn with_guide_executor(mut self, exec: Arc<dyn GuideExecutor>) -> Self {
        self.guide_executor = exec;
        self
    }

    pub fn with_sensor_executor(mut self, exec: Arc<dyn SensorExecutor>) -> Self {
        self.sensor_executor = exec;
        self
    }

    /// Run all configured verification loops on the proposed output.
    /// Returns Ok(()) if all pass. Returns Err(message) if any fail.
    pub async fn verify_output(&self, proposed_output: &str, original_request: &str) -> Result<(), String> {
        // 1. Computational Guide (Fastest, cheapest)
        if let Some(guide) = &self.computational_guide {
            match self.guide_executor.execute_guide(guide, &[]).await {
                Ok(VerificationStatus::Failed(reason)) => {
                    return Err(format!("Computational Guide Failed:\n{}", reason));
                }
                Err(e) => {
                    return Err(format!("Computational Guide Execution Error: {}", e));
                }
                _ => {}
            }
        }

        // 2. Visual Sensor
        if let Some(sensor) = &self.visual_sensor {
            match self.sensor_executor.execute_sensor(sensor).await {
                Ok(VerificationStatus::Failed(reason)) => {
                    return Err(format!("Visual Sensor Failed:\n{}", reason));
                }
                Err(e) => {
                    return Err(format!("Visual Sensor Execution Error: {}", e));
                }
                _ => {}
            }
        }

        // 3. Inferential Judge (Most expensive, runs last)
        if let Some(judge) = &self.inferential_judge {
            match judge.evaluate(proposed_output, original_request).await {
                Ok(VerificationStatus::Failed(reason)) => {
                    return Err(format!("LLM Judge Rejected Output:\n{}", reason));
                }
                Err(e) => {
                    return Err(format!("LLM Judge Execution Error: {}", e));
                }
                _ => {}
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Usage;
    use tokio::sync::Mutex;

    struct MockLlm {
        responses: Mutex<Vec<String>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            let content = if resps.is_empty() {
                "APPROVE".to_string()
            } else {
                resps.remove(0)
            };
            Ok(ChatResponse {
                message: Message::assistant(content),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id".to_string()),
            })
        }
    }

    struct MockGuideExecutor {
        result: VerificationStatus,
    }

    #[async_trait::async_trait]
    impl GuideExecutor for MockGuideExecutor {
        async fn execute_guide(&self, _guide: &ComputationalGuide, _recent_files: &[String]) -> Result<VerificationStatus, String> {
            Ok(self.result.clone())
        }
    }

    struct MockSensorExecutor {
        result: VerificationStatus,
    }

    #[async_trait::async_trait]
    impl SensorExecutor for MockSensorExecutor {
        async fn execute_sensor(&self, _sensor: &VisualSensor) -> Result<VerificationStatus, String> {
            Ok(self.result.clone())
        }
    }

    #[tokio::test]
    async fn test_verification_coordinator_all_pass() {
        let llm = Arc::new(MockLlm { responses: Mutex::new(vec!["APPROVE".to_string()]) });
        let coord = VerificationCoordinator::new()
            .with_computational_guide("echo 1".to_string(), ".".to_string())
            .with_visual_sensor("echo 1".to_string(), ".".to_string())
            .with_inferential_judge(llm, "test-model".to_string(), None)
            .with_guide_executor(Arc::new(MockGuideExecutor { result: VerificationStatus::Passed }))
            .with_sensor_executor(Arc::new(MockSensorExecutor { result: VerificationStatus::Passed }));

        let result = coord.verify_output("final answer", "do it").await;
        assert!(result.is_ok(), "Expected all verifications to pass");
    }

    #[tokio::test]
    async fn test_verification_coordinator_guide_fails() {
        let llm = Arc::new(MockLlm { responses: Mutex::new(vec!["APPROVE".to_string()]) });
        let coord = VerificationCoordinator::new()
            .with_computational_guide("echo 1".to_string(), ".".to_string())
            .with_visual_sensor("echo 1".to_string(), ".".to_string())
            .with_inferential_judge(llm, "test-model".to_string(), None)
            .with_guide_executor(Arc::new(MockGuideExecutor { result: VerificationStatus::Failed("syntax error".to_string()) }))
            .with_sensor_executor(Arc::new(MockSensorExecutor { result: VerificationStatus::Passed }));

        let result = coord.verify_output("final answer", "do it").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Computational Guide Failed"));
    }

    #[tokio::test]
    async fn test_verification_coordinator_sensor_fails() {
        let llm = Arc::new(MockLlm { responses: Mutex::new(vec!["APPROVE".to_string()]) });
        let coord = VerificationCoordinator::new()
            .with_computational_guide("echo 1".to_string(), ".".to_string())
            .with_visual_sensor("echo 1".to_string(), ".".to_string())
            .with_inferential_judge(llm, "test-model".to_string(), None)
            .with_guide_executor(Arc::new(MockGuideExecutor { result: VerificationStatus::Passed }))
            .with_sensor_executor(Arc::new(MockSensorExecutor { result: VerificationStatus::Failed("pixels differ".to_string()) }));

        let result = coord.verify_output("final answer", "do it").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Visual Sensor Failed"));
    }

    #[tokio::test]
    async fn test_verification_coordinator_judge_fails() {
        let llm = Arc::new(MockLlm { responses: Mutex::new(vec!["REJECT: output does not make sense".to_string()]) });
        let coord = VerificationCoordinator::new()
            .with_computational_guide("echo 1".to_string(), ".".to_string())
            .with_visual_sensor("echo 1".to_string(), ".".to_string())
            .with_inferential_judge(llm, "test-model".to_string(), None)
            .with_guide_executor(Arc::new(MockGuideExecutor { result: VerificationStatus::Passed }))
            .with_sensor_executor(Arc::new(MockSensorExecutor { result: VerificationStatus::Passed }));

        let result = coord.verify_output("final answer", "do it").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("LLM Judge Rejected Output"));
        assert!(result.unwrap_err().contains("output does not make sense"));
    }
}