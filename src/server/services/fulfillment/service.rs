use crate::db::get_pool;
use uuid::Uuid;
use crate::integrations::registry::IntegrationsRegistry;
use std::sync::Arc;

pub struct FulfillmentService {
    registry: Arc<IntegrationsRegistry>,
}

#[derive(serde::Serialize)]
pub struct FulfillmentOption {
    pub method: String,
    pub cost: f64,
    pub estimated_days: u32,
}

impl FulfillmentService {
    pub fn new(registry: Arc<IntegrationsRegistry>) -> Self {
        Self { registry }
    }

    pub async fn calculate_fulfillment_options(&self, tenant_id: &str, customer_address: &str) -> Result<Vec<FulfillmentOption>, String> {
        let pool = get_pool();
        let profile = sqlx::query_as::<_, crate::domain::repository::models::FulfillmentProfile>(
            "SELECT * FROM fulfillment_profiles WHERE tenant_id = $1"
        )
        .bind(tenant_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut options = Vec::new();

        if let Some(p) = profile {
            // Mock distance calculation
            let distance_miles = if customer_address.contains("Local") { 2.0 } else { 15.0 };

            if p.enable_local_delivery.unwrap_or(false) && distance_miles <= p.local_delivery_radius_miles.unwrap_or(5.0) {
                options.push(FulfillmentOption {
                    method: "local_delivery".to_string(),
                    cost: 5.0,
                    estimated_days: 1,
                });
            }

            if p.enable_pickup.unwrap_or(false) {
                options.push(FulfillmentOption {
                    method: "pickup".to_string(),
                    cost: 0.0,
                    estimated_days: 0,
                });
            }

            if p.enable_shipping.unwrap_or(false) {
                options.push(FulfillmentOption {
                    method: "shipping".to_string(),
                    cost: 10.0, // Mock standard shipping rate
                    estimated_days: 3,
                });
            }
        } else {
            // Default to shipping if no profile
            options.push(FulfillmentOption {
                method: "shipping".to_string(),
                cost: 10.0,
                estimated_days: 3,
            });
        }

        if options.is_empty() {
             options.push(FulfillmentOption {
                method: "shipping".to_string(),
                cost: 10.0,
                estimated_days: 3,
            });
        }

        Ok(options)
    }

    pub async fn generate_shipping_label(&self, tenant_id: &str, order_id: &str, to_address: &str, from_address: &str) -> Result<String, String> {
        let label_url = match self.registry.create_shipment("easypost", to_address, from_address, "standard_box").await {
            Ok(url) => url,
            Err(_) => {
                // Mock fallback label if easypost isn't set up yet
                "https://easypost.com/labels/mock_label_fallback.pdf".to_string()
            }
        };

        let pool = get_pool();
        let id = Uuid::new_v4().to_string();

        sqlx::query(
            "INSERT INTO shipping_labels (id, order_id, tracking_number, label_url) VALUES ($1, $2, $3, $4)"
        )
        .bind(&id)
        .bind(order_id)
        .bind(format!("TRACK_{}", Uuid::new_v4().to_string().chars().take(8).collect::<String>()))
        .bind(&label_url)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(label_url)
    }
}
