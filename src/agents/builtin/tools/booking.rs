use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;
use super::{Tool, ToolExecutor};
use chrono::{DateTime, Utc};

// Types defined here to avoid circular dependency with server_lib
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Service {
    pub id: String,
    pub tenant_id: String,
    pub title: String,
    pub description: Option<String>,
    pub price_cents: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BookingRecord {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: String,
    pub product_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub status: String,
}

#[derive(Default)]
pub struct BookingStore {
    pub services: Vec<Service>,
    pub bookings: Vec<BookingRecord>,
}

pub type SharedBookingStore = Arc<RwLock<BookingStore>>;

pub struct BookingGetServicesExecutor {
    pub store: SharedBookingStore,
}

#[async_trait::async_trait]
impl ToolExecutor for BookingGetServicesExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let tenant_id = args["tenant_id"].as_str().ok_or_else(|| ToolError::LlmRecoverable("tenant_id is required".to_string()))?;
        let store = self.store.read().await;
        let services: Vec<_> = store.services.iter().filter(|s| s.tenant_id == tenant_id).cloned().collect();
        Ok(json!(services).to_string())
    }
}

pub fn booking_get_services_tool(store: SharedBookingStore) -> Tool {
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
        execute: Arc::new(BookingGetServicesExecutor { store }),
    }
}

pub struct BookingUpsertServiceExecutor {
    pub store: SharedBookingStore,
}

#[async_trait::async_trait]
impl ToolExecutor for BookingUpsertServiceExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let tenant_id = args["tenant_id"].as_str().ok_or_else(|| ToolError::LlmRecoverable("tenant_id is required".to_string()))?;
        let id = args["id"].as_str().map(|s| s.to_string()).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let title = args["title"].as_str().ok_or_else(|| ToolError::LlmRecoverable("title is required".to_string()))?;
        let description = args["description"].as_str().map(|s| s.to_string());
        let price_cents = args["price_cents"].as_i64().ok_or_else(|| ToolError::LlmRecoverable("price_cents is required".to_string()))?;

        let service = Service {
            id: id.clone(),
            tenant_id: tenant_id.to_string(),
            title: title.to_string(),
            description,
            price_cents,
        };

        let mut store = self.store.write().await;
        if let Some(existing) = store.services.iter_mut().find(|s| s.id == id) {
            *existing = service;
        } else {
            store.services.push(service);
        }

        Ok(json!({"status": "success", "message": "Service upserted successfully", "id": id}).to_string())
    }
}

pub fn booking_upsert_service_tool(store: SharedBookingStore) -> Tool {
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
        execute: Arc::new(BookingUpsertServiceExecutor { store }),
    }
}

pub struct BookingListAppointmentsExecutor {
    pub store: SharedBookingStore,
}

#[async_trait::async_trait]
impl ToolExecutor for BookingListAppointmentsExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let tenant_id = args["tenant_id"].as_str().ok_or_else(|| ToolError::LlmRecoverable("tenant_id is required".to_string()))?;
        let store = self.store.read().await;
        let bookings: Vec<_> = store.bookings.iter().filter(|b| b.tenant_id == tenant_id).cloned().collect();
        Ok(json!(bookings).to_string())
    }
}

pub fn booking_list_appointments_tool(store: SharedBookingStore) -> Tool {
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
        execute: Arc::new(BookingListAppointmentsExecutor { store }),
    }
}

pub struct BookingCreateAppointmentExecutor {
    pub store: SharedBookingStore,
}

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

        let mut store = self.store.write().await;
        store.bookings.push(booking);

        Ok(json!({"status": "success", "message": "Appointment created successfully"}).to_string())
    }
}

pub fn booking_create_appointment_tool(store: SharedBookingStore) -> Tool {
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
        execute: Arc::new(BookingCreateAppointmentExecutor { store }),
    }
}
