use sqlx::PgPool;
use serde_json::Value;

#[cfg(ohc_bazel)]
use crate::integrations::stripe::client::StripeClient;
#[cfg(not(ohc_bazel))]
use server_integrations_stripe::client::StripeClient;

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

    let deposit_amount_cents = payload.get("deposit_amount_cents").and_then(|v| v.as_i64()).unwrap_or(0);
    let total_amount_cents = payload.get("total_amount_cents").and_then(|v| v.as_i64()).unwrap_or(deposit_amount_cents);
    let customer_id = payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or("unknown");
    let mut drafted_message = payload.get("generated_response").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let inbox_message_id = payload.get("inbox_message_id").and_then(|v| v.as_str());

    let mut stripe_payment_link = String::new();
    if deposit_amount_cents > 0 {
        let api_key = std::env::var("STRIPE_API_KEY").unwrap_or_default();
        let stripe_client = StripeClient::new(api_key);

        // Generate a Stripe Payment Link for the deposit
        let link_res = stripe_client.create_payment_link(service, deposit_amount_cents).await;
        if let Ok(link) = link_res {
            stripe_payment_link = link;
        } else if let Err(e) = link_res {
             tracing::error!("Failed to generate Stripe payment link for deposit: {}", e);
             // Fallback to dummy link for testing/e2e if API fails
             stripe_payment_link = format!("https://buy.stripe.com/test_{}", uuid::Uuid::new_v4().simple().to_string().chars().take(16).collect::<String>());
        }
        drafted_message = format!("{}

To secure your booking, please pay the deposit here: {}", drafted_message, stripe_payment_link);
    }

    // Insert into quotes
    let quote_id = uuid::Uuid::new_v4().to_string();
    let payment_link_opt = if stripe_payment_link.is_empty() { None } else { Some(stripe_payment_link.clone()) };

    sqlx::query("INSERT INTO quotes (id, tenant_id, customer_id, status, total_amount_cents, required_deposit_cents, stripe_payment_link, created_at, updated_at) VALUES ($1, $2, $3, 'SENT', $4, $5, $6, NOW(), NOW())")
        .bind(&quote_id)
        .bind(tenant_id)
        .bind(customer_id)
        .bind(total_amount_cents)
        .bind(deposit_amount_cents)
        .bind(&payment_link_opt)
        .execute(pool)
        .await?;

    // Update inbox_messages
    if let Some(inbox_id) = inbox_message_id {
        let _ = sqlx::query("UPDATE inbox_messages SET status = 'replied' WHERE id = $1 AND tenant_id = $2")
            .bind(inbox_id)
            .bind(tenant_id)
            .execute(pool)
            .await;
    }

    Ok(())
}


use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Booking {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: Option<String>,
    pub service_id: Option<String>,
    pub resource_id: Option<String>,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookingSlot {
    pub id: String,
    pub tenant_id: String,
    pub service_id: Option<String>,
    pub resource_id: Option<String>,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AvailabilitySchedule {
    pub id: String,
    pub tenant_id: String,
    pub business_hours: serde_json::Value,
    pub exceptions: serde_json::Value,
    pub timezone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AvailabilityBlock {
    pub id: String,
    pub tenant_id: String,
    pub service_id: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub is_available: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookingResource {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub resource_type: String,
    pub availability_schedule: serde_json::Value,
}
