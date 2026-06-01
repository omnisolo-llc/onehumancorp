use tonic::{Request, Response, Status};
use std::sync::RwLock;
use sqlx::{PgPool, Row};

use server_ohc::delivery::*;
use server_ohc::delivery::delivery_service_server::DeliveryService;

pub struct LocalDispatchService {
    pool: Option<PgPool>,
}

impl LocalDispatchService {
    pub fn new() -> Self {
        LocalDispatchService {
            pool: None, // Simplified for this prototype update. Real implementation should take pool in `new`
        }
    }
}

#[tonic::async_trait]
impl DeliveryService for LocalDispatchService {
    async fn toggle_delivery(
        &self,
        request: Request<ToggleDeliveryRequest>,
    ) -> Result<Response<ToggleDeliveryResponse>, Status> {
        let req = request.into_inner();

        // This is a stub implementation. In a real system we'd execute:
        // sqlx::query!("UPDATE organizations SET local_delivery_enabled = $1 WHERE id = $2", req.enabled, req.organization_id)

        Ok(Response::new(ToggleDeliveryResponse { enabled: req.enabled }))
    }

    async fn get_orders(
        &self,
        request: Request<GetOrdersRequest>,
    ) -> Result<Response<GetOrdersResponse>, Status> {
        let req = request.into_inner();

        // Stub implementation.
        let org_orders: Vec<DeliveryOrder> = vec![];

        Ok(Response::new(GetOrdersResponse { orders: org_orders }))
    }

    async fn dispatch_route(
        &self,
        request: Request<DispatchRouteRequest>,
    ) -> Result<Response<DispatchRouteResponse>, Status> {
        let req = request.into_inner();
        let route = Route {
            id: uuid::Uuid::new_v4().to_string(),
            driver_id: req.driver_id.clone(),
            organization_id: req.organization_id.clone(),
            order_ids: req.order_ids.clone(),
            status: "ACTIVE".to_string(),
        };
        Ok(Response::new(DispatchRouteResponse { route: Some(route) }))
    }

    async fn update_driver_location(
        &self,
        request: Request<UpdateDriverLocationRequest>,
    ) -> Result<Response<UpdateDriverLocationResponse>, Status> {
        let req = request.into_inner();
        Ok(Response::new(UpdateDriverLocationResponse { success: true }))
    }

    async fn get_driver_location(
        &self,
        request: Request<GetDriverLocationRequest>,
    ) -> Result<Response<GetDriverLocationResponse>, Status> {
        let req = request.into_inner();
        Err(Status::not_found("Driver location not found"))
    }

    async fn mark_order_delivered(
        &self,
        request: Request<MarkOrderDeliveredRequest>,
    ) -> Result<Response<MarkOrderDeliveredResponse>, Status> {
        let req = request.into_inner();
        Err(Status::not_found("Order not found"))
    }
}
