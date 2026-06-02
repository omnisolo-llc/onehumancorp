use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::ohc::orchestration::Hub;
use ::server_ohc::app::booking_engine_service_client::BookingEngineServiceClient;
use ::server_ohc::app::{CheckAvailabilityRequest, ReserveTimeSlotRequest};

#[derive(Deserialize)]
pub struct AvailabilityQuery {
    tenant_id: String,
    product_id: String,
    date: String,
}

pub async fn get_availability(
    State(hub): State<Arc<Hub>>,
    Query(query): Query<AvailabilityQuery>,
) -> impl IntoResponse {
    // We would normally connect to the gRPC service, but here we can just use NativeBookingService
    let svc = crate::services::booking::NativeBookingService { redis_client: hub.redis_client.clone() };

    let req = tonic::Request::new(CheckAvailabilityRequest {
        tenant_id: query.tenant_id,
        product_id: query.product_id,
        date: query.date,
    });

    use ::server_ohc::app::booking_engine_service_server::BookingEngineService;

    match svc.check_availability(req).await {
        Ok(response) => {
            let res = response.into_inner();

            let json_res = serde_json::json!({
                "available_slots": res.available_slots.into_iter().map(|s| {
                    serde_json::json!({
                        "start_time": s.start_time,
                        "end_time": s.end_time
                    })
                }).collect::<Vec<_>>()
            });

            Json(json_res).into_response()
        },
        Err(e) => {
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.message().to_string()).into_response()
        }
    }
}

#[derive(Deserialize)]
pub struct ReserveRequest {
    tenant_id: String,
    customer_id: String,
    product_id: String,
    start_time: String,
    end_time: String,
}

pub async fn reserve_time_slot(
    State(hub): State<Arc<Hub>>,
    Json(payload): Json<ReserveRequest>,
) -> impl IntoResponse {
    let svc = crate::services::booking::NativeBookingService { redis_client: hub.redis_client.clone() };

    let req = tonic::Request::new(ReserveTimeSlotRequest {
        tenant_id: payload.tenant_id,
        customer_id: payload.customer_id,
        product_id: payload.product_id,
        start_time: payload.start_time,
        end_time: payload.end_time,
    });

    use ::server_ohc::app::booking_engine_service_server::BookingEngineService;

    match svc.reserve_time_slot(req).await {
        Ok(response) => {
            let res = response.into_inner();

            let json_res = serde_json::json!({
                "booking_id": res.booking_id,
                "deposit_stripe_link": res.deposit_stripe_link
            });

            Json(json_res).into_response()
        },
        Err(e) => {
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.message().to_string()).into_response()
        }
    }
}
