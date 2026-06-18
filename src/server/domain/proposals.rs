use sqlx::{PgPool, Row};
use serde_json::Value;

pub async fn handle_proposal_action(tenant_id: &str, payload: &Value, pool: &PgPool) -> Result<(), sqlx::Error> {
    if let Some(proposal_id) = payload.get("proposal_id").and_then(|v| v.as_str()) {
        tracing::info!("Approved proposal draft: {}", proposal_id);

        let mut tx = pool.begin().await?;

        // 1. Mark proposal as sent
        sqlx::query("UPDATE proposals SET status = 'SENT', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
            .bind(proposal_id) // using TEXT as per 138_proposals.sql
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        // 2. Mock generating a stripe deposit link if required
        // Fetch proposal details to see if deposit required
        let row = sqlx::query("SELECT required_deposit_cents FROM proposals WHERE id = $1 AND tenant_id = $2")
            .bind(proposal_id)
            .bind(tenant_id)
            .fetch_optional(&mut *tx)
            .await?;

        if let Some(r) = row {
            let deposit: Option<i64> = r.try_get("required_deposit_cents").ok();
            if let Some(deposit) = deposit {
                if deposit > 0 {
                    let mock_link = format!("https://checkout.stripe.com/c/pay/cs_test_{}", uuid::Uuid::new_v4());
                    sqlx::query("UPDATE proposals SET checkout_url = $1 WHERE id = $2 AND tenant_id = $3")
                        .bind(mock_link)
                        .bind(proposal_id)
                        .bind(tenant_id)
                        .execute(&mut *tx)
                        .await?;
                }
            }
        }

        tx.commit().await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_handle_proposal_action_no_id() {
        // Without proposal_id, it should early return Ok(()) without hitting DB
        let pool = PgPool::connect_lazy("postgres://dummy:dummy@localhost/dummy").unwrap();
        let payload = json!({});
        let res = handle_proposal_action("tenant_1", &payload, &pool).await;
        assert!(res.is_ok());
    }
}
