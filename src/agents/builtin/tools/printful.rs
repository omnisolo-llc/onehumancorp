use ohc_builtin_agent_core::types::ToolError;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};

// ── PrintfulGenerateMockup ───────────────────────────────────────────────────

#[derive(Deserialize)]
struct PrintfulGenerateMockupArgs {
    product_id: String,
    design_url: String,
}

struct PrintfulGenerateMockupExecutor;

#[async_trait::async_trait]
impl PydanticToolExecutor<PrintfulGenerateMockupArgs> for PrintfulGenerateMockupExecutor {
    async fn execute_typed(&self, args: PrintfulGenerateMockupArgs) -> Result<String, ToolError> {
        Ok(json!({
            "mockup_url": format!("https://api.printful.com/mockups/{}/mock_image.png", args.product_id),
            "design_url": args.design_url
        }).to_string())
    }
}

// ── PrintfulCreateOrder ──────────────────────────────────────────────────────

#[derive(Deserialize)]
struct PrintfulCreateOrderArgs {
    product_id: String,
    design_url: String,
    address: String,
}

struct PrintfulCreateOrderExecutor;

#[async_trait::async_trait]
impl PydanticToolExecutor<PrintfulCreateOrderArgs> for PrintfulCreateOrderExecutor {
    async fn execute_typed(&self, args: PrintfulCreateOrderArgs) -> Result<String, ToolError> {
        Ok(json!({
            "order_id": format!("mock_order_id_{}_{}", args.product_id, args.address.len()),
            "status": "pending",
        }).to_string())
    }
}

// ── Tool constructors ─────────────────────────────────────────────────────────

pub fn printful_generate_mockup_tool() -> Tool {
    Tool {
        name: "printful_generate_mockup".to_string(),
        description: "Generates a realistic mockup image for a print-on-demand product using a design file.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "product_id": {
                    "type": "string",
                    "description": "The Printful product ID (e.g., '71' for standard t-shirt)."
                },
                "design_url": {
                    "type": "string",
                    "description": "The URL of the design image to place on the product."
                }
            },
            "required": ["product_id", "design_url"]
        }),
        execute: Arc::new(PydanticAdapter::new(PrintfulGenerateMockupExecutor)),
    }
}

pub fn printful_create_order_tool() -> Tool {
    Tool {
        name: "printful_create_order".to_string(),
        description: "Submits a fulfillment order to Printful for a custom dropshipped product.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "product_id": {
                    "type": "string",
                    "description": "The Printful product ID to fulfill."
                },
                "design_url": {
                    "type": "string",
                    "description": "The URL of the design image for the product."
                },
                "address": {
                    "type": "string",
                    "description": "The full shipping address of the customer."
                }
            },
            "required": ["product_id", "design_url", "address"]
        }),
        execute: Arc::new(PydanticAdapter::new(PrintfulCreateOrderExecutor)),
    }
}
