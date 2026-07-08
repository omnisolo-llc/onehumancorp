use sqlx::PgPool;
use tracing::info;
use serde_json::Value;

pub async fn schedule_feedback_request(
    tenant_id: &str,
    customer_id: &str,
    reference_id: &str,
    reference_type: &str,
    pool: &PgPool
) -> Result<(), sqlx::Error> {
    info!("Evaluating feedback schedule for tenant: {}, ref: {}", tenant_id, reference_id);

    // Get active reputation campaign for this trigger
    let campaign: Option<(String, i32)> = sqlx::query_as(
        r#"
        SELECT id, extract(epoch from delay_interval)::int
        FROM reputation_campaigns
        WHERE tenant_id = $1 AND is_active = TRUE
        LIMIT 1
        "#
    )
    .bind(tenant_id)
    .fetch_optional(pool)
    .await?;

    if let Some((campaign_id, delay_seconds)) = campaign {
        let request_id = uuid::Uuid::new_v4().to_string();
        // Just use delay_seconds in the query directly to simplify timestamp logic for now
        let query = format!(
            r#"
            INSERT INTO feedback_requests
            (id, tenant_id, campaign_id, customer_id, reference_id, reference_type, status, scheduled_for)
            VALUES ($1, $2, $3, $4, $5, $6, 'scheduled', NOW() + interval '{} seconds')
            "#, delay_seconds
        );

        sqlx::query(&query)
        .bind(&request_id)
        .bind(tenant_id)
        .bind(&campaign_id)
        .bind(customer_id)
        .bind(reference_id)
        .bind(reference_type)
        .execute(pool)
        .await?;

        info!("Scheduled feedback request {} for tenant {}", request_id, tenant_id);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dummy_reputation() {
        assert_eq!(1, 1);
    }
}
