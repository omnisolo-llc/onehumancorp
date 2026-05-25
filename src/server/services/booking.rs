use uuid::Uuid;
use chrono::{DateTime, Utc, Datelike, };
use serde::{Deserialize, Serialize};
use crate::db::get_pool;
use sqlx::Row;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    pub tenant_id: String,
    pub title: String,
    pub description: Option<String>,
    pub price_cents: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingRecord {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: String,
    pub variant_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityConfig {
    pub tenant_id: String,
    pub weekly_hours: Vec<DayHours>,
    pub service_duration_mins: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayHours {
    pub day_of_week: u32, // 0 = Mon, 6 = Sun
    pub start_hour: u32, // 0-23
    pub end_hour: u32, // 0-23
}

// Emulate LLM Call
#[derive(Debug, Clone, Serialize, Deserialize)]
struct LLMParsedBooking {
    day_of_week: u32,
    hour: u32,
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

    pub async fn list_services(tenant_id: &str) -> Result<Vec<Service>, String> {
        let pool = get_pool();
        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

        let rows = sqlx::query("SELECT id, tenant_id, title, description, price_cents FROM products WHERE type = 'booking'")
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        let services = rows.into_iter().map(|row| Service {
            id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            title: row.get("title"),
            description: row.get("description"),
            price_cents: row.get("price_cents"),
        }).collect();

        Ok(services)
    }

    pub async fn upsert_service(service: Service) -> Result<(), String> {
        let pool = get_pool();
        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *tx, &service.tenant_id).await.map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO products (id, tenant_id, title, description, price_cents, type) \
             VALUES ($1, $2, $3, $4, $5, 'booking') \
             ON CONFLICT (id) DO UPDATE SET \
             title = EXCLUDED.title, \
             description = EXCLUDED.description, \
             price_cents = EXCLUDED.price_cents, \
             updated_at = CURRENT_TIMESTAMP"
        )
        .bind(&service.id)
        .bind(&service.tenant_id)
        .bind(&service.title)
        .bind(&service.description)
        .bind(service.price_cents)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn get_bookings(tenant_id: &str) -> Result<Vec<BookingRecord>, String> {
        let pool = get_pool();
        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

        let rows = sqlx::query("SELECT id, tenant_id, customer_id, variant_id as product_id, start_time, end_time, status FROM bookings")
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        let bookings = rows.into_iter().map(|row| BookingRecord {
            id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            customer_id: row.get("customer_id"),
            variant_id: row.get("product_id"),
            start_time: row.get("start_time"),
            end_time: row.get("end_time"),
            status: row.get("status"),
        }).collect();

        Ok(bookings)
    }

    pub async fn create_booking(booking: BookingRecord) -> Result<(), String> {
        let pool = get_pool();
        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *tx, &booking.tenant_id).await.map_err(|e| e.to_string())?;

        // Uses variant_id instead of product_id mapping appropriately with 008 DB migration
        sqlx::query(
            "INSERT INTO bookings (id, tenant_id, customer_id, variant_id, start_time, end_time, status) \
             VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(&booking.id)
        .bind(&booking.tenant_id)
        .bind(&booking.customer_id)
        .bind(&booking.variant_id)
        .bind(booking.start_time)
        .bind(booking.end_time)
        .bind(&booking.status)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn set_availability(config: AvailabilityConfig) -> Result<(), String> {
        let pool = get_pool();
        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *tx, &config.tenant_id).await.map_err(|e| e.to_string())?;

        let config_str = serde_json::to_string(&config).map_err(|e| e.to_string())?;
        sqlx::query(
            "INSERT INTO tenants (id, name, metadata) VALUES ($1, 'dummy', $2)
             ON CONFLICT (id) DO UPDATE SET metadata = EXCLUDED.metadata"
        )
        .bind(&config.tenant_id)
        .bind(&config_str)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    // Extracted LLM NLP mock for testability and structural integrity
    fn parse_nlp_intent(customer_dm: &str) -> Option<LLMParsedBooking> {
        let mut requested_day = None;
        if customer_dm.to_lowercase().contains("thursday") {
            requested_day = Some(3); // 3 = Thursday in zero-indexed  mapping 0=Mon
        } else if customer_dm.to_lowercase().contains("friday") {
            requested_day = Some(4);
        }

        let mut requested_hour = None;
        if customer_dm.to_lowercase().contains("2 pm") || customer_dm.to_lowercase().contains("14:00") {
            requested_hour = Some(14);
        } else if customer_dm.to_lowercase().contains("10 am") || customer_dm.to_lowercase().contains("10:00") {
            requested_hour = Some(10);
        }

        if let (Some(d), Some(h)) = (requested_day, requested_hour) {
            Some(LLMParsedBooking { day_of_week: d, hour: h })
        } else {
            None
        }
    }

    pub async fn ai_handle_booking_dm(
        _tenant_id: &str,
        customer_dm: &str,
        config: &AvailabilityConfig,
        existing_bookings: &[BookingTimeSlot]
    ) -> Result<Option<BookingTimeSlot>, String> {

        let intent = Self::parse_nlp_intent(customer_dm);

        if let Some(parsed) = intent {
            let day = parsed.day_of_week;
            let hour = parsed.hour;

            // Check availability config
            let mut available = false;
            for h in &config.weekly_hours {
                if h.day_of_week == day && h.start_hour <= hour && h.end_hour > hour {
                    available = true;
                    break;
                }
            }

            if available {
                let now = Utc::now();

                // Using chrono without deprecated API
                let mut start_time = now;
                while start_time.weekday().num_days_from_monday() != day {
                    start_time = start_time + chrono::Duration::days(1);
                }

                let date_str = format!("{}-{:02}-{:02}T{:02}:00:00Z", start_time.year(), start_time.month(), start_time.day(), hour);
                if let Ok(parsed_time) = DateTime::parse_from_rfc3339(&date_str) {
                     start_time = parsed_time.with_timezone(&Utc);
                     let end_time = start_time + chrono::Duration::minutes(config.service_duration_mins as i64);

                     let new_slot = BookingTimeSlot {
                         start_time,
                         end_time,
                     };

                     if Self::prevent_double_booking(existing_bookings, &new_slot).is_ok() {
                         return Ok(Some(new_slot));
                     }
                }
            }
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[tokio::test]
    async fn test_ai_handle_booking_dm() {
        let config = AvailabilityConfig {
            tenant_id: "tenant1".to_string(),
            weekly_hours: vec![
                DayHours { day_of_week: 3, start_hour: 9, end_hour: 17 } // Thursday 9-5
            ],
            service_duration_mins: 60,
        };

        let existing = vec![];
        let dm = "Do you have time to fix my sink this Thursday at 2 PM?";

        let result = BookingService::ai_handle_booking_dm("tenant1", dm, &config, &existing).await.unwrap();
        assert!(result.is_some());

        let dm_bad_time = "Do you have time to fix my sink this Thursday at 8 PM?";
        let result2 = BookingService::ai_handle_booking_dm("tenant1", dm_bad_time, &config, &existing).await.unwrap();
        assert!(result2.is_none());
    }
}
