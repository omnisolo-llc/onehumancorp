use sea_orm::*;
use std::sync::Arc;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::booking_models::{booking, time_slot};
use crate::orchestration::locks::{DistributedLock, RedisLock, LockGuard};

#[derive(Debug, Serialize, Deserialize)]
pub struct AvailabilityQuery {
    pub tenant_id: String,
    pub service_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeSlotDTO {
    pub id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub status: String,
}

pub struct BookingGateway {
    db: Arc<DatabaseConnection>,
    redis_url: Option<String>,
}

impl BookingGateway {
    pub fn new(db: Arc<DatabaseConnection>, redis_url: Option<String>) -> Self {
        Self { db, redis_url }
    }

    pub async fn query_availability(&self, query: AvailabilityQuery) -> Result<Vec<TimeSlotDTO>, DbErr> {
        let slots = time_slot::Entity::find()
            .filter(time_slot::Column::TenantId.eq(&query.tenant_id))
            .filter(time_slot::Column::ServiceId.eq(&query.service_id))
            .filter(time_slot::Column::StartTime.gte(query.start_time))
            .filter(time_slot::Column::EndTime.lte(query.end_time))
            .filter(time_slot::Column::Status.eq("available"))
            .all(&*self.db)
            .await?;

        Ok(slots
            .into_iter()
            .map(|s| TimeSlotDTO {
                id: s.id,
                start_time: s.start_time,
                end_time: s.end_time,
                status: s.status,
            })
            .collect())
    }

    pub async fn hold_slot(&self, tenant_id: &str, slot_id: &str) -> Result<bool, String> {
        if let Some(url) = &self.redis_url {
            if let Ok(client) = redis::Client::open(url.as_str()) {
                let redis_lock = RedisLock::new(client);

                // Provisional hold for 10 minutes
                if let Ok(mut lock_guard) = redis_lock.acquire_resource(tenant_id, "booking_slot", slot_id).await {
                    // Update DB to soft_locked
                    if let Ok(Some(slot)) = time_slot::Entity::find_by_id(slot_id).one(&*self.db).await {
                        if slot.status == "available" {
                            let mut active_slot: time_slot::ActiveModel = slot.into();
                            active_slot.status = Set("soft_locked".to_string());
                            active_slot.update(&*self.db).await.map_err(|e| e.to_string())?;
                            return Ok(true);
                        } else {
                            // Release lock if it wasn't available
                            lock_guard.release().await;
                        }
                    }
                }
                return Ok(false);
            }
        }
        Err("Redis not configured".to_string())
    }

    pub async fn confirm_booking(&self, tenant_id: &str, booking_id: &str) -> Result<(), DbErr> {
         if let Some(b) = booking::Entity::find_by_id(booking_id).one(&*self.db).await? {
             let mut active_booking: booking::ActiveModel = b.into();
             active_booking.status = Set("confirmed".to_string());
             active_booking.update(&*self.db).await?;
         }
         Ok(())
    }
}
