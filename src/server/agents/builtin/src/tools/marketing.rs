use serde_json::{json, Value};
use std::sync::Arc;
use super::{Tool, ToolExecutor};

pub struct QrGenerateExecutor;

#[async_trait::async_trait]
impl ToolExecutor for QrGenerateExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let content = args["content"]
            .as_str()
            .ok_or("qr_generate: content is required")?;

        let label = args["label"].as_str().unwrap_or("QR Code");

        // Functional QR generation using qrcode crate.
        info!("Generating QR code for content: {} with label: {}", content, label);

        use qrcode::QrCode;

        let code = QrCode::new(content.as_bytes())
            .map_err(|e| format!("failed to generate QR code: {}", e))?;

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
        is_mutating: true,
        name: "qr_generate".to_string(),
        description: "Generate a QR code for a given URL or text content.".to_string(),
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

use tracing::info;
