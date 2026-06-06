use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::db::get_pool;
use sqlx::Row;
use ::server_utils::cache::HybridCache;
use std::sync::OnceLock;

static SERVICES_CACHE: OnceLock<HybridCache<Vec<Service>>> = OnceLock::new();
static BOOKINGS_CACHE: OnceLock<HybridCache<Vec<BookingRecord>>> = OnceLock::new();

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Quote {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub customer_id: Uuid,
    pub amount: i64,
    pub status: String,
    pub booking_id: Option<Uuid>,
    pub required_deposit: i64,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Invoice {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub invoice_type: String, // 'Deposit' or 'Final'
    pub amount: i64,
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
    pub product_id: String,
    pub quote_id: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub status: String,
}

pub struct BookingService;

impl BookingService {
    pub fn create_draft_quote(
        tenant_id: Uuid,
        customer_id: Uuid,
        amount: i64,
        required_deposit: i64,
        expires_at: DateTime<Utc>,
    ) -> Quote {
        Quote {
            id: Uuid::new_v4(),
            tenant_id,
            customer_id,
            amount,
            status: "draft".to_string(),
            booking_id: None,
            required_deposit,
            expires_at,
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

        // Phase 1 Dual-Write to new unified schema (Offerings table)
        let metadata = "{}"; // Empty JSONB for service
        let _ = sqlx::query(
            "INSERT INTO offerings (id, tenant_id, type, title, description, price_cents, metadata) \
             VALUES ($1, $2, 'service', $3, $4, $5, $6) \
             ON CONFLICT (id) DO UPDATE SET \
             title = EXCLUDED.title, \
             description = EXCLUDED.description, \
             price_cents = EXCLUDED.price_cents, \
             metadata = EXCLUDED.metadata"
        )
        .bind(&service.id)
        .bind(&service.tenant_id)
        .bind(&service.title)
        .bind(&service.description)
        .bind(service.price_cents)
        .bind(metadata)
        .execute(&mut *tx)
        .await;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn get_bookings(tenant_id: &str, mobile_optimized: bool) -> Result<Vec<BookingRecord>, String> {
        let cache_key = format!("booking:bookings:{}:{}", tenant_id, mobile_optimized);
        let cache = BOOKINGS_CACHE.get_or_init(|| HybridCache::new(None));

        if let Some(bookings) = cache.get(&cache_key).await {
            return Ok(bookings);
        }

        let pool = get_pool();
        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

        let rows = sqlx::query("SELECT id, tenant_id, customer_id, product_id, quote_id, start_time, end_time, status FROM bookings")
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        let bookings = rows.into_iter().map(|row| BookingRecord {
            id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            customer_id: row.get("customer_id"),
            product_id: row.get("product_id"),
            quote_id: row.try_get("quote_id").ok(),
            start_time: row.get("start_time"),
            end_time: row.get("end_time"),
            status: row.get("status"),
        }).collect::<Vec<_>>();

        cache.set(&cache_key, bookings.clone(), std::time::Duration::from_secs(60)).await;

        Ok(bookings)
    }

    pub async fn create_booking(booking: BookingRecord) -> Result<(), String> {
        let pool = get_pool();
        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_org_context(&mut *tx, &booking.tenant_id).await.map_err(|e| e.to_string())?;

        let now = chrono::Utc::now();
        if booking.start_time.signed_duration_since(now).num_hours() < 48 {
            tokio::spawn(async move {
                let _ = crate::dispatch_critical_sms("urgent_booking", "You have an urgent booking coming up soon!").await;
            });
        }

        sqlx::query(
            "INSERT INTO bookings (id, tenant_id, customer_id, product_id, quote_id, start_time, end_time, status) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(&booking.id)
        .bind(&booking.tenant_id)
        .bind(&booking.customer_id)
        .bind(&booking.product_id)
        .bind(&booking.quote_id)
        .bind(booking.start_time)
        .bind(booking.end_time)
        .bind(&booking.status)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        // Phase 1 Dual-Write to new unified schema (Transaction table)
        let amount_cents = 0; // Default amount since it's not present in BookingRecord
        let _ = sqlx::query(
            "INSERT INTO transactions (id, tenant_id, offering_id, customer_id, type, status, amount_cents) \
             VALUES ($1, $2, $3, $4, 'booking', $5, $6) \
             ON CONFLICT (id) DO NOTHING"
        )
        .bind(&booking.id)
        .bind(&booking.tenant_id)
        .bind(&booking.product_id)
        .bind(&booking.customer_id)
        .bind(&booking.status)
        .bind(amount_cents)
        .execute(&mut *tx)
        .await;

        tx.commit().await.map_err(|e| e.to_string())?;
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
        let required_deposit = 5000;
        let expires_at = Utc::now() + chrono::Duration::days(7);

        let quote = BookingService::create_draft_quote(tenant_id, customer_id, amount, required_deposit, expires_at);

        assert_eq!(quote.tenant_id, tenant_id);
        assert_eq!(quote.customer_id, customer_id);
        assert_eq!(quote.amount, amount);
        assert_eq!(quote.required_deposit, required_deposit);
        assert_eq!(quote.expires_at, expires_at);
        assert_eq!(quote.status, "draft");
        assert!(quote.booking_id.is_none());
    }

    #[test]
    fn test_approve_quote() {
        let tenant_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let amount = 15000;
        let required_deposit = 5000;
        let expires_at = Utc::now() + chrono::Duration::days(7);

        let mut quote = BookingService::create_draft_quote(tenant_id, customer_id, amount, required_deposit, expires_at);

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
use tonic::{Request, Response, Status};
use ::server_ohc::app::booking_engine_service_server::BookingEngineService;
use ::server_ohc::app::{
    CheckAvailabilityRequest, CheckAvailabilityResponse, TimeSlot,
    ReserveTimeSlotRequest, ReserveTimeSlotResponse, CreateConversationalCheckoutRequest,
    ConversationalCheckoutSession,
};

pub struct NativeBookingService {
    pub redis_client: Option<redis::Client>,
}

#[tonic::async_trait]
impl BookingEngineService for NativeBookingService {
    async fn check_availability(
        &self,
        request: Request<CheckAvailabilityRequest>,
    ) -> Result<Response<CheckAvailabilityResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => {
                let spiffe_id_str = request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
                ::server_auth::parse_spiffe_id(spiffe_id_str).map_err(|_| Status::unauthenticated("invalid spiffe id"))?.0
            }
        };

        if tenant_id.is_empty() {
            return Err(Status::unauthenticated("missing tenant identity in session"));
        }

        let mut req = request.into_inner();
        req.tenant_id = tenant_id.clone();

        let _product_id = req.product_id;
        let date_str = req.date;

        let pool = crate::db::get_pool();
        let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // Simplified query: find bookings overlapping with this date
        // In reality, you'd check specific business hours. We'll return dummy slots filtered by DB.
        let rows = sqlx::query(
            "SELECT start_time, end_time FROM bookings WHERE tenant_id = $1 AND start_time::date = $2::date"
        )
        .bind(&tenant_id)
        .bind(&date_str)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let _ = tx.commit().await;

        let existing_slots: Vec<(DateTime<Utc>, DateTime<Utc>)> = rows.into_iter().filter_map(|row| {
            let st: Option<DateTime<Utc>> = row.get("start_time");
            let et: Option<DateTime<Utc>> = row.get("end_time");
            if let (Some(s), Some(e)) = (st, et) { Some((s, e)) } else { None }
        }).collect();

        // Let's generate slots from 9 AM to 5 PM
        let date_parsed = chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .map_err(|_e| Status::invalid_argument("Invalid date format, use YYYY-MM-DD"))?;

        let mut available_slots = vec![];
        for hour in 9..17 {
            let st_naive = date_parsed.and_hms_opt(hour, 0, 0).unwrap();
            let et_naive = date_parsed.and_hms_opt(hour + 1, 0, 0).unwrap();
            let st = DateTime::<Utc>::from_naive_utc_and_offset(st_naive, Utc);
            let et = DateTime::<Utc>::from_naive_utc_and_offset(et_naive, Utc);

            let mut overlap = false;
            for (est, eet) in &existing_slots {
                if st < *eet && et > *est {
                    overlap = true;
                    break;
                }
            }

            if !overlap {
                available_slots.push(TimeSlot {
                    start_time: st.to_rfc3339(),
                    end_time: et.to_rfc3339(),
                });
            }
        }

        Ok(Response::new(CheckAvailabilityResponse { available_slots }))
    }

    async fn reserve_time_slot(
        &self,
        request: Request<ReserveTimeSlotRequest>,
    ) -> Result<Response<ReserveTimeSlotResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => {
                let spiffe_id_str = request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
                ::server_auth::parse_spiffe_id(spiffe_id_str).map_err(|_| Status::unauthenticated("invalid spiffe id"))?.0
            }
        };

        if tenant_id.is_empty() {
            return Err(Status::unauthenticated("missing tenant identity in session"));
        }

        let mut req = request.into_inner();
        req.tenant_id = tenant_id.clone();
        let customer_id = req.customer_id;
        let product_id = req.product_id;
        let start_time_str = req.start_time;
        let end_time_str = req.end_time;

        let start_time = DateTime::parse_from_rfc3339(&start_time_str)
            .map_err(|_| Status::invalid_argument("Invalid start_time RFC3339 format"))?
            .with_timezone(&Utc);
        let end_time = DateTime::parse_from_rfc3339(&end_time_str)
            .map_err(|_| Status::invalid_argument("Invalid end_time RFC3339 format"))?
            .with_timezone(&Utc);

        // Redis Lock
        let lock_key = format!("ohc:lock:{}:timeslot:{}", tenant_id, start_time.timestamp());
        if let Some(client) = &self.redis_client {
            let mut conn = client.get_multiplexed_async_connection().await
                .map_err(|e| Status::internal(format!("Redis conn failed: {}", e)))?;
            let acquired: bool = redis::cmd("SET")
                .arg(&lock_key)
                .arg("1")
                .arg("EX")
                .arg(60) // 60s TTL
                .arg("NX")
                .query_async(&mut conn)
                .await
                .unwrap_or(false);

            if !acquired {
                return Err(Status::already_exists("Time slot is currently being locked by another request"));
            }
        }

        // DB check inside transaction
        let pool = crate::db::get_pool();
        let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // Check overlaps
        let overlap_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM bookings WHERE tenant_id = $1 AND start_time < $3 AND end_time > $2"
        )
        .bind(&tenant_id)
        .bind(&start_time)
        .bind(&end_time)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        if overlap_count > 0 {
            let _ = tx.rollback().await;
            return Err(Status::already_exists("Time slot already booked"));
        }

        let booking_id = Uuid::new_v4().to_string();

        sqlx::query(
            "INSERT INTO bookings (id, tenant_id, customer_id, product_id, start_time, end_time, status) \
             VALUES ($1, $2, $3, $4, $5, $6, 'pending')"
        )
        .bind(&booking_id)
        .bind(&tenant_id)
        .bind(&customer_id)
        .bind(&product_id)
        .bind(start_time)
        .bind(end_time)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        // Generate dummy stripe link
        let deposit_stripe_link = format!("https://checkout.stripe.com/pay/cs_test_{}", booking_id.replace("-", ""));

        Ok(Response::new(ReserveTimeSlotResponse {
            booking_id,
            deposit_stripe_link,
        }))
    }

    async fn create_conversational_checkout(
        &self,
        request: Request<CreateConversationalCheckoutRequest>,
    ) -> Result<Response<ConversationalCheckoutSession>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => {
                let spiffe_id_str = request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
                ::server_auth::parse_spiffe_id(spiffe_id_str).map_err(|_| Status::unauthenticated("invalid spiffe id"))?.0
            }
        };

        if tenant_id.is_empty() {
            return Err(Status::unauthenticated("missing tenant identity in session"));
        }

        let mut req = request.into_inner();
        req.tenant_id = tenant_id.clone();

        let session_id = Uuid::new_v4().to_string();
        let expires_at = Utc::now() + chrono::Duration::minutes(15);

        let inventory_lock_id = format!("ohc:lock:{}:inventory:{}:{}", req.tenant_id, req.product_id, session_id);

        if let Some(client) = &self.redis_client {
            let mut conn = client.get_multiplexed_async_connection().await
                .map_err(|e| Status::internal(format!("Redis conn failed: {}", e)))?;
            let _acquired: bool = redis::cmd("SET")
                .arg(&inventory_lock_id)
                .arg("1")
                .arg("EX")
                .arg(900) // 15 min TTL
                .query_async(&mut conn)
                .await
                .unwrap_or(false);
        }

        let checkout_url = format!("https://checkout.stripe.com/pay/cs_test_{}", session_id.replace("-", ""));

        Ok(Response::new(ConversationalCheckoutSession {
            session_id,
            tenant_id: req.tenant_id,
            customer_id: req.customer_id,
            amount_cents: req.amount_cents,
            inventory_lock_id,
            checkout_url,
            status: "pending".to_string(),
            expires_at_unix: expires_at.timestamp(),
        }))
    }
}


#[cfg(test)]
mod native_booking_tests {
    use super::*;
    use tonic::Request;
    use ::server_ohc::app::booking_engine_service_server::BookingEngineService;
    use ::server_ohc::app::{ReserveTimeSlotRequest, CreateConversationalCheckoutRequest};

    #[tokio::test]
    async fn test_native_booking_invalid_timeslot_format() {
        let svc = NativeBookingService { redis_client: None };
        let mut req = Request::new(ReserveTimeSlotRequest {
            tenant_id: "t1".to_string(),
            customer_id: "c1".to_string(),
            product_id: "p1".to_string(),
            start_time: "invalid_time".to_string(),
            end_time: "invalid_time".to_string(),
        });
        req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
            spiffe_id: "test".to_string(),
            org_id: "t1".to_string(),
            agent_id: "test".to_string(),
        });

        let res = svc.reserve_time_slot(req).await;
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().code(), tonic::Code::InvalidArgument);
    }

    #[tokio::test]
    async fn test_native_check_availability_invalid_date() {
        let svc = NativeBookingService { redis_client: None };
        let mut req = Request::new(::server_ohc::app::CheckAvailabilityRequest {
            tenant_id: "t1".to_string(),
            product_id: "p1".to_string(),
            date: "invalid-date".to_string(),
        });
        req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
            spiffe_id: "test".to_string(),
            org_id: "t1".to_string(),
            agent_id: "test".to_string(),
        });

        let res = svc.check_availability(req).await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_native_create_conversational_checkout() {
        let svc = NativeBookingService { redis_client: None };
        let mut req = Request::new(CreateConversationalCheckoutRequest {
            tenant_id: "t1".to_string(),
            customer_id: "c1".to_string(),
            amount_cents: 1000,
            product_id: "p1".to_string(),
        });
        req.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
            spiffe_id: "test".to_string(),
            org_id: "t1".to_string(),
            agent_id: "test".to_string(),
        });

        let res = svc.create_conversational_checkout(req).await;
        assert!(res.is_ok());
        let session = res.unwrap().into_inner();
        assert_eq!(session.tenant_id, "t1");
        assert_eq!(session.customer_id, "c1");
        assert_eq!(session.amount_cents, 1000);
        assert!(session.checkout_url.starts_with("https://checkout.stripe.com/pay/cs_test_"));
        assert_eq!(session.status, "pending");
    }
}
