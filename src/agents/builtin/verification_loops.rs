use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message};
use crate::llm::LlmClient;
use std::sync::Arc;
use serde::Deserialize;
use crate::output_parser::{LlmClientForParser, parse_structured_output};

/// A feedforward verification loop using linters, type-checkers, or unit tests.
#[async_trait::async_trait]
pub trait ComputationalGuide: Send + Sync {
    async fn verify(&self, code: &str, context: &str) -> Result<(), Vec<String>>;
}

/// A feedback verification loop using visual checks (screenshots via Playwright and/or Desktop/Mobile UI tests).
#[async_trait::async_trait]
pub trait VisualVerifier: Send + Sync {
    async fn verify_visual(&self, ui_state_path: &str) -> Result<(), Vec<String>>;
}

/// A feedback verification loop using a separate LLM-as-judge subagent.
#[async_trait::async_trait]
pub trait InferentialSensor: Send + Sync {
    async fn verify_inferential(&self, output: &str, task: &str) -> Result<(), Vec<String>>;
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

    pub async fn run_computational_guides(&self, code: &str, context: &str) -> Result<(), Vec<String>> {
        let futures = self.computational.iter().map(|g| g.verify(code, context));
        let results = futures::future::join_all(futures).await;
        let errors: Vec<String> = results.into_iter().filter_map(|r| r.err()).flatten().collect();
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }

    pub async fn run_visual_verifiers(&self, ui_state_path: &str) -> Result<(), Vec<String>> {
        let futures = self.visual.iter().map(|v| v.verify_visual(ui_state_path));
        let results = futures::future::join_all(futures).await;
        let errors: Vec<String> = results.into_iter().filter_map(|r| r.err()).flatten().collect();
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }

    pub async fn run_inferential_sensors(&self, output: &str, task: &str) -> Result<(), Vec<String>> {
        let futures = self.inferential.iter().map(|s| s.verify_inferential(output, task));
        let results = futures::future::join_all(futures).await;
        let errors: Vec<String> = results.into_iter().filter_map(|r| r.err()).flatten().collect();
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

/// An InferentialSensor that uses an LlmClient to act as a judge.
pub struct LlmJudgeSensor {
    pub llm: Arc<dyn LlmClient>,
}

#[derive(Deserialize)]
struct JudgeEvaluation {
    status: String,
    reason: String,
    confidence: f32,
    correctness_score: f32,
    completeness_score: f32,
}

#[async_trait::async_trait]
impl InferentialSensor for LlmJudgeSensor {
    async fn verify_inferential(&self, output: &str, task: &str) -> Result<(), Vec<String>> {
        let system_prompt = "You are an expert judge. Evaluate the following output for correctness, completeness, and adherence to constraints. Provide your evaluation structured exactly as requested, where status is either 'APPROVE' or 'REJECT'. Include correctness_score and completeness_score as floats between 0.0 and 1.0.";
        let user_prompt = format!("Task: {}
Output: {}", task, output);

        let req = ChatRequest {
            model: "judge-model".to_string(),
            system: system_prompt.to_string(),
            messages: vec![Message::user(user_prompt)],
            tools: vec![],
            max_tokens: 500,
            temperature: 0.0,
        };

        struct ParserAdapter { llm: Arc<dyn LlmClient>, }
        #[async_trait::async_trait]
        impl LlmClientForParser for ParserAdapter {
            async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> { self.llm.chat(req).await }
        }
        let parser_client = Arc::new(ParserAdapter { llm: self.llm.clone() }) as Arc<dyn LlmClientForParser>;

        match parse_structured_output::<JudgeEvaluation>(&parser_client, req, 3).await {
            Ok(eval) => {
                if eval.status.to_uppercase() == "REJECT" || eval.correctness_score < 0.7 || eval.completeness_score < 0.7 {
                    Err(vec![format!("Reason: {}. Confidence: {:.2}. Correctness: {:.2}, Completeness: {:.2}", eval.reason, eval.confidence, eval.correctness_score, eval.completeness_score)])
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(vec![format!("LLM Error: {}", e)]),
        }
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
        async fn verify(&self, _code: &str, _context: &str) -> Result<(), Vec<String>> {
            if self.should_pass {
                Ok(())
            } else {
                Err(vec!["Computational check failed".to_string()])
            }
        }
    }

    struct MockVisualVerifier {
        should_pass: bool,
    }
    #[async_trait::async_trait]
    impl VisualVerifier for MockVisualVerifier {
        async fn verify_visual(&self, _ui_state_path: &str) -> Result<(), Vec<String>> {
            if self.should_pass {
                Ok(())
            } else {
                Err(vec!["Visual check failed".to_string()])
            }
        }
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
        let pass_llm = Arc::new(MockLlmClient { response_text: r#"{"status": "APPROVE", "reason": "Looks good", "confidence": 0.9, "correctness_score": 0.9, "completeness_score": 0.9}"#.to_string() });
        let judge = LlmJudgeSensor { llm: pass_llm };
        assert!(judge.verify_inferential("output", "task").await.is_ok());

        let fail_llm = Arc::new(MockLlmClient { response_text: r#"{"status": "REJECT", "reason": "Bad", "confidence": 0.8, "correctness_score": 0.5, "completeness_score": 0.5}"#.to_string() });
        let judge_fail = LlmJudgeSensor { llm: fail_llm };
        let res = judge_fail.verify_inferential("output", "task").await;
        assert!(res.is_err());
        assert!(res.unwrap_err()[0].contains("Reason: Bad. Confidence: 0.80"));
    }
}
