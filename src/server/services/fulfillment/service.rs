use tonic::{Request, Response, Status};
use std::sync::{Arc, RwLock};
use chrono::Utc;
use uuid::Uuid;

use crate::hub::Hub;
use super::db::FulfillmentDb;

pub mod proto {
    tonic::include_proto!("ohc.fulfillment");
}

use proto::fulfillment_service_server::FulfillmentService;
use proto::*;

pub struct MyFulfillmentService {
    hub: Arc<Hub>,
    db: FulfillmentDb,
}

impl MyFulfillmentService {
    pub fn new(hub: Arc<Hub>) -> Self {
        let pool = hub.pool.clone();
        MyFulfillmentService {
            hub,
            db: FulfillmentDb::new(pool),
        }
    }
}

#[tonic::async_trait]
impl FulfillmentService for MyFulfillmentService {
    async fn dispatch_fulfillment(
        &self,
        request: Request<DispatchRequest>,
    ) -> Result<Response<DispatchResponse>, Status> {
        let req = request.into_inner();

        let origin = req.origin.clone().unwrap_or(Location { latitude: 0.0, longitude: 0.0, address: String::new() });
        let dest = req.destination.clone().unwrap_or(Location { latitude: 0.0, longitude: 0.0, address: String::new() });

        let couriers = self.db.get_available_couriers(&req.tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;

        let mut best_courier = None;
        let mut lowest_cost = std::f64::MAX;

        for courier in couriers.iter() {
            let dx = origin.longitude - dest.longitude;
            let dy = origin.latitude - dest.latitude;
            let distance = (dx * dx + dy * dy).sqrt() * 69.0;

            let cost = courier.base_cost + distance * courier.cost_per_mile;

            if cost < lowest_cost {
                lowest_cost = cost;
                best_courier = Some(courier.clone());
            }
        }

        let assigned_method = best_courier.clone().map_or(FulfillmentMethod::Unspecified as i32, |c| c.method);
        let courier_id = best_courier.map_or(String::new(), |c| c.id);

        let now = Utc::now().timestamp_millis();

        let order = FulfillmentOrder {
            id: Uuid::new_v4().to_string(),
            tenant_id: req.tenant_id,
            order_id: req.order_id,
            assigned_method,
            state: FulfillmentState::Preparing as i32,
            courier_id,
            origin: req.origin,
            destination: req.destination,
            estimated_prep_time_ms: req.prep_time_ms,
            estimated_delivery_time_ms: 1800000,
            estimated_cost: lowest_cost,
            created_at_unix: now,
            updated_at_unix: now,
        };

        self.db.create_order(&order).await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(DispatchResponse { order: Some(order) }))
    }

    async fn update_fulfillment_state(
        &self,
        request: Request<UpdateStateRequest>,
    ) -> Result<Response<UpdateStateResponse>, Status> {
        let req = request.into_inner();

        self.db.update_order_state(&req.tenant_id, &req.fulfillment_id, req.state).await.map_err(|e| Status::internal(e.to_string()))?;

        let order = self.db.get_order(&req.tenant_id, &req.fulfillment_id).await.map_err(|e| Status::internal(e.to_string()))?.ok_or_else(|| Status::not_found("Order not found"))?;

        Ok(Response::new(UpdateStateResponse {
            order: Some(order),
        }))
    }

    async fn get_fulfillment(
        &self,
        request: Request<GetFulfillmentRequest>,
    ) -> Result<Response<GetFulfillmentResponse>, Status> {
        let req = request.into_inner();

        let order = self.db.get_order(&req.tenant_id, &req.fulfillment_id).await.map_err(|e| Status::internal(e.to_string()))?.ok_or_else(|| Status::not_found("Order not found"))?;

        Ok(Response::new(GetFulfillmentResponse {
            order: Some(order),
        }))
    }

    async fn list_couriers(
        &self,
        request: Request<ListCouriersRequest>,
    ) -> Result<Response<ListCouriersResponse>, Status> {
        let req = request.into_inner();

        let couriers = self.db.get_available_couriers(&req.tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListCouriersResponse { couriers }))
    }

    async fn register_courier(
        &self,
        request: Request<RegisterCourierRequest>,
    ) -> Result<Response<RegisterCourierResponse>, Status> {
        let req = request.into_inner();

        if let Some(mut courier) = req.courier {
            if courier.id.is_empty() {
                courier.id = Uuid::new_v4().to_string();
            }

            self.db.register_courier(&courier, &req.tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;

            Ok(Response::new(RegisterCourierResponse { courier: Some(courier) }))
        } else {
            Err(Status::invalid_argument("Courier information missing"))
        }
    }
}
