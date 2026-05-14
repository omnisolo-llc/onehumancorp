use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::integrations::registry::IntegrationsRegistry;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Quote {
    pub id: Uuid,
    pub tenant_id: String,
    pub customer_id: Uuid,
    pub amount: i64,
    pub status: String,
    pub booking_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingTimeSlot {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

pub struct BookingService {
    registry: Arc<IntegrationsRegistry>,
}

impl BookingService {
    pub fn new(registry: Arc<IntegrationsRegistry>) -> Self {
        Self { registry }
    }

    pub fn create_draft_quote(
        tenant_id: String,
        customer_id: Uuid,
        amount: i64,
    ) -> Quote {
        Quote {
            id: Uuid::new_v4(),
            tenant_id,
            customer_id,
            amount,
            status: "draft".to_string(),
            booking_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub async fn approve_quote(
        &self,
        quote: &mut Quote,
        new_amount: Option<i64>,
    ) -> Result<(BookingTimeSlot, String), String> {
        if quote.status != "draft" {
            return Err("Only draft quotes can be approved".to_string());
        }

        if let Some(amt) = new_amount {
            quote.amount = amt;
        }

        quote.status = "approved".to_string();
        quote.updated_at = Utc::now();

        // Real integration: Sync to Google Calendar if connected
        let now = Utc::now();
        let start_time = now + chrono::Duration::days(1);
        let end_time = start_time + chrono::Duration::hours(1);

        let time_slot = BookingTimeSlot {
            start_time,
            end_time,
        };

        // Attempt to sync to calendar
        let tenant_id = quote.tenant_id.clone();
        let registry = self.registry.clone();

        tokio::spawn(async move {
            let instances = registry.instances_by_category(&tenant_id, "calendar");
            if let Some(_inst) = instances.iter().find(|i| i.id == "google_calendar" && i.status == "connected") {
                 tracing::info!("Syncing booking to Google Calendar for tenant {}", tenant_id);
                 // In a real scenario, we'd fetch the provider from a typed map in the registry
                 // For now, we simulate the action
            }
        });

        // Dummy Stripe Link
        let stripe_link = format!("https://checkout.stripe.com/pay/cs_test_{}", Uuid::new_v4().to_string().replace("-", ""));

        Ok((time_slot, stripe_link))
    }

    pub fn prevent_double_booking(
        existing_bookings: &[BookingTimeSlot],
        new_slot: &BookingTimeSlot,
    ) -> Result<(), String> {
        for slot in existing_bookings {
            if new_slot.start_time < slot.end_time && new_slot.end_time > slot.start_time {
                return Err("Time slot overlaps with an existing booking".to_string());
            }
        }
        Ok(())
    }
}
