use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use std::sync::Arc;
use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize)]
pub struct QrGenerateArgs {
    pub content: String,
    pub label: Option<String>,
}

pub struct QrGenerateExecutor;

#[async_trait::async_trait]
impl PydanticToolExecutor<QrGenerateArgs> for QrGenerateExecutor {
    async fn execute_typed(
        &self,
        args: QrGenerateArgs,
    ) -> Result<String, ToolError> {
        let content = args.content;
        let label = args.label.unwrap_or_else(|| "QR Code".to_string());

        // Functional QR generation using qrcode crate.
        info!("Generating QR code for content: {} with label: {}", content, label);

        use qrcode::QrCode;

        let code = QrCode::new(content.as_bytes())
            .map_err(|e| ToolError::LlmRecoverable(format!("failed to generate QR code: {}", e)))?;

        // Render the bits into a string.
        let image_str = code.render::<char>()
            .quiet_zone(false)
            .module_dimensions(1, 1)
            .build();

        let message = format!("QR code for '{}' has been generated.", content);

        Ok(json!({
            "status": "success",
            "message": message,
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
        execute: Arc::new(PydanticAdapter::new(QrGenerateExecutor)),
    }
}
