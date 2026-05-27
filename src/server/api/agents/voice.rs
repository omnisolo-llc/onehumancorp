use axum::{
    extract::{State, Json, Query},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::services::booking::{BookingRecord, BookingService};
use ::server_common::Claims;
use axum::extract::Extension;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct VoiceToggleRequest {
    pub is_on: bool,
    pub language: Option<String>,
    pub persona: Option<String>,
    pub enable_bookings: Option<bool>,
    pub enable_quotes: Option<bool>,
    pub enable_faqs: Option<bool>,
}

#[derive(Serialize)]
pub struct VoiceToggleResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct VoiceWebhookPayload {
    pub message: VoiceMessage,
}

#[derive(Deserialize, Debug)]
pub struct VoiceMessage {
    pub toolCalls: Option<Vec<ToolCall>>,
}

#[derive(Deserialize, Debug)]
pub struct ToolCall {
    pub id: String,
    pub function: FunctionCall,
}

#[derive(Deserialize, Debug)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String, // JSON string
}

#[derive(Serialize)]
pub struct VoiceWebhookResponse {
    pub results: Vec<ToolResult>,
}

#[derive(Serialize)]
pub struct ToolResult {
    pub toolCallId: String,
    pub result: String,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/toggle", post(handle_toggle))
        .route("/webhook", post(handle_webhook))
        .with_state(orchestrator)
}

async fn handle_toggle(
    Extension(claims): Extension<Claims>,
    Json(payload): Json<VoiceToggleRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(VoiceToggleResponse { success: false, message: "Unauthorized".to_string() })).into_response(),
    };

    tracing::info!("Received toggle request for tenant {}: {:?}", tenant_id, payload);

    if payload.is_on {
        tracing::info!("Provisioning phone number for tenant {}...", tenant_id);
    }

    (StatusCode::OK, Json(VoiceToggleResponse { success: true, message: "Settings updated".to_string() })).into_response()
}

async fn handle_webhook(
    State(_orchestrator): State<Arc<DepartmentOrchestrator>>,
    Query(params): Query<HashMap<String, String>>,
    Json(payload): Json<VoiceWebhookPayload>,
) -> impl IntoResponse {
    let tenant_id = params.get("tenant_id").cloned().unwrap_or_else(|| "default-tenant-id".to_string());

    let mut results = Vec::new();

    if let Some(tool_calls) = payload.message.toolCalls {
        for call in tool_calls {
            let result_str = match call.function.name.as_str() {
                "check_availability" => {
                    #[derive(Deserialize)]
                    struct CheckArgs { start_time: DateTime<Utc>, end_time: DateTime<Utc> }
                    if let Ok(args) = serde_json::from_str::<CheckArgs>(&call.function.arguments) {
                        match BookingService::check_availability(&tenant_id, args.start_time, args.end_time).await {
                            Ok(true) => "Available".to_string(),
                            Ok(false) => "Unavailable".to_string(),
                            Err(_) => "Error checking availability".to_string(),
                        }
                    } else {
                        "Invalid arguments".to_string()
                    }
                },
                "book_appointment" => {
                    #[derive(Deserialize)]
                    struct BookArgs { start_time: DateTime<Utc>, end_time: DateTime<Utc>, customer_id: String, product_id: String }
                    if let Ok(args) = serde_json::from_str::<BookArgs>(&call.function.arguments) {
                        let record = BookingRecord {
                            id: Uuid::new_v4().to_string(),
                            tenant_id: tenant_id.clone(),
                            customer_id: args.customer_id,
                            product_id: args.product_id,
                            start_time: args.start_time,
                            end_time: Some(args.end_time),
                            status: "confirmed".to_string(),
                        };
                        match BookingService::create_booking(record).await {
                            Ok(_) => {
                                let _ = crate::dispatch_critical_sms("new_order", "New Booking via Voice").await;
                                "Successfully booked".to_string()
                            },
                            Err(_) => "Error creating booking".to_string(),
                        }
                    } else {
                        "Invalid arguments".to_string()
                    }
                },
                "send_sms_payment_link" => {
                    let _ = crate::dispatch_critical_sms("new_order", "Here is your payment link: https://pay.ohc.com/123").await;
                    "SMS sent".to_string()
                },
                _ => "Unknown function".to_string(),
            };

            results.push(ToolResult {
                toolCallId: call.id,
                result: result_str,
            });
        }
    }

    (StatusCode::OK, Json(VoiceWebhookResponse { results })).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;
    use tower::ServiceExt;
    use axum::body::Body;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_voice_webhook_unknown_function() {
        let orchestrator = Arc::new(DepartmentOrchestrator::new(Arc::new(crate::msgbus::MemoryBus::new()), crate::db::get_pool(), "node1".to_string()));
        let app = router(orchestrator);

        let payload = serde_json::json!({
            "message": {
                "toolCalls": [
                    {
                        "id": "call_123",
                        "function": {
                            "name": "unknown_function",
                            "arguments": "{}"
                        }
                    }
                ]
            }
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/webhook?tenant_id=node1")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let resp: VoiceWebhookResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(resp.results.len(), 1);
        assert_eq!(resp.results[0].result, "Unknown function");
    }
}
