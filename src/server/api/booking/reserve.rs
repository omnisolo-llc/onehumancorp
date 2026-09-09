use crate::db::DB;
use axum::{
    Router,
    extract::{Extension, Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::sync::Arc;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReserveRequest {
    pub customer_id: Option<String>,
    pub customer_name: Option<String>,
    pub customer_email: Option<String>,
    pub service_id: String,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Serialize)]
pub struct ReserveResponse {
    pub success: bool,
    pub booking_id: Option<String>,
    pub error: Option<String>,
    pub checkout_url: Option<String>,
}

fn reservation_window(
    start_time: &str,
    end_time: &str,
) -> Option<(chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)> {
    let start_time = chrono::DateTime::parse_from_rfc3339(start_time)
        .ok()?
        .with_timezone(&chrono::Utc);
    let end_time = chrono::DateTime::parse_from_rfc3339(end_time)
        .ok()?
        .with_timezone(&chrono::Utc);
    (end_time > start_time && end_time - start_time <= chrono::Duration::hours(24))
        .then_some((start_time, end_time))
}

fn valid_customer_name(value: &str) -> bool {
    let value = value.trim();
    !value.is_empty()
        && value.chars().count() <= 200
        && !value
            .chars()
            .any(|character| matches!(character, '\0' | '\r' | '\n'))
}

fn valid_customer_email(value: &str) -> bool {
    let value = value.trim();
    let Some((local, domain)) = value.split_once('@') else {
        return false;
    };
    !local.is_empty()
        && domain.contains('.')
        && !domain.starts_with('.')
        && !domain.ends_with('.')
        && value.len() <= 320
        && !value.chars().any(char::is_whitespace)
        && !value.contains('\0')
}

fn valid_customer_id(value: &str) -> bool {
    let value = value.trim();
    !value.is_empty()
        && value.len() <= 128
        && value
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.'))
}

fn required_deposit_cents(price_cents: i64, metadata: &serde_json::Value) -> Option<i64> {
    if metadata
        .get("requires_deposit")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        return None;
    }
    let amount = metadata
        .get("deposit_amount_cents")
        .and_then(serde_json::Value::as_i64)
        .unwrap_or(price_cents);
    (amount > 0).then_some(amount.min(price_cents))
}

fn booking_feed_payloads(
    booking_id: &str,
    service_id: &str,
    start_time: &str,
    end_time: &str,
) -> (serde_json::Value, serde_json::Value) {
    (
        serde_json::json!({
            "booking_id": booking_id,
            "service_id": service_id,
            "start_time": start_time,
            "end_time": end_time,
            "description": "New booking request",
        }),
        serde_json::json!({
            "action_type": "approve_booking",
            "feature_type": "booking_approval",
            "booking_id": booking_id,
        }),
    )
}

fn frontend_url() -> Result<reqwest::Url, String> {
    let value = std::env::var("FRONTEND_URL")
        .map_err(|_| "FRONTEND_URL is required for paid bookings".to_string())?;
    let mut url =
        reqwest::Url::parse(value.trim()).map_err(|_| "invalid FRONTEND_URL".to_string())?;
    let local_http =
        url.scheme() == "http" && matches!(url.host_str(), Some("127.0.0.1" | "localhost"));
    if (url.scheme() != "https" && !local_http)
        || url.username() != ""
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err("invalid FRONTEND_URL".to_string());
    }
    url.set_path("");
    Ok(url)
}

fn trusted_stripe_checkout_url(value: &str) -> Option<String> {
    let url = reqwest::Url::parse(value).ok()?;
    (url.scheme() == "https"
        && url.host_str() == Some("checkout.stripe.com")
        && url.username().is_empty()
        && url.password().is_none())
    .then(|| url.to_string())
}

async fn create_booking_checkout(
    booking_id: &str,
    tenant_id: &str,
    service_id: &str,
    customer_id: &str,
    amount_cents: i64,
) -> Result<String, String> {
    let api_key =
        std::env::var("STRIPE_API_KEY").map_err(|_| "Stripe is not configured".to_string())?;
    let api_key = api_key.trim();
    if api_key.is_empty() || matches!(api_key, "sk_test" | "sk_test_123" | "sk_test_mock") {
        return Err("Stripe is not configured".to_string());
    }
    if amount_cents <= 0 {
        return Err("booking amount must be positive".to_string());
    }
    let base = frontend_url()?;
    let mut success = base.join("booking").map_err(|error| error.to_string())?;
    success
        .query_pairs_mut()
        .append_pair("tenant", tenant_id)
        .append_pair("service_id", service_id)
        .append_pair("booking_id", booking_id)
        .append_pair("payment", "success");
    let mut cancel = base.join("booking").map_err(|error| error.to_string())?;
    cancel
        .query_pairs_mut()
        .append_pair("tenant", tenant_id)
        .append_pair("service_id", service_id)
        .append_pair("booking_id", booking_id)
        .append_pair("payment", "cancelled");

    let form = [
        ("mode", "payment".to_string()),
        ("success_url", success.to_string()),
        ("cancel_url", cancel.to_string()),
        ("client_reference_id", customer_id.to_string()),
        ("line_items[0][price_data][currency]", "usd".to_string()),
        (
            "line_items[0][price_data][unit_amount]",
            amount_cents.to_string(),
        ),
        (
            "line_items[0][price_data][product_data][name]",
            "Booking deposit".to_string(),
        ),
        ("line_items[0][quantity]", "1".to_string()),
        ("metadata[booking_id]", booking_id.to_string()),
        ("metadata[tenant_id]", tenant_id.to_string()),
        ("metadata[service_id]", service_id.to_string()),
    ];
    let response = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|error| error.to_string())?
        .post("https://api.stripe.com/v1/checkout/sessions")
        .basic_auth(api_key, Some(""))
        .form(&form)
        .send()
        .await
        .map_err(|error| format!("Stripe checkout request failed: {error}"))?;
    if !response.status().is_success() {
        return Err(format!("Stripe checkout returned {}", response.status()));
    }
    let payload: serde_json::Value = response
        .json()
        .await
        .map_err(|_| "Stripe returned an invalid response".to_string())?;
    payload
        .get("url")
        .and_then(serde_json::Value::as_str)
        .and_then(trusted_stripe_checkout_url)
        .ok_or_else(|| "Stripe returned an invalid checkout URL".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reservation_window_rejects_inverted_and_oversized_ranges() {
        assert!(reservation_window("2026-07-15T10:00:00Z", "2026-07-15T11:00:00Z",).is_some());
        assert!(reservation_window("2026-07-15T11:00:00Z", "2026-07-15T10:00:00Z",).is_none());
        assert!(reservation_window("2026-07-15T10:00:00Z", "2026-07-17T10:00:01Z",).is_none());
    }

    #[test]
    fn customer_contact_validation_rejects_non_email_ids_and_control_characters() {
        assert!(valid_customer_name("Jane Doe"));
        assert!(!valid_customer_name("Jane\nDoe"));
        assert!(valid_customer_email("jane@example.com"));
        assert!(!valid_customer_email("jane.example.com"));
        assert!(!valid_customer_email("jane@localhost"));
        assert!(valid_customer_id("e2e-customer-bakery"));
        assert!(!valid_customer_id("../../other-tenant"));
    }

    #[test]
    fn checkout_is_required_only_when_product_metadata_requires_a_deposit() {
        assert_eq!(required_deposit_cents(7_500, &serde_json::json!({})), None);
        assert_eq!(
            required_deposit_cents(
                7_500,
                &serde_json::json!({"requires_deposit": true, "deposit_amount_cents": 2_500}),
            ),
            Some(2_500),
        );
        assert_eq!(
            required_deposit_cents(
                7_500,
                &serde_json::json!({"requires_deposit": true, "deposit_amount_cents": 20_000}),
            ),
            Some(7_500),
        );
    }

    #[test]
    fn booking_feed_item_contains_an_actionable_approval() {
        let (context, action) = booking_feed_payloads(
            "booking-1",
            "service-1",
            "2026-08-10T09:00:00Z",
            "2026-08-10T10:00:00Z",
        );
        assert_eq!(context["booking_id"], "booking-1");
        assert_eq!(action["action_type"], "approve_booking");
        assert_eq!(action["feature_type"], "booking_approval");
        assert_eq!(action["booking_id"], "booking-1");
    }

    #[test]
    fn checkout_urls_are_restricted_to_stripe() {
        assert!(
            trusted_stripe_checkout_url("https://checkout.stripe.com/c/pay/cs_live_123").is_some()
        );
        assert!(trusted_stripe_checkout_url("javascript:alert(1)").is_none());
        assert!(
            trusted_stripe_checkout_url("https://checkout.stripe.com.attacker.test/pay").is_none()
        );
        assert!(trusted_stripe_checkout_url("https://user:pass@checkout.stripe.com/pay").is_none());
    }
}

pub fn router<S>(db: Arc<DB>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(handle_reserve))
        .with_state(db)
}

async fn handle_reserve(
    State(db): State<Arc<DB>>,
    claims: Option<Extension<::server_common::Claims>>,
    Json(payload): Json<ReserveRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims
        .as_ref()
        .and_then(|Extension(claims)| ::server_common::auth_utils::signed_tenant_id(claims))
    {
        Some(tenant_id) => tenant_id,
        _ => {
            return (
                axum::http::StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({"error": "unauthorized"})),
            )
                .into_response();
        }
    };
    if payload.service_id.trim().is_empty() || payload.service_id.chars().count() > 128 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "invalid service_id"})),
        )
            .into_response();
    }
    let Some((st, et)) = reservation_window(&payload.start_time, &payload.end_time) else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "invalid reservation window"})),
        )
            .into_response();
    };
    let supplied_customer_id = match payload.customer_id.as_deref() {
        Some(customer_id) => {
            if valid_customer_id(customer_id) {
                Some(customer_id.trim().to_string())
            } else {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({"error": "invalid customer_id"})),
                )
                    .into_response();
            }
        }
        None => None,
    };
    let customer_name = payload.customer_name.as_deref().map(str::trim);
    let customer_email = payload.customer_email.as_deref().map(str::trim);
    if supplied_customer_id.is_none()
        && (!customer_name.is_some_and(valid_customer_name)
            || !customer_email.is_some_and(valid_customer_email))
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "valid customer contact is required"})),
        )
            .into_response();
    }

    let pool = db.pool.clone();

    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("failed to begin tx: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ReserveResponse {
                    success: false,
                    booking_id: None,
                    error: Some("internal error".to_string()),
                    checkout_url: None,
                }),
            )
                .into_response();
        }
    };

    if let Err(error) = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        tracing::error!("failed to bind reservation tenant context: {error}"); // pii-safe
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "internal error"})),
        )
            .into_response();
    }

    let c_id = if let Some(customer_id) = supplied_customer_id {
        Some(customer_id)
    } else {
        let email = customer_email.expect("validated customer email");
        match sqlx::query_scalar::<_, String>(
            "SELECT id FROM customers WHERE tenant_id = $1 AND lower(email) = lower($2) ORDER BY created_at ASC LIMIT 1",
        )
        .bind(&tenant_id)
        .bind(email)
        .fetch_optional(&mut *tx)
        .await
        {
            Ok(Some(customer_id)) => Some(customer_id),
            Ok(None) => {
                let customer_id = uuid::Uuid::new_v4().to_string();
                if let Err(error) = sqlx::query(
                    "INSERT INTO customers (id, tenant_id, name, email) VALUES ($1, $2, $3, $4)",
                )
                .bind(&customer_id)
                .bind(&tenant_id)
                .bind(customer_name.expect("validated customer name"))
                .bind(email)
                .execute(&mut *tx)
                .await
                {
                    tracing::error!("failed to create booking customer: {error}");
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({"error": "internal error"})),
                    )
                        .into_response();
                }
                Some(customer_id)
            }
            Err(error) => {
                tracing::error!("failed to resolve booking customer: {error}");
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "internal error"})),
                )
                    .into_response();
            }
        }
    };

    let booking_id = uuid::Uuid::new_v4().to_string();

    let product = match sqlx::query(
        "SELECT price_cents, metadata FROM products WHERE id = $1 AND tenant_id = $2",
    )
    .bind(&payload.service_id)
    .bind(&tenant_id)
    .fetch_optional(&mut *tx)
    .await
    {
        Ok(Some(product)) => product,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "service not found"})),
            )
                .into_response();
        }
        Err(error) => {
            tracing::error!("failed to load reservation service: {error}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "internal error"})),
            )
                .into_response();
        }
    };
    let price_cents: i64 = product.try_get("price_cents").unwrap_or(0);
    let metadata: serde_json::Value = product
        .try_get("metadata")
        .unwrap_or_else(|_| serde_json::json!({}));
    let deposit_cents = required_deposit_cents(price_cents, &metadata);

    let claimed_slot = sqlx::query(
        "UPDATE availability_blocks SET is_available = false WHERE tenant_id = $1 AND service_id = $2 AND start_time = $3 AND end_time = $4 AND is_available = true RETURNING id",
    )
    .bind(&tenant_id)
    .bind(&payload.service_id)
    .bind(st)
    .bind(et)
    .fetch_optional(&mut *tx)
    .await;
    match claimed_slot {
        Ok(Some(_)) => {}
        Ok(None) => {
            return (
                StatusCode::CONFLICT,
                Json(serde_json::json!({"error": "slot unavailable"})),
            )
                .into_response();
        }
        Err(error) => {
            tracing::error!("failed to claim reservation slot: {error}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "internal error"})),
            )
                .into_response();
        }
    }

    let booking_status = if deposit_cents.is_some() {
        "pending_payment"
    } else {
        "pending"
    };
    let res = sqlx::query(
        r#"
        INSERT INTO bookings (id, tenant_id, customer_id, product_id, start_time, end_time, status)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(&booking_id)
    .bind(&tenant_id)
    .bind(&c_id)
    .bind(&payload.service_id)
    .bind(st)
    .bind(et)
    .bind(booking_status)
    .execute(&mut *tx)
    .await;

    if let Err(e) = res {
        tracing::error!("failed to insert booking: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ReserveResponse {
                success: false,
                booking_id: None,
                error: Some("failed to create booking".to_string()),
                checkout_url: None,
            }),
        )
            .into_response();
    }

    // Add feed item within the same transaction as the reservation.
    let feed_id = uuid::Uuid::new_v4().to_string();
    let (feed_context, proposed_action) = booking_feed_payloads(
        &booking_id,
        &payload.service_id,
        &payload.start_time,
        &payload.end_time,
    );
    let feed_result = sqlx::query(
        r#"
        INSERT INTO agent_feed_items (id, tenant_id, event_source, lifecycle_state, context_payload, proposed_action)
        VALUES ($1, $2, 'booking_request', 'PENDING_APPROVAL', $3, $4)
        "#,
    )
    .bind(&feed_id)
    .bind(&tenant_id)
    .bind(feed_context)
    .bind(proposed_action)
    .execute(&mut *tx)
    .await;
    if let Err(error) = feed_result {
        tracing::error!("failed to create booking feed item: {error}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "internal error"})),
        )
            .into_response();
    }
    let checkout_url = if let Some(deposit_cents) = deposit_cents {
        match create_booking_checkout(
            &booking_id,
            &tenant_id,
            &payload.service_id,
            &c_id
                .expect("booking customer is always resolved")
                .to_string(),
            deposit_cents,
        )
        .await
        {
            Ok(url) => Some(url),
            Err(error) => {
                tracing::error!("failed to create booking checkout: {error}");
                return (
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(serde_json::json!({"error": "deposit checkout is unavailable"})),
                )
                    .into_response();
            }
        }
    } else {
        None
    };

    if let Err(error) = tx.commit().await {
        tracing::error!("failed to commit reservation: {error}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "internal error"})),
        )
            .into_response();
    }

    (
        StatusCode::OK,
        Json(ReserveResponse {
            success: true,
            booking_id: Some(booking_id),
            error: None,
            checkout_url,
        }),
    )
        .into_response()
}
