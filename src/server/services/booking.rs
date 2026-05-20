use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

pub struct BookingService {
    pub pool: sqlx::PgPool,
    pub lock: std::sync::Arc<dyn crate::msgbus::DistributedLock>,
    pub node_id: String,
}

impl BookingService {
    pub fn new(pool: sqlx::PgPool, lock: std::sync::Arc<dyn crate::msgbus::DistributedLock>, node_id: String) -> Self {
        Self { pool, lock, node_id }
    }

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

    // Prevents double booking for a given time slot by acquiring a distributed lock and querying the DB
    pub async fn prevent_double_booking(
        &self,
        tenant_id: Uuid,
        service_id: Uuid,
        new_slot: &BookingTimeSlot,
    ) -> Result<(), String> {
        let resource = format!("booking_lock:{}", service_id);
        if !self.lock.acquire_lock(&resource, &self.node_id, 30).await.unwrap_or(false) {
            return Err("Failed to acquire lock for booking".to_string());
        }

        // Check for overlaps
        let overlap = sqlx::query(
            r#"
            SELECT id FROM bookings
            WHERE tenant_id = $1 AND service_id = $2
            AND (
                (start_time < $4 AND end_time > $3)
            )
            LIMIT 1
            "#,
        )
        .bind(tenant_id.to_string())
        .bind(service_id.to_string())
        .bind(new_slot.start_time.to_rfc3339())
        .bind(new_slot.end_time.to_rfc3339())
        .fetch_optional(&self.pool)
        .await;

        let res = match overlap {
            Ok(Some(_)) => Err("Time slot overlaps with an existing booking".to_string()),
            Ok(None) => {
                let id = Uuid::new_v4().to_string();
                let insert = sqlx::query(
                    r#"
                    INSERT INTO bookings (id, tenant_id, service_id, start_time, end_time, status)
                    VALUES ($1, $2, $3, $4, $5, 'confirmed')
                    "#,
                )
                .bind(id)
                .bind(tenant_id.to_string())
                .bind(service_id.to_string())
                .bind(new_slot.start_time.to_rfc3339())
                .bind(new_slot.end_time.to_rfc3339())
                .execute(&self.pool)
                .await;

                if insert.is_err() {
                    Err("Failed to insert booking".to_string())
                } else {
                    Ok(())
                }
            },
            Err(e) => Err(format!("Database error: {}", e)),
        };

        let _ = self.lock.release_lock(&resource, &self.node_id).await;
        res
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

    #[tokio::test]
    async fn test_prevent_double_booking_success() {
        let pool = crate::db::get_pool();

        let lock = std::sync::Arc::new(crate::msgbus::MemoryBus::new());
        let service = BookingService::new(pool, lock, "test_node".to_string());

        let tenant_id = Uuid::new_v4();
        let service_id = Uuid::new_v4();
        let now = Utc::now();

        // Clear possible previous test data
        sqlx::query("DELETE FROM bookings WHERE tenant_id = $1").bind(tenant_id.to_string()).execute(&service.pool).await.unwrap();

        let new_slot = BookingTimeSlot {
            start_time: now,
            end_time: now + chrono::Duration::hours(1),
        };

        let result = service.prevent_double_booking(tenant_id, service_id, &new_slot).await;
        assert!(result.is_ok(), "Booking should succeed");
    }

    #[tokio::test]
    async fn test_concurrent_booking_conflict() {
        let pool = crate::db::get_pool();

        let lock = std::sync::Arc::new(crate::msgbus::MemoryBus::new());
        let service = BookingService::new(pool, lock, "test_node".to_string());

        let tenant_id = Uuid::new_v4();
        let service_id = Uuid::new_v4();
        let now = Utc::now();

        // Clear possible previous test data
        sqlx::query("DELETE FROM bookings WHERE tenant_id = $1").bind(tenant_id.to_string()).execute(&service.pool).await.unwrap();

        let new_slot1 = BookingTimeSlot {
            start_time: now,
            end_time: now + chrono::Duration::hours(1),
        };

        let new_slot2 = BookingTimeSlot {
            start_time: now + chrono::Duration::minutes(30),
            end_time: now + chrono::Duration::minutes(90),
        };

        let result1 = service.prevent_double_booking(tenant_id, service_id, &new_slot1).await;
        assert!(result1.is_ok(), "First booking should succeed");

        let result2 = service.prevent_double_booking(tenant_id, service_id, &new_slot2).await;
        assert!(result2.is_err(), "Second overlapping booking should fail");
    }
}
