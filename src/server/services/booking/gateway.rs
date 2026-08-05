use chrono::{DateTime, Utc, TimeZone};
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tonic::Status;
use uuid::Uuid;
use std::sync::Arc;

use crate::services::booking::models::{time_slots, bookings};

pub struct BookingGateway {
    db: DatabaseConnection,
    redis: Arc<redis::Client>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AvailableSlot {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

impl BookingGateway {
    pub fn new(db: DatabaseConnection, redis: Arc<redis::Client>) -> Self {
        Self { db, redis }
    }

    pub async fn query_availability(
        &self,
        tenant_id: &str,
        service_id: &str,
        date: &str,
    ) -> Result<Vec<AvailableSlot>, Status> {
        let target_date = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .map_err(|_| Status::invalid_argument("Invalid date format, expected YYYY-MM-DD"))?;
        let start_of_day = Utc.from_utc_datetime(&target_date.and_hms_opt(0, 0, 0).unwrap());
        let end_of_day = start_of_day + chrono::Duration::days(1);

        let available_slots = time_slots::Entity::find()
            .filter(time_slots::Column::TenantId.eq(tenant_id))
            .filter(time_slots::Column::ServiceId.eq(service_id.to_string()))
            .filter(time_slots::Column::Status.eq("available"))
            .filter(time_slots::Column::StartTime.gte(start_of_day))
            .filter(time_slots::Column::StartTime.lt(end_of_day))
            .order_by_asc(time_slots::Column::StartTime)
            .all(&self.db)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let slots = available_slots
            .into_iter()
            .map(|s| AvailableSlot {
                start_time: s.start_time,
                end_time: s.end_time,
            })
            .collect();

        Ok(slots)
    }

    pub async fn hold_slot(
        &self,
        tenant_id: &str,
        service_id: &str,
        customer_id: &str,
        start_time: DateTime<Utc>,
        _end_time: DateTime<Utc>,
    ) -> Result<String, Status> {
        let lock_key = format!("ohc:lock:{}:timeslot:{}", tenant_id, start_time.timestamp());

        // Basic Redlock-like setup holding for 10 minutes (600s)
        let mut conn = self.redis.get_multiplexed_async_connection().await.map_err(|e| Status::internal(e.to_string()))?;
        let acquired: bool = redis::cmd("SET")
            .arg(&lock_key)
            .arg("held")
            .arg("NX")
            .arg("EX")
            .arg(600)
            .query_async(&mut conn)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        if !acquired {
            return Err(Status::failed_precondition("Time slot is currently being held or already booked"));
        }

        let slot = time_slots::Entity::find()
            .filter(time_slots::Column::TenantId.eq(tenant_id))
            .filter(time_slots::Column::ServiceId.eq(service_id.to_string()))
            .filter(time_slots::Column::StartTime.eq(start_time))
            .filter(time_slots::Column::Status.eq("available"))
            .one(&self.db)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        if let Some(s) = slot {
            let mut active: time_slots::ActiveModel = s.into();
            active.status = Set("held".to_string());
            let updated = active.update(&self.db).await.map_err(|e| Status::internal(e.to_string()))?;

            let booking_id = Uuid::new_v4().to_string();
            let new_booking = bookings::ActiveModel {
                id: Set(booking_id.clone()),
                tenant_id: Set(tenant_id.to_string()),
                customer_id: Set(customer_id.to_string()),
                service_id: Set(Some(service_id.to_string())),
                payment_intent_id: Set(None),
                start_time: Set(updated.start_time),
                end_time: Set(Some(updated.end_time)),
                status: Set("pending".to_string()),
                created_at: Set(Utc::now()),
                updated_at: Set(Utc::now()),
            };

            new_booking.insert(&self.db).await.map_err(|e| Status::internal(e.to_string()))?;
            return Ok(booking_id);
        }

        Err(Status::failed_precondition("Time slot not available"))
    }

    pub async fn confirm_booking(&self, tenant_id: &str, booking_id: &str) -> Result<(), Status> {
        let booking = bookings::Entity::find()
            .filter(bookings::Column::TenantId.eq(tenant_id))
            .filter(bookings::Column::Id.eq(booking_id.to_string()))
            .one(&self.db)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        if let Some(b) = booking {
            let mut active: bookings::ActiveModel = b.clone().into();
            active.status = Set("confirmed".to_string());
            active.updated_at = Set(Utc::now());
            active.update(&self.db).await.map_err(|e| Status::internal(e.to_string()))?;

            if let Some(sid) = b.service_id {
                let slot = time_slots::Entity::find()
                    .filter(time_slots::Column::TenantId.eq(tenant_id))
                    .filter(time_slots::Column::ServiceId.eq(sid))
                    .filter(time_slots::Column::StartTime.eq(b.start_time))
                    .one(&self.db)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?;

                if let Some(s) = slot {
                    let mut active_slot: time_slots::ActiveModel = s.into();
                    active_slot.status = Set("booked".to_string());
                    active_slot.update(&self.db).await.map_err(|e| Status::internal(e.to_string()))?;
                }
            }

            return Ok(());
        }

        Err(Status::not_found("Booking not found"))
    }
}
