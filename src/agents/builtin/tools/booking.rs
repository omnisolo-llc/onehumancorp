use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use super::{Tool, ToolExecutor};
use crate::server::services::booking::{BookingService, Service, BookingRecord};
use chrono::{DateTime, Utc};

pub struct BookingGetServicesExecutor;

#[async_trait::async_trait]
impl ToolExecutor for BookingGetServicesExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let tenant_id = args["tenant_id"].as_str().ok_or_else(|| ToolError::LlmRecoverable("tenant_id is required".to_string()))?;
        let services = BookingService::list_services(tenant_id).await.map_err(|e| ToolError::Fatal(e))?;
        Ok(json!(services).to_string())
    }
}

pub fn booking_get_services_tool() -> Tool {
    Tool {
        name: "booking_get_services".to_string(),
        description: "List all available services for booking in the business.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "tenant_id": { "type": "string", "description": "The unique identifier of the business tenant." }
            },
            "required": ["tenant_id"]
        }),
        execute: Arc::new(BookingGetServicesExecutor),
    }
}

pub struct BookingUpsertServiceExecutor;

#[async_trait::async_trait]
impl ToolExecutor for BookingUpsertServiceExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let tenant_id = args["tenant_id"].as_str().ok_or_else(|| ToolError::LlmRecoverable("tenant_id is required".to_string()))?;
        let id = args["id"].as_str().map(|s| s.to_string()).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let title = args["title"].as_str().ok_or_else(|| ToolError::LlmRecoverable("title is required".to_string()))?;
        let description = args["description"].as_str().map(|s| s.to_string());
        let price_cents = args["price_cents"].as_i64().ok_or_else(|| ToolError::LlmRecoverable("price_cents is required".to_string()))?;

        let service = Service {
            id,
            tenant_id: tenant_id.to_string(),
            title: title.to_string(),
            description,
            price_cents,
        };

        BookingService::upsert_service(service).await.map_err(|e| ToolError::Fatal(e))?;
        Ok(json!({"status": "success", "message": "Service upserted successfully"}).to_string())
    }
}

pub fn booking_upsert_service_tool() -> Tool {
    Tool {
        name: "booking_upsert_service".to_string(),
        description: "Add or update a service that customers can book.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "tenant_id": { "type": "string" },
                "id": { "type": "string", "description": "Optional service ID. If omitted, a new service is created." },
                "title": { "type": "string" },
                "description": { "type": "string" },
                "price_cents": { "type": "integer" }
            },
            "required": ["tenant_id", "title", "price_cents"]
        }),
        execute: Arc::new(BookingUpsertServiceExecutor),
    }
}

pub struct BookingListAppointmentsExecutor;

#[async_trait::async_trait]
impl ToolExecutor for BookingListAppointmentsExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let tenant_id = args["tenant_id"].as_str().ok_or_else(|| ToolError::LlmRecoverable("tenant_id is required".to_string()))?;
        let bookings = BookingService::get_bookings(tenant_id).await.map_err(|e| ToolError::Fatal(e))?;
        Ok(json!(bookings).to_string())
    }
}

pub fn booking_list_appointments_tool() -> Tool {
    Tool {
        name: "booking_list_appointments".to_string(),
        description: "List all scheduled appointments for the business.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "tenant_id": { "type": "string" }
            },
            "required": ["tenant_id"]
        }),
        execute: Arc::new(BookingListAppointmentsExecutor),
    }
}

pub struct BookingCreateAppointmentExecutor;

#[async_trait::async_trait]
impl ToolExecutor for BookingCreateAppointmentExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let tenant_id = args["tenant_id"].as_str().ok_or_else(|| ToolError::LlmRecoverable("tenant_id is required".to_string()))?;
        let customer_id = args["customer_id"].as_str().ok_or_else(|| ToolError::LlmRecoverable("customer_id is required".to_string()))?;
        let service_id = args["service_id"].as_str().ok_or_else(|| ToolError::LlmRecoverable("service_id is required".to_string()))?;
        let start_time_str = args["start_time"].as_str().ok_or_else(|| ToolError::LlmRecoverable("start_time is required".to_string()))?;
        let end_time_str = args["end_time"].as_str();

        let start_time = DateTime::parse_from_rfc3339(start_time_str)
            .map_err(|e| ToolError::LlmRecoverable(format!("invalid start_time format: {}", e)))?
            .with_timezone(&Utc);

        let end_time = if let Some(et) = end_time_str {
            Some(DateTime::parse_from_rfc3339(et)
                .map_err(|e| ToolError::LlmRecoverable(format!("invalid end_time format: {}", e)))?
                .with_timezone(&Utc))
        } else {
            None
        };

        let booking = BookingRecord {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            customer_id: customer_id.to_string(),
            product_id: service_id.to_string(),
            start_time,
            end_time,
            status: "confirmed".to_string(),
        };

        BookingService::create_booking(booking).await.map_err(|e| ToolError::Fatal(e))?;
        Ok(json!({"status": "success", "message": "Appointment created successfully"}).to_string())
    }
}

pub fn booking_create_appointment_tool() -> Tool {
    Tool {
        name: "booking_create_appointment".to_string(),
        description: "Schedule a new appointment for a customer.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "tenant_id": { "type": "string" },
                "customer_id": { "type": "string" },
                "service_id": { "type": "string" },
                "start_time": { "type": "string", "description": "Start time in RFC3339 format." },
                "end_time": { "type": "string", "description": "Optional end time in RFC3339 format." }
            },
            "required": ["tenant_id", "customer_id", "service_id", "start_time"]
        }),
        execute: Arc::new(BookingCreateAppointmentExecutor),
    }
}
