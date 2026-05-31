use tonic::{Request, Response, Status};
use std::sync::Arc;
use crate::hub::Hub;
use ::server_ohc::delivery::*;
use ::server_ohc::delivery::delivery_service_server::DeliveryService;
use chrono::Utc;
use uuid::Uuid;

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
    async fn create_dispatch_session(
        &self,
        request: Request<CreateDispatchRequest>,
    ) -> Result<Response<CreateDispatchResponse>, Status> {
        let spiffe_id_str = ::server_auth::extract_spiffe_id_from_metadata(request.metadata())
            .map_err(|e| Status::unauthenticated(e))?;
        let (tenant_id, _) = ::server_auth::parse_spiffe_id(&spiffe_id_str)?;
        let org_id = if tenant_id.is_empty() { "system".to_string() } else { tenant_id };

        let req = request.into_inner();
        let session_id = Uuid::new_v4().to_string();
        let now = Utc::now().timestamp();

        let pool = &self.hub.pool;

        sqlx::query(
            "INSERT INTO dispatch_sessions (id, organization_id, order_id, status, promised_time, delivery_fee_cents, delivery_address)
             VALUES ($1, $2, $3, $4, to_timestamp($5), $6, $7)"
        )
        .bind(&session_id)
        .bind(&org_id)
        .bind(&req.order_id)
        .bind("PENDING")
        .bind(req.promised_time_unix)
        .bind(req.delivery_fee_cents)
        .bind(&req.delivery_address)
        .execute(pool)
        .await
        .map_err(|e| Status::internal(format!("Failed to create dispatch session: {}", e)))?;

        let session = DispatchSession {
            id: session_id.clone(),
            organization_id: org_id.clone(),
            order_id: req.order_id,
            status: "PENDING".to_string(),
            active_courier_id: "".to_string(),
            promised_time_unix: req.promised_time_unix,
            delivery_fee_cents: req.delivery_fee_cents,
            delivery_address: req.delivery_address,
            created_at_unix: now,
            updated_at_unix: now,
        };

        Ok(Response::new(CreateDispatchResponse {
            session: Some(session),
        }))
    }

    async fn accept_dispatch(
        &self,
        request: Request<AcceptDispatchRequest>,
    ) -> Result<Response<AcceptDispatchResponse>, Status> {
        let spiffe_id_str = ::server_auth::extract_spiffe_id_from_metadata(request.metadata())
            .map_err(|e| Status::unauthenticated(e))?;
        let (tenant_id, _) = ::server_auth::parse_spiffe_id(&spiffe_id_str)?;
        let org_id = if tenant_id.is_empty() { "system".to_string() } else { tenant_id };

        let req = request.into_inner();
        let pool = &self.hub.pool;

        let res = sqlx::query(
            "UPDATE dispatch_sessions SET status = 'ACCEPTED', active_courier_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND organization_id = $3 AND status = 'PENDING' RETURNING *"
        )
        .bind(&req.courier_id)
        .bind(&req.dispatch_session_id)
        .bind(&org_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| Status::internal(format!("Failed to accept dispatch session: {}", e)))?;

        if let Some(row) = res {
            use sqlx::Row;
            let session = DispatchSession {
                id: req.dispatch_session_id,
                organization_id: org_id,
                order_id: row.get("order_id"),
                status: row.get("status"),
                active_courier_id: row.get("active_courier_id"),
                promised_time_unix: row.try_get::<chrono::DateTime<Utc>, _>("promised_time").map(|dt| dt.timestamp()).unwrap_or(0),
                delivery_fee_cents: row.try_get::<i32, _>("delivery_fee_cents").unwrap_or(0) as i64,
                delivery_address: row.get("delivery_address"),
                created_at_unix: row.try_get::<chrono::DateTime<Utc>, _>("created_at").map(|dt| dt.timestamp()).unwrap_or(0),
                updated_at_unix: row.try_get::<chrono::DateTime<Utc>, _>("updated_at").map(|dt| dt.timestamp()).unwrap_or(0),
            };

            Ok(Response::new(AcceptDispatchResponse {
                session: Some(session),
            }))
        } else {
            Err(Status::not_found("Dispatch session not found or not in PENDING state"))
        }
    }

    async fn update_location(
        &self,
        request: Request<UpdateLocationRequest>,
    ) -> Result<Response<UpdateLocationResponse>, Status> {
        let spiffe_id_str = ::server_auth::extract_spiffe_id_from_metadata(request.metadata())
            .map_err(|e| Status::unauthenticated(e))?;
        let (tenant_id, _) = ::server_auth::parse_spiffe_id(&spiffe_id_str)?;
        let org_id = if tenant_id.is_empty() { "system".to_string() } else { tenant_id };

        let req = request.into_inner();
        let pool = &self.hub.pool;
        let update_id = Uuid::new_v4().to_string();

        sqlx::query(
            "INSERT INTO location_updates (id, organization_id, dispatch_session_id, courier_id, lat, lng)
             VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(&update_id)
        .bind(&org_id)
        .bind(&req.dispatch_session_id)
        .bind(&req.courier_id)
        .bind(req.lat)
        .bind(req.lng)
        .execute(pool)
        .await
        .map_err(|e| Status::internal(format!("Failed to update location: {}", e)))?;

        // In a real application, we would stream this update to a NATS mesh
        // for real-time mobile tracking as described in the architecture.

        Ok(Response::new(UpdateLocationResponse {
            success: true,
        }))
    }

    async fn get_dispatch_session(
        &self,
        request: Request<GetDispatchRequest>,
    ) -> Result<Response<GetDispatchResponse>, Status> {
        let spiffe_id_str = ::server_auth::extract_spiffe_id_from_metadata(request.metadata())
            .map_err(|e| Status::unauthenticated(e))?;
        let (tenant_id, _) = ::server_auth::parse_spiffe_id(&spiffe_id_str)?;
        let org_id = if tenant_id.is_empty() { "system".to_string() } else { tenant_id };

        let req = request.into_inner();
        let pool = &self.hub.pool;

        let session_row = sqlx::query(
            "SELECT * FROM dispatch_sessions WHERE id = $1 AND organization_id = $2"
        )
        .bind(&req.dispatch_session_id)
        .bind(&org_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

        if let Some(row) = session_row {
            use sqlx::Row;
            let active_courier_id: Option<String> = row.try_get("active_courier_id").ok();

            let mut courier_proto: Option<Courier> = None;
            if let Some(cid) = active_courier_id.clone() {
                let courier_row = sqlx::query(
                    "SELECT * FROM couriers WHERE id = $1 AND organization_id = $2"
                )
                .bind(&cid)
                .bind(&org_id)
                .fetch_optional(pool)
                .await
                .map_err(|e| Status::internal(format!("Database error fetching courier: {}", e)))?;

                if let Some(crow) = courier_row {
                    courier_proto = Some(Courier {
                        id: cid,
                        type_: crow.get("type"),
                        name: crow.get("name"),
                        phone: crow.get("phone"),
                        vehicle_type: crow.try_get("vehicle_type").unwrap_or_default(),
                        status: crow.get("status"),
                    });
                }
            }

            let session = DispatchSession {
                id: req.dispatch_session_id,
                organization_id: org_id,
                order_id: row.get("order_id"),
                status: row.get("status"),
                active_courier_id: active_courier_id.unwrap_or_default(),
                promised_time_unix: row.try_get::<chrono::DateTime<Utc>, _>("promised_time").map(|dt| dt.timestamp()).unwrap_or(0),
                delivery_fee_cents: row.try_get::<i32, _>("delivery_fee_cents").unwrap_or(0) as i64,
                delivery_address: row.get("delivery_address"),
                created_at_unix: row.try_get::<chrono::DateTime<Utc>, _>("created_at").map(|dt| dt.timestamp()).unwrap_or(0),
                updated_at_unix: row.try_get::<chrono::DateTime<Utc>, _>("updated_at").map(|dt| dt.timestamp()).unwrap_or(0),
            };

            Ok(Response::new(GetDispatchResponse {
                session: Some(session),
                courier: courier_proto,
            }))
        } else {
            Err(Status::not_found("Dispatch session not found"))
        }
    }
}
