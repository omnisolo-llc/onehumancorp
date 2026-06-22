use tonic::{Request, Response, Status};
use dispatch_proto_lib::ohc::dispatch::dispatch_service_server::DispatchService;
use sqlx::Row;
use sqlx::postgres::PgRow;

use dispatch_proto_lib::ohc::dispatch::{
    CreateRouteRequest, CreateRouteResponse,
    GetRouteRequest, GetRouteResponse,
    AddStopRequest, AddStopResponse,
    UpdateCourierLocationRequest, UpdateCourierLocationResponse,
    GetTravelPaddingRequest, GetTravelPaddingResponse,
    HandleCourierWebhookRequest, HandleCourierWebhookResponse,
    LocalDispatchRoute, LocalDispatchStop, CourierDispatch,
};

pub struct DispatchServiceImpl {
    pool: sqlx::PgPool,
}

impl DispatchServiceImpl {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    fn extract_tenant_id<T>(&self, request: &Request<T>) -> Result<String, Status> {
        match request.metadata().get("x-tenant-id") {
            Some(tenant_id) => tenant_id.to_str().map(|s| s.to_string()).map_err(|_| Status::unauthenticated("Invalid tenant format")),
            None => Err(Status::unauthenticated("Missing x-tenant-id header"))
        }
    }
}

#[tonic::async_trait]
impl DispatchService for DispatchServiceImpl {
    async fn create_route(&self, request: Request<CreateRouteRequest>) -> Result<Response<CreateRouteResponse>, Status> {
        let tenant_id = self.extract_tenant_id(&request)?;
        let req = request.into_inner();
        let route = req.route.ok_or_else(|| Status::invalid_argument("Route is required"))?;

        let id = uuid::Uuid::new_v4().to_string();

        let row = sqlx::query(
            r#"
            INSERT INTO local_dispatch_routes (id, tenant_id, vehicle_id, start_time, end_time, status)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, vehicle_id, start_time, end_time, status
            "#
        )
        .bind(&id)
        .bind(&tenant_id)
        .bind(route.vehicle_id.as_str())
        .bind("1970-01-01T00:00:00Z")
        .bind("1970-01-01T00:00:00Z")
        .bind(&route.status)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(CreateRouteResponse {
            route: Some(LocalDispatchRoute {
                id: row.get::<String, _>("id"),
                tenant_id: row.get::<String, _>("tenant_id"),
                vehicle_id: row.get::<Option<String>, _>("vehicle_id").unwrap_or_default(),
                start_time: "1970-01-01T00:00:00Z".to_string(),
                end_time: "1970-01-01T00:00:00Z".to_string(),
                status: row.get::<String, _>("status"),
                stops: vec![],
            }),
        }))
    }

    async fn get_route(&self, request: Request<GetRouteRequest>) -> Result<Response<GetRouteResponse>, Status> {
        let tenant_id = self.extract_tenant_id(&request)?;
        let req = request.into_inner();

        let row = sqlx::query(
            r#"
            SELECT id, tenant_id, vehicle_id, start_time, end_time, status
            FROM local_dispatch_routes
            WHERE id = $1 AND tenant_id = $2
            "#
        )
        .bind(&req.id)
        .bind(&tenant_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        if let Some(row) = row {
            // Also fetch stops to be suitable for offline edge-caching
            let stops_rows = sqlx::query(
                r#"
                SELECT id, route_id, tenant_id, location_id, sequence_number, estimated_arrival, actual_arrival, status
                FROM local_dispatch_stops
                WHERE route_id = $1 AND tenant_id = $2
                ORDER BY sequence_number ASC
                "#
            )
            .bind(&req.id)
            .bind(&tenant_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

            let mut stops = vec![];
            for sr in stops_rows {
                stops.push(LocalDispatchStop {
                    id: sr.get::<String, _>("id"),
                    route_id: sr.get::<String, _>("route_id"),
                    tenant_id: sr.get::<String, _>("tenant_id"),
                    location_id: sr.get::<String, _>("location_id"),
                    sequence_number: sr.get::<i32, _>("sequence_number"),
                    estimated_arrival: "1970-01-01T00:00:00Z".to_string(),
                    actual_arrival: "1970-01-01T00:00:00Z".to_string(),
                    status: sr.get::<String, _>("status"),
                });
            }

            Ok(Response::new(GetRouteResponse {
                route: Some(LocalDispatchRoute {
                    id: row.get::<String, _>("id"),
                    tenant_id: row.get::<String, _>("tenant_id"),
                    vehicle_id: row.get::<Option<String>, _>("vehicle_id").unwrap_or_default(),
                    start_time: "1970-01-01T00:00:00Z".to_string(),
                    end_time: "1970-01-01T00:00:00Z".to_string(),
                    status: row.get::<String, _>("status"),
                    stops,
                }),
            }))
        } else {
            Err(Status::not_found("Route not found"))
        }
    }

    async fn add_stop(&self, request: Request<AddStopRequest>) -> Result<Response<AddStopResponse>, Status> {
        let tenant_id = self.extract_tenant_id(&request)?;
        let req = request.into_inner();
        let stop = req.stop.ok_or_else(|| Status::invalid_argument("Stop is required"))?;

        let id = uuid::Uuid::new_v4().to_string();

        let row = sqlx::query(
            r#"
            INSERT INTO local_dispatch_stops (id, route_id, tenant_id, location_id, sequence_number, estimated_arrival, status)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, route_id, tenant_id, location_id, sequence_number, estimated_arrival, actual_arrival, status
            "#
        )
        .bind(&id)
        .bind(&stop.route_id)
        .bind(&tenant_id)
        .bind(&stop.location_id)
        .bind(stop.sequence_number)
        .bind("1970-01-01T00:00:00Z")
        .bind(&stop.status)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(AddStopResponse {
            stop: Some(LocalDispatchStop {
                id: row.get::<String, _>("id"),
                route_id: row.get::<String, _>("route_id"),
                tenant_id: row.get::<String, _>("tenant_id"),
                location_id: row.get::<String, _>("location_id"),
                sequence_number: row.get::<i32, _>("sequence_number"),
                estimated_arrival: "1970-01-01T00:00:00Z".to_string(),
                actual_arrival: "1970-01-01T00:00:00Z".to_string(),
                status: row.get::<String, _>("status"),
            }),
        }))
    }

    async fn update_courier_location(&self, request: Request<UpdateCourierLocationRequest>) -> Result<Response<UpdateCourierLocationResponse>, Status> {
        let tenant_id = self.extract_tenant_id(&request)?;
        let req = request.into_inner();

        let row = sqlx::query(
            r#"
            UPDATE courier_dispatches
            SET current_location_lat = $2, current_location_lng = $3
            WHERE id = $1 AND tenant_id = $4
            RETURNING id, tenant_id, courier_name, courier_phone, current_location_lat, current_location_lng, status
            "#
        )
        .bind(&req.id)
        .bind(&req.current_location_lat)
        .bind(&req.current_location_lng)
        .bind(&tenant_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        if let Some(row) = row {
            Ok(Response::new(UpdateCourierLocationResponse {
                courier: Some(CourierDispatch {
                    id: row.get::<String, _>("id"),
                    tenant_id: row.get::<String, _>("tenant_id"),
                    courier_name: row.get::<Option<String>, _>("courier_name").unwrap_or_default(),
                    courier_phone: row.get::<Option<String>, _>("courier_phone").unwrap_or_default(),
                    current_location_lat: row.get::<Option<f64>, _>("current_location_lat").unwrap_or_default(),
                    current_location_lng: row.get::<Option<f64>, _>("current_location_lng").unwrap_or_default(),
                    status: row.get::<String, _>("status"),
                }),
            }))
        } else {
            Err(Status::not_found("Courier dispatch not found"))
        }
    }

    async fn get_travel_padding(&self, _request: Request<GetTravelPaddingRequest>) -> Result<Response<GetTravelPaddingResponse>, Status> {
        // AI Logic goes here to calculate padding
        Ok(Response::new(GetTravelPaddingResponse {
            estimated_minutes: 15,
        }))
    }

    async fn handle_courier_webhook(&self, request: Request<HandleCourierWebhookRequest>) -> Result<Response<HandleCourierWebhookResponse>, Status> {
        let tenant_id = self.extract_tenant_id(&request)?;
        let req = request.into_inner();
        let event = req.event.ok_or_else(|| Status::invalid_argument("Event is required"))?;

        sqlx::query(
            r#"
            UPDATE courier_dispatches
            SET status = $2
            WHERE id = $1 AND tenant_id = $3
            "#
        )
        .bind(&event.courier_dispatch_id)
        .bind(&event.status)
        .bind(&tenant_id)
        .execute(&self.pool)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(HandleCourierWebhookResponse {
            success: true,
        }))
    }
}
