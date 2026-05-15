use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::integrations::google_calendar::availability::{AvailabilityCalculator, BusinessHours, AvailableSlot};
use crate::integrations::google_calendar::sync::CalendarSyncService;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Quote {
    pub id: Uuid,
    pub tenant_id: Uuid,
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

pub struct BookingService;

impl BookingService {
    pub fn create_draft_quote(
        tenant_id: Uuid,
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

    pub fn approve_quote(
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

        // Dummy booking time slot - e.g., tomorrow at 10 AM
        let now = Utc::now();
        let start_time = now + chrono::Duration::days(1);
        let end_time = start_time + chrono::Duration::hours(1);

        let time_slot = BookingTimeSlot {
            start_time,
            end_time,
        };

        // Dummy Stripe Link
        let stripe_link = format!("https://checkout.stripe.com/pay/cs_test_{}", Uuid::new_v4().to_string().replace("-", ""));

        Ok((time_slot, stripe_link))
    }

    // Prevents double booking for a given time slot (dummy logic for now, real logic would query DB)
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

    /// Fetches available time slots for a business owner by checking their Google Calendar
    pub async fn fetch_available_slots_from_calendar(
        access_token: String,
        calendar_id: &str,
        business_hours: &BusinessHours,
        search_start: DateTime<Utc>,
        search_end: DateTime<Utc>,
        slot_duration: chrono::Duration,
    ) -> Result<Vec<AvailableSlot>, String> {
        let sync_service = CalendarSyncService::new(access_token);

        let freebusy = sync_service
            .fetch_freebusy(vec![calendar_id.to_string()], search_start, search_end)
            .await?;

        AvailabilityCalculator::calculate_available_slots(
            &freebusy,
            calendar_id,
            business_hours,
            search_start,
            search_end,
            slot_duration,
        )
    }

    /// Creates a confirmed booking event directly in the business owner's Google Calendar
    pub async fn push_booking_to_calendar(
        access_token: String,
        calendar_id: &str,
        summary: &str,
        description: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<(), String> {
        let sync_service = CalendarSyncService::new(access_token);

        sync_service
            .push_booking(calendar_id, summary, description, start_time, end_time)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_draft_quote() {
        let tenant_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let amount = 15000;

        let quote = BookingService::create_draft_quote(tenant_id, customer_id, amount);

        assert_eq!(quote.tenant_id, tenant_id);
        assert_eq!(quote.customer_id, customer_id);
        assert_eq!(quote.amount, amount);
        assert_eq!(quote.status, "draft");
        assert!(quote.booking_id.is_none());
    }

    #[test]
    fn test_approve_quote() {
        let tenant_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let amount = 15000;

        let mut quote = BookingService::create_draft_quote(tenant_id, customer_id, amount);

        let new_amount = Some(20000);
        let result = BookingService::approve_quote(&mut quote, new_amount);

        assert!(result.is_ok());
        let (time_slot, stripe_link) = result.unwrap();

        assert_eq!(quote.status, "approved");
        assert_eq!(quote.amount, 20000);
        assert!(stripe_link.starts_with("https://checkout.stripe.com/pay/cs_test_"));
        assert!(time_slot.start_time < time_slot.end_time);
    }

    #[test]
    fn test_prevent_double_booking() {
        let now = Utc::now();
        let slot1 = BookingTimeSlot {
            start_time: now,
            end_time: now + chrono::Duration::hours(1),
        };
        let slot2 = BookingTimeSlot {
            start_time: now + chrono::Duration::hours(2),
            end_time: now + chrono::Duration::hours(3),
        };

        let existing = vec![slot1, slot2];

        // Non-overlapping
        let new_slot = BookingTimeSlot {
            start_time: now + chrono::Duration::hours(1),
            end_time: now + chrono::Duration::hours(2),
        };
        assert!(BookingService::prevent_double_booking(&existing, &new_slot).is_ok());

        // Overlapping
        let overlapping_slot = BookingTimeSlot {
            start_time: now + chrono::Duration::minutes(30),
            end_time: now + chrono::Duration::minutes(90),
        };
        assert!(BookingService::prevent_double_booking(&existing, &overlapping_slot).is_err());
    }
}
