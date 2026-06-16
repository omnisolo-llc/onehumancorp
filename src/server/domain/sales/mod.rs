
use serde_json::Value;
use sqlx::PgPool;

pub async fn handle_quote_draft(tenant_id: String, payload: Value, pool: PgPool) -> Result<(), String> {
    if let Some(quote_id) = payload.get("quote_id").and_then(|v| v.as_str()) {
        tracing::info!("Approved quote draft: {}", quote_id);
        let id = uuid::Uuid::parse_str(quote_id).map_err(|e| e.to_string())?;
        sqlx::query("UPDATE quotes SET status = 'SENT', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
            .bind(id)
            .bind(&tenant_id)
            .execute(&pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Missing quote_id".to_string())
    }
}
