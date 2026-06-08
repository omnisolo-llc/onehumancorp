use crate::domain::custom_quotes::{CustomQuote, QuoteStatus, LineItem};
use uuid::Uuid;
use sqlx::{PgPool, postgres::PgRow, Row};
use std::sync::Arc;

#[derive(Clone)]
pub struct CustomQuotesRepository {
    pool: Arc<PgPool>,
}

impl CustomQuotesRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create_quote(&self, quote: &CustomQuote) -> Result<(), sqlx::Error> {
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
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_quote(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<CustomQuote>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT id, tenant_id, customer_id, status, total_amount, proposed_completion_date, line_items, original_request, created_at, updated_at
            FROM custom_quotes
            WHERE id = $1 AND tenant_id = $2
            "#
        )
        .bind(id)
        .bind(tenant_id)
        .fetch_optional(&*self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(self.map_row(row)?))
        } else {
            Ok(None)
        }
    }

    pub async fn update_status(&self, tenant_id: Uuid, id: Uuid, status: QuoteStatus) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE custom_quotes
            SET status = $1, updated_at = CURRENT_TIMESTAMP
            WHERE id = $2 AND tenant_id = $3
            "#
        )
        .bind(status.to_string())
        .bind(id)
        .bind(tenant_id)
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    fn map_row(&self, row: PgRow) -> Result<CustomQuote, sqlx::Error> {
        let line_items_json: serde_json::Value = row.try_get("line_items")?;
        let line_items: Vec<LineItem> = serde_json::from_value(line_items_json).unwrap_or_default();
        let status_str: String = row.try_get("status")?;
        let status = status_str.parse().unwrap_or(QuoteStatus::Draft);

        Ok(CustomQuote {
            id: row.try_get("id")?,
            tenant_id: row.try_get("tenant_id")?,
            customer_id: row.try_get("customer_id")?,
            status,
            total_amount: row.try_get("total_amount")?,
            proposed_completion_date: row.try_get("proposed_completion_date")?,
            line_items,
            original_request: row.try_get("original_request")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}
