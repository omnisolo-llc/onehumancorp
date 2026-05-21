use crate::agent::AgentRunConfig;
use crate::types::{ChatRequest, Message};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait VerificationLoop: Send + Sync {
    async fn verify(
        &self,
        cfg: &AgentRunConfig,
        last_assistant_content: &str,
        messages: &mut Vec<Message>,
        llm: Arc<dyn crate::llm::LlmClient>,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;
}

pub struct ComputationalGuide;

#[async_trait]
impl VerificationLoop for ComputationalGuide {
    async fn verify(
        &self,
        cfg: &AgentRunConfig,
        _last_assistant_content: &str,
        messages: &mut Vec<Message>,
        _llm: Arc<dyn crate::llm::LlmClient>,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        if !cfg.enable_computational_guides || cfg.computational_guide_command.is_empty() {
            return Ok(true);
        }

        let wd = cfg.workspace_path.clone().unwrap_or_else(|| ".".to_string());
        let mut cmd = std::process::Command::new("bash");
        cmd.arg("-c").arg(&cfg.computational_guide_command).current_dir(wd);

        match cmd.output() {
            Ok(output) => {
                if !output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let err_msg = format!(
                        "Computational guide verification failed (command: {}).\nStdout: {}\nStderr: {}\nPlease correct your work and use tools to fix the issue before providing the final answer.",
                        cfg.computational_guide_command, stdout, stderr
                    );
                    messages.push(Message::user(err_msg));
                    return Ok(false);
                }
            }
            Err(e) => {
                let err_msg = format!("Failed to execute computational guide command '{}': {}", cfg.computational_guide_command, e);
                messages.push(Message::user(err_msg));
                return Ok(false);
            }
        }

        Ok(true)
    }
}

pub struct VisualVerifier;

#[async_trait]
impl VerificationLoop for VisualVerifier {
    async fn verify(
        &self,
        cfg: &AgentRunConfig,
        _last_assistant_content: &str,
        messages: &mut Vec<Message>,
        _llm: Arc<dyn crate::llm::LlmClient>,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        if !cfg.enable_visual_verification || cfg.visual_verification_command.is_empty() {
            return Ok(true);
        }

        let wd = cfg.workspace_path.clone().unwrap_or_else(|| ".".to_string());
        let mut cmd = std::process::Command::new("bash");
        cmd.arg("-c").arg(&cfg.visual_verification_command).current_dir(wd);

        match cmd.output() {
            Ok(output) => {
                if !output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let err_msg = format!(
                        "Visual verification failed (command: {}).\nStdout: {}\nStderr: {}\nPlease correct your work based on the visual feedback and use tools to fix the issue.",
                        cfg.visual_verification_command, stdout, stderr
                    );
                    messages.push(Message::user(err_msg));
                    return Ok(false);
                } else {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    if stdout.contains("REJECT") {
                        let err_msg = format!("Visual verification rejected the output. Reason: {}\nPlease correct your work and use tools to fix the issue.", stdout.trim());
                        messages.push(Message::user(err_msg));
                        return Ok(false);
                    }
                }
            }
            Err(e) => {
                let err_msg = format!("Failed to execute visual verification command '{}': {}", cfg.visual_verification_command, e);
                messages.push(Message::user(err_msg));
                return Ok(false);
            }
        }

        Ok(true)
    }
}

pub struct InferentialSensor;

#[async_trait]
impl VerificationLoop for InferentialSensor {
    async fn verify(
        &self,
        cfg: &AgentRunConfig,
        last_assistant_content: &str,
        messages: &mut Vec<Message>,
        llm: Arc<dyn crate::llm::LlmClient>,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        if !cfg.enable_llm_judge {
            return Ok(true);
        }

        #[derive(serde::Deserialize)]
        struct JudgeEvaluation {
            status: String,
            reason: String,
            confidence: f32,
        }

        let judge_req = ChatRequest {
            model: cfg.model.clone(),
            system: "You are an expert judge. Evaluate the following output for correctness, completeness, and adherence to constraints. Provide your evaluation structured exactly as requested, where status is either 'APPROVE' or 'REJECT'.".to_string(),
            messages: vec![Message::user(format!("Evaluate this output:\n{}", last_assistant_content))],
            tools: vec![],
            max_tokens: 500,
            temperature: 0.0,
        };

        struct ParserClientWrapper {
            llm: std::sync::Arc<dyn crate::llm::LlmClient>,
        }
        #[async_trait::async_trait]
        impl crate::output_parser::LlmClientForParser for ParserClientWrapper {
            async fn chat(&self, req: crate::types::ChatRequest) -> Result<crate::types::ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                self.llm.chat(req).await
            }
        }

        let parser_client: std::sync::Arc<dyn crate::output_parser::LlmClientForParser> = std::sync::Arc::new(ParserClientWrapper { llm });
        match crate::output_parser::parse_structured_output::<JudgeEvaluation>(&parser_client, judge_req, 3).await {
            Ok(eval) => {
                if eval.status.to_uppercase() == "REJECT" {
                    let err_msg = format!("Your previous output was evaluated by an LLM-as-judge and rejected. Reason: {}. Confidence: {:.2}. Please correct your work and use tools if necessary.", eval.reason, eval.confidence);
                    messages.push(Message::user(err_msg));
                    return Ok(false);
                }
            },
            Err(e) => {
                let err = format!("LLM Judge error: {}", e);
                return Err(err.into());
            }
        }

        Ok(true)
    }
}

pub struct VerificationRegistry {
    loops: Vec<Arc<dyn VerificationLoop>>,
}

impl VerificationRegistry {
    pub fn new() -> Self {
        Self { loops: Vec::new() }
    }

    pub fn register(&mut self, v_loop: Arc<dyn VerificationLoop>) {
        self.loops.push(v_loop);
    }

    pub async fn run_all(
        &self,
        cfg: &AgentRunConfig,
        last_assistant_content: &str,
        messages: &mut Vec<Message>,
        llm: Arc<dyn crate::llm::LlmClient>,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        for v_loop in &self.loops {
            let passed = v_loop.verify(cfg, last_assistant_content, messages, llm.clone()).await?;
            if !passed {
                return Ok(false); // Short circuit if any verification step fails
            }
        }
        Ok(true)
    }
}

pub fn default_verification_registry() -> VerificationRegistry {
    let mut registry = VerificationRegistry::new();
    registry.register(Arc::new(ComputationalGuide));
    registry.register(Arc::new(VisualVerifier));
    registry.register(Arc::new(InferentialSensor));
    registry
}
