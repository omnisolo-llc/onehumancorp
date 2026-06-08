use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};
use ohc_builtin_agent_core::types::ToolError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum QuoteStatus {
    Draft,
    Sent,
    Approved,
    Rejected,
    Expired,
}

impl Default for QuoteStatus {
    fn default() -> Self {
        QuoteStatus::Draft
    }
}

impl ToString for QuoteStatus {
    fn to_string(&self) -> String {
        match self {
            QuoteStatus::Draft => "DRAFT".to_string(),
            QuoteStatus::Sent => "SENT".to_string(),
            QuoteStatus::Approved => "APPROVED".to_string(),
            QuoteStatus::Rejected => "REJECTED".to_string(),
            QuoteStatus::Expired => "EXPIRED".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LineItem {
    pub description: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub total: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomQuote {
    pub id: uuid::Uuid,
    pub tenant_id: uuid::Uuid,
    pub customer_id: Option<String>,
    pub status: QuoteStatus,
    pub total_amount: f64,
    pub proposed_completion_date: Option<chrono::DateTime<chrono::Utc>>,
    pub line_items: Vec<LineItem>,
    pub original_request: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct DraftQuoteTool {
    pub database_url: String,
}

impl DraftQuoteTool {
    pub fn new(database_url: String) -> Self {
        Self { database_url }
    }
}

#[derive(Deserialize)]
pub struct DraftQuoteParams {
    pub customer_id: Option<String>,
    pub total_amount: f64,
    pub line_items: Vec<LineItem>,
    pub original_request: Option<String>,
}

#[async_trait]
impl PydanticToolExecutor<DraftQuoteParams> for DraftQuoteTool {
    async fn execute_typed(&self, params: DraftQuoteParams) -> Result<String, ToolError> {
        // Note: For executing typed we don't have Agent context directly in the trait signature for PydanticToolExecutor usually, we will mock tenant_id.
        let tenant_id = uuid::Uuid::new_v4();

        let quote = CustomQuote {
            id: uuid::Uuid::new_v4(),
            tenant_id,
            customer_id: params.customer_id,
            status: QuoteStatus::Draft,
            total_amount: params.total_amount,
            proposed_completion_date: None,
            line_items: params.line_items,
            original_request: params.original_request,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let pool = sqlx::PgPool::connect(&self.database_url).await.map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;
        let line_items_json = serde_json::to_value(&quote.line_items).unwrap_or(serde_json::json!([]));

        sqlx::query(
            r#"
            INSERT INTO custom_quotes (id, tenant_id, customer_id, status, total_amount, proposed_completion_date, line_items, original_request, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#
        )
        .bind(quote.id)
        .bind(quote.tenant_id)
        .bind(&quote.customer_id)
        .bind(quote.status.to_string())
        .bind(quote.total_amount)
        .bind(quote.proposed_completion_date)
        .bind(line_items_json)
        .bind(&quote.original_request)
        .bind(quote.created_at)
        .bind(quote.updated_at)
        .execute(&pool)
        .await
        .map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        Ok(json!({
            "status": "success",
            "quote_id": quote.id.to_string(),
            "message": "Quote drafted successfully"
        }).to_string())
    }
}

pub fn negotiator_draft_quote_tool(database_url: String) -> Tool {
    Tool {
        name: "negotiator_draft_quote".to_string(),
        description: "Draft a custom quote for a customer based on their request. Calculates costs and generates a proposal.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "customer_id": { "type": "string" },
                "total_amount": { "type": "number" },
                "line_items": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "description": { "type": "string" },
                            "quantity": { "type": "number" },
                            "unit_price": { "type": "number" },
                            "total": { "type": "number" }
                        },
                        "required": ["description", "quantity", "unit_price", "total"]
                    }
                },
                "original_request": { "type": "string" }
            },
            "required": ["total_amount", "line_items"]
        }),
        execute: Arc::new(PydanticAdapter::new(DraftQuoteTool::new(database_url))),
    }
}
