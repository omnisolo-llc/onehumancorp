use sqlx::PgPool;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct QuoteRequest {
    pub customer_id: Uuid,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuoteResponse {
    pub quote_id: Uuid,
    pub status: String,
    pub amount: Option<f64>,
}

pub async fn handle_quote_request(
    pool: &PgPool,
    tenant_id: Uuid,
    req: QuoteRequest,
) -> Result<QuoteResponse, Box<dyn std::error::Error + Send + Sync>> {
    // 1. Check if conversation exists or create new
    let conversation_id = Uuid::new_v4(); // Mocking for now, would normally look up or create

    // 2. Mock Agentic analysis of the message to determine service & price
    let proposed_price = 150.00;

    // 3. Draft a quote
    let quote_id = Uuid::new_v4();

    sqlx::query(
        r#"
        INSERT INTO quote_drafts (id, tenant_id, conversation_id, proposed_price, status)
        VALUES ($1, $2, $3, $4, 'draft')
        "#
    )
    .bind(quote_id)
    .bind(tenant_id)
    .bind(conversation_id)
    .bind(proposed_price)
    .execute(pool)
    .await?;

    Ok(QuoteResponse {
        quote_id,
        status: "draft".to_string(),
        amount: Some(proposed_price),
    })
}

pub async fn approve_quote(
    pool: &PgPool,
    tenant_id: Uuid,
    quote_id: Uuid,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    sqlx::query(
        r#"
        UPDATE quote_drafts
        SET status = 'approved', updated_at = CURRENT_TIMESTAMP
        WHERE id = $1 AND tenant_id = $2
        "#
    )
    .bind(quote_id)
    .bind(tenant_id)
    .execute(pool)
    .await?;

    Ok(())
}
