use std::sync::Arc;
use async_trait::async_trait;
use ohc_builtin_agent_core::types::{ChatRequest, Message};
use ohc_builtin_agent_llm::LlmClient;

#[async_trait]
pub trait Verifier: Send + Sync + std::fmt::Debug {
    async fn verify(&self, final_output: &str, original_prompt: &str) -> Result<(), String>;
}

#[derive(Debug)]
pub struct ComputationalVerifier {
    pub command: String,
}

#[async_trait]
impl Verifier for ComputationalVerifier {
    async fn verify(&self, _final_output: &str, _original_prompt: &str) -> Result<(), String> {
        let output = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(&self.command)
            .output()
            .await
            .map_err(|e| format!("Failed to execute computational verifier: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Computational verification failed: {}", stderr));
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct VisualVerifier {
    pub playwright_script_path: String,
}

#[async_trait]
impl Verifier for VisualVerifier {
    async fn verify(&self, _final_output: &str, _original_prompt: &str) -> Result<(), String> {
        let output = tokio::process::Command::new("node")
            .arg(&self.playwright_script_path)
            .output()
            .await
            .map_err(|e| format!("Failed to execute visual verifier: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Visual verification failed (screenshot mismatch/error): {}", stderr));
        }
        Ok(())
    }
}

pub struct InferentialVerifier {
    pub llm: Arc<dyn LlmClient>,
    pub model: String,
}

impl std::fmt::Debug for InferentialVerifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InferentialVerifier")
         .field("model", &self.model)
         .finish()
    }
}

#[async_trait]
impl Verifier for InferentialVerifier {
    async fn verify(&self, final_output: &str, original_prompt: &str) -> Result<(), String> {
        let system_prompt = "You are a senior principal quality assurance AI. Your task is to evaluate the provided final output against the original user request. You must determine if the final output completely and correctly satisfies the request. If it does, respond exactly with 'PASS'. If it does not, respond with 'FAIL:' followed by a detailed explanation of what is wrong and what needs to be fixed. Be extremely strict.";
        let user_prompt = format!("Original Request: {}\n\nFinal Output: {}", original_prompt, final_output);

        let req = ChatRequest {
            model: self.model.clone(),
            system: system_prompt.to_string(),
            messages: vec![Message::user(user_prompt)],
            tools: vec![],
            max_tokens: 1024,
            temperature: 0.0,
        };

        let resp = self.llm.chat(req).await.map_err(|e| format!("LLM evaluation failed: {:?}", e))?;
        let content = resp.message.content.trim();

        if content.starts_with("PASS") {
            Ok(())
        } else {
            Err(format!("Inferential verification failed: {}", content))
        }
    }
}

#[derive(Debug, Default)]
pub struct VerificationLoops {
    pub verifiers: Vec<Box<dyn Verifier>>,
}

impl VerificationLoops {
    pub async fn run_all(&self, final_output: &str, original_prompt: &str) -> Result<(), String> {
        for verifier in &self.verifiers {
            verifier.verify(final_output, original_prompt).await?;
        }
        Ok(())
    }
}
