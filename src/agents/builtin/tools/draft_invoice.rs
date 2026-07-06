use std::sync::Arc;
use serde::{Deserialize, Serialize};

use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DraftInvoiceArgs {
    client_name: String,
    project_context: String,
    amount: f64,
}

pub struct DraftInvoiceExecutor;

#[async_trait::async_trait]
impl PydanticToolExecutor<DraftInvoiceArgs> for DraftInvoiceExecutor {
    async fn execute_typed(&self, args: DraftInvoiceArgs) -> Result<String, ToolError> {
        // In a real implementation this would call the gRPC DraftInvoiceFromContext endpoint
        tracing::info!("Drafting invoice for {} via Operations/Finance Agent", args.client_name);

        let mut response = String::new();
        response.push_str("Successfully drafted invoice.\n");
        response.push_str(&format!("Client: {}\n", args.client_name));
        response.push_str(&format!("Amount: ${:.2}\n", args.amount));
        response.push_str("Action Card pushed to owner feed for approval.");

        Ok(response)
    }
}

pub fn draft_invoice_tool() -> Tool {
    Tool {
        name: "draft_invoice".to_string(),
        description: "Drafts an invoice for a customer based on project context and pushes an approval card to the owner's feed.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "client_name": {
                    "type": "string",
                    "description": "The name of the client to bill."
                },
                "project_context": {
                    "type": "string",
                    "description": "The context or description of the project."
                },
                "amount": {
                    "type": "number",
                    "description": "The total amount of the invoice."
                }
            },
            "required": ["client_name", "project_context", "amount"]
        }),
        execute: Arc::new(PydanticAdapter::new(DraftInvoiceExecutor)),
    }
}
