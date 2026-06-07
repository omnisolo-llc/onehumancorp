use axum::{
    extract::{State, Json},
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::DB;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::DepartmentEvent;
use crate::services::booking::NativeBookingService;
use ::server_ohc::app::CreateConversationalCheckoutRequest;
use ::server_ohc::app::booking_engine_service_server::BookingEngineService;
use tonic::Request;
use uuid::Uuid;
use ohc_builtin_agent::mesh::transport::MeshTransport;

#[derive(Clone)]
pub struct BookingApiState {
    pub orchestrator: Arc<DepartmentOrchestrator>,
    pub db: Arc<DB>,
    pub redis_client: Option<redis::Client>,
}

#[derive(Deserialize, Debug)]
pub struct BookingRequestPayload {
    pub description: String,
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct BookingRequestResponse {
    pub success: bool,
    pub request_id: String,
    pub status: String,
}

pub async fn booking_request_handler(
    State(state): State<BookingApiState>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<BookingRequestPayload>,
) -> impl IntoResponse {
    let tenant_id = headers
        .get("x-tenant-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("default")
        .to_string();

    let request_id = format!("req_real_{}", Uuid::new_v4().simple().to_string().chars().take(8).collect::<String>());

    let event_payload = serde_json::json!({
        "feature_type": "quote_draft",
        "customer_inquiry": payload.description,
        "suggested_price": "150",
        "scope": format!("Service inquiry: {}", payload.description),
        "suggested_time": "Tue 2 PM",
        "source": "booking_request",
        "file_name": payload.file_name,
        "request_id": request_id,
    });

    let event = DepartmentEvent {
        id: Uuid::new_v4().to_string(),
        tenant_id: tenant_id.clone(),
        event_type: "tenant.message.received".to_string(),
        payload: event_payload,
    };

    if let Err(e) = state.orchestrator.dispatch_event(event).await {
        tracing::error!("Failed to trigger orchestrator for booking request: {}", e);
        return (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(serde_json::json!({ "error": "Internal server error" })),
        )
            .into_response();
    }

    (
        axum::http::StatusCode::OK,
        axum::Json(BookingRequestResponse {
            success: true,
            request_id,
            status: "pending_agent_review".to_string(),
        }),
    )
        .into_response()
}

pub async fn conversational_checkout_handler(
    State(state): State<BookingApiState>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<CreateConversationalCheckoutRequest>,
) -> impl IntoResponse {
    let mut req = Request::new(payload);

    // Inject auth extensions normally present via gRPC interceptors
    let spiffe_id = headers
        .get("x-spiffe-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("spiffe://ohc.local/ns/default/sa/test")
        .to_string();

    let tenant_id = headers
        .get("x-tenant-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("default")
        .to_string();

    req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
        spiffe_id: spiffe_id.clone(),
        org_id: tenant_id.clone(),
        agent_id: "api_client".to_string(),
    });

    let booking_service = NativeBookingService {
        redis_client: state.redis_client.clone(),
    };

    match booking_service.create_conversational_checkout(req).await {
        Ok(res) => (
            axum::http::StatusCode::OK,
            axum::Json(res.into_inner()),
        ).into_response(),
        Err(status) => {
            tracing::error!("Failed to create conversational checkout: {:?}", status);
            let http_status = match status.code() {
                tonic::Code::InvalidArgument => axum::http::StatusCode::BAD_REQUEST,
                tonic::Code::Unauthenticated => axum::http::StatusCode::UNAUTHORIZED,
                tonic::Code::ResourceExhausted => axum::http::StatusCode::CONFLICT,
                _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            };
            (
                http_status,
                axum::Json(serde_json::json!({ "error": status.message() })),
            ).into_response()
        }
    }
}

pub fn router<S: Clone + Send + Sync + 'static>(orchestrator: Arc<DepartmentOrchestrator>, db: Arc<DB>, redis_client: Option<redis::Client>) -> Router<S> {
    let state = BookingApiState {
        orchestrator,
        db,
        redis_client,
    };

    Router::new()
        .route("/request", post(booking_request_handler))
        .route("/conversational_checkout", post(conversational_checkout_handler))
        .with_state(state)
}
