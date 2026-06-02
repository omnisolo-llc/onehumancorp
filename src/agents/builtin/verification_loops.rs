use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message};
use crate::llm::LlmClient;
use std::sync::Arc;
use serde::Deserialize;
use crate::output_parser::{LlmClientForParser, parse_structured_output};

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
pub struct PlaywrightVisualVerifier;

#[async_trait::async_trait]
impl VisualVerifier for PlaywrightVisualVerifier {
    async fn verify_visual(&self, ui_state_path: &str) -> Result<(), String> {
        let output = std::process::Command::new("npx")
            .arg("playwright")
            .arg("screenshot")
            .arg(ui_state_path)
            .arg("test.png")
            .output()
            .map_err(|e| format!("Failed to execute Playwright: {}", e))?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Visual check failed. Playwright error: {}", stderr))
        }
    }
}

pub struct LlmJudgeSensor {
    pub llm: Arc<dyn LlmClient>,
    pub max_retries: usize,
    pub approval_threshold: f32, // e.g. 4.0 out of 5.0
}

impl LlmJudgeSensor {
    pub fn new(llm: Arc<dyn LlmClient>, max_retries: usize, approval_threshold: f32) -> Self {
        Self { llm, max_retries, approval_threshold }
    }
}

/// SOTA Harness Patterns (2025-2026): Verification Loops - Inferential/Sensors
/// Structured Scoring Rubric for LLM-as-judge subagent evaluation.
#[derive(Deserialize, Debug)]
struct JudgeEvaluationRubric {
    correctness_score: f32,  // 1.0 to 5.0
    completeness_score: f32, // 1.0 to 5.0
    security_score: f32,     // 1.0 to 5.0
    reasoning: String,
    suggested_fixes: Vec<String>,
}

#[async_trait::async_trait]
impl InferentialSensor for LlmJudgeSensor {
    #[tracing::instrument(skip(self, output, task), fields(task_len=task.len(), output_len=output.len()))]
    async fn verify_inferential(&self, output: &str, task: &str) -> Result<(), String> {
        let system_prompt = "You are an expert subagent judge in a SOTA Verification Loop. \
        Evaluate the provided output against the task requirements. \
        Score 'correctness_score', 'completeness_score', and 'security_score' on a scale of 1.0 to 5.0. \
        Provide detailed reasoning and actionable 'suggested_fixes' if scores are low. \
        Be rigorous.";

        let user_prompt = format!("Task Requirements:\n{}\n\nExecution Output:\n{}", task, output);

        let req = ChatRequest {
            model: "judge-model".to_string(),
            system: system_prompt.to_string(),
            messages: vec![Message::user(user_prompt)],
            tools: vec![],
            max_tokens: 1000,
            temperature: 0.0,
        };

        struct ParserAdapter { llm: Arc<dyn LlmClient>, }
        #[async_trait::async_trait]
        impl LlmClientForParser for ParserAdapter {
            async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> { self.llm.chat(req).await }
        }

        let parser_client = Arc::new(ParserAdapter { llm: self.llm.clone() }) as Arc<dyn LlmClientForParser>;

        let mut attempts = 0;
        let mut last_error = String::new();

        while attempts <= self.max_retries {
            if attempts > 0 {
                tracing::warn!("LlmJudgeSensor retry attempt {} for inferential verification.", attempts);
            }

            // Pass 0 as max_retries to parse_structured_output to avoid nested retries,
            // as we are manually managing retries in this loop.
            match parse_structured_output::<JudgeEvaluationRubric>(&parser_client, req.clone(), 0).await {
                Ok(eval) => {
                    tracing::info!("Judge Evaluation: {:?}", eval);

                    // Calculate composite score (could weight differently in future)
                    let composite_score = (eval.correctness_score + eval.completeness_score + eval.security_score) / 3.0;

                    if composite_score >= self.approval_threshold {
                        tracing::info!("Inferential verification passed with score: {:.2}", composite_score);
                        return Ok(());
                    } else {
                        tracing::warn!("Inferential verification failed. Score: {:.2}. Reason: {}", composite_score, eval.reasoning);
                        let feedback = format!(
                            "Inferential verification rejected the output. Composite Score: {:.2}/{:.2}.\nReasoning: {}\nSuggested Fixes: {:?}",
                            composite_score, self.approval_threshold, eval.reasoning, eval.suggested_fixes
                        );
                        // Regression: Fail fast on genuine rubric failure, let the parent Orchestrator retry the task itself.
                        return Err(feedback);
                    }
                }
                Err(e) => {
                    last_error = format!("Output Parsing Error: {}", e);
                    tracing::error!("Judge subagent parsing failed: {}", last_error);
                    attempts += 1;
                }
            }
        }

        Err(format!("LlmJudgeSensor failed after {} retries. Last error: {}", self.max_retries, last_error))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::Usage;

    struct MockComputationalGuide {
        should_pass: bool,
    }
    #[async_trait::async_trait]
    impl ComputationalGuide for MockComputationalGuide {
        async fn verify(&self, _code: &str, _context: &str) -> Result<(), String> {
            if self.should_pass {
                Ok(())
            } else {
                Err("Computational check failed".to_string())
            }
        }
    }

    struct MockVisualVerifier {
        should_pass: bool,
    }
    #[async_trait::async_trait]
    impl VisualVerifier for MockVisualVerifier {
        async fn verify_visual(&self, _ui_state_path: &str) -> Result<(), String> {
            if self.should_pass {
                Ok(())
            } else {
                Err("Visual check failed".to_string())
            }
        }
    }

    #[tokio::test]
    async fn test_playwright_visual_verifier() {
        // We will mock the implementation via Command to fail smoothly if npx doesn't exist,
        // but test the struct initialization.
        let verifier = PlaywrightVisualVerifier;
        // Run against an invalid path, expecting an error
        let res = verifier.verify_visual("invalid_path_123").await;
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert!(err.contains("Playwright error:") || err.contains("Failed to execute Playwright:"));
    }

    struct MockLlmClient {
        response_text: String,
    }
    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let msg = Message {
                role: ohc_builtin_agent_core::types::Role::Assistant,
                content: self.response_text.clone(),
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            };
            Ok(ChatResponse { response_id: Some("test".to_string()), stop_reason: "".to_string(),
                message: msg,
                usage: Usage::default(),
            })
        }
    }
    #[async_trait::async_trait]
    impl LlmClientForParser for MockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let tool_call = ohc_builtin_agent_core::types::ToolCall {
                id: "call_1".to_string(),
                name: "structured_output".to_string(),
                arguments: serde_json::json!({
                    "data": serde_json::from_str::<serde_json::Value>(&self.response_text).unwrap_or(serde_json::json!({}))
                }),
            };

            let msg = Message {
                role: ohc_builtin_agent_core::types::Role::Assistant,
                content: "".to_string(),
                tool_calls: vec![tool_call],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            };
            Ok(ChatResponse { response_id: Some("test".to_string()), stop_reason: "".to_string(),
                message: msg,
                usage: Usage::default(),
            })
        }
    }

    #[tokio::test]
    async fn test_verification_manager() {
        let mut manager = VerificationManager::new();

        manager.add_computational(Arc::new(MockComputationalGuide { should_pass: true }));
        manager.add_visual(Arc::new(MockVisualVerifier { should_pass: true }));

        assert!(manager.run_computational_guides("", "").await.is_ok());
        assert!(manager.run_visual_verifiers("").await.is_ok());

        let mut fail_manager = VerificationManager::new();
        fail_manager.add_computational(Arc::new(MockComputationalGuide { should_pass: false }));
        assert!(fail_manager.run_computational_guides("", "").await.is_err());
    }

    #[tokio::test]
    async fn test_llm_judge_sensor() {
        let pass_llm = Arc::new(MockLlmClient { response_text: r#"{"correctness_score": 5.0, "completeness_score": 5.0, "security_score": 5.0, "reasoning": "Looks good", "suggested_fixes": []}"#.to_string() });
        let judge = LlmJudgeSensor::new(pass_llm, 2, 4.0);
        assert!(judge.verify_inferential("output", "task").await.is_ok());

        let fail_llm = Arc::new(MockLlmClient { response_text: r#"{"correctness_score": 1.0, "completeness_score": 1.0, "security_score": 1.0, "reasoning": "Bad", "suggested_fixes": ["Fix this"]}"#.to_string() });
        let judge_fail = LlmJudgeSensor::new(fail_llm, 2, 4.0);
        let res = judge_fail.verify_inferential("output", "task").await;
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("Reasoning: Bad"));
    }
}
