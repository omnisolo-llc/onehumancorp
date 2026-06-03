use axum::{Json, response::IntoResponse, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct POSOfflineItem {
    pub product_id: String,
    pub quantity: i32,
    pub unit_price_cents: i64,
}

#[derive(Deserialize, Debug)]
pub struct POSOfflineTransaction {
    pub idempotency_key: String,
    pub amount_cents: i64,
    pub currency: String,
    pub payment_method: String,
    pub stripe_payment_intent_id: Option<String>,
    pub items: Vec<POSOfflineItem>,
}

#[derive(Deserialize, Debug)]
pub struct BatchPOSSyncRequest {
    pub transactions: Vec<POSOfflineTransaction>,
}

#[derive(Serialize)]
pub struct BatchPOSSyncResponse {
    pub success: bool,
    pub queued_count: usize,
}

pub async fn pos_sync_handler(
    State(db): State<sqlx::PgPool>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<BatchPOSSyncRequest>,
) -> impl IntoResponse {
    tracing::info!("Received {} POS offline transactions for sync.", payload.transactions.len());

    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    let (tenant_id, _) = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));

    if tenant_id.is_empty() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(BatchPOSSyncResponse { success: false, queued_count: 0 }),
        ).into_response();
    }

    let mut queued_count = 0;

    // We will use the existing OHCJobQueue to enqueue the processing.
    let job_queue = crate::orchestration::queue::OHCJobQueue::new(Arc::new(db.clone()));

    for transaction in &payload.transactions {
        let job_payload = serde_json::json!({
            "idempotency_key": transaction.idempotency_key,
            "amount_cents": transaction.amount_cents,
            "currency": transaction.currency,
            "payment_method": transaction.payment_method,
            "stripe_payment_intent_id": transaction.stripe_payment_intent_id,
            "items": transaction.items.iter().map(|i| serde_json::json!({
                "product_id": i.product_id,
                "quantity": i.quantity,
                "unit_price_cents": i.unit_price_cents
            })).collect::<Vec<_>>()
        });

        match job_queue.enqueue(&tenant_id, "pos_sync_transaction", &job_payload).await {
            Ok(_) => queued_count += 1,
            Err(e) => tracing::error!("Failed to enqueue pos_sync_transaction job: {}", e),
        }
    }

    (
        StatusCode::OK,
        Json(BatchPOSSyncResponse { success: true, queued_count }),
    ).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;

    #[tokio::test]
    async fn test_pos_sync_unauthorized() {
        let pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://localhost/dummy").unwrap();
        let state = State(pool);

        let req = BatchPOSSyncRequest { transactions: vec![] };
        let headers = HeaderMap::new();

        let response = pos_sync_handler(state, headers, Json(req)).await.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}

    #[tokio::test]
    async fn test_pos_sync_success() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") {
            return;
        }

        let pool = sqlx::postgres::PgPoolOptions::new().connect(&database_url).await.unwrap();

        // Setup test data
        sqlx::query("INSERT INTO tenants (id, name) VALUES ('tenant-pos-offline', 'Offline Test Tenant') ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();

        let state = State(pool.clone());

        let req = BatchPOSSyncRequest {
            transactions: vec![
                POSOfflineTransaction {
                    idempotency_key: "test_key".to_string(),
                    amount_cents: 1000,
                    currency: "usd".to_string(),
                    payment_method: "card_present".to_string(),
                    stripe_payment_intent_id: Some("pi_123".to_string()),
                    items: vec![
                        POSOfflineItem {
                            product_id: "prod-pos-1".to_string(),
                            quantity: 1,
                            unit_price_cents: 1000,
                        }
                    ],
                }
            ],
        };

        let mut headers = HeaderMap::new();
        headers.insert("x-spiffe-id", "spiffe://ohc/org/tenant-pos-offline/agent/x".parse().unwrap());

        let response = pos_sync_handler(state, headers, Json(req)).await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }
