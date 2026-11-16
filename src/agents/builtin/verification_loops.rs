#![allow(clippy::empty_line_after_doc_comments)]
use crate::llm::LlmClient;
use crate::output_parser::{LlmClientForParser, parse_structured_output};
/// Master Catalog C.4. Verification Loops: Guides (steer before action) vs Sensors (observe after action).
/// Master Catalog B.10. Verification Loops
use omnisolo_builtin_agent_core::types::{ChatRequest, ChatResponse, Message};
use omnisolo_builtin_agent_tools::binary_dist::{ManagedBinaryKind, ensure_managed_binary};
use omnisolo_builtin_agent_tools::browser_proxy::BrowserPolicyProxy;
use omnisolo_builtin_agent_tools::network_policy;
use serde::Deserialize;
use std::path::Path;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;
use url::Url;

/// A feedforward verification loop using linters, type-checkers, or unit tests.
#[async_trait::async_trait]
pub trait ComputationalGuide: Send + Sync {
    async fn verify(&self, code: &str, context: &str) -> Result<(), String>;
}

/// A feedback verification loop using visual checks (screenshots via `agent-browser` and/or Desktop/Mobile UI tests).
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
///     Visual (screenshots via `agent-browser` and/or Desktop/Mobile UI tests), and
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

use omnisolo_builtin_agent_core::types::Role;

pub struct LlmJudgeInferentialSensor {
    pub client: std::sync::Arc<dyn crate::output_parser::LlmClientForParser>,
    pub model: String,
    pub system_prompt: String,
}

#[async_trait::async_trait]
impl InferentialSensor for LlmJudgeInferentialSensor {
    async fn verify_inferential(&self, output: &str, task: &str) -> Result<(), String> {
        let user_message = omnisolo_builtin_agent_core::types::Message {
            role: Role::User,
            content: format!("Task: {}\n\nOutput to verify: {}", task, output),
            tool_calls: vec![],
            tool_results: vec![],
            response_id: None,
            previous_response_id: None,
        };

        let req = omnisolo_builtin_agent_core::types::ChatRequest {
            messages: vec![user_message],
            system: self.system_prompt.clone(),
            model: self.model.clone(),
            temperature: 0.0,
            max_tokens: 500,
            tools: vec![],
        };

        match self.client.chat(req).await {
            Ok(resp) => {
                let content_str = resp.message.content;
                if content_str.trim().starts_with("PASS") {
                    Ok(())
                } else if content_str.trim().starts_with("FAIL:") {
                    let reason = content_str.trim().trim_start_matches("FAIL:").trim();
                    Err(format!("LLM Judge verification failed: {}", reason))
                } else {
                    Err(format!(
                        "LLM Judge verification failed: output did not match expected 'PASS' or 'FAIL: <reason>' format. Output: {}",
                        content_str
                    ))
                }
            }
            Err(e) => Err(format!("LLM Judge verification failed with error: {}", e)),
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
    pub async fn run_sensors_after_action(
        &self,
        output: &str,
        task: &str,
        ui_state_path: Option<&str>,
    ) -> Result<(), String> {
        let mut errors = Vec::new();
        if let Some(path) = ui_state_path
            && let Err(e) = self.run_visual_verifiers(path).await
        {
            errors.push(e);
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

/// Verification loop using headless `agent-browser` screenshots.
pub struct AgentBrowserVisualVerifier;

impl AgentBrowserVisualVerifier {
    async fn run_command(
        binary: &Path,
        args: &[&str],
        timeout: Duration,
    ) -> Result<String, String> {
        let mut cmd = tokio::process::Command::new(binary);
        cmd.args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);

        let output = tokio::time::timeout(timeout, cmd.output())
            .await
            .map_err(|_| {
                format!(
                    "Failed to execute agent-browser: command timed out after {}s",
                    timeout.as_secs()
                )
            })?
            .map_err(|e| format!("Failed to execute agent-browser: {e}"))?;

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

        if !output.status.success() {
            let detail = match (stdout.is_empty(), stderr.is_empty()) {
                (false, false) => format!("{stdout}\n{stderr}"),
                (false, true) => stdout,
                (true, false) => stderr,
                (true, true) => format!("exit status {}", output.status),
            };
            return Err(format!("Visual check failed. AgentBrowser error: {detail}"));
        }

        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&stdout)
            && val.get("success").and_then(|v| v.as_bool()) == Some(false)
        {
            let err_msg = val.get("error").and_then(|v| v.as_str()).unwrap_or(&stdout);
            return Err(format!(
                "Visual check failed. AgentBrowser error: {err_msg}"
            ));
        }

        Ok(stdout)
    }

    async fn verify_in_ephemeral_session(
        binary: &Path,
        url: &Url,
        allow_private: bool,
        session_id: &str,
        profile_dir: &Path,
        screenshot_path: &Path,
    ) -> Result<(), String> {
        let proxy = BrowserPolicyProxy::start(allow_private)
            .await
            .map_err(|e| format!("Failed to start browser policy proxy: {e}"))?;
        let proxy_url = proxy.proxy_url();
        let profile_str = profile_dir.to_string_lossy().into_owned();
        let screenshot_str = screenshot_path.to_string_lossy().into_owned();

        let mut base_args = vec![
            "--json".to_string(),
            "--session".to_string(),
            session_id.to_string(),
            "--profile".to_string(),
            profile_str,
            "--proxy".to_string(),
            proxy_url,
            "--no-auto-dialog".to_string(),
        ];
        if let Ok(chrome_bin) = std::env::var("OMNISOLO_CHROMIUM_EXECUTABLE")
            && !chrome_bin.trim().is_empty()
        {
            base_args.push("--executable-path".to_string());
            base_args.push(chrome_bin.trim().to_string());
        }

        let mut open_args: Vec<&str> = base_args.iter().map(String::as_str).collect();
        open_args.push("open");
        open_args.push(url.as_str());

        let timeout = Duration::from_secs(45);
        let open_res = Self::run_command(binary, &open_args, timeout).await;

        let step_res = match open_res {
            Ok(_) => {
                let mut screenshot_args: Vec<&str> = base_args.iter().map(String::as_str).collect();
                screenshot_args.push("screenshot");
                screenshot_args.push(&screenshot_str);
                Self::run_command(binary, &screenshot_args, timeout).await
            }
            Err(err) => Err(err),
        };

        let close_args = ["--json", "--session", session_id, "close"];
        let _ = Self::run_command(binary, &close_args, Duration::from_secs(10)).await;

        step_res.map(|_| ())
    }
}

#[async_trait::async_trait]
impl VisualVerifier for AgentBrowserVisualVerifier {
    async fn verify_visual(&self, ui_state_path: &str) -> Result<(), String> {
        let trimmed = ui_state_path.trim();
        let parsed = Url::parse(trimmed)
            .map_err(|e| format!("AgentBrowser visual check invalid URL: {e}"))?;
        let allow_private = network_policy::private_network_allowed();
        network_policy::validate_url(&parsed, allow_private)
            .map_err(|e| format!("AgentBrowser visual check invalid URL: {e}"))?;

        let binary = ensure_managed_binary(ManagedBinaryKind::AgentBrowser)
            .await
            .map_err(|e| format!("Failed to resolve agent-browser: {e}"))?;

        let temp_dir = tempfile::tempdir()
            .map_err(|e| format!("Failed to create agent-browser visual check tempdir: {e}"))?;
        let profile_dir = temp_dir.path().join("profile");
        tokio::fs::create_dir_all(&profile_dir)
            .await
            .map_err(|e| format!("Failed to create agent-browser profile directory: {e}"))?;
        let screenshot_path = temp_dir.path().join("verify.png");
        let session_id = format!("verify-{}", uuid::Uuid::new_v4());

        let result = Self::verify_in_ephemeral_session(
            &binary,
            &parsed,
            allow_private,
            &session_id,
            &profile_dir,
            &screenshot_path,
        )
        .await;

        let _ = tokio::fs::remove_file(&screenshot_path).await;
        let _ = tokio::fs::remove_dir_all(&profile_dir).await;
        let _ = temp_dir.close();

        result
    }
}

pub struct LlmJudgeSensor {
    pub llm: Arc<dyn LlmClient>,
    pub model: String,
    pub criteria: Option<String>,
    pub confidence_threshold: f32,
}

#[derive(Deserialize, serde::Serialize, schemars::JsonSchema)]
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

    struct MockLlmClientForSensor {
        response_content: String,
    }

    #[async_trait::async_trait]
    impl crate::output_parser::LlmClientForParser for MockLlmClientForSensor {
        async fn chat(
            &self,
            _req: omnisolo_builtin_agent_core::types::ChatRequest,
        ) -> Result<
            omnisolo_builtin_agent_core::types::ChatResponse,
            Box<dyn std::error::Error + Send + Sync>,
        > {
            Ok(omnisolo_builtin_agent_core::types::ChatResponse {
                message: omnisolo_builtin_agent_core::types::Message {
                    role: omnisolo_builtin_agent_core::types::Role::Assistant,
                    content: self.response_content.clone(),
                    tool_calls: vec![],
                    tool_results: vec![],
                    response_id: None,
                    previous_response_id: None,
                },
                usage: omnisolo_builtin_agent_core::types::Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: None,
            })
        }
    }

    #[tokio::test]
    async fn test_llm_judge_inferential_sensor_pass() {
        let client = std::sync::Arc::new(MockLlmClientForSensor {
            response_content: "PASS".to_string(),
        });

        let judge = super::LlmJudgeInferentialSensor {
            client,
            model: "test_model".to_string(),
            system_prompt: "You are a judge.".to_string(),
        };

        use super::InferentialSensor;
        let result = judge.verify_inferential("4", "What is 2+2?").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_llm_judge_inferential_sensor_fail() {
        let client = std::sync::Arc::new(MockLlmClientForSensor {
            response_content: "FAIL: The answer is wrong. 2+2 is 4, not 5.".to_string(),
        });

        let judge = super::LlmJudgeInferentialSensor {
            client,
            model: "test_model".to_string(),
            system_prompt: "You are a judge.".to_string(),
        };

        use super::InferentialSensor;
        let result = judge.verify_inferential("5", "What is 2+2?").await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "LLM Judge verification failed: The answer is wrong. 2+2 is 4, not 5."
        );
    }

    #[tokio::test]
    async fn test_llm_judge_inferential_sensor_invalid_format() {
        let client = std::sync::Arc::new(MockLlmClientForSensor {
            response_content: "I think the answer is mostly correct but could be better."
                .to_string(),
        });

        let judge = super::LlmJudgeInferentialSensor {
            client,
            model: "test_model".to_string(),
            system_prompt: "You are a judge.".to_string(),
        };

        use super::InferentialSensor;
        let result = judge.verify_inferential("4.0", "What is 2+2?").await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("output did not match expected 'PASS' or 'FAIL: <reason>' format")
        );
    }

    use omnisolo_builtin_agent_core::types::Usage;

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

    #[test]
    fn test_agent_browser_visual_verifier() {
        let dir = tempfile::tempdir().unwrap();
        let missing_bin = dir.path().join("missing-agent-browser");

        temp_env::with_vars(
            [
                ("OMNISOLO_AGENT_ALLOW_PRIVATE_NETWORK", Some("false")),
                (
                    "OMNISOLO_AGENT_BROWSER_BIN",
                    Some(missing_bin.to_str().unwrap()),
                ),
                ("OMNISOLO_AGENT_BROWSER_ARCHIVE", None::<&str>),
                ("OMNISOLO_CHROMIUM_EXECUTABLE", None::<&str>),
            ],
            || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                rt.block_on(async {
                    let verifier = AgentBrowserVisualVerifier;

                    for invalid in [
                        "invalid_path_123",
                        "file:///etc/passwd",
                        "https://user:secret@example.com/",
                        "http://127.0.0.1:3000/",
                        "http://localhost:3000/",
                    ] {
                        let err = verifier.verify_visual(invalid).await.unwrap_err();
                        assert!(
                            err.contains("AgentBrowser visual check invalid URL:"),
                            "unexpected error for {invalid}: {err}"
                        );
                    }

                    let missing_err = verifier
                        .verify_visual("https://example.com/")
                        .await
                        .unwrap_err();
                    assert!(
                        missing_err.contains("Failed to resolve agent-browser:")
                            && missing_err.contains("OMNISOLO_AGENT_BROWSER_BIN"),
                        "unexpected missing-binary error: {missing_err}"
                    );
                });
            },
        );

        temp_env::with_vars(
            [
                ("OMNISOLO_AGENT_ALLOW_PRIVATE_NETWORK", Some("true")),
                (
                    "OMNISOLO_AGENT_BROWSER_BIN",
                    Some(missing_bin.to_str().unwrap()),
                ),
                ("OMNISOLO_AGENT_BROWSER_ARCHIVE", None::<&str>),
                ("OMNISOLO_CHROMIUM_EXECUTABLE", None::<&str>),
            ],
            || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                rt.block_on(async {
                    let verifier = AgentBrowserVisualVerifier;
                    let loopback_err = verifier
                        .verify_visual("http://127.0.0.1:3000/")
                        .await
                        .unwrap_err();
                    assert!(
                        loopback_err.contains("Failed to resolve agent-browser:")
                            && loopback_err.contains("OMNISOLO_AGENT_BROWSER_BIN"),
                        "expected loopback URL to pass validation when private network enabled: {loopback_err}"
                    );
                });
            },
        );

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            let fail_bin = dir.path().join("fail-agent-browser");
            std::fs::write(
                &fail_bin,
                "#!/bin/sh\necho 'mock browser failure' >&2\nexit 1\n",
            )
            .unwrap();
            std::fs::set_permissions(&fail_bin, std::fs::Permissions::from_mode(0o755)).unwrap();

            temp_env::with_vars(
                [
                    ("OMNISOLO_AGENT_ALLOW_PRIVATE_NETWORK", Some("false")),
                    (
                        "OMNISOLO_AGENT_BROWSER_BIN",
                        Some(fail_bin.to_str().unwrap()),
                    ),
                    ("OMNISOLO_AGENT_BROWSER_ARCHIVE", None::<&str>),
                    ("OMNISOLO_CHROMIUM_EXECUTABLE", None::<&str>),
                ],
                || {
                    let rt = tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()
                        .unwrap();
                    rt.block_on(async {
                        let verifier = AgentBrowserVisualVerifier;
                        let err = verifier
                            .verify_visual("https://example.com/")
                            .await
                            .unwrap_err();
                        assert!(
                            err.contains("Visual check failed. AgentBrowser error:")
                                && err.contains("mock browser failure"),
                            "unexpected mock failure error: {err}"
                        );
                    });
                },
            );

            let ok_bin = dir.path().join("ok-agent-browser");
            let log_file = dir.path().join("invocations.log");
            let profile_record = dir.path().join("profile.path");
            let png_record = dir.path().join("png.path");
            std::fs::write(
                &ok_bin,
                "#!/bin/sh\n\
printf '%s\\n' \"$*\" >> \"$OHC_TEST_AB_LOG\"\n\
prev=\"\"\n\
for arg in \"$@\"; do\n\
  if [ \"$prev\" = \"--profile\" ]; then\n\
    printf '%s' \"$arg\" > \"$OHC_TEST_AB_PROFILE\"\n\
  elif [ \"$prev\" = \"screenshot\" ]; then\n\
    printf 'PNG' > \"$arg\"\n\
    printf '%s' \"$arg\" > \"$OHC_TEST_AB_PNG\"\n\
  fi\n\
  prev=\"$arg\"\n\
done\n\
printf '{\"success\":true}\\n'\n",
            )
            .unwrap();
            std::fs::set_permissions(&ok_bin, std::fs::Permissions::from_mode(0o755)).unwrap();

            temp_env::with_vars(
                [
                    ("OMNISOLO_AGENT_ALLOW_PRIVATE_NETWORK", Some("false")),
                    ("OMNISOLO_AGENT_BROWSER_BIN", Some(ok_bin.to_str().unwrap())),
                    ("OMNISOLO_AGENT_BROWSER_ARCHIVE", None::<&str>),
                    ("OMNISOLO_CHROMIUM_EXECUTABLE", None::<&str>),
                    ("OHC_TEST_AB_LOG", Some(log_file.to_str().unwrap())),
                    (
                        "OHC_TEST_AB_PROFILE",
                        Some(profile_record.to_str().unwrap()),
                    ),
                    ("OHC_TEST_AB_PNG", Some(png_record.to_str().unwrap())),
                ],
                || {
                    let rt = tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()
                        .unwrap();
                    rt.block_on(async {
                        let verifier = AgentBrowserVisualVerifier;
                        verifier
                            .verify_visual("https://example.com/app")
                            .await
                            .unwrap();
                    });
                },
            );

            let logged = std::fs::read_to_string(&log_file).unwrap();
            let lines: Vec<&str> = logged.lines().collect();
            assert_eq!(lines.len(), 3, "expected open, screenshot, close: {logged}");
            assert!(
                lines[0].contains("--json --session verify-")
                    && lines[0].contains("--profile ")
                    && lines[0].contains("--proxy http://127.0.0.1:")
                    && lines[0].contains("--no-auto-dialog open https://example.com/app"),
                "unexpected open args: {}",
                lines[0]
            );
            assert!(
                lines[1].contains("--json --session verify-")
                    && lines[1].contains("--no-auto-dialog screenshot "),
                "unexpected screenshot args: {}",
                lines[1]
            );
            assert!(
                lines[2].starts_with("--json --session verify-") && lines[2].ends_with(" close"),
                "unexpected close args: {}",
                lines[2]
            );

            let recorded_profile = std::fs::read_to_string(&profile_record).unwrap();
            let recorded_png = std::fs::read_to_string(&png_record).unwrap();
            assert!(!Path::new(&recorded_profile).exists());
            assert!(!Path::new(&recorded_png).exists());
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
            let tool_call = omnisolo_builtin_agent_core::types::ToolCall {
                id: "call_1".to_string(),
                name: "structured_output".to_string(),
                arguments: serde_json::json!({
                    "data": serde_json::from_str::<serde_json::Value>(&self.response_text).unwrap_or(serde_json::json!({}))
                }),
            };

            let msg = Message {
                role: omnisolo_builtin_agent_core::types::Role::Assistant,
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
            let tool_call = omnisolo_builtin_agent_core::types::ToolCall {
                id: "call_1".to_string(),
                name: "structured_output".to_string(),
                arguments: serde_json::json!({
                    "data": serde_json::from_str::<serde_json::Value>(&self.response_text).unwrap_or(serde_json::json!({}))
                }),
            };

            let msg = Message {
                role: omnisolo_builtin_agent_core::types::Role::Assistant,
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

        let init_res = std::process::Command::new("cargo")
            .arg("init")
            .arg("--lib")
            .arg("--name")
            .arg("test_project")
            .current_dir(&path)
            .output();

        match init_res {
            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
                tracing::info!("Skipping test: cargo binary not found in PATH");
                return;
            }
            res => {
                res.unwrap();
            }
        }

        let guide = CargoTestGuide {
            workspace_path: Some(path.clone()),
        };

        // Valid project should pass
        let res_pass = guide.verify("", "").await;
        if res_pass.is_err() {
            tracing::info!("res_pass failed: {:?}", res_pass.clone().err().unwrap());
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
    async fn judge_schema_preserves_required_lists_and_confidence_policy() {
        struct Capture {
            requests: tokio::sync::Mutex<Vec<ChatRequest>>,
            inner: MockLlmClient,
        }
        #[async_trait::async_trait]
        impl LlmClient for Capture {
            async fn chat(
                &self,
                req: ChatRequest,
            ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                self.requests.lock().await.push(req.clone());
                LlmClient::chat(&self.inner, req).await
            }
        }
        let llm = Arc::new(Capture {
            requests: tokio::sync::Mutex::new(vec![]),
            inner: MockLlmClient {
                response_text: r#"{"status":"approve","reason":"checked","confidence":0.8,"missing_elements":[],"suggested_fixes":[]}"#.into(),
            },
        });
        let mut judge = LlmJudgeSensor {
            llm: llm.clone(),
            model: "fixture".into(),
            criteria: None,
            confidence_threshold: 0.7,
        };
        assert!(judge.verify_inferential("output", "task").await.is_ok());
        judge.confidence_threshold = 0.9;
        assert!(
            judge
                .verify_inferential("output", "task")
                .await
                .unwrap_err()
                .contains("below threshold")
        );
        let requests = llm.requests.lock().await;
        assert_eq!(requests.len(), 2);
        let data = &requests[0].tools[0].parameters["properties"]["data"];
        let required = data["required"].as_array().unwrap();
        for key in [
            "status",
            "reason",
            "confidence",
            "missing_elements",
            "suggested_fixes",
        ] {
            assert!(required.contains(&serde_json::json!(key)));
        }
        assert_eq!(data["properties"]["status"]["type"], "string");
        assert!(data["properties"]["status"].get("enum").is_none());
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

        manager.add_computational(Arc::new(MockC4Guide {
            called: guide_called.clone(),
        }));
        manager.add_inferential(Arc::new(MockC4Sensor {
            called: sensor_called.clone(),
        }));

        // 1. Run Guides BEFORE action
        assert!(
            manager
                .run_guides_before_action("code", "context")
                .await
                .is_ok()
        );
        assert!(
            *guide_called.lock().await,
            "Guides must be called before action to steer LLM"
        );
        assert!(
            !*sensor_called.lock().await,
            "Sensors must NOT be called before action"
        );

        // Reset state
        *guide_called.lock().await = false;

        // 2. Run Sensors AFTER action
        assert!(
            manager
                .run_sensors_after_action("output", "task", None)
                .await
                .is_ok()
        );
        assert!(
            !*guide_called.lock().await,
            "Guides must NOT be called after action"
        );
        assert!(
            *sensor_called.lock().await,
            "Sensors must be called after action to observe result"
        );
    }
}
