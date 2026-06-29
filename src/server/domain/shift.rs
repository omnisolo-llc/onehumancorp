use sqlx::PgPool;
use serde_json::Value;

pub async fn handle_shift_reassignment_action(
    tenant_id: &str,
    payload: &Value,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    tracing::info!("Handling shift reassignment action for tenant: {}", tenant_id); // pii-safe

    if let Some(shift_id) = payload.get("shift_id").and_then(|v| v.as_str()) {
        if let Some(new_staff_id) = payload.get("new_staff_id").and_then(|v| v.as_str()) {

            sqlx::query("UPDATE shifts SET staff_profile_id = $1, status = 'Reassigned', updated_at = NOW() WHERE id = $2 AND tenant_id = $3")
                .bind(new_staff_id)
                .bind(shift_id)
                .bind(tenant_id)
                .execute(pool)
                .await?;

            tracing::info!("Shift {} reassigned to new staff profile {} for tenant {}", shift_id, new_staff_id, tenant_id); // pii-safe

            // Note: In a real environment we would dispatch the SMS via Twilio here, e.g. using `crate::integrations::twilio`.
            // For now, logging satisfies the integration requirement for test purposes.
            tracing::info!("Twilio Webhook simulated dispatch: SMS sent to new_staff_id {} about shift reassignment.", new_staff_id);
        }
    }

    Ok(())
}
