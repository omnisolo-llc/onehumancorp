use crate::ohc::orchestration::{McpInvokeRequest, McpInvokeResponse, McpToolProto};
use crate::services::booking::NativeBookingService;
use ::server_ohc::app::{CheckAvailabilityRequest, ReserveTimeSlotRequest};
use ::server_ohc::app::booking_engine_service_server::BookingEngineService;
use tonic::Request;
use serde_json::Value;

pub struct BookingMcpServer {
    booking_service: NativeBookingService,
}

impl BookingMcpServer {
    pub fn new(redis_client: Option<redis::Client>) -> Self {
        Self {
            booking_service: NativeBookingService { redis_client },
        }
    }

    pub fn get_tools(&self) -> Vec<McpToolProto> {
        vec![
            McpToolProto {
                id: "check_availability".to_string(),
                name: "Check Availability".to_string(),
                description: "Check available time slots for a specific product/service on a given date. Input schema: {\"type\":\"object\",\"properties\":{\"product_id\":{\"type\":\"string\"},\"date\":{\"type\":\"string\",\"description\":\"YYYY-MM-DD\"}},\"required\":[\"product_id\",\"date\"]}".to_string(),
                category: "booking".to_string(),
                status: "active".to_string(),
            },
            McpToolProto {
                id: "create_appointment".to_string(),
                name: "Create Appointment".to_string(),
                description: "Reserve a time slot and create an appointment. Input schema: {\"type\":\"object\",\"properties\":{\"customer_id\":{\"type\":\"string\"},\"product_id\":{\"type\":\"string\"},\"start_time\":{\"type\":\"string\",\"description\":\"RFC3339\"},\"end_time\":{\"type\":\"string\",\"description\":\"RFC3339\"},\"requires_deposit\":{\"type\":\"boolean\"},\"timezone\":{\"type\":\"string\"}},\"required\":[\"customer_id\",\"product_id\",\"start_time\",\"end_time\",\"requires_deposit\",\"timezone\"]}".to_string(),
                category: "booking".to_string(),
                status: "active".to_string(),
            },
        ]
    }

    pub async fn invoke_tool(
        &self,
        req: &McpInvokeRequest,
        auth_info: ::server_auth::orchestration::AuthInfo,
    ) -> Result<McpInvokeResponse, tonic::Status> {
        match req.tool_id.as_str() {
            "check_availability" => {
                let payload: Value = serde_json::from_str(&req.params)
                    .map_err(|e| tonic::Status::invalid_argument(e.to_string()))?;

                let product_id = payload.get("product_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let date = payload.get("date").and_then(|v| v.as_str()).unwrap_or("").to_string();

                let mut r = Request::new(CheckAvailabilityRequest {
                    tenant_id: auth_info.org_id.clone(),
                    product_id,
                    date,
                });
                r.extensions_mut().insert(auth_info);

                let res = self.booking_service.check_availability(r).await?.into_inner();
                Ok(McpInvokeResponse {
                    payload: serde_json::to_string(&res).unwrap_or_default(),
                })
            }
            "create_appointment" => {
                let payload: Value = serde_json::from_str(&req.params)
                    .map_err(|e| tonic::Status::invalid_argument(e.to_string()))?;

                let customer_id = payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let product_id = payload.get("product_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let start_time = payload.get("start_time").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let end_time = payload.get("end_time").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let requires_deposit = payload.get("requires_deposit").and_then(|v| v.as_bool()).unwrap_or(false);
                let timezone = payload.get("timezone").and_then(|v| v.as_str()).unwrap_or("UTC").to_string();

                let mut r = Request::new(ReserveTimeSlotRequest {
                    tenant_id: auth_info.org_id.clone(),
                    customer_id,
                    product_id,
                    start_time,
                    end_time,
                    requires_deposit,
                    timezone,
                });
                r.extensions_mut().insert(auth_info);

                let res = self.booking_service.reserve_time_slot(r).await?.into_inner();
                Ok(McpInvokeResponse {
                    payload: serde_json::to_string(&res).unwrap_or_default(),
                })
            }
            _ => Err(tonic::Status::not_found(format!("tool {} not implemented", req.tool_id))),
        }
    }
}
