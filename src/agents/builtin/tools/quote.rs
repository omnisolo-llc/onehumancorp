use super::{
    Tool,
    pydantic::{PydanticAdapter, PydanticToolExecutor},
};
use crate::{booking::SharedBookingStore, tenant::TenantContext};
use ohc_builtin_agent_core::types::ToolError;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct GenerateQuoteArgs {
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

pub struct GenerateQuoteExecutor {
    pub store: SharedBookingStore,
    pub tenant: TenantContext,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<GenerateQuoteArgs> for GenerateQuoteExecutor {
    async fn execute_typed(&self, args: GenerateQuoteArgs) -> Result<String, ToolError> {
        let tenant_id = self.tenant.as_str();
        let customer_id = args.customer_id;
        let total_amount_cents = args.total_amount_cents;
        let required_deposit_cents = args.required_deposit_cents;
        let checkout_url = args.checkout_url;

        let quote_id = Uuid::new_v4();

        let store = self.store.read().await;
        let pool = store.get_pool().await?;

        let mut tx = pool.begin().await.map_err(|e| {
            ToolError::LlmRecoverable(format!("Failed to begin transaction: {}", e))
        })?;

        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                ToolError::LlmRecoverable(format!("Failed to set tenant context: {}", e))
            })?;

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
                "INSERT INTO quote_line_items (id, quote_id, description, unit_price_cents, quantity, is_optional, created_at, updated_at, tenant_id) VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW(), $7)"
            )
            .bind(item_id)
            .bind(quote_id)
            .bind(&item.description)
            .bind(item.unit_price_cents)
            .bind(item.quantity)
            .bind(item.is_optional)
            .bind(tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| ToolError::LlmRecoverable(format!("DB insert quote_line_item failed: {}", e)))?;
        }

        tx.commit().await.map_err(|e| {
            ToolError::LlmRecoverable(format!("Failed to commit transaction: {}", e))
        })?;

        Ok(json!({
            "status": "success",
            "message": "Quote generated successfully.",
            "quote_id": quote_id.to_string()
        })
        .to_string())
    }
}

pub fn generate_quote_tool(store: SharedBookingStore, tenant: TenantContext) -> Tool {
    Tool {
        name: "generate_quote".to_string(),
        description: "Generate a structured quote for a customer inquiry.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
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
            "required": ["customer_id", "total_amount_cents", "required_deposit_cents", "line_items"]
        }),
        execute: Arc::new(PydanticAdapter::new(GenerateQuoteExecutor {
            store,
            tenant,
        })),
    }
}
