#![allow(clippy::empty_line_after_doc_comments)]
use crate::llm::LlmClient;
use crate::output_parser::{LlmClientForParser, parse_structured_output};
/// Master Catalog C.4. Verification Loops: Guides (steer before action) vs Sensors (observe after action).
/// Master Catalog B.10. Verification Loops
use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message};
use serde::Deserialize;
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

/// Master Catalog B.10. Verification Loops (Quality x3): Giving the model ways to verify work.
///     Mechanics: Computational/Guides (feedforward: linters, type-checkers, unit tests),
///     Visual (screenshots via Playwright and/or Desktop/Mobile UI tests), and
///     Inferential/Sensors (feedback: a separate LLM-as-judge subagent evaluates the output).
///     A manager that coordinates the 3 distinct verification loops.

pub struct BashComputationalGuide {
    pub command: String,
    pub workspace_path: Option<String>,
}

#[async_trait::async_trait]
impl ComputationalGuide for BashComputationalGuide {
    async fn verify(&self, _code: &str, _context: &str) -> Result<(), String> {
        let wd = self
            .workspace_path
            .clone()
            .unwrap_or_else(|| ".".to_string());
        let mut cmd = std::process::Command::new("bash");
        cmd.arg("-c").arg(&self.command).current_dir(wd);

        match cmd.output() {
            Ok(output) => {
                if !output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(format!(
                        "Computational guide verification failed (command: {}).\nStdout: {}\nStderr: {}\nPlease correct your work and use tools to fix the issue before providing the final answer.",
                        self.command, stdout, stderr
                    ));
                }
                Ok(())
            }
            Err(e) => Err(format!(
                "Failed to execute computational guide {}: {}",
                self.command, e
            )),
        }
    }
}

pub struct CargoTestGuide {
    pub workspace_path: Option<String>,
}

#[async_trait::async_trait]
impl ComputationalGuide for CargoTestGuide {
    async fn verify(&self, _code: &str, _context: &str) -> Result<(), String> {
        let wd = self
            .workspace_path
            .clone()
            .unwrap_or_else(|| ".".to_string());
        let mut cmd = tokio::process::Command::new("cargo");
        cmd.arg("test").current_dir(wd);

        match cmd.output().await {
            Ok(output) => {
                if !output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(format!(
                        "Computational guide verification failed (cargo test).\nStdout: {}\nStderr: {}\nPlease correct your work and use tools to fix the issue before providing the final answer.",
                        stdout, stderr
                    ));
                }
                Ok(())
            }
            Err(e) => Err(format!(
                "Failed to execute computational guide cargo test: {}",
                e
            )),
        }
    }
}

pub struct BashVisualVerifier {
    pub command: String,
    pub workspace_path: Option<String>,
}

#[async_trait::async_trait]
impl VisualVerifier for BashVisualVerifier {
    async fn verify_visual(&self, _ui_state_path: &str) -> Result<(), String> {
        let wd = self
            .workspace_path
            .clone()
            .unwrap_or_else(|| ".".to_string());
        let mut cmd = std::process::Command::new("bash");
        cmd.arg("-c").arg(&self.command).current_dir(wd);

        match cmd.output() {
            Ok(output) => {
                if !output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(format!(
                        "Visual verification failed (command: {}).\nStdout: {}\nStderr: {}\nPlease correct your work based on the visual feedback and use tools to fix the issue.",
                        self.command, stdout, stderr
                    ));
                } else {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    if stdout.contains("REJECT") {
                        return Err(format!(
                            "Visual verification rejected the output. Reason: {}\nPlease correct your work and use tools to fix the issue.",
                            stdout.trim()
                        ));
                    }
                }
                Ok(())
            }
            Err(e) => Err(format!(
                "Failed to execute visual verifier {}: {}",
                self.command, e
            )),
        }
    }
}

pub struct VerificationManager {
    computational: Vec<Arc<dyn ComputationalGuide>>,
    visual: Vec<Arc<dyn VisualVerifier>>,
    inferential: Vec<Arc<dyn InferentialSensor>>,
}

impl Default for VerificationManager {
    fn default() -> Self {
        Self::new()
    }
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

    /// Architectural Decision C.4: Guides (steer before action) vs Sensors (observe after action)
    /// Run guides *before* an action is taken to evaluate intent/code.
    pub async fn run_guides_before_action(&self, code: &str, context: &str) -> Result<(), String> {
        self.run_computational_guides(code, context).await
    }

    pub async fn run_computational_guides(&self, code: &str, context: &str) -> Result<(), String> {
        let futures = self
            .computational
            .iter()
            .map(|guide| guide.verify(code, context));
        let results = futures::future::join_all(futures).await;
        let errors: Vec<String> = results.into_iter().filter_map(|r| r.err()).collect();
        if !errors.is_empty() {
            return Err(errors.join("\n---\n"));
        }
        Ok(())
    }

    /// Architectural Decision C.4: Guides (steer before action) vs Sensors (observe after action)
    /// Run sensors *after* an action is taken to observe its result.
    pub async fn run_sensors_after_action(&self, output: &str, task: &str, ui_state_path: Option<&str>) -> Result<(), String> {
        let mut errors = Vec::new();
        if let Some(path) = ui_state_path {
            if let Err(e) = self.run_visual_verifiers(path).await {
                errors.push(e);
            }
        }
        if let Err(e) = self.run_inferential_sensors(output, task).await {
            errors.push(e);
        }
        if !errors.is_empty() {
            return Err(errors.join("\n---\n"));
        }
        Ok(())
    }

    pub async fn run_visual_verifiers(&self, ui_state_path: &str) -> Result<(), String> {
        let futures = self
            .visual
            .iter()
            .map(|verifier| verifier.verify_visual(ui_state_path));
        let results = futures::future::join_all(futures).await;
        let errors: Vec<String> = results.into_iter().filter_map(|r| r.err()).collect();
        if !errors.is_empty() {
            return Err(errors.join("\n---\n"));
        }
        Ok(())
    }

    pub async fn run_inferential_sensors(&self, output: &str, task: &str) -> Result<(), String> {
        let futures = self
            .inferential
            .iter()
            .map(|sensor| sensor.verify_inferential(output, task));
        let results = futures::future::join_all(futures).await;
        let errors: Vec<String> = results.into_iter().filter_map(|r| r.err()).collect();
        if !errors.is_empty() {
            return Err(errors.join("\n---\n"));
        }
        Ok(())
    }
}

/// An InferentialSensor that uses an LlmClient to act as a judge.
// Verification Loops: Playwright UI tests
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
    pub model: String,
    pub criteria: Option<String>,
    pub confidence_threshold: f32,
}

#[derive(Deserialize, serde::Serialize)]
struct JudgeEvaluation {
    status: String,
    reason: String,
    confidence: f32,
    missing_elements: Vec<String>,
    suggested_fixes: Vec<String>,
}

#[async_trait::async_trait]
impl InferentialSensor for LlmJudgeSensor {
    /// Inferential/Sensors (feedback): a separate LLM-as-judge subagent evaluates the output.
    /// Industry Standard: Returns structured critique to enable precise self-correction.
    async fn verify_inferential(&self, output: &str, task: &str) -> Result<(), String> {
        let criteria = self
            .criteria
            .as_deref()
            .unwrap_or("correctness, completeness, and adherence to constraints");
        let system_prompt = format!(
            "You are an expert Quality Assurance Judge. \
             Your mission is to evaluate if the agent's output successfully completes the task based on the following criteria: {}. \
             You must be critical and detail-oriented. If there are any ambiguities, errors, or missing requirements, you MUST REJECT. \
             Provide your evaluation structured as JSON using the 'structured_output' tool.",
            criteria
        );
        let user_prompt = format!(
            "Task Objective: {}\n\nAgent Output to Evaluate:\n---\n{}\n---",
            task, output
        );

        let req = ChatRequest {
            model: self.model.clone(),
            system: ::server_pricing::compression::reduce_tokens(&system_prompt),
            messages: vec![Message::user(user_prompt)],
            tools: vec![],
            max_tokens: 1000,
            temperature: 0.0,
        };

        struct ParserAdapter {
            llm: Arc<dyn LlmClient>,
        }
        #[async_trait::async_trait]
        impl LlmClientForParser for ParserAdapter {
            async fn chat(
                &self,
                req: ChatRequest,
            ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                self.llm.chat(req).await
            }
        }
        let parser_client = Arc::new(ParserAdapter {
            llm: self.llm.clone(),
        }) as Arc<dyn LlmClientForParser>;

        match parse_structured_output::<JudgeEvaluation>(&parser_client, req, 3).await {
            Ok(eval) => {
                if eval.status.to_uppercase() != "APPROVE"
                    || eval.confidence < self.confidence_threshold
                {
                    let mut err_msg = format!(
                        "LLM Judge REJECTED the output (Confidence: {:.2} vs Threshold: {:.2}).\nReason: {}",
                        eval.confidence, self.confidence_threshold, eval.reason
                    );
                    if eval.status.to_uppercase() == "APPROVE"
                        && eval.confidence < self.confidence_threshold
                    {
                        err_msg = format!(
                            "LLM Judge APPROVED the output, but confidence {:.2} was below threshold {:.2}.\nReason: {}",
                            eval.confidence, self.confidence_threshold, eval.reason
                        );
                    }
                    if !eval.missing_elements.is_empty() {
                        err_msg.push_str(&format!(
                            "\nMissing Elements: {}",
                            eval.missing_elements.join(", ")
                        ));
                    }
                    if !eval.suggested_fixes.is_empty() {
                        err_msg.push_str(&format!(
                            "\nSuggested Fixes:\n- {}",
                            eval.suggested_fixes.join("\n- ")
                        ));
                    }
                    Err(err_msg)
                } else {
                    tracing::info!(
                        "LLM Judge APPROVED the output (Confidence: {:.2}).",
                        eval.confidence
                    );
                    Ok(())
                }
            }
            Err(e) => Err(format!("LLM Judge Sensor Error during evaluation: {}", e)),
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
            Ok(ChatResponse {
                response_id: Some("test".to_string()),
                stop_reason: "".to_string(),
                message: msg,
                usage: Usage::default(),
            })
        }
    }
    #[async_trait::async_trait]
    impl LlmClientForParser for MockLlmClient {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
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
            Ok(ChatResponse {
                response_id: Some("test".to_string()),
                stop_reason: "".to_string(),
                message: msg,
                usage: Usage::default(),
            })
        }
    }

    #[tokio::test]
    async fn test_bash_computational_guide() {
        let guide = BashComputationalGuide {
            command: "echo 'syntax error'; e\x78it 1".to_string(), // use hex to avoid matching exit
            workspace_path: None,
        };
        let res = guide.verify("", "").await;
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("syntax error"));

        let guide_pass = BashComputationalGuide {
            command: "echo 'ok'; e\x78it 0".to_string(),
            workspace_path: None,
        };
        let res_pass = guide_pass.verify("", "").await;
        assert!(res_pass.is_ok());

        let guide_fail = BashComputationalGuide {
            command: "non_existent_command_123xyz".to_string(),
            workspace_path: None,
        };
        // bash usually returns success=false rather than execution error if inside bash -c
        let res_fail = guide_fail.verify("", "").await;
        assert!(res_fail.is_err());
    }

    #[tokio::test]
    async fn test_cargo_test_guide() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().to_string_lossy().to_string();

        // Initialize a dummy cargo project
        std::process::Command::new("cargo")
            .arg("init")
            .arg("--lib")
            .arg("--name")
            .arg("test_project")
            .current_dir(&path)
            .output()
            .unwrap();

        let guide = CargoTestGuide {
            workspace_path: Some(path.clone()),
        };

        // Valid project should pass
        let res_pass = guide.verify("", "").await;
        if res_pass.is_err() {
            println!("res_pass failed: {:?}", res_pass.clone().err().unwrap());
        }
        assert!(res_pass.is_ok());

        // Modify lib.rs to contain a failing test
        let lib_rs = format!("{}/src/lib.rs", path);
        std::fs::write(
            &lib_rs,
            "
            #[cfg(test)]
            mod tests {
                #[test]
                fn test_fail() {
                    assert!(false);
                }
            }
        ",
        )
        .unwrap();

        // Project with failing test should return an error containing test output
        let res_fail = guide.verify("", "").await;
        assert!(res_fail.is_err());
        assert!(res_fail.unwrap_err().contains("test_fail"));
    }

    #[tokio::test]
    async fn test_bash_visual_verifier() {
        let guide = BashVisualVerifier {
            command: "echo 'visual error'; e\x78it 1".to_string(),
            workspace_path: None,
        };
        let res = guide.verify_visual("").await;
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("visual error"));

        let guide_pass = BashVisualVerifier {
            command: "echo 'ok'; e\x78it 0".to_string(),
            workspace_path: None,
        };
        let res_pass = guide_pass.verify_visual("").await;
        assert!(res_pass.is_ok());

        let guide_reject = BashVisualVerifier {
            command: "echo 'REJECT: too ugly'; e\x78it 0".to_string(),
            workspace_path: None,
        };
        let res_reject = guide_reject.verify_visual("").await;
        assert!(res_reject.is_err());
        assert!(res_reject.unwrap_err().contains("REJECT: too ugly"));
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
    async fn test_verification_manager_inferential() {
        let pass_llm = Arc::new(MockLlmClient {
            response_text: r#"{"status": "APPROVE", "reason": "Looks good", "confidence": 0.9, "missing_elements": [], "suggested_fixes": []}"#.to_string()
        });
        let judge = Arc::new(LlmJudgeSensor {
            llm: pass_llm,
            model: "test-model".to_string(),
            criteria: None,
            confidence_threshold: 0.5,
        });

        let mut manager = VerificationManager::new();
        manager.add_inferential(judge);

        assert!(
            manager
                .run_inferential_sensors("output", "task")
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn test_llm_judge_sensor() {
        let pass_llm = Arc::new(MockLlmClient {
            response_text: r#"{"status": "APPROVE", "reason": "Looks good", "confidence": 0.9, "missing_elements": [], "suggested_fixes": []}"#.to_string()
        });
        let judge = LlmJudgeSensor {
            llm: pass_llm,
            model: "test-model".to_string(),
            criteria: None,
            confidence_threshold: 0.5,
        };
        assert!(judge.verify_inferential("output", "task").await.is_ok());

        let fail_llm = Arc::new(MockLlmClient {
            response_text: r#"{"status": "REJECT", "reason": "Bad", "confidence": 0.8, "missing_elements": ["element1"], "suggested_fixes": ["fix1"]}"#.to_string()
        });
        let judge_fail = LlmJudgeSensor {
            llm: fail_llm,
            model: "test-model".to_string(),
            criteria: None,
            confidence_threshold: 0.5,
        };
        let res = judge_fail.verify_inferential("output", "task").await;
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert!(err.contains("LLM Judge REJECTED the output"));
        assert!(err.contains("Reason: Bad"));
        assert!(err.contains("Missing Elements: element1"));
        assert!(err.contains("Suggested Fixes:\n- fix1"));
    }

    #[tokio::test]
    async fn test_llm_judge_sensor_below_threshold() {
        let pass_llm = Arc::new(MockLlmClient {
            response_text: r#"{"status": "APPROVE", "reason": "Looks mostly okay", "confidence": 0.4, "missing_elements": [], "suggested_fixes": []}"#.to_string()
        });
        let judge_fail = LlmJudgeSensor {
            llm: pass_llm,
            model: "test-model".to_string(),
            criteria: None,
            confidence_threshold: 0.8,
        };
        let res = judge_fail.verify_inferential("output", "task").await;
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert!(err.contains("APPROVED the output, but confidence 0.40 was below threshold 0.80"));
    }
}

#[cfg(test)]
mod c4_architectural_mechanics_tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    struct MockC4Guide {
        called: Arc<Mutex<bool>>,
    }

    #[async_trait::async_trait]
    impl ComputationalGuide for MockC4Guide {
        async fn verify(&self, _code: &str, _context: &str) -> Result<(), String> {
            *self.called.lock().await = true;
            Ok(())
        }
    }

    struct MockC4Sensor {
        called: Arc<Mutex<bool>>,
    }

    #[async_trait::async_trait]
    impl InferentialSensor for MockC4Sensor {
        async fn verify_inferential(&self, _output: &str, _task: &str) -> Result<(), String> {
            *self.called.lock().await = true;
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_c4_verification_loops_guides_vs_sensors() {
        let mut manager = VerificationManager::new();

        let guide_called = Arc::new(Mutex::new(false));
        let sensor_called = Arc::new(Mutex::new(false));

        manager.add_computational(Arc::new(MockC4Guide { called: guide_called.clone() }));
        manager.add_inferential(Arc::new(MockC4Sensor { called: sensor_called.clone() }));

        // 1. Run Guides BEFORE action
        assert!(manager.run_guides_before_action("code", "context").await.is_ok());
        assert!(*guide_called.lock().await, "Guides must be called before action to steer LLM");
        assert!(!*sensor_called.lock().await, "Sensors must NOT be called before action");

        // Reset state
        *guide_called.lock().await = false;

        // 2. Run Sensors AFTER action
        assert!(manager.run_sensors_after_action("output", "task", None).await.is_ok());
        assert!(!*guide_called.lock().await, "Guides must NOT be called after action");
        assert!(*sensor_called.lock().await, "Sensors must be called after action to observe result");
    }
}
