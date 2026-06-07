use tonic::{Request, Response, Status};
use sqlx::{PgPool, Row};
use uuid::Uuid;
use delivery_proto::ohc::api::v1::delivery_service_server::DeliveryService;
use delivery_proto::ohc::api::v1::{
    ConfigureDeliveryZoneRequest, ConfigureDeliveryZoneResponse, DeliveryTask, DeliveryZone,
    GetDailyItineraryRequest, GetDailyItineraryResponse, GetDeliveryZoneRequest,
    GetDeliveryZoneResponse, RoutePlan, UpdateDeliveryTaskStatusRequest,
    UpdateDeliveryTaskStatusResponse, CanDeliverToLocationRequest, CanDeliverToLocationResponse
};
use tracing::instrument;

pub struct DeliveryServiceImpl {
    pub pool: PgPool,
}

impl DeliveryServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl DeliveryService for DeliveryServiceImpl {
    #[instrument(skip(self))]
    async fn configure_delivery_zone(
        &self,
        request: Request<ConfigureDeliveryZoneRequest>,
    ) -> Result<Response<ConfigureDeliveryZoneResponse>, Status> {
        let req = request.into_inner();
        let org_id = req.organization_id;

        if org_id.is_empty() {
             return Err(Status::invalid_argument("organization_id is required"));
        }

        let id = Uuid::new_v4();

        let row = sqlx::query(
            r#"
            INSERT INTO delivery_zones (id, organization_id, polygon, flat_fee_cents, min_order_value_cents)
            VALUES ($1, $2, ST_GeomFromText($3, 4326), $4, $5)
            ON CONFLICT (organization_id) DO UPDATE SET
                polygon = ST_GeomFromText($3, 4326),
                flat_fee_cents = $4,
                min_order_value_cents = $5,
                updated_at = CURRENT_TIMESTAMP
            RETURNING id, organization_id, ST_AsText(polygon) as polygon_wkt, flat_fee_cents, min_order_value_cents,
                      EXTRACT(EPOCH FROM created_at)::BIGINT as created_at_unix,
                      EXTRACT(EPOCH FROM updated_at)::BIGINT as updated_at_unix
            "#,
        )
        .bind(id)
        .bind(&org_id)
        .bind(&req.polygon_wkt)
        .bind(req.flat_fee_cents)
        .bind(req.min_order_value_cents)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

        let zone = DeliveryZone {
            id: row.get::<Uuid, _>("id").to_string(),
            organization_id: row.get::<String, _>("organization_id"),
            polygon_wkt: row.get::<String, _>("polygon_wkt"),
            flat_fee_cents: row.get::<i64, _>("flat_fee_cents"),
            min_order_value_cents: row.get::<i64, _>("min_order_value_cents"),
            created_at_unix: row.get::<i64, _>("created_at_unix"),
            updated_at_unix: row.get::<i64, _>("updated_at_unix"),
        };

        Ok(Response::new(ConfigureDeliveryZoneResponse {
            zone: Some(zone),
        }))
    }

    #[instrument(skip(self))]
    async fn get_delivery_zone(
        &self,
        request: Request<GetDeliveryZoneRequest>,
    ) -> Result<Response<GetDeliveryZoneResponse>, Status> {
        let req = request.into_inner();
        let org_id = req.organization_id;

        if org_id.is_empty() {
            return Err(Status::invalid_argument("organization_id is required"));
        }

        let row = sqlx::query(
            r#"
            SELECT id, organization_id, ST_AsText(polygon) as polygon_wkt, flat_fee_cents, min_order_value_cents,
                   EXTRACT(EPOCH FROM created_at)::BIGINT as created_at_unix,
                   EXTRACT(EPOCH FROM updated_at)::BIGINT as updated_at_unix
            FROM delivery_zones
            WHERE organization_id = $1
            "#,
        )
        .bind(&org_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

        match row {
            Some(r) => {
                let zone = DeliveryZone {
                    id: r.get::<Uuid, _>("id").to_string(),
                    organization_id: r.get::<String, _>("organization_id"),
                    polygon_wkt: r.get::<String, _>("polygon_wkt"),
                    flat_fee_cents: r.get::<i64, _>("flat_fee_cents"),
                    min_order_value_cents: r.get::<i64, _>("min_order_value_cents"),
                    created_at_unix: r.get::<i64, _>("created_at_unix"),
                    updated_at_unix: r.get::<i64, _>("updated_at_unix"),
                };
                Ok(Response::new(GetDeliveryZoneResponse { zone: Some(zone) }))
            }
            None => Ok(Response::new(GetDeliveryZoneResponse { zone: None })),
        }
    }

    #[instrument(skip(self))]
    async fn get_daily_itinerary(
        &self,
        request: Request<GetDailyItineraryRequest>,
    ) -> Result<Response<GetDailyItineraryResponse>, Status> {
         let req = request.into_inner();
         let org_id = req.organization_id;

         if org_id.is_empty() || req.delivery_date.is_empty() {
             return Err(Status::invalid_argument("organization_id and delivery_date are required"));
         }

         let plan_row = sqlx::query(
             r#"
             SELECT id, organization_id, delivery_date::TEXT, waypoint_sequence::TEXT,
                    EXTRACT(EPOCH FROM created_at)::BIGINT as created_at_unix,
                    EXTRACT(EPOCH FROM updated_at)::BIGINT as updated_at_unix
             FROM route_plans
             WHERE organization_id = $1 AND delivery_date = $2::DATE
             "#
         )
         .bind(&org_id)
         .bind(&req.delivery_date)
         .fetch_optional(&self.pool)
         .await
         .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

         let mut route_plan = None;
         let mut tasks = Vec::new();

         if let Some(r) = plan_row {
             let r_plan = RoutePlan {
                 id: r.get::<Uuid, _>("id").to_string(),
                 organization_id: r.get::<String, _>("organization_id"),
                 delivery_date: r.get::<String, _>("delivery_date"),
                 waypoint_sequence_json: r.get::<String, _>("waypoint_sequence"),
                 created_at_unix: r.get::<i64, _>("created_at_unix"),
                 updated_at_unix: r.get::<i64, _>("updated_at_unix"),
             };
             route_plan = Some(r_plan.clone());

             let task_rows = sqlx::query(
                 r#"
                 SELECT id, organization_id, order_id, driver_id, route_plan_id, status,
                        EXTRACT(EPOCH FROM estimated_arrival)::BIGINT as estimated_arrival_unix,
                        delivery_location_lat as lat, delivery_location_lng as lng,
                        EXTRACT(EPOCH FROM created_at)::BIGINT as created_at_unix,
                        EXTRACT(EPOCH FROM updated_at)::BIGINT as updated_at_unix
                 FROM delivery_tasks
                 WHERE route_plan_id = $1
                 "#
             )
             .bind(Uuid::parse_str(&r_plan.id).unwrap())
             .fetch_all(&self.pool)
             .await
             .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

             for tr in task_rows {
                 tasks.push(DeliveryTask {
                     id: tr.get::<Uuid, _>("id").to_string(),
                     organization_id: tr.get::<String, _>("organization_id"),
                     order_id: tr.get::<String, _>("order_id"),
                     driver_id: tr.get::<Option<String>, _>("driver_id").unwrap_or_default(),
                     route_plan_id: tr.get::<Option<Uuid>, _>("route_plan_id").map(|u| u.to_string()).unwrap_or_default(),
                     status: tr.get::<String, _>("status"),
                     estimated_arrival_unix: tr.get::<Option<i64>, _>("estimated_arrival_unix").unwrap_or(0),
                     delivery_location_lat: tr.get::<Option<f64>, _>("lat").unwrap_or(0.0),
                     delivery_location_lng: tr.get::<Option<f64>, _>("lng").unwrap_or(0.0),
                     created_at_unix: tr.get::<i64, _>("created_at_unix"),
                     updated_at_unix: tr.get::<i64, _>("updated_at_unix"),
                 });
             }
         }

         Ok(Response::new(GetDailyItineraryResponse {
             route_plan,
             tasks,
         }))
    }

    #[instrument(skip(self))]
    async fn update_delivery_task_status(
        &self,
        request: Request<UpdateDeliveryTaskStatusRequest>,
    ) -> Result<Response<UpdateDeliveryTaskStatusResponse>, Status> {
         let req = request.into_inner();

         if req.organization_id.is_empty() || req.task_id.is_empty() || req.status.is_empty() {
             return Err(Status::invalid_argument("organization_id, task_id, and status are required"));
         }

         let task_uuid = Uuid::parse_str(&req.task_id).map_err(|_| Status::invalid_argument("Invalid task_id format"))?;

         let row = sqlx::query(
             r#"
             UPDATE delivery_tasks
             SET status = $1, updated_at = CURRENT_TIMESTAMP
             WHERE id = $2 AND organization_id = $3
             RETURNING id, organization_id, order_id, driver_id, route_plan_id, status,
                       EXTRACT(EPOCH FROM estimated_arrival)::BIGINT as estimated_arrival_unix,
                       delivery_location_lat as lat, delivery_location_lng as lng,
                       EXTRACT(EPOCH FROM created_at)::BIGINT as created_at_unix,
                       EXTRACT(EPOCH FROM updated_at)::BIGINT as updated_at_unix
             "#
         )
         .bind(&req.status)
         .bind(task_uuid)
         .bind(&req.organization_id)
         .fetch_one(&self.pool)
         .await
         .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

         let task = DeliveryTask {
             id: row.get::<Uuid, _>("id").to_string(),
             organization_id: row.get::<String, _>("organization_id"),
             order_id: row.get::<String, _>("order_id"),
             driver_id: row.get::<Option<String>, _>("driver_id").unwrap_or_default(),
             route_plan_id: row.get::<Option<Uuid>, _>("route_plan_id").map(|u| u.to_string()).unwrap_or_default(),
             status: row.get::<String, _>("status"),
             estimated_arrival_unix: row.get::<Option<i64>, _>("estimated_arrival_unix").unwrap_or(0),
             delivery_location_lat: row.get::<Option<f64>, _>("lat").unwrap_or(0.0),
             delivery_location_lng: row.get::<Option<f64>, _>("lng").unwrap_or(0.0),
             created_at_unix: row.get::<i64, _>("created_at_unix"),
             updated_at_unix: row.get::<i64, _>("updated_at_unix"),
         };

         Ok(Response::new(UpdateDeliveryTaskStatusResponse { task: Some(task) }))
    }

    #[instrument(skip(self))]
    async fn can_deliver_to_location(
        &self,
        request: Request<CanDeliverToLocationRequest>,
    ) -> Result<Response<CanDeliverToLocationResponse>, Status> {
         let req = request.into_inner();

         if req.organization_id.is_empty() {
             return Err(Status::invalid_argument("organization_id is required"));
         }

         let row = sqlx::query(
             r#"
             SELECT flat_fee_cents, ST_Contains(polygon, ST_SetSRID(ST_MakePoint($1, $2), 4326)) as can_deliver
             FROM delivery_zones
             WHERE organization_id = $3
             "#
         )
         .bind(req.lng)
         .bind(req.lat)
         .bind(&req.organization_id)
         .fetch_optional(&self.pool)
         .await
         .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

         match row {
             Some(r) => {
                 let can_deliver = r.get::<bool, _>("can_deliver");
                 let fee = if can_deliver { r.get::<i64, _>("flat_fee_cents") } else { 0 };
                 Ok(Response::new(CanDeliverToLocationResponse {
                     can_deliver,
                     flat_fee_cents: fee,
                 }))
             }
             None => Ok(Response::new(CanDeliverToLocationResponse {
                 can_deliver: false,
                 flat_fee_cents: 0,
             })),
         }
    }
}
