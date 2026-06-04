use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;
use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};
use serde::Deserialize;
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

#[derive(Deserialize)]
pub struct BookingGetServicesArgs {
    pub tenant_id: String,
}

pub struct BookingGetServicesExecutor {
    pub store: SharedBookingStore,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<BookingGetServicesArgs> for BookingGetServicesExecutor {
    async fn execute_typed(&self, args: BookingGetServicesArgs) -> Result<String, ToolError> {
        let tenant_id = args.tenant_id;
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
        execute: Arc::new(PydanticAdapter::new(BookingGetServicesExecutor { store })),
    }
}

#[derive(Deserialize)]
pub struct BookingUpsertServiceArgs {
    pub tenant_id: String,
    pub id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub price_cents: i64,
}

pub struct BookingUpsertServiceExecutor {
    pub store: SharedBookingStore,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<BookingUpsertServiceArgs> for BookingUpsertServiceExecutor {
    async fn execute_typed(&self, args: BookingUpsertServiceArgs) -> Result<String, ToolError> {
        let tenant_id = args.tenant_id;
        let id = args.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let title = args.title;
        let description = args.description;
        let price_cents = args.price_cents;

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
        execute: Arc::new(PydanticAdapter::new(BookingUpsertServiceExecutor { store })),
    }
}

#[derive(Deserialize)]
pub struct BookingListAppointmentsArgs {
    pub tenant_id: String,
}

pub struct BookingListAppointmentsExecutor {
    pub store: SharedBookingStore,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<BookingListAppointmentsArgs> for BookingListAppointmentsExecutor {
    async fn execute_typed(&self, args: BookingListAppointmentsArgs) -> Result<String, ToolError> {
        let tenant_id = args.tenant_id;
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
        execute: Arc::new(PydanticAdapter::new(BookingListAppointmentsExecutor { store })),
    }
}

#[derive(Deserialize)]
pub struct BookingCreateAppointmentArgs {
    pub tenant_id: String,
    pub customer_id: String,
    pub service_id: String,
    pub start_time: String,
    pub end_time: Option<String>,
}

pub struct BookingCreateAppointmentExecutor {
    pub store: SharedBookingStore,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<BookingCreateAppointmentArgs> for BookingCreateAppointmentExecutor {
    async fn execute_typed(&self, args: BookingCreateAppointmentArgs) -> Result<String, ToolError> {
        let tenant_id = args.tenant_id;
        let customer_id = args.customer_id;
        let service_id = args.service_id;
        let start_time_str = args.start_time;
        let end_time_str = args.end_time;

        let start_time = DateTime::parse_from_rfc3339(&start_time_str)
            .map_err(|e| ToolError::LlmRecoverable(format!("invalid start_time format: {}", e)))?
            .with_timezone(&Utc);

        let end_time = if let Some(et) = end_time_str {
            Some(DateTime::parse_from_rfc3339(&et)
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
        execute: Arc::new(PydanticAdapter::new(BookingCreateAppointmentExecutor { store })),
    }
}
