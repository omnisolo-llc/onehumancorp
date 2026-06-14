use std::sync::Arc;
use tokio::time::{sleep, Duration};
use crate::db::get_pool;
use uuid::Uuid;

pub async fn run_loyalty_worker() {
    let pool = get_pool();
    loop {
        if let Err(e) = check_loyalty_milestones_and_churn(&pool).await {
            tracing::error!("Error in loyalty worker: {}", e);
        }
        sleep(Duration::from_secs(3600)).await; // Run hourly
    }
}

async fn check_loyalty_milestones_and_churn(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Identify churn risk: top spenders (credits > 100) who haven't purchased in 60 days
    let churn_risks: Vec<(uuid::Uuid, String, i64)> = sqlx::query_as(
        "SELECT customer_id, tenant_id, credits
         FROM loyalty_profiles
         WHERE credits >= 100
           AND last_purchase_date < CURRENT_TIMESTAMP - INTERVAL '60 days'
           AND NOT EXISTS (
               SELECT 1 FROM department_tasks
               WHERE department = 'customer_success'
                 AND event_type = 'LoyaltyChurnRisk'
                 AND payload->>'customer_id' = loyalty_profiles.customer_id::text
                 AND created_at > CURRENT_TIMESTAMP - INTERVAL '30 days'
           )"
    )
    .fetch_all(pool)
    .await?;

    for (customer_id, tenant_id, credits) in churn_risks {
        let task_id = Uuid::new_v4().to_string();
        let payload = serde_json::json!({
            "customer_id": customer_id,
            "credits": credits,
            "message": format!("VIP Customer {} hasn't visited in 60 days. Approve 15% win-back offer?", customer_id)
        });

        sqlx::query(
            "INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status)
             VALUES ($1, $2, 'customer_success', 'LoyaltyChurnRisk', $3::jsonb, 'PENDING')"
        )
        .bind(task_id)
        .bind(&tenant_id)
        .bind(payload)
        .execute(pool)
        .await?;
    }

    Ok(())
}
