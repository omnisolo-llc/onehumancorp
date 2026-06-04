use axum::{Json, response::IntoResponse, http::StatusCode, extract::State};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SquareWebhookEvent {
    pub r#type: String,
    pub data: SquareWebhookData,
}

#[derive(Deserialize, Debug)]
pub struct SquareWebhookData {
    pub object: SquareWebhookObject,
}

#[derive(Deserialize, Debug)]
pub struct SquareWebhookObject {
    pub inventory_counts: Option<Vec<SquareInventoryCount>>,
}

#[derive(Deserialize, Debug)]
pub struct SquareInventoryCount {
    pub catalog_object_id: String,
    pub state: String,
    pub location_id: Option<String>,
    pub quantity: String,
}

pub async fn square_webhook_handler(
    State(db): State<sqlx::PgPool>,
    Json(payload): Json<SquareWebhookEvent>,
) -> impl IntoResponse {
    tracing::info!("Received Square webhook event: {}", payload.r#type);

    if payload.r#type != "inventory.count.updated" {
        return (StatusCode::OK, "Event ignored").into_response();
    }

    if let Some(counts) = payload.data.object.inventory_counts {
        for count in counts {
            if count.state != "IN_STOCK" {
                continue; // We only care about IN_STOCK state for simplicity
            }

            let quantity: i32 = match count.quantity.parse() {
                Ok(q) => q,
                Err(_) => {
                    tracing::warn!("Square webhook: invalid quantity {}", count.quantity);
                    continue;
                }
            };

            let catalog_object_id = count.catalog_object_id;

            let query = "
                UPDATE products
                SET inventory_count = $1, updated_at = CURRENT_TIMESTAMP
                WHERE metadata->>'square_item_id' = $2
            ";

            match sqlx::query(query)
                .bind(quantity)
                .bind(&catalog_object_id)
                .execute(&db)
                .await
            {
                Ok(result) => {
                    if result.rows_affected() > 0 {
                        tracing::info!("Square webhook: updated product mapped to {} to count {}", catalog_object_id, quantity);
                    } else {
                        tracing::debug!("Square webhook: product for {} not found in OHC", catalog_object_id);
                    }
                }
                Err(e) => {
                    tracing::error!("Square webhook: Failed to update inventory: {}", e);
                }
            }
        }
    }

    (StatusCode::OK, "Webhook processed").into_response()
}
