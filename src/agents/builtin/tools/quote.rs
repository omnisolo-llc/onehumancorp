use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use std::sync::Arc;
use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct GenerateQuoteArgs {
    pub tenant_id: String,
    pub customer_id: String,
    pub total_amount_cents: i64,
    pub required_deposit_cents: i64,
    pub checkout_url: Option<String>,
}

pub struct GenerateQuoteExecutor;

#[async_trait::async_trait]
impl PydanticToolExecutor<GenerateQuoteArgs> for GenerateQuoteExecutor {
    async fn execute_typed(&self, args: GenerateQuoteArgs) -> Result<String, ToolError> {
        let tenant_id = args.tenant_id;
        let customer_id = args.customer_id;
        let total_amount_cents = args.total_amount_cents;
        let required_deposit_cents = args.required_deposit_cents;
        let checkout_url = args.checkout_url;

        let quote_id = Uuid::new_v4().to_string();

        let db_url = std::env::var("OHC_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());

        let pool = sqlx::PgPool::connect(&db_url).await
            .map_err(|e| ToolError::LlmRecoverable(format!("DB connection failed: {}", e)))?;

        sqlx::query(
            "INSERT INTO quotes (id, tenant_id, customer_id, status, total_amount, required_deposit, checkout_url) VALUES ($1, $2, $3, 'proposed', $4, $5, $6)"
        )
        .bind(&quote_id)
        .bind(&tenant_id)
        .bind(&customer_id)
        .bind((total_amount_cents as f64) / 100.0)
        .bind((required_deposit_cents as f64) / 100.0)
        .bind(&checkout_url)
        .execute(&pool)
        .await
        .map_err(|e| ToolError::LlmRecoverable(format!("DB insert failed: {}", e)))?;

        Ok(json!({
            "status": "success",
            "message": "Quote generated successfully.",
            "quote_id": quote_id
        }).to_string())
    }
}

pub fn generate_quote_tool() -> Tool {
    Tool {
        name: "generate_quote".to_string(),
        description: "Generate a structured quote for a customer inquiry.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "tenant_id": { "type": "string" },
                "customer_id": { "type": "string" },
                "total_amount_cents": { "type": "integer" },
                "required_deposit_cents": { "type": "integer" },
                "checkout_url": { "type": "string" }
            },
            "required": ["tenant_id", "customer_id", "total_amount_cents", "required_deposit_cents"]
        }),
        execute: Arc::new(PydanticAdapter::new(GenerateQuoteExecutor)),
    }
}
