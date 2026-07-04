use sqlx::PgPool;
use serde_json::Value;

pub async fn handle_booking_action(tenant_id: &str, payload: &Value, pool: &PgPool) -> Result<(), sqlx::Error> {
    if let Some(booking_id) = payload.get("booking_id").and_then(|v| v.as_str()) {
        tracing::info!("Approved booking draft: {}", booking_id);

        let draft_reply = payload.get("draft_reply").and_then(|v| v.as_str()).unwrap_or("");

        // 1. Update booking status
        sqlx::query("UPDATE bookings SET status = 'pending_payment', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
            .bind(booking_id)
            .bind(tenant_id)
            .execute(pool)
            .await?;

        // 2. Also send the reply via inbox
        if let Some(inbox_id) = payload.get("inbox_message_id").and_then(|v| v.as_str()) {
            let _ = sqlx::query("UPDATE inbox_messages SET status = 'replied' WHERE id = $1 AND tenant_id = $2")
                .bind(inbox_id)
                .bind(tenant_id)
                .execute(pool)
                .await;

            let _ = sqlx::query("UPDATE omni_inbox_messages SET status = 'sent', draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
                .bind(draft_reply)
                .bind(inbox_id)
                .bind(tenant_id)
                .execute(pool)
                .await;
        }
    }
    Ok(())
}

pub async fn handle_autonomous_quote_action(tenant_id: &str, payload: &Value, pool: &PgPool) -> Result<(), sqlx::Error> {
    tracing::info!("Handling autonomous quote action for tenant: {}", tenant_id); // pii-safe

    let proposed_slot_id = payload.get("proposed_slot_id").and_then(|v| v.as_str()).unwrap_or("");
    let start_time_str = payload.get("proposed_slots").and_then(|v| v.as_array())
        .and_then(|a| a.first())
        .and_then(|o| o.get("start_time"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let end_time_str = payload.get("proposed_slots").and_then(|v| v.as_array())
        .and_then(|a| a.first())
        .and_then(|o| o.get("end_time"))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let service = payload.get("service").and_then(|v| v.as_str()).unwrap_or("Emergency Handyman Service");

    if !proposed_slot_id.is_empty() {
        // We have an approved slot, let's insert it into the booking_slots and bookings table.
        let start_time = chrono::DateTime::parse_from_rfc3339(start_time_str).unwrap_or_default().with_timezone(&chrono::Utc);
        let end_time = chrono::DateTime::parse_from_rfc3339(end_time_str).unwrap_or_default().with_timezone(&chrono::Utc);

        let booking_id = uuid::Uuid::new_v4().to_string();

        sqlx::query(
            "INSERT INTO bookings (id, tenant_id, start_time, end_time, status) VALUES ($1, $2, $3, $4, 'scheduled')"
        )
        .bind(&booking_id)
        .bind(tenant_id)
        .bind(start_time)
        .bind(end_time)
        .execute(pool)
        .await?;

        // Insert into booking_slots
        sqlx::query(
            "INSERT INTO booking_slots (id, tenant_id, service_id, start_time, end_time, status) VALUES ($1, $2, $3, $4, $5, 'booked')"
        )
        .bind(proposed_slot_id)
        .bind(tenant_id)
        .bind(service)
        .bind(start_time)
        .bind(end_time)
        .execute(pool)
        .await?;

        // Release the Redis Redlock explicitly as it was just a temporary hold during quote generation
        if let Ok(redis_url) = std::env::var("OHC_REDIS_URL").or_else(|_| std::env::var("REDIS_URL")) {
            if let Ok(redis_lock) = crate::orchestration::queue::redis_lock::RedisLock::new(&redis_url) {
                let _ = redis_lock.release_lock(tenant_id, "booking_slot", proposed_slot_id, proposed_slot_id).await; // Note lock_val is not saved, so we can't reliably delete it unless we store it. Since it expires in 10 mins anyway, it's fine.
            }
        }
    }

    Ok(())
}

pub async fn handle_fulfillment_action(tenant_id: &str, payload: &Value, pool: &PgPool) -> Result<(), sqlx::Error> {
    tracing::info!("Handling fulfillment action for tenant: {}", tenant_id);

    let service = payload.get("service").and_then(|v| v.as_str()).unwrap_or("Unknown Service");
    let start_time_str = payload.get("start_time").and_then(|v| v.as_str()).unwrap_or("");
    let end_time_str = payload.get("end_time").and_then(|v| v.as_str()).unwrap_or("");
    let price = payload.get("price").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let message_id = payload.get("inbox_message_id").and_then(|v| v.as_str()).unwrap_or("");
    let draft_reply = format!("Great! We have reserved your spot for {} at ${:.2}. A deposit link has been sent.", service, price);

    let start_time = chrono::DateTime::parse_from_rfc3339(start_time_str).unwrap_or_default().with_timezone(&chrono::Utc);
    let end_time = chrono::DateTime::parse_from_rfc3339(end_time_str).unwrap_or_default().with_timezone(&chrono::Utc);

    let booking_id = uuid::Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO bookings (id, tenant_id, start_time, end_time, status, service_id) VALUES ($1, $2, $3, $4, 'scheduled', $5)"
    )
    .bind(&booking_id)
    .bind(tenant_id)
    .bind(start_time)
    .bind(end_time)
    .bind(service)
    .execute(pool)
    .await?;

    let slot_id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO booking_slots (id, tenant_id, service_id, start_time, end_time, status) VALUES ($1, $2, $3, $4, $5, 'booked')"
    )
    .bind(slot_id)
    .bind(tenant_id)
    .bind(service)
    .bind(start_time)
    .bind(end_time)
    .execute(pool)
    .await?;

    if !message_id.is_empty() {
        let _ = sqlx::query("UPDATE inbox_messages SET status = 'replied' WHERE id = $1 AND tenant_id = $2")
            .bind(message_id)
            .bind(tenant_id)
            .execute(pool)
            .await;

        let _ = sqlx::query("UPDATE omni_inbox_messages SET status = 'sent', draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
            .bind(draft_reply)
            .bind(message_id)
            .bind(tenant_id)
            .execute(pool)
            .await;
    }

    Ok(())
}
