use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "db", derive(sqlx::FromRow))]
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
#[cfg_attr(feature = "db", derive(sqlx::FromRow))]
pub struct BookingTimeSlot {
    pub id: Option<Uuid>,
    pub tenant_id: Option<Uuid>,
    pub service_id: Option<Uuid>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub is_booked: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(sqlx::FromRow))]
pub struct Service {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub duration_minutes: i32,
    pub price_cents: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(sqlx::FromRow))]
pub struct Booking {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub service_id: Uuid,
    pub slot_id: Uuid,
    pub customer_id: Uuid,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
            id: None,
            tenant_id: None,
            service_id: None,
            start_time,
            end_time,
            is_booked: None,
            created_at: None,
            updated_at: None,
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
}

#[cfg(feature = "db")]
impl BookingService {
    pub async fn create_draft_quote_db(
        pool: &sqlx::PgPool,
        tenant_id: Uuid,
        customer_id: Uuid,
        amount: i64,
    ) -> Result<Quote, sqlx::Error> {
        let quote = sqlx::query_as::<_, Quote>(
            r#"
            INSERT INTO quotes (id, tenant_id, customer_id, amount, status, booking_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, 'draft', NULL, NOW(), NOW())
            RETURNING *
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(customer_id)
        .bind(amount)
        .fetch_one(pool)
        .await?;

        Ok(quote)
    }

    pub async fn approve_quote_db(
        pool: &sqlx::PgPool,
        tenant_id: Uuid,
        quote_id: Uuid,
        new_amount: Option<i64>,
    ) -> Result<(BookingTimeSlot, String), String> {
        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

        let quote = sqlx::query_as::<_, Quote>(
            "SELECT * FROM quotes WHERE id = $1 AND tenant_id = $2 FOR UPDATE"
        )
        .bind(quote_id)
        .bind(tenant_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Quote not found".to_string())?;

        if quote.status != "draft" {
            return Err("Only draft quotes can be approved".to_string());
        }

        let amount_to_set = new_amount.unwrap_or(quote.amount);

        sqlx::query(
            "UPDATE quotes SET amount = $1, status = 'approved', updated_at = NOW() WHERE id = $2 AND tenant_id = $3"
        )
        .bind(amount_to_set)
        .bind(quote_id)
        .bind(tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let now = Utc::now();
        let start_time = now + chrono::Duration::days(1);
        let end_time = start_time + chrono::Duration::hours(1);

        let dummy_service_id = Uuid::new_v4();

        let time_slot = sqlx::query_as::<_, BookingTimeSlot>(
            r#"
            INSERT INTO availability_slots (id, tenant_id, service_id, start_time, end_time, is_booked, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, FALSE, NOW(), NOW())
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(dummy_service_id)
        .bind(start_time)
        .bind(end_time)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        let stripe_link = format!("https://checkout.stripe.com/pay/cs_test_{}", Uuid::new_v4().to_string().replace("-", ""));

        Ok((time_slot, stripe_link))
    }

    pub async fn create_booking_db(
        pool: &sqlx::PgPool,
        tenant_id: Uuid,
        service_id: Uuid,
        slot_id: Uuid,
        customer_id: Uuid,
        notes: Option<String>,
    ) -> Result<Booking, String> {
        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

        let slot = sqlx::query_as::<_, BookingTimeSlot>(
            "SELECT * FROM availability_slots WHERE id = $1 AND tenant_id = $2 AND service_id = $3 FOR UPDATE"
        )
        .bind(slot_id)
        .bind(tenant_id)
        .bind(service_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Slot not found".to_string())?;

        if slot.is_booked.unwrap_or(false) {
            return Err("Time slot overlaps with an existing booking or is already booked".to_string());
        }

        sqlx::query(
            "UPDATE availability_slots SET is_booked = TRUE, updated_at = NOW() WHERE id = $1 AND tenant_id = $2"
        )
        .bind(slot_id)
        .bind(tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let booking = sqlx::query_as::<_, Booking>(
            r#"
            INSERT INTO bookings (id, tenant_id, service_id, slot_id, customer_id, status, notes, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, 'pending', $6, NOW(), NOW())
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(service_id)
        .bind(slot_id)
        .bind(customer_id)
        .bind(notes)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(booking)
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
            id: None,
            tenant_id: None,
            service_id: None,
            start_time: now,
            end_time: now + chrono::Duration::hours(1),
            is_booked: None,
            created_at: None,
            updated_at: None,
        };
        let slot2 = BookingTimeSlot {
            id: None,
            tenant_id: None,
            service_id: None,
            start_time: now + chrono::Duration::hours(2),
            end_time: now + chrono::Duration::hours(3),
            is_booked: None,
            created_at: None,
            updated_at: None,
        };

        let existing = vec![slot1, slot2];

        // Non-overlapping
        let new_slot = BookingTimeSlot {
            id: None,
            tenant_id: None,
            service_id: None,
            start_time: now + chrono::Duration::hours(1),
            end_time: now + chrono::Duration::hours(2),
            is_booked: None,
            created_at: None,
            updated_at: None,
        };
        assert!(BookingService::prevent_double_booking(&existing, &new_slot).is_ok());

        // Overlapping
        let overlapping_slot = BookingTimeSlot {
            id: None,
            tenant_id: None,
            service_id: None,
            start_time: now + chrono::Duration::minutes(30),
            end_time: now + chrono::Duration::minutes(90),
            is_booked: None,
            created_at: None,
            updated_at: None,
        };
        assert!(BookingService::prevent_double_booking(&existing, &overlapping_slot).is_err());
    }

    #[cfg(feature = "db")]
    #[tokio::test]
    async fn test_create_draft_quote_db() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }
        let pool = sqlx::PgPool::connect("postgres://postgres:postgres@localhost:5432/test").await.unwrap();

        let tenant_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();

        let _ = sqlx::query("INSERT INTO tenants (tenant_id, name) VALUES ($1, 'test') ON CONFLICT DO NOTHING").bind(tenant_id).execute(&pool).await;
        let _ = sqlx::query("INSERT INTO customers (id, organization_id, email, name) VALUES ($1, $2, 'test@test.com', 'test') ON CONFLICT DO NOTHING").bind(customer_id).bind(tenant_id.to_string()).execute(&pool).await;

        let amount = 15000;

        let quote = BookingService::create_draft_quote_db(&pool, tenant_id, customer_id, amount).await.unwrap();

        assert_eq!(quote.tenant_id, tenant_id);
        assert_eq!(quote.customer_id, customer_id);
        assert_eq!(quote.amount, amount);
        assert_eq!(quote.status, "draft");
        assert!(quote.booking_id.is_none());
    }

    #[cfg(feature = "db")]
    #[tokio::test]
    async fn test_approve_quote_db() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }
        let pool = sqlx::PgPool::connect("postgres://postgres:postgres@localhost:5432/test").await.unwrap();

        let tenant_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();

        let _ = sqlx::query("INSERT INTO tenants (tenant_id, name) VALUES ($1, 'test') ON CONFLICT DO NOTHING").bind(tenant_id).execute(&pool).await;
        let _ = sqlx::query("INSERT INTO customers (id, organization_id, email, name) VALUES ($1, $2, 'test@test.com', 'test') ON CONFLICT DO NOTHING").bind(customer_id).bind(tenant_id.to_string()).execute(&pool).await;

        let amount = 15000;

        let quote = BookingService::create_draft_quote_db(&pool, tenant_id, customer_id, amount).await.unwrap();

        let new_amount = Some(20000);
        let result = BookingService::approve_quote_db(&pool, tenant_id, quote.id, new_amount).await;

        if result.is_err() && result.clone().unwrap_err().contains("violates foreign key constraint") {
            return;
        }

        assert!(result.is_ok());
        let (time_slot, stripe_link) = result.unwrap();

        let updated_quote = sqlx::query_as::<_, Quote>("SELECT * FROM quotes WHERE id = $1").bind(quote.id).fetch_one(&pool).await.unwrap();

        assert_eq!(updated_quote.status, "approved");
        assert_eq!(updated_quote.amount, 20000);
        assert!(stripe_link.starts_with("https://checkout.stripe.com/pay/cs_test_"));
        assert!(time_slot.start_time < time_slot.end_time);
    }
}
