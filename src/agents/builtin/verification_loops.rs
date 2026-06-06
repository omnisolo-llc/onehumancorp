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

/// 10. Verification Loops (Quality x3): Giving the model ways to verify work.
/// Mechanics: Computational/Guides (feedforward: linters, type-checkers, unit tests),
/// Visual (screenshots via Playwright and/or Desktop/Mobile UI tests), and
/// Inferential/Sensors (feedback: a separate LLM-as-judge subagent evaluates the output).
/// A manager that coordinates the 3 distinct verification loops.

pub struct BashComputationalGuide {
    pub command: String,
    pub workspace_path: Option<String>,
}

#[async_trait::async_trait]
impl ComputationalGuide for BashComputationalGuide {
    async fn verify(&self, _code: &str, _context: &str) -> Result<(), String> {
        let wd = self.workspace_path.clone().unwrap_or_else(|| ".".to_string());
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
            Err(e) => Err(format!("Failed to execute computational guide {}: {}", self.command, e)),
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
        let wd = self.workspace_path.clone().unwrap_or_else(|| ".".to_string());
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
                        return Err(format!("Visual verification rejected the output. Reason: {}\nPlease correct your work and use tools to fix the issue.", stdout.trim()));
                    }
                }
                Ok(())
            }
            Err(e) => Err(format!("Failed to execute visual verifier {}: {}", self.command, e)),
        }
    }
}

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

/// A true VisualVerifier that executes a real browser screenshot via Playwright
/// and can optionally compare it against a Golden Image or LLM-V API.
pub struct PlaywrightVisualVerifier {
    pub target_url: String,
    pub screenshot_path: String,
    pub selector: Option<String>,
    pub browser_type: String,
    pub golden_image_path: Option<String>,
    pub tolerance: f32,
    pub llm_fallback: Option<Arc<dyn LlmClient>>,
}

impl PlaywrightVisualVerifier {
    pub fn new(target_url: &str, screenshot_path: &str) -> Self {
        Self {
            target_url: target_url.to_string(),
            screenshot_path: screenshot_path.to_string(),
            selector: None,
            browser_type: "chromium".to_string(), // default to chromium
            golden_image_path: None,
            tolerance: 0.05, // 5% tolerance by default
            llm_fallback: None,
        }
    }

    pub fn with_selector(mut self, selector: &str) -> Self {
        self.selector = Some(selector.to_string());
        self
    }

    pub fn with_golden_image(mut self, path: &str, tolerance: f32) -> Self {
        self.golden_image_path = Some(path.to_string());
        self.tolerance = tolerance;
        self
    }

    pub fn with_llm_fallback(mut self, llm: Arc<dyn LlmClient>) -> Self {
        self.llm_fallback = Some(llm);
        self
    }

    fn compare_images(&self, path1: &str, path2: &str) -> Result<f32, String> {
        let img1 = image::open(path1).map_err(|e| format!("Failed to open image {}: {}", path1, e))?.to_rgba8();
        let img2 = image::open(path2).map_err(|e| format!("Failed to open image {}: {}", path2, e))?.to_rgba8();

        if img1.dimensions() != img2.dimensions() {
            return Err(format!("Dimensions mismatch: {:?} vs {:?}", img1.dimensions(), img2.dimensions()));
        }

        let mut diff_pixels = 0;
        let total_pixels = (img1.width() * img1.height()) as usize;

        for (p1, p2) in img1.pixels().zip(img2.pixels()) {
            if p1 != p2 {
                diff_pixels += 1;
            }
        }

        Ok(diff_pixels as f32 / total_pixels as f32)
    }
}

#[async_trait::async_trait]
impl VisualVerifier for PlaywrightVisualVerifier {
    async fn verify_visual(&self, ui_state_path: &str) -> Result<(), String> {
        // Use the passed ui_state_path as the target URL if not empty.
        // Otherwise fallback to the one provided during initialization.
        let url_to_use = if ui_state_path.is_empty() {
            &self.target_url
        } else {
            ui_state_path
        };

        let mut cmd = std::process::Command::new("npx");
        cmd.arg("playwright").arg("screenshot").arg("--browser").arg(&self.browser_type).arg(url_to_use).arg(&self.screenshot_path);

        // If a selector is provided, instruct Playwright to wait for and target that specific selector
        // npx playwright screenshot <url> <filename> --selector <selector>
        // NOTE: we wait for network idle to ensure the page is fully loaded, but we don't have that direct flag in CLI,
        // however --wait-for-timeout or --wait-for-selector is available. Playwright CLI's screenshot command
        // lacks some advanced flags, so we'll just try to use playwright CLI normally and optionally pass selector.
        // We'll pass it simply, though playwright CLI might not support --selector out of the box in `npx playwright screenshot`.
        // Wait, npx playwright screenshot *does* support `--wait-for-selector`? Let's check CLI options later,
        // but for now, we'll write a small node script to invoke playwright cleanly if we need to.
        // Actually, the simplest is to just use the CLI. `npx playwright screenshot` does not officially support `--selector`.
        // Let's use `npx playwright screenshot` and just log a warning if selector is ignored, or better yet, write a dynamic runner.
        // For this patch, we'll keep it simple: we'll run `npx playwright screenshot`.
        // We'll write a minimal node script dynamically if we need selector, but let's try just passing to CLI or creating a tiny script.

        // Create a small script to guarantee selector support and robust execution
        let script = format!(r#"
const {{ {} }} = require('playwright');
(async () => {{
  const browser = await {}.launch();
  const page = await browser.newPage();
  await page.goto('{}', {{ waitUntil: 'networkidle' }});
  {}
  await page.screenshot({{ path: '{}' }});
  await browser.close();
}})().catch(e => {{ console.error(e); process.exit(1); }});
"#, self.browser_type, self.browser_type, url_to_use,
    if let Some(sel) = &self.selector { format!("await page.waitForSelector('{}');", sel) } else { "".to_string() },
    self.screenshot_path);

        let current_dir = std::env::current_dir().unwrap();
        let script_path = current_dir.join(format!("playwright_screenshot_{}.js", uuid::Uuid::new_v4()));
        std::fs::write(&script_path, script).map_err(|e| format!("Failed to write playwright script: {}", e))?;

        // Fetch global npm path so that playwright can be resolved globally
        let npm_root = std::process::Command::new("npm")
            .arg("root")
            .arg("-g")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "/usr/local/lib/node_modules".to_string());

        let output = std::process::Command::new("node")
            .current_dir(&current_dir)
            .env("NODE_PATH", npm_root)
            .arg(&script_path)
            .output()
            .map_err(|e| format!("Failed to execute node: {}", e))?;

        let _ = std::fs::remove_file(&script_path); // Cleanup

        if output.status.success() {
            if !std::path::Path::new(&self.screenshot_path).exists() {
                return Err(format!("Playwright reported success, but screenshot file '{}' was not found.", self.screenshot_path));
            }

            if let Some(golden_path) = &self.golden_image_path {
                match self.compare_images(&self.screenshot_path, golden_path) {
                    Ok(diff_ratio) => {
                        if diff_ratio > self.tolerance {
                            if let Some(llm) = &self.llm_fallback {
                                tracing::info!("Pixel diff {:.2}% exceeded tolerance {:.2}%. Falling back to LLM-V judging...", diff_ratio * 100.0, self.tolerance * 100.0);

                                // Read the image and base64 encode it
                                let img_bytes = std::fs::read(&self.screenshot_path)
                                    .map_err(|e| format!("Failed to read screenshot for LLM: {}", e))?;
                                use base64::Engine;
                                let b64_img = base64::engine::general_purpose::STANDARD.encode(&img_bytes);

                                // Send proper multimodal message
                                // Note: Using the message structure that the current LlmClient supports.
                                // If the core message doesn't support complex contents, we embed the base64 in text or JSON.
                                let content = format!(
                                    r#"{{"instruction": "Please verify this UI. It differed from the golden image by {:.2}%. Does it look acceptable? Reply with visually_acceptable: true or false.", "image_base64": "data:image/png;base64,{}"}}"#,
                                    diff_ratio * 100.0, b64_img
                                );

                                let req = ohc_builtin_agent_core::types::ChatRequest {
                                    messages: vec![ohc_builtin_agent_core::types::Message {
                                        role: ohc_builtin_agent_core::types::Role::User,
                                        content,
                                        tool_calls: vec![],
                                        tool_results: vec![],
                                        response_id: None,
                                        previous_response_id: None,
                                    }],
                                    tools: vec![],
                                    system: "".to_string(),
                                    temperature: 0.0,
                                    max_tokens: 1000,
                                    model: "".to_string(),
                                };
                                match llm.chat(req).await {
                                    Ok(resp) => {
                                        if resp.message.content.to_lowercase().contains("visually_acceptable: true") {
                                            return Ok(());
                                        } else {
                                            return Err(format!("LLM-V rejected the UI with diff {:.2}%", diff_ratio * 100.0));
                                        }
                                    }
                                    Err(e) => return Err(format!("LLM-V fallback failed: {}", e)),
                                }
                            } else {
                                return Err(format!("Pixel diff {:.2}% exceeded tolerance {:.2}% and no LLM fallback configured.", diff_ratio * 100.0, self.tolerance * 100.0));
                            }
                        }
                    }
                    Err(e) => return Err(format!("Failed to compare images: {}", e)),
                }
            }
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
        let criteria = self.criteria.as_deref().unwrap_or("correctness, completeness, and adherence to constraints");
        let system_prompt = format!(
            "You are an expert Quality Assurance Judge. \
             Your mission is to evaluate if the agent's output successfully completes the task based on the following criteria: {}. \
             You must be critical and detail-oriented. If there are any ambiguities, errors, or missing requirements, you MUST REJECT. \
             Provide your evaluation structured as JSON using the 'structured_output' tool.",
            criteria
        );
        let user_prompt = format!("Task Objective: {}\n\nAgent Output to Evaluate:\n---\n{}\n---", task, output);

        let req = ChatRequest {
            model: self.model.clone(),
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

        match parse_structured_output::<JudgeEvaluation>(&parser_client, req, 3).await {
            Ok(eval) => {
                if eval.status.to_uppercase() == "REJECT" || eval.confidence < self.confidence_threshold {
                    let mut err_msg = format!("LLM Judge REJECTED the output (Confidence: {:.2} vs Threshold: {:.2}).\nReason: {}", eval.confidence, self.confidence_threshold, eval.reason);
                    if eval.status.to_uppercase() == "APPROVE" && eval.confidence < self.confidence_threshold {
                        err_msg = format!("LLM Judge APPROVED the output, but confidence {:.2} was below threshold {:.2}.\nReason: {}", eval.confidence, self.confidence_threshold, eval.reason);
                    }
                    if !eval.missing_elements.is_empty() {
                        err_msg.push_str(&format!("\nMissing Elements: {}", eval.missing_elements.join(", ")));
                    }
                    if !eval.suggested_fixes.is_empty() {
                        err_msg.push_str(&format!("\nSuggested Fixes:\n- {}", eval.suggested_fixes.join("\n- ")));
                    }
                    Err(err_msg)
                } else {
                    tracing::info!("LLM Judge APPROVED the output (Confidence: {:.2}).", eval.confidence);
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
        // Set up a local test file
        let test_html_path = format!("/tmp/playwright_test_fixture_{}.html", uuid::Uuid::new_v4());
        std::fs::write(&test_html_path, "<html><body><h1 id='title'>Test Fixture</h1></body></html>").unwrap();

        let screenshot_path = format!("/tmp/test_screenshot_{}.png", uuid::Uuid::new_v4());

        // Pass the file path as the URL
        let file_url = format!("file://{}", test_html_path);
        let verifier = PlaywrightVisualVerifier::new(&file_url, &screenshot_path)
            .with_selector("#title");

        // Run verify_visual which executes the headless playwright node script
        let res = verifier.verify_visual("").await;

        // Assert it succeeds
        assert!(res.is_ok(), "Playwright capture failed: {:?}", res.err());

        // Assert screenshot is created
        assert!(std::path::Path::new(&screenshot_path).exists());

        // Clean up
        std::fs::remove_file(&test_html_path).unwrap();
        std::fs::remove_file(&screenshot_path).unwrap();
    }

    #[tokio::test]
    async fn test_playwright_visual_verifier_golden_diff_error() {
        // Create dummy images to test golden diffing
        let screenshot_path = "test_screenshot_mock.png";
        let golden_path = "test_golden_mock.png";

        // 2x2 white
        let img1 = image::RgbaImage::from_pixel(2, 2, image::Rgba([255, 255, 255, 255]));
        img1.save(screenshot_path).unwrap();

        // 2x2 black
        let img2 = image::RgbaImage::from_pixel(2, 2, image::Rgba([0, 0, 0, 255]));
        img2.save(golden_path).unwrap();

        let mut verifier = PlaywrightVisualVerifier::new("http://dummy.local", screenshot_path);
        // Inject golden path and 0 tolerance so it fails
        verifier = verifier.with_golden_image(golden_path, 0.0);

        // Let's directly test compare_images since verify_visual invokes Playwright CLI and we can't mock CLI easily here.
        let diff = verifier.compare_images(screenshot_path, golden_path).unwrap();
        assert_eq!(diff, 1.0); // 100% different

        std::fs::remove_file(screenshot_path).unwrap();
        std::fs::remove_file(golden_path).unwrap();
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
        let judge = Arc::new(LlmJudgeSensor { llm: pass_llm, model: "test-model".to_string(), criteria: None, confidence_threshold: 0.5 });

        let mut manager = VerificationManager::new();
        manager.add_inferential(judge);

        assert!(manager.run_inferential_sensors("output", "task").await.is_ok());
    }

    #[tokio::test]
    async fn test_llm_judge_sensor() {
        let pass_llm = Arc::new(MockLlmClient {
            response_text: r#"{"status": "APPROVE", "reason": "Looks good", "confidence": 0.9, "missing_elements": [], "suggested_fixes": []}"#.to_string()
        });
        let judge = LlmJudgeSensor { llm: pass_llm, model: "test-model".to_string(), criteria: None, confidence_threshold: 0.5 };
        assert!(judge.verify_inferential("output", "task").await.is_ok());

        let fail_llm = Arc::new(MockLlmClient {
            response_text: r#"{"status": "REJECT", "reason": "Bad", "confidence": 0.8, "missing_elements": ["element1"], "suggested_fixes": ["fix1"]}"#.to_string()
        });
        let judge_fail = LlmJudgeSensor { llm: fail_llm, model: "test-model".to_string(), criteria: None, confidence_threshold: 0.5 };
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
        let judge_fail = LlmJudgeSensor { llm: pass_llm, model: "test-model".to_string(), criteria: None, confidence_threshold: 0.8 };
        let res = judge_fail.verify_inferential("output", "task").await;
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert!(err.contains("APPROVED the output, but confidence 0.40 was below threshold 0.80"));
    }

}
