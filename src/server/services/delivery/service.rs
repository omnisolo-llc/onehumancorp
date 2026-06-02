use tonic::{Request, Response, Status};
use ::server_ohc::delivery::*;
use ::server_ohc::delivery::delivery_service_server::DeliveryService;
use std::sync::Arc;
use crate::hub::Hub;
use crate::db::DbStore;
use sqlx::Row;

pub struct MyDeliveryService {
    hub: Arc<Hub>,
}

impl MyDeliveryService {
    pub fn new(hub: Arc<Hub>) -> Self {
        MyDeliveryService { hub }
    }
}

#[tonic::async_trait]
impl DeliveryService for MyDeliveryService {
    async fn configure_delivery_zone(
        &self,
        request: Request<ConfigureDeliveryZoneRequest>,
    ) -> Result<Response<ConfigureDeliveryZoneResponse>, Status> {
        let req = request.into_inner();
        let mut zone = req.zone.ok_or_else(|| Status::invalid_argument("zone is required"))?;
        let org_id = crate::common::auth_utils::get_org_id_from_context().await.unwrap_or_else(|_| zone.organization_id.clone());
        if org_id.is_empty() {
            return Err(Status::unauthenticated("Organization ID missing"));
        }

        if zone.id.is_empty() {
            zone.id = format!("zone_{}", uuid::Uuid::new_v4());
        }
        zone.organization_id = org_id.clone();

        let pool = self.hub.db.pool.clone();

        sqlx::query(
            "INSERT INTO delivery_zones (id, organization_id, polygon, flat_fee, min_order_value)
             VALUES ($1, $2, ST_GeomFromGeoJSON($3), $4, $5)
             ON CONFLICT (id) DO UPDATE SET polygon = ST_GeomFromGeoJSON($3), flat_fee = $4, min_order_value = $5"
        )
        .bind(&zone.id)
        .bind(&zone.organization_id)
        .bind(&zone.polygon_geojson)
        .bind(&zone.flat_fee)
        .bind(&zone.min_order_value)
        .execute(&pool)
        .await
        .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

        Ok(Response::new(ConfigureDeliveryZoneResponse { zone: Some(zone) }))
    }

    async fn get_delivery_zone(
        &self,
        request: Request<GetDeliveryZoneRequest>,
    ) -> Result<Response<GetDeliveryZoneResponse>, Status> {
        let req = request.into_inner();
        let org_id = crate::common::auth_utils::get_org_id_from_context().await.unwrap_or_else(|_| req.organization_id.clone());
        let pool = self.hub.db.pool.clone();

        let row = sqlx::query(
            "SELECT id, organization_id, ST_AsGeoJSON(polygon) as polygon_geojson, flat_fee, min_order_value
             FROM delivery_zones WHERE organization_id = $1 LIMIT 1"
        )
        .bind(org_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

        if let Some(r) = row {
            // let zone = DeliveryZone {
            //     id: r.id,
            //     organization_id: r.organization_id,
            //     polygon_geojson: r.polygon_geojson.unwrap_or_default(),
            //     flat_fee: r.flat_fee.to_string().parse::<f64>().unwrap_or(0.0),
            //     min_order_value: r.min_order_value.to_string().parse::<f64>().unwrap_or(0.0),
            // };
            // Need to retrieve dynamically
            let z_id: String = r.get("id");
            let z_org: String = r.get("organization_id");
            let p_geo: Option<String> = r.try_get("polygon_geojson").unwrap_or_default();
            let flat: rust_decimal::Decimal = r.get("flat_fee");
            let mov: rust_decimal::Decimal = r.get("min_order_value");
            use rust_decimal::prelude::ToPrimitive;

            let zone = DeliveryZone {
                id: z_id,
                organization_id: z_org,
                polygon_geojson: p_geo.unwrap_or_default(),
                flat_fee: flat.to_f64().unwrap_or(0.0),
                min_order_value: mov.to_f64().unwrap_or(0.0),
            };

            Ok(Response::new(GetDeliveryZoneResponse { zone: Some(zone) }))
        } else {
            Err(Status::not_found("Delivery zone not found"))
        }
    }

    async fn fetch_daily_itinerary(
        &self,
        request: Request<FetchDailyItineraryRequest>,
    ) -> Result<Response<FetchDailyItineraryResponse>, Status> {
        let req = request.into_inner();
        let org_id = crate::common::auth_utils::get_org_id_from_context().await.unwrap_or_else(|_| req.organization_id.clone());

        // Ensure a basic Route Plan exists for the day
        let pool = self.hub.db.pool.clone();

        let date = chrono::NaiveDate::parse_from_str(&req.delivery_date, "%Y-%m-%d")
            .unwrap_or_else(|_| chrono::Utc::now().naive_utc().date());

        let row = sqlx::query(
            "SELECT id, organization_id, delivery_date::text, waypoint_sequence::text as seq
             FROM route_plans
             WHERE organization_id = $1 AND delivery_date = $2 LIMIT 1"
        )
        .bind(&org_id)
        .bind(date)
        .fetch_optional(&pool)
        .await
        .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

        let route_plan = if let Some(r) = row {
            RoutePlan {
                id: r.get("id"),
                organization_id: r.get("organization_id"),
                delivery_date: r.try_get("delivery_date").unwrap_or_default(),
                waypoint_sequence_json: r.try_get("seq").unwrap_or_else(|_| "[]".to_string()),
            }
        } else {
            // Auto-create a route plan and generate sequence
            let new_id = format!("route_{}", uuid::Uuid::new_v4());
            sqlx::query(
                "INSERT INTO route_plans (id, organization_id, delivery_date) VALUES ($1, $2, $3)"
            )
            .bind(&new_id)
            .bind(&org_id)
            .bind(date)
            .execute(&pool).await.map_err(|e| Status::internal(format!("Database error: {}", e)))?;

            RoutePlan {
                id: new_id,
                organization_id: org_id.clone(),
                delivery_date: req.delivery_date,
                waypoint_sequence_json: "[]".to_string(),
            }
        };

        // Fetch Tasks (basic order by id for the naive heuristic)
        let tasks_rows = sqlx::query(
            "SELECT id, organization_id, order_id, driver_id, route_plan_id, status,
                    extract(epoch from estimated_arrival)::bigint as arrival_unix,
                    ST_X(delivery_location) as lng, ST_Y(delivery_location) as lat
             FROM delivery_tasks
             WHERE route_plan_id = $1 OR (organization_id = $2 AND status = 'PENDING')"
        )
        .bind(&route_plan.id)
        .bind(&org_id)
        .fetch_all(&pool).await.map_err(|e| Status::internal(format!("Database error: {}", e)))?;

        let mut tasks = Vec::new();
        for r in tasks_rows {
            tasks.push(DeliveryTask {
                id: r.get("id"),
                organization_id: r.get("organization_id"),
                order_id: r.get("order_id"),
                driver_id: r.try_get("driver_id").unwrap_or_default(),
                route_plan_id: r.try_get("route_plan_id").unwrap_or_default(),
                status: r.get("status"),
                estimated_arrival_unix: r.try_get("arrival_unix").unwrap_or_default(),
                delivery_location: Some(Coordinate {
                    lat: r.try_get("lat").unwrap_or_default(),
                    lng: r.try_get("lng").unwrap_or_default()
                }),
                address_text: String::new(),
            });
        }

        Ok(Response::new(FetchDailyItineraryResponse {
            route_plan: Some(route_plan),
            tasks,
        }))
    }

    async fn update_task_status(
        &self,
        request: Request<UpdateTaskStatusRequest>,
    ) -> Result<Response<UpdateTaskStatusResponse>, Status> {
        let req = request.into_inner();
        let pool = self.hub.db.pool.clone();

        let r = sqlx::query(
            "UPDATE delivery_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2
             RETURNING id, organization_id, order_id, driver_id, route_plan_id, status,
                    extract(epoch from estimated_arrival)::bigint as arrival_unix,
                    ST_X(delivery_location) as lng, ST_Y(delivery_location) as lat"
        )
        .bind(req.status)
        .bind(req.task_id)
        .fetch_optional(&pool).await.map_err(|e| Status::internal(format!("Database error: {}", e)))?;

        if let Some(r) = r {
            let task = DeliveryTask {
                id: r.get("id"),
                organization_id: r.get("organization_id"),
                order_id: r.get("order_id"),
                driver_id: r.try_get("driver_id").unwrap_or_default(),
                route_plan_id: r.try_get("route_plan_id").unwrap_or_default(),
                status: r.get("status"),
                estimated_arrival_unix: r.try_get("arrival_unix").unwrap_or_default(),
                delivery_location: Some(Coordinate {
                    lat: r.try_get("lat").unwrap_or_default(),
                    lng: r.try_get("lng").unwrap_or_default()
                }),
                address_text: String::new(),
            };
            Ok(Response::new(UpdateTaskStatusResponse { task: Some(task) }))
        } else {
            Err(Status::not_found("Task not found"))
        }
    }

    async fn start_delivery_route(
        &self,
        _request: Request<StartDeliveryRouteRequest>,
    ) -> Result<Response<StartDeliveryRouteResponse>, Status> {
        Err(Status::unimplemented("Not implemented"))
    }
}
