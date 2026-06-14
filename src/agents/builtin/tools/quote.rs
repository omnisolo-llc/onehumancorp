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
    pub line_items: Vec<QuoteLineItemArgs>,
}

#[derive(Deserialize)]
pub struct QuoteLineItemArgs {
    pub description: String,
    pub unit_price_cents: i64,
    pub quantity: i32,
    pub is_optional: bool,
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

        let quote_id = Uuid::new_v4();

        let db_url = std::env::var("OHC_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());

        let pool = sqlx::PgPool::connect(&db_url).await
            .map_err(|e| ToolError::LlmRecoverable(format!("DB connection failed: {}", e)))?;

        let mut tx = pool.begin().await
            .map_err(|e| ToolError::LlmRecoverable(format!("Failed to begin transaction: {}", e)))?;

        sqlx::query(
            "INSERT INTO quotes (id, tenant_id, customer_id, status, total_amount, required_deposit, checkout_url, created_at, updated_at) VALUES ($1, $2, $3, 'DRAFT', $4, $5, $6, NOW(), NOW())"
        )
        .bind(quote_id)
        .bind(&tenant_id)
        .bind(match Uuid::parse_str(&customer_id) {
            Ok(u) => u,
            Err(_) => return Err(ToolError::LlmRecoverable("Invalid customer_id format".to_string()))
        })
        .bind(total_amount_cents)
        .bind(required_deposit_cents)
        .bind(&checkout_url)
        .execute(&mut *tx)
        .await
        .map_err(|e| ToolError::LlmRecoverable(format!("DB insert quote failed: {}", e)))?;

        for item in args.line_items {
            let item_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO quote_line_items (id, quote_id, description, unit_price_cents, quantity, is_optional, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())"
            )
            .bind(item_id)
            .bind(quote_id)
            .bind(&item.description)
            .bind(item.unit_price_cents)
            .bind(item.quantity)
            .bind(item.is_optional)
            .execute(&mut *tx)
            .await
            .map_err(|e| ToolError::LlmRecoverable(format!("DB insert quote_line_item failed: {}", e)))?;
        }

        tx.commit().await
            .map_err(|e| ToolError::LlmRecoverable(format!("Failed to commit transaction: {}", e)))?;

        Ok(json!({
            "status": "success",
            "message": "Quote generated successfully.",
            "quote_id": quote_id.to_string()
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
                "customer_id": { "type": "string", "description": "UUID of the customer" },
                "total_amount_cents": { "type": "integer" },
                "required_deposit_cents": { "type": "integer" },
                "checkout_url": { "type": "string" },
                "line_items": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "description": { "type": "string" },
                            "unit_price_cents": { "type": "integer" },
                            "quantity": { "type": "integer" },
                            "is_optional": { "type": "boolean" }
                        },
                        "required": ["description", "unit_price_cents", "quantity", "is_optional"]
                    }
                }
            },
            "required": ["tenant_id", "customer_id", "total_amount_cents", "required_deposit_cents", "line_items"]
        }),
        execute: Arc::new(PydanticAdapter::new(GenerateQuoteExecutor)),
    }
}


#[derive(Deserialize)]
pub struct DraftEstimateArgs {
    pub tenant_id: String,
    pub quote_request_id: String,
    pub customer_id: String,
    pub total_amount_cents: i64,
    pub required_deposit_cents: i64,
    pub line_items: Vec<QuoteLineItemArgs>,
}

pub struct DraftEstimateExecutor;

#[async_trait::async_trait]
impl PydanticToolExecutor<DraftEstimateArgs> for DraftEstimateExecutor {
    async fn execute_typed(&self, args: DraftEstimateArgs) -> Result<String, ToolError> {
        let tenant_id = args.tenant_id;
        let customer_id = args.customer_id;
        let quote_request_id = args.quote_request_id;
        let total_amount_cents = args.total_amount_cents;
        let required_deposit_cents = args.required_deposit_cents;

        let estimate_id = Uuid::new_v4();

        let db_url = std::env::var("OHC_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());

        let pool = sqlx::PgPool::connect(&db_url).await
            .map_err(|e| ToolError::LlmRecoverable(format!("DB connection failed: {}", e)))?;

        let mut tx = pool.begin().await
            .map_err(|e| ToolError::LlmRecoverable(format!("Failed to begin transaction: {}", e)))?;

        // 1. Create Estimate
        sqlx::query(
            "INSERT INTO estimates (id, tenant_id, quote_request_id, customer_id, status, total_amount_cents, required_deposit_cents, created_at, updated_at) VALUES ($1, $2, $3, $4, 'DRAFT', $5, $6, NOW(), NOW())"
        )
        .bind(estimate_id.to_string())
        .bind(&tenant_id)
        .bind(&quote_request_id)
        .bind(&customer_id)
        .bind(total_amount_cents)
        .bind(required_deposit_cents)
        .execute(&mut *tx)
        .await
        .map_err(|e| ToolError::LlmRecoverable(format!("DB insert estimate failed: {}", e)))?;

        // 2. Insert Line Items
        for item in args.line_items {
            let item_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO estimate_line_items (id, estimate_id, description, unit_price_cents, quantity, is_optional, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())"
            )
            .bind(item_id.to_string())
            .bind(estimate_id.to_string())
            .bind(&item.description)
            .bind(item.unit_price_cents)
            .bind(item.quantity)
            .bind(item.is_optional)
            .execute(&mut *tx)
            .await
            .map_err(|e| ToolError::LlmRecoverable(format!("DB insert estimate_line_item failed: {}", e)))?;
        }

        // 3. Mark QuoteRequest as PROCESSED
        sqlx::query(
            "UPDATE quote_requests SET status = 'PROCESSED', updated_at = NOW() WHERE id = $1 AND tenant_id = $2"
        )
        .bind(&quote_request_id)
        .bind(&tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| ToolError::LlmRecoverable(format!("DB update quote_request failed: {}", e)))?;

        tx.commit().await
            .map_err(|e| ToolError::LlmRecoverable(format!("Failed to commit transaction: {}", e)))?;

        Ok(json!({
            "status": "success",
            "message": "Estimate drafted successfully.",
            "estimate_id": estimate_id.to_string()
        }).to_string())
    }
}

pub fn draft_estimate_tool() -> Tool {
    Tool {
        name: "draft_estimate".to_string(),
        description: "Draft a service estimate based on a customer quote request containing text and images.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "tenant_id": { "type": "string" },
                "quote_request_id": { "type": "string", "description": "ID of the QuoteRequest being fulfilled" },
                "customer_id": { "type": "string", "description": "ID of the customer" },
                "total_amount_cents": { "type": "integer" },
                "required_deposit_cents": { "type": "integer" },
                "line_items": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "description": { "type": "string" },
                            "unit_price_cents": { "type": "integer" },
                            "quantity": { "type": "integer" },
                            "is_optional": { "type": "boolean" }
                        },
                        "required": ["description", "unit_price_cents", "quantity", "is_optional"]
                    }
                }
            },
            "required": ["tenant_id", "quote_request_id", "customer_id", "total_amount_cents", "required_deposit_cents", "line_items"]
        }),
        execute: Arc::new(PydanticAdapter::new(DraftEstimateExecutor)),
    }
}
