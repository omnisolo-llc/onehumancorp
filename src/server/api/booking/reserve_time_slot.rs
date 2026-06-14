use axum::{
    extract::{Extension, Json},
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ReserveTimeSlotPayload {
    pub tenant_id: String,
    pub product_id: String,
    pub customer_id: String,
    pub start_time: String,
    pub end_time: String,
    #[serde(default)]
    pub requires_deposit: bool,
    pub timezone: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct ReserveTimeSlotResponse {
    pub success: bool,
    pub booking_id: Option<String>,
    pub deposit_stripe_link: Option<String>,
}

pub async fn handle_reserve_time_slot(
    headers: axum::http::HeaderMap,
    user: Option<Extension<::server_common::Claims>>,
    Json(payload): Json<ReserveTimeSlotPayload>,
) -> impl IntoResponse {
    let tenant_id = match headers.get("x-tenant-id").and_then(|h| h.to_str().ok()) {
        Some(t) if !t.trim().is_empty() => t.to_string(),
        _ => payload.tenant_id.clone(),
    };
    if tenant_id.is_empty() {
        return (axum::http::StatusCode::UNAUTHORIZED, axum::Json(serde_json::json!({"error": "unauthorized"}))).into_response();
    }

    let port = std::env::var("PORT").unwrap_or_else(|_| "18789".to_string());
    let url = format!("http://127.0.0.1:{}", port);

    let mut client = match ::server_ohc::app::booking_engine_service_client::BookingEngineServiceClient::connect(url).await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to connect to BookingEngineService: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "backend connection failed"}))).into_response();
        }
    };

    let req = ::server_ohc::app::ReserveTimeSlotRequest {
        tenant_id: tenant_id.clone(),
        customer_id: payload.customer_id,
        product_id: payload.product_id,
        start_time: payload.start_time,
        end_time: payload.end_time,
        requires_deposit: payload.requires_deposit,
        timezone: payload.timezone.unwrap_or_else(|| "UTC".to_string()),
    };

    let mut tonic_req = tonic::Request::new(req);

    let spiffe_id = if let Some(Extension(claims)) = user {
        claims.sub
    } else {
        "".to_string()
    };

    if !spiffe_id.is_empty() {
        if let Ok(m) = spiffe_id.parse() {
            tonic_req.metadata_mut().insert("x-spiffe-id", m);
        }
    }

    tonic_req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
        org_id: tenant_id,
        agent_id: spiffe_id.clone(),
        spiffe_id: spiffe_id,
    });

    match client.reserve_time_slot(tonic_req).await {
        Ok(resp) => {
            let data = resp.into_inner();
            (
                StatusCode::OK,
                Json(ReserveTimeSlotResponse {
                    success: true,
                    booking_id: Some(data.booking_id),
                    deposit_stripe_link: Some(data.deposit_stripe_link),
                }),
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!("Failed to reserve time slot: {:?}", e);
            (
                StatusCode::BAD_REQUEST,
                Json(ReserveTimeSlotResponse {
                    success: false,
                    booking_id: None,
                    deposit_stripe_link: None,
                }),
            )
                .into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;

    #[tokio::test]
    async fn test_handle_reserve_time_slot_unauthorized() {
        let headers = HeaderMap::new();
        let payload = Json(ReserveTimeSlotPayload {
            tenant_id: "".to_string(),
            product_id: "".to_string(),
            customer_id: "".to_string(),
            start_time: "".to_string(),
            end_time: "".to_string(),
            requires_deposit: false,
            timezone: None,
            description: None,
        });

        let resp = handle_reserve_time_slot(headers, None, payload).await.into_response();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
}
