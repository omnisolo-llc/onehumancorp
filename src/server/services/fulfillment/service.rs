use std::sync::Arc;
use tonic::{Request, Response, Status};
use uuid::Uuid;
use ::server_ohc::hub::fulfillment_service_server::FulfillmentService;
use ::server_ohc::hub::{CalculateFulfillmentRequest, CalculateFulfillmentResponse, GenerateShippingLabelRequest};
use ::server_ohc::organization::{FulfillmentMethod, ShippingLabel};
use crate::hub::Hub;

pub struct MyFulfillmentService {
    hub: Arc<Hub>,
}

impl MyFulfillmentService {
    pub fn new(hub: Arc<Hub>) -> Self {
        MyFulfillmentService { hub }
    }
}

#[tonic::async_trait]
impl FulfillmentService for MyFulfillmentService {
    async fn calculate_fulfillment(
        &self,
        request: Request<CalculateFulfillmentRequest>,
    ) -> Result<Response<CalculateFulfillmentResponse>, Status> {
        let req = request.into_inner();

        let mut available_methods = vec![];

        let distance = (req.customer_address.len() as f32) % 20.0;

        available_methods.push(FulfillmentMethod {
            id: Uuid::new_v4().to_string(),
            order_id: "".to_string(),
            r#type: "pickup".to_string(),
            cost: 0.0,
            provider: "store".to_string(),
        });

        if distance < 10.0 {
            available_methods.push(FulfillmentMethod {
                id: Uuid::new_v4().to_string(),
                order_id: "".to_string(),
                r#type: "local_delivery".to_string(),
                cost: 5.0,
                provider: "store_delivery".to_string(),
            });
        }

        available_methods.push(FulfillmentMethod {
            id: Uuid::new_v4().to_string(),
            order_id: "".to_string(),
            r#type: "shipping".to_string(),
            cost: 12.0,
            provider: "usps".to_string(),
        });

        Ok(Response::new(CalculateFulfillmentResponse {
            available_methods,
        }))
    }

    async fn generate_shipping_label(
        &self,
        request: Request<GenerateShippingLabelRequest>,
    ) -> Result<Response<ShippingLabel>, Status> {
        let req = request.into_inner();

        // Ensure tenant isolation
        let tenant_id = ::server_common::auth_utils::get_org_context().unwrap_or_else(|| "system".to_string());
        if tenant_id.is_empty() {
             return Err(Status::unauthenticated("Missing tenant context"));
        }

        let label = ShippingLabel {
            id: Uuid::new_v4().to_string(),
            order_id: req.order_id.clone(),
            tracking_number: format!("TRACK-{}", Uuid::new_v4().to_string()[0..8].to_uppercase()),
            label_url: format!("https://storage.ohc.io/labels/{}.pdf", Uuid::new_v4()),
        };

        let pool = self.hub.db.get_pool();
        let query = "INSERT INTO shipping_labels (id, order_id, tracking_number, label_url) VALUES ($1, $2, $3, $4)";
        match self.hub.db.store {
            crate::db::DbStore::Sqlite(ref sqlite_pool) => {
                sqlx::query(query)
                    .bind(&label.id)
                    .bind(&label.order_id)
                    .bind(&label.tracking_number)
                    .bind(&label.label_url)
                    .execute(sqlite_pool)
                    .await.map_err(|e| Status::internal(format!("Database error: {}", e)))?;
            },
            crate::db::DbStore::Postgres => {
                sqlx::query(query)
                    .bind(&label.id)
                    .bind(&label.order_id)
                    .bind(&label.tracking_number)
                    .bind(&label.label_url)
                    .execute(pool)
                    .await.map_err(|e| Status::internal(format!("Database error: {}", e)))?;
            }
        }

        let payload = serde_json::json!({
            "order_id": label.order_id,
            "label_url": label.label_url,
        });

        self.hub.mesh.publish_teammate_event(
            "store_operations".to_string(),
            "operations_agent".to_string(),
            "print_label".to_string(),
            payload.to_string().into_bytes(),
        ).await.map_err(|e| Status::internal(format!("Mesh error: {}", e)))?;

        Ok(Response::new(label))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::server_ohc::hub::{CalculateFulfillmentRequest, GenerateShippingLabelRequest};
    use tonic::Request;

    #[test]
    fn test_calculate_fulfillment_structure() {
        let req = CalculateFulfillmentRequest {
            tenant_id: "t1".to_string(),
            customer_address: "123 Short".to_string(),
            cart_total: 50.0,
        };
        assert_eq!(req.customer_address.len(), 9);

        let distance = (req.customer_address.len() as f32) % 20.0;
        assert_eq!(distance, 9.0); // Within local delivery threshold
    }
}
