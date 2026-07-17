use axum::{
    extract::{Extension, State},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct FetchRatesRequest {
    pub orderId: String,
    pub weight: String,
    pub dimensions: String,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct PurchaseLabelRequest {
    pub orderId: String,
    pub rateId: String,
}

#[derive(Serialize)]
pub struct RatesResponse {
    pub rates: Vec<crate::integrations::shippo::client::ShippoRate>,
}

pub fn router<S: Clone + Send + Sync + 'static>(db: std::sync::Arc<crate::db::DB>) -> Router<S> {
    Router::new()
        .route("/rates", post(fetch_rates))
        .route("/label", post(purchase_label))
        .with_state(db)
}

async fn fetch_rates(
    Json(payload): Json<FetchRatesRequest>,
) -> impl IntoResponse {
    let weight_f64 = payload.weight.parse::<f64>().unwrap_or(1.0);

    let registry = crate::integrations::registry::IntegrationsRegistry::new();

    match registry.fetch_rates("shippo", weight_f64, &payload.dimensions).await {
        Ok(rates) => (StatusCode::OK, Json(RatesResponse { rates })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e }))).into_response(),
    }
}

async fn purchase_label(
    State(db): State<std::sync::Arc<crate::db::DB>>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<PurchaseLabelRequest>,
) -> impl IntoResponse {
    let registry = crate::integrations::registry::IntegrationsRegistry::new();

    let tenant_id = match claims.organization_id.as_deref().filter(|s| !s.is_empty()).or(Some(claims.sub.as_str()).filter(|s| !s.is_empty())) {
        Some(t) => t.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "unauthorized" }))).into_response(),
    };

    match registry.purchase_label("shippo", &payload.rateId).await {
        Ok(response) => {
            if let Ok(mut tx) = db.pool.begin().await {
                // Determine whether it's SQLite or Postgres to bind properly.
                // In Postgres we bind UUID, in SQLite String
                let task_id = uuid::Uuid::new_v4();
                match &db.store {
                    crate::db::DbStore::Postgres => {
                        let _ = sqlx::query("UPDATE orders SET status = 'shipped' WHERE id = $1 AND tenant_id = $2")
                            .bind(&payload.orderId)
                            .bind(&tenant_id)
                            .execute(&mut *tx)
                            .await;

                        let _ = sqlx::query("INSERT INTO delivery_tasks (id, organization_id, order_id, status, provider, provider_delivery_id) VALUES ($1, $2, $3, 'shipped', 'shippo', $4)")
                            .bind(task_id)
                            .bind(&tenant_id)
                            .bind(&payload.orderId)
                            .bind(&response.tracking_number)
                            .execute(&mut *tx)
                            .await;
                    },
                    crate::db::DbStore::Sqlite(_) => {
                        let _ = sqlx::query("UPDATE orders SET status = 'shipped' WHERE id = ? AND tenant_id = ?")
                            .bind(&payload.orderId)
                            .bind(&tenant_id)
                            .execute(&mut *tx)
                            .await;

                        let _ = sqlx::query("INSERT INTO delivery_tasks (id, organization_id, order_id, status, provider, provider_delivery_id) VALUES (?, ?, ?, 'shipped', 'shippo', ?)")
                            .bind(task_id.to_string())
                            .bind(&tenant_id)
                            .bind(&payload.orderId)
                            .bind(&response.tracking_number)
                            .execute(&mut *tx)
                            .await;
                    }
                }

                let _ = tx.commit().await;

                let draft_reply = format!("Great news! Your order {} has shipped. You can track it here: {}", payload.orderId, response.tracking_number);
                let _ = crate::domain::action_router::dispatch_action(
                    "ambassador_reply",
                    &tenant_id,
                    &serde_json::json!({ "draft_reply": draft_reply, "order_id": payload.orderId }),
                    &db.pool
                ).await;
            }

            (StatusCode::OK, Json(response)).into_response()
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e }))).into_response(),
    }
}
