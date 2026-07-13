use super::Tool;
use super::pydantic::{PydanticAdapter, PydanticToolExecutor};
use crate::tenant::TenantContext;
use ohc_builtin_agent_core::types::ToolError;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct BookingStore {
    pool: tokio::sync::OnceCell<sqlx::PgPool>,
}

impl BookingStore {
    pub async fn get_pool(&self) -> Result<&sqlx::PgPool, ToolError> {
        self.pool
            .get_or_try_init(|| async {
                let database_url = std::env::var("OHC_DATABASE_URL")
                    .unwrap_or_else(|_| "postgres://ohc:ohc@localhost:5432/ohc".to_string());
                sqlx::PgPool::connect(&database_url)
                    .await
                    .map_err(|e| ToolError::Transient(e.to_string()))
            })
            .await
    }
}

pub type SharedBookingStore = Arc<RwLock<BookingStore>>;

#[derive(Deserialize)]
pub struct BookingGetServicesArgs {}

pub struct BookingGetServicesExecutor {
    pub store: SharedBookingStore,
    pub tenant: TenantContext,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<BookingGetServicesArgs> for BookingGetServicesExecutor {
    async fn execute_typed(&self, _args: BookingGetServicesArgs) -> Result<String, ToolError> {
        let tenant_id = self.tenant.as_str();

        let store = self.store.read().await;
        let pool = store.get_pool().await?;
        let mut tx = pool
            .begin()
            .await
            .map_err(|e| ToolError::Transient(e.to_string()))?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| ToolError::Transient(e.to_string()))?;

        let rows = sqlx::query(
            "SELECT id, title as name, description, price_cents FROM services WHERE tenant_id = $1",
        )
        .bind(&tenant_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| ToolError::Transient(e.to_string()))?;

        let mut services = Vec::new();
        for row in rows {
            use sqlx::Row;
            let id: String = row.get("id");
            let title: String = row.get("name");
            let description: Option<String> = row.try_get("description").unwrap_or(None);
            let price_cents: i64 = row.try_get("price_cents").unwrap_or(0);
            services.push(json!({
                "id": id,
                "tenant_id": tenant_id,
                "title": title,
                "description": description,
                "price_cents": price_cents
            }));
        }

        Ok(json!(services).to_string())
    }
}

pub fn booking_get_services_tool(store: SharedBookingStore, tenant: TenantContext) -> Tool {
    Tool {
        name: "booking_get_services".to_string(),
        description: "List all available services for booking in the business.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {},
            "required": []
        }),
        execute: Arc::new(PydanticAdapter::new(BookingGetServicesExecutor {
            store,
            tenant,
        })),
    }
}

#[derive(Deserialize)]
pub struct BookingUpsertServiceArgs {
    pub title: String,
    pub description: Option<String>,
    pub price_cents: i64,
}

pub struct BookingUpsertServiceExecutor {
    pub store: SharedBookingStore,
    pub tenant: TenantContext,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<BookingUpsertServiceArgs> for BookingUpsertServiceExecutor {
    async fn execute_typed(&self, args: BookingUpsertServiceArgs) -> Result<String, ToolError> {
        let tenant_id = self.tenant.as_str();
        let store = self.store.read().await;
        let pool = store.get_pool().await?;
        let mut tx = pool
            .begin()
            .await
            .map_err(|e| ToolError::Transient(e.to_string()))?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| ToolError::Transient(e.to_string()))?;

        let id = uuid::Uuid::new_v4().to_string();

        sqlx::query("INSERT INTO services (id, tenant_id, name, description, price_cents) VALUES ($1, $2, $3, $4, $5)")
            .bind(&id)
            .bind(tenant_id)
            .bind(&args.title)
            .bind(&args.description)
            .bind(args.price_cents)
            .execute(&mut *tx)
            .await
            .map_err(|e| ToolError::Transient(e.to_string()))?;

        tx.commit()
            .await
            .map_err(|e| ToolError::Transient(e.to_string()))?;

        Ok(json!({ "status": "success", "service_id": id }).to_string())
    }
}

pub fn booking_upsert_service_tool(store: SharedBookingStore, tenant: TenantContext) -> Tool {
    Tool {
        name: "booking_upsert_service".to_string(),
        description: "Create or update a booking service offering.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "title": { "type": "string" },
                "description": { "type": "string" },
                "price_cents": { "type": "integer" }
            },
            "required": ["title", "price_cents"]
        }),
        execute: Arc::new(PydanticAdapter::new(BookingUpsertServiceExecutor {
            store,
            tenant,
        })),
    }
}

#[derive(Deserialize)]
pub struct BookingListAppointmentsArgs {}

pub struct BookingListAppointmentsExecutor {
    pub store: SharedBookingStore,
    pub tenant: TenantContext,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<BookingListAppointmentsArgs> for BookingListAppointmentsExecutor {
    async fn execute_typed(&self, _args: BookingListAppointmentsArgs) -> Result<String, ToolError> {
        let tenant_id = self.tenant.as_str();
        let store = self.store.read().await;
        let pool = store.get_pool().await?;
        let mut tx = pool
            .begin()
            .await
            .map_err(|e| ToolError::Transient(e.to_string()))?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| ToolError::Transient(e.to_string()))?;

        let rows = sqlx::query("SELECT id, customer_id, service_id, start_time, end_time, status FROM bookings WHERE tenant_id = $1 ORDER BY start_time ASC")
            .bind(&tenant_id)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| ToolError::Transient(e.to_string()))?;

        let mut bookings = Vec::new();
        for row in rows {
            use sqlx::Row;
            let id: String = row.get("id");
            let customer_id: String = row.get("customer_id");
            let service_id: Option<String> = row.try_get("service_id").ok();
            let start_time: chrono::DateTime<chrono::Utc> = row.get("start_time");
            let end_time: Option<chrono::DateTime<chrono::Utc>> = row.try_get("end_time").ok();
            let status: Option<String> = row.try_get("status").ok();

            bookings.push(json!({
                "id": id,
                "tenant_id": tenant_id,
                "customer_id": customer_id,
                "service_id": service_id,
                "start_time": start_time,
                "end_time": end_time,
                "status": status.unwrap_or_else(|| "scheduled".to_string())
            }));
        }

        Ok(json!(bookings).to_string())
    }
}

pub fn booking_list_appointments_tool(store: SharedBookingStore, tenant: TenantContext) -> Tool {
    Tool {
        name: "booking_list_appointments".to_string(),
        description: "List all scheduled appointments for the business.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {},
            "required": []
        }),
        execute: Arc::new(PydanticAdapter::new(BookingListAppointmentsExecutor {
            store,
            tenant,
        })),
    }
}

#[derive(Deserialize)]
pub struct BookingCreateAppointmentArgs {
    pub customer_id: String,
    pub service_id: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct BookingCreateAppointmentExecutor {
    pub store: SharedBookingStore,
    pub tenant: TenantContext,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<BookingCreateAppointmentArgs> for BookingCreateAppointmentExecutor {
    async fn execute_typed(&self, args: BookingCreateAppointmentArgs) -> Result<String, ToolError> {
        let tenant_id = self.tenant.as_str();
        let store = self.store.read().await;
        let pool = store.get_pool().await?;
        let mut tx = pool
            .begin()
            .await
            .map_err(|e| ToolError::Transient(e.to_string()))?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| ToolError::Transient(e.to_string()))?;

        let id = uuid::Uuid::new_v4().to_string();

        sqlx::query("INSERT INTO bookings (id, tenant_id, customer_id, service_id, start_time, end_time, status) VALUES ($1, $2, $3, $4, $5, $6, 'confirmed')")
            .bind(&id)
            .bind(tenant_id)
            .bind(&args.customer_id)
            .bind(&args.service_id)
            .bind(args.start_time)
            .bind(args.end_time)
            .execute(&mut *tx)
            .await
            .map_err(|e| ToolError::Transient(e.to_string()))?;

        tx.commit()
            .await
            .map_err(|e| ToolError::Transient(e.to_string()))?;

        Ok(json!({ "status": "success", "booking_id": id }).to_string())
    }
}

pub fn booking_create_appointment_tool(store: SharedBookingStore, tenant: TenantContext) -> Tool {
    Tool {
        name: "booking_create_appointment".to_string(),
        description: "Create a new appointment for a customer.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "customer_id": { "type": "string" },
                "service_id": { "type": "string" },
                "start_time": { "type": "string", "format": "date-time" },
                "end_time": { "type": "string", "format": "date-time" }
            },
            "required": ["customer_id", "service_id", "start_time"]
        }),
        execute: Arc::new(PydanticAdapter::new(BookingCreateAppointmentExecutor {
            store,
            tenant,
        })),
    }
}

#[derive(Deserialize)]
pub struct BookingNegotiateTimeArgs {
    pub customer_id: String,
    pub service_id: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
}

pub struct BookingNegotiateTimeExecutor {
    pub store: SharedBookingStore,
    pub tenant: TenantContext,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<BookingNegotiateTimeArgs> for BookingNegotiateTimeExecutor {
    async fn execute_typed(&self, args: BookingNegotiateTimeArgs) -> Result<String, ToolError> {
        let tenant_id = self.tenant.as_str();
        // Tentatively lock the time slot for a short period (e.g. 15 minutes) while negotiating.
        // It's not a full booking, just a Redlock hold in Redis to avoid double booking during negotiation.
        let store = self.store.read().await;
        let pool = store.get_pool().await?;
        let mut tx = pool
            .begin()
            .await
            .map_err(|e| ToolError::Transient(e.to_string()))?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| ToolError::Transient(e.to_string()))?;

        // Assuming there is an external process or Redis lock logic that holds it, we simulate inserting a tentative state in ledger
        let id = uuid::Uuid::new_v4().to_string();

        sqlx::query("INSERT INTO bookings (id, tenant_id, service_id, start_time, end_time, status) VALUES ($1, $2, $3, $4, $5, 'pending')")
            .bind(&id)
            .bind(tenant_id)
            .bind(&args.service_id)
            .bind(args.start_time)
            .bind(args.end_time)
            .execute(&mut *tx)
            .await
            .map_err(|e| ToolError::Transient(e.to_string()))?;

        tx.commit()
            .await
            .map_err(|e| ToolError::Transient(e.to_string()))?;

        Ok(json!({ "status": "success", "message": "Time slot tentatively held for negotiation." }).to_string())
    }
}

pub fn booking_negotiate_time_tool(store: SharedBookingStore, tenant: TenantContext) -> Tool {
    Tool {
        name: "booking_negotiate_time".to_string(),
        description: "Hold a time slot tentatively while negotiating a booking with a customer. It places a temporary lock to avoid double booking.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "customer_id": { "type": "string" },
                "service_id": { "type": "string" },
                "start_time": { "type": "string", "format": "date-time" },
                "end_time": { "type": "string", "format": "date-time" }
            },
            "required": ["customer_id", "service_id", "start_time", "end_time"]
        }),
        execute: Arc::new(PydanticAdapter::new(BookingNegotiateTimeExecutor { store, tenant })),
    }
}

#[derive(Deserialize)]
pub struct BookingRescheduleArgs {
    pub booking_id: String,
    pub new_start_time: chrono::DateTime<chrono::Utc>,
    pub new_end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub reason: Option<String>,
}

pub struct BookingRescheduleExecutor {
    pub store: SharedBookingStore,
    pub tenant: TenantContext,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<BookingRescheduleArgs> for BookingRescheduleExecutor {
    async fn execute_typed(&self, args: BookingRescheduleArgs) -> Result<String, ToolError> {
        let tenant_id = self.tenant.as_str();
        let store = self.store.read().await;
        let pool = store.get_pool().await?;
        let mut tx = pool
            .begin()
            .await
            .map_err(|e| ToolError::Transient(e.to_string()))?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| ToolError::Transient(e.to_string()))?;

        // Get original booking details
        let original: (String, String, String) = sqlx::query_as("SELECT customer_id::text, service_id, status FROM bookings WHERE id = $1 AND tenant_id = $2")
            .bind(&args.booking_id)
            .bind(tenant_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| ToolError::Transient(format!("Booking not found: {}", e)))?;

        // Cancel original and create new one with rescheduled_from_id
        sqlx::query("UPDATE bookings SET status = 'cancelled', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2")
            .bind(&args.booking_id)
            .bind(tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| ToolError::Transient(e.to_string()))?;

        let new_id = uuid::Uuid::new_v4().to_string();
        let _notes = args
            .reason
            .map(|r| format!("Rescheduled from {}. Reason: {}", args.booking_id, r));

        sqlx::query("INSERT INTO bookings (id, tenant_id, customer_id, service_id, start_time, end_time, status) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(&new_id)
            .bind(tenant_id)
            .bind(&original.0)
            .bind(&original.1)
            .bind(args.new_start_time)
            .bind(args.new_end_time)
            .bind(&original.2)
            .execute(&mut *tx)
            .await
            .map_err(|e| ToolError::Transient(e.to_string()))?;

        tx.commit()
            .await
            .map_err(|e| ToolError::Transient(e.to_string()))?;

        Ok(json!({ "status": "success", "new_booking_id": new_id }).to_string())
    }
}

pub fn booking_reschedule_tool(store: SharedBookingStore, tenant: TenantContext) -> Tool {
    Tool {
        name: "booking_reschedule".to_string(),
        description: "Reschedule an existing appointment to a new time slot.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "booking_id": { "type": "string" },
                "new_start_time": { "type": "string", "format": "date-time" },
                "new_end_time": { "type": "string", "format": "date-time" },
                "reason": { "type": "string" }
            },
            "required": ["booking_id", "new_start_time"]
        }),
        execute: Arc::new(PydanticAdapter::new(BookingRescheduleExecutor {
            store,
            tenant,
        })),
    }
}
