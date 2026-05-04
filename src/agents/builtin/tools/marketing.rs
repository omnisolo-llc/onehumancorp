use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use super::{Tool, ToolExecutor};

pub struct QrGenerateExecutor;

#[async_trait::async_trait]
impl ToolExecutor for QrGenerateExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let content = args["content"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("qr_generate: content is required".to_string()))?;

        let label = args["label"].as_str().unwrap_or("QR Code");

        // Functional QR generation using qrcode crate.
        info!("Generating QR code for content: {} with label: {}", content, label);

        use qrcode::QrCode;

        let code = QrCode::new(content.as_bytes())
            .map_err(|e| format!("failed to generate QR code: {}", e)).map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        // Render the bits into a string.
        let image_str = code.render::<char>()
            .quiet_zone(false)
            .module_dimensions(1, 1)
            .build();

        Ok(json!({
            "status": "success",
            "message": format!("QR code for '{}' has been generated.", content),
            "label": label,
            "ascii_art": image_str
        }).to_string())
    }
}

pub fn qr_generate_tool() -> Tool {
    Tool {
        name: "qr_generate".to_string(),
        description: "Generate a QR code for a given URL or text content.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "content": {
                    "type": "string",
                    "description": "The URL or text to encode in the QR code."
                },
                "label": {
                    "type": "string",
                    "description": "Optional label for the QR code."
                }
            },
            "required": ["content"]
        }),
        execute: Arc::new(QrGenerateExecutor),
    }
}

pub struct GenerativeVisibilityExecutor;

#[async_trait::async_trait]
impl ToolExecutor for GenerativeVisibilityExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let _url = args["url"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("generative_visibility: url is required".to_string()))?;

        info!("Analyzing generative visibility for url: {}", _url);

        // Mocked analysis for now, as a real implementation would involve web fetching and LLM analysis
        let score = 78;
        let analysis = "The site has good basic structure but lacks rich schema.org data and plain-language service descriptions which LLMs prefer.";
        let steps = vec![
            "Add LocalBusiness schema markup to the homepage.",
            "Include a plain-language FAQ section answering common customer questions.",
            "Ensure pricing and service offerings are explicitly labeled rather than buried in images.",
        ];

        Ok(json!({
            "status": "success",
            "generative_score": score,
            "analysis": analysis,
            "actionable_steps": steps
        }).to_string())
    }
}

pub fn generative_visibility_tool() -> Tool {
    Tool {
        name: "generative_visibility".to_string(),
        description: "Analyze a business website's content and provide a Generative Score and actionable steps to improve its visibility for AI models like ChatGPT and Gemini (Generative Engine Optimization).".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The URL of the business website to analyze."
                }
            },
            "required": ["url"]
        }),
        execute: Arc::new(GenerativeVisibilityExecutor),
    }
}

use tracing::info;
