use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::db::get_pool;
use sqlx::Row;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

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

    pub async fn get_bookings(tenant_id: &str) -> Result<Vec<BookingRecord>, String> {
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
        }).collect();

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
    SyncCalendarRequest, SyncCalendarResponse, ReserveTimeSlotRequest, ReserveTimeSlotResponse, CreateConversationalCheckoutRequest,
    ConversationalCheckoutSession,
};

const TIMESLOT_LOCK_TTL: Duration = Duration::from_secs(60);
const INVENTORY_LOCK_TTL: Duration = Duration::from_secs(15 * 60);
const INVENTORY_CAPACITY_LOCK_TTL: Duration = Duration::from_secs(10);

#[derive(Debug, Clone, PartialEq, Eq)]
struct SoftLockReceipt {
    key: String,
    owner: String,
}

#[derive(Debug)]
struct LocalSoftLock {
    owner: String,
    expires_at: Instant,
}

#[derive(Debug, Default)]
struct LocalBookingSoftLockStore {
    locks: Mutex<HashMap<String, LocalSoftLock>>,
}

impl LocalBookingSoftLockStore {
    fn new() -> Self {
        Self {
            locks: Mutex::new(HashMap::new()),
        }
    }

    async fn acquire(&self, key: &str, owner: &str, ttl: Duration) -> bool {
        let mut locks = self.locks.lock().await;
        Self::prune_expired(&mut locks);
        if locks.contains_key(key) {
            return false;
        }

        locks.insert(
            key.to_string(),
            LocalSoftLock {
                owner: owner.to_string(),
                expires_at: Instant::now() + ttl,
            },
        );
        true
    }

    async fn release(&self, key: &str, owner: &str) -> bool {
        let mut locks = self.locks.lock().await;
        Self::prune_expired(&mut locks);
        match locks.get(key) {
            Some(lock) if lock.owner == owner => locks.remove(key).is_some(),
            _ => false,
        }
    }

    async fn exists(&self, key: &str) -> bool {
        let mut locks = self.locks.lock().await;
        Self::prune_expired(&mut locks);
        locks.contains_key(key)
    }

    async fn count_prefix(&self, prefix: &str) -> usize {
        let mut locks = self.locks.lock().await;
        Self::prune_expired(&mut locks);
        locks.keys().filter(|key| key.starts_with(prefix)).count()
    }

    fn prune_expired(locks: &mut HashMap<String, LocalSoftLock>) {
        let now = Instant::now();
        locks.retain(|_, lock| lock.expires_at > now);
    }
}

#[derive(Clone)]
struct BookingSoftLockStore {
    redis_client: Option<redis::Client>,
    local: Arc<LocalBookingSoftLockStore>,
}

impl BookingSoftLockStore {
    fn for_service(redis_client: Option<redis::Client>) -> Self {
        static LOCAL_LOCKS: OnceLock<Arc<LocalBookingSoftLockStore>> = OnceLock::new();
        Self {
            redis_client,
            local: LOCAL_LOCKS
                .get_or_init(|| Arc::new(LocalBookingSoftLockStore::new()))
                .clone(),
        }
    }

    #[cfg(test)]
    fn isolated_for_tests() -> Self {
        Self {
            redis_client: None,
            local: Arc::new(LocalBookingSoftLockStore::new()),
        }
    }

    async fn acquire_capacity_lock(
        &self,
        tenant_id: &str,
        product_id: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        owner: &str,
        ttl: Duration,
    ) -> Result<Option<SoftLockReceipt>, String> {
        let key = capacity_lock_key(tenant_id, product_id, start_time, end_time);
        self.acquire_key(key, owner, ttl).await
    }

    async fn is_capacity_locked(
        &self,
        tenant_id: &str,
        product_id: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<bool, String> {
        let key = capacity_lock_key(tenant_id, product_id, start_time, end_time);
        self.key_exists(&key).await
    }

    async fn acquire_inventory_lock(
        &self,
        tenant_id: &str,
        product_id: &str,
        session_id: &str,
        product_capacity: i64,
        ttl: Duration,
    ) -> Result<Option<SoftLockReceipt>, String> {
        if product_capacity <= 0 {
            return Ok(None);
        }

        let capacity_key = inventory_capacity_lock_key(tenant_id, product_id);
        let Some(capacity_lock) = self
            .acquire_key(
                capacity_key,
                session_id,
                INVENTORY_CAPACITY_LOCK_TTL,
            )
            .await?
        else {
            return Ok(None);
        };

        let active_locks = match self.active_inventory_lock_count(tenant_id, product_id).await {
            Ok(count) => count,
            Err(e) => {
                let _ = self.release(&capacity_lock).await;
                return Err(e);
            }
        };
        if active_locks >= product_capacity as usize {
            self.release(&capacity_lock).await?;
            return Ok(None);
        }

        let inventory_key = inventory_lock_key(tenant_id, product_id, session_id);
        let acquired = match self.acquire_key(inventory_key, session_id, ttl).await {
            Ok(lock) => lock,
            Err(e) => {
                let _ = self.release(&capacity_lock).await;
                return Err(e);
            }
        };
        self.release(&capacity_lock).await?;
        Ok(acquired)
    }

    async fn release(&self, receipt: &SoftLockReceipt) -> Result<bool, String> {
        if let Some(client) = &self.redis_client {
            return redis_release_if_owner(client, &receipt.key, &receipt.owner).await;
        }

        Ok(self.local.release(&receipt.key, &receipt.owner).await)
    }

    async fn acquire_key(
        &self,
        key: String,
        owner: &str,
        ttl: Duration,
    ) -> Result<Option<SoftLockReceipt>, String> {
        if let Some(client) = &self.redis_client {
            if redis_acquire_key(client, &key, owner, ttl).await? {
                return Ok(Some(SoftLockReceipt {
                    key,
                    owner: owner.to_string(),
                }));
            }
            return Ok(None);
        }

        if self.local.acquire(&key, owner, ttl).await {
            return Ok(Some(SoftLockReceipt {
                key,
                owner: owner.to_string(),
            }));
        }
        Ok(None)
    }

    async fn key_exists(&self, key: &str) -> Result<bool, String> {
        if let Some(client) = &self.redis_client {
            return redis_key_exists(client, key).await;
        }

        Ok(self.local.exists(key).await)
    }

    async fn active_inventory_lock_count(
        &self,
        tenant_id: &str,
        product_id: &str,
    ) -> Result<usize, String> {
        let prefix = inventory_lock_prefix(tenant_id, product_id);
        if let Some(client) = &self.redis_client {
            return redis_scan_count(client, &format!("{}*", prefix)).await;
        }

        Ok(self.local.count_prefix(&prefix).await)
    }
}

fn capacity_lock_key(
    tenant_id: &str,
    product_id: &str,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
) -> String {
    // Adheres to ohc:lock:{tenant_id}:{resource_type}:{resource_id}
    format!(
        "ohc:lock:{}:capacity:{}_{}_{}",
        tenant_id,
        product_id,
        start_time.timestamp(),
        end_time.timestamp()
    )
}

fn inventory_capacity_lock_key(tenant_id: &str, product_id: &str) -> String {
    format!("ohc:lock:{}:inventory_capacity:{}", tenant_id, product_id)
}

fn inventory_lock_prefix(tenant_id: &str, product_id: &str) -> String {
    format!("ohc:lock:{}:inventory:{}:", tenant_id, product_id)
}

fn inventory_lock_key(tenant_id: &str, product_id: &str, session_id: &str) -> String {
    format!("{}{}", inventory_lock_prefix(tenant_id, product_id), session_id)
}

async fn redis_connection(
    client: &redis::Client,
) -> Result<redis::aio::MultiplexedConnection, String> {
    client
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| e.to_string())
}

async fn redis_acquire_key(
    client: &redis::Client,
    key: &str,
    owner: &str,
    ttl: Duration,
) -> Result<bool, String> {
    let mut conn = redis_connection(client).await?;
    redis::cmd("SET")
        .arg(key)
        .arg(owner)
        .arg("NX")
        .arg("EX")
        .arg(ttl.as_secs().max(1))
        .query_async(&mut conn)
        .await
        .map_err(|e| e.to_string())
}

async fn redis_key_exists(client: &redis::Client, key: &str) -> Result<bool, String> {
    let mut conn = redis_connection(client).await?;
    redis::cmd("EXISTS")
        .arg(key)
        .query_async(&mut conn)
        .await
        .map_err(|e| e.to_string())
}

async fn redis_release_if_owner(
    client: &redis::Client,
    key: &str,
    owner: &str,
) -> Result<bool, String> {
    let mut conn = redis_connection(client).await?;
    let script = redis::Script::new(
        r#"
        if redis.call("get", KEYS[1]) == ARGV[1] then
            return redis.call("del", KEYS[1])
        else
            return 0
        end
        "#,
    );
    let released: i32 = script
        .key(key)
        .arg(owner)
        .invoke_async(&mut conn)
        .await
        .map_err(|e| e.to_string())?;
    Ok(released == 1)
}

async fn redis_scan_count(client: &redis::Client, pattern: &str) -> Result<usize, String> {
    let mut conn = redis_connection(client).await?;
    let mut cursor = 0_u64;
    let mut count = 0_usize;

    loop {
        let (next_cursor, keys): (u64, Vec<String>) = redis::cmd("SCAN")
            .arg(cursor)
            .arg("MATCH")
            .arg(pattern)
            .arg("COUNT")
            .arg(100)
            .query_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;
        count += keys.len();
        cursor = next_cursor;
        if cursor == 0 {
            break;
        }
    }

    Ok(count)
}

pub struct NativeBookingService {
    pub redis_client: Option<redis::Client>,
}

impl NativeBookingService {
    fn soft_lock_store(&self) -> BookingSoftLockStore {
        BookingSoftLockStore::for_service(self.redis_client.clone())
    }

    pub async fn confirm_booking(
        &self,
        booking_id: &str,
    ) -> Result<(), Status> {
        let pool = crate::db::get_pool();
        let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        // Extract tenant_id from the booking
        let tenant_id: String = sqlx::query_scalar("SELECT tenant_id FROM bookings WHERE id = $1")
            .bind(booking_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| Status::internal(format!("Booking not found: {}", e)))?;

        // Update booking state
        let update_res = sqlx::query("UPDATE bookings SET status = 'confirmed', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND status IN ('pending', 'pending_payment')")
            .bind(booking_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        if update_res.rows_affected() == 0 {
            let _ = tx.rollback().await;
            return Err(Status::failed_precondition("Booking cannot be confirmed from current state"));
        }

        // Simulate confirmation email via shared_tasks
        let task_id = Uuid::new_v4().to_string();
        let title = format!("Send Confirmation Email for Booking {}", booking_id);
        let desc = "Automatically send booking confirmation after successful deposit / confirmation.".to_string();

        if let Err(e) = sqlx::query(
            "INSERT INTO shared_tasks (id, organization_id, title, description, status) VALUES ($1, $2, $3, $4, 'PENDING')"
        )
        .bind(task_id)
        .bind(&tenant_id)
        .bind(title)
        .bind(desc)
        .execute(&mut *tx)
        .await {
             let _ = tx.rollback().await;
             return Err(Status::internal(e.to_string()));
        }

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(())
    }

    async fn product_inventory_capacity(
        tenant_id: &str,
        product_id: &str,
    ) -> Result<i64, Status> {
        let pool = crate::db::get_pool();
        let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        crate::common::auth_utils::set_org_context(&mut *tx, tenant_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let capacity: Option<i64> = sqlx::query_scalar(
            "SELECT inventory_count::BIGINT FROM products WHERE tenant_id = $1 AND id = $2"
        )
        .bind(tenant_id)
        .bind(product_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;
        capacity.ok_or_else(|| Status::not_found("Product inventory capacity not found"))
    }
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

        let product_id = req.product_id;
        let date_str = req.date;

        let date_parsed = chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .map_err(|_e| Status::invalid_argument("Invalid date format, use YYYY-MM-DD"))?;

        let pool = crate::db::get_pool();
        let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let rows = sqlx::query(
            "SELECT start_time, end_time FROM bookings \
             WHERE tenant_id = $1 AND product_id = $2 AND start_time::date = $3::date \
             AND COALESCE(status, 'pending') <> 'cancelled'"
        )
        .bind(&tenant_id)
        .bind(&product_id)
        .bind(&date_str)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let existing_slots: Vec<(DateTime<Utc>, DateTime<Utc>)> = rows.into_iter().filter_map(|row| {
            let st: Option<DateTime<Utc>> = row.get("start_time");
            let et: Option<DateTime<Utc>> = row.get("end_time");
            if let (Some(s), Some(e)) = (st, et) { Some((s, e)) } else { None }
        }).collect();

        // Fetch exceptions / business hours from availability_schedules (if any)
        let schedule_rows = sqlx::query(
            "SELECT business_hours, exceptions FROM availability_schedules WHERE tenant_id = $1"
        )
        .bind(&tenant_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let mut blocked_slots = Vec::new();
        for row in schedule_rows {
             let exceptions_json: serde_json::Value = row.try_get("exceptions").unwrap_or(serde_json::json!([]));
             if let Some(arr) = exceptions_json.as_array() {
                 for ex in arr {
                      let st_str = ex.get("start_time").and_then(|v| v.as_str()).unwrap_or("");
                      let et_str = ex.get("end_time").and_then(|v| v.as_str()).unwrap_or("");
                      if let (Ok(st), Ok(et)) = (DateTime::parse_from_rfc3339(st_str), DateTime::parse_from_rfc3339(et_str)) {
                          blocked_slots.push((st.with_timezone(&Utc), et.with_timezone(&Utc)));
                      }
                 }
             }
        }

        let _ = tx.commit().await;

        let soft_locks = self.soft_lock_store();
        let mut available_slots = vec![];
        for hour in 9..17 {
            let st_naive = date_parsed.and_hms_opt(hour, 0, 0).unwrap();
            let et_naive = date_parsed.and_hms_opt(hour + 1, 0, 0).unwrap();
            let st = DateTime::<Utc>::from_naive_utc_and_offset(st_naive, Utc);
            let et = DateTime::<Utc>::from_naive_utc_and_offset(et_naive, Utc);

            let mut overlap = false;
            let all_busy = existing_slots.iter().chain(blocked_slots.iter());
            for (est, eet) in all_busy {
                if st < *eet && et > *est {
                    overlap = true;
                    break;
                }
            }

            let soft_locked = soft_locks
                .is_capacity_locked(&tenant_id, &product_id, st, et)
                .await
                .map_err(Status::internal)?;

            if !overlap && !soft_locked {
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

        if end_time <= start_time {
            return Err(Status::invalid_argument("end_time must be after start_time"));
        }

        let booking_id = Uuid::new_v4().to_string();
        let soft_locks = self.soft_lock_store();
        let Some(capacity_lock) = soft_locks
            .acquire_capacity_lock(
                &tenant_id,
                &product_id,
                start_time,
                end_time,
                &booking_id,
                TIMESLOT_LOCK_TTL,
            )
            .await
            .map_err(Status::internal)?
        else {
            return Err(Status::already_exists("Time slot is currently being held by another request"));
        };

        let pool = crate::db::get_pool();
        let mut tx = match pool.begin().await {
            Ok(tx) => tx,
            Err(e) => {
                let _ = soft_locks.release(&capacity_lock).await;
                return Err(Status::internal(e.to_string()));
            }
        };
        if let Err(e) = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
            let _ = soft_locks.release(&capacity_lock).await;
            return Err(Status::internal(e.to_string()));
        }

        let travel_buffer: i64 = match sqlx::query_scalar(
            "SELECT COALESCE(travel_buffer_minutes, 0) FROM services WHERE tenant_id = $1 AND id = $2"
        )
        .bind(&tenant_id)
        .bind(&product_id)
        .fetch_optional(&mut *tx)
        .await
        {
            Ok(Some(buf)) => buf,
            Ok(None) => 0,
            Err(e) => {
                let _ = tx.rollback().await;
                let _ = soft_locks.release(&capacity_lock).await;
                return Err(Status::internal(e.to_string()));
            }
        };

        let buffered_start_time = start_time - chrono::Duration::minutes(travel_buffer);
        let buffered_end_time = end_time + chrono::Duration::minutes(travel_buffer);

        let overlap_count: i64 = match sqlx::query_scalar(
            "SELECT COUNT(*) FROM bookings \
             WHERE tenant_id = $1 AND product_id = $2 AND start_time < $4 AND end_time > $3 \
             AND COALESCE(status, 'pending') <> 'cancelled'"
        )
        .bind(&tenant_id)
        .bind(&product_id)
        .bind(&buffered_start_time)
        .bind(&buffered_end_time)
        .fetch_one(&mut *tx)
        .await
        {
            Ok(count) => count,
            Err(e) => {
                let _ = tx.rollback().await;
                let _ = soft_locks.release(&capacity_lock).await;
                return Err(Status::internal(e.to_string()));
            }
        };

        if overlap_count > 0 {
            let _ = tx.rollback().await;
            let _ = soft_locks.release(&capacity_lock).await;
            return Err(Status::already_exists("Time slot already booked"));
        }

        let initial_status = if req.requires_deposit { "pending_payment" } else { "pending" };
        let payment_intent_id = if req.requires_deposit { Some(format!("pi_test_{}", Uuid::new_v4().to_string().replace("-", ""))) } else { None };

        if let Err(e) = sqlx::query(
            "INSERT INTO bookings (id, tenant_id, customer_id, product_id, start_time, end_time, status, payment_intent_id) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(&booking_id)
        .bind(&tenant_id)
        .bind(&customer_id)
        .bind(&product_id)
        .bind(start_time)
        .bind(end_time)
        .bind(initial_status)
        .bind(&payment_intent_id)
        .execute(&mut *tx)
        .await
        {
            let _ = tx.rollback().await;
            let _ = soft_locks.release(&capacity_lock).await;
            return Err(Status::internal(e.to_string()));
        }

        if let Err(e) = tx.commit().await {
            let _ = soft_locks.release(&capacity_lock).await;
            return Err(Status::internal(e.to_string()));
        }

        soft_locks
            .release(&capacity_lock)
            .await
            .map_err(Status::internal)?;

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

        let inventory_capacity =
            Self::product_inventory_capacity(&req.tenant_id, &req.product_id).await?;

        // If capacity is 1, check if there's an active POS transaction locking this item.
        if inventory_capacity <= 1 {
            let pos_lock_key = format!("ohc:lock:{}:inventory:{}", req.tenant_id, req.product_id);
            if let Some(client) = &self.redis_client {
                if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                    let is_locked: bool = redis::cmd("EXISTS").arg(&pos_lock_key).query_async(&mut conn).await.unwrap_or(false);
                    if is_locked {
                        return Err(Status::resource_exhausted("Product inventory is currently being checked out in-store"));
                    }
                }
            }
        }
        let soft_locks = self.soft_lock_store();
        let inventory_lock = soft_locks
            .acquire_inventory_lock(
                &req.tenant_id,
                &req.product_id,
                &session_id,
                inventory_capacity,
                INVENTORY_LOCK_TTL,
            )
            .await
            .map_err(Status::internal)?
            .ok_or_else(|| Status::resource_exhausted("Product inventory is currently fully held"))?;

        let inventory_lock_id = inventory_lock.key;

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

    async fn sync_calendar(
        &self,
        _request: Request<SyncCalendarRequest>,
    ) -> Result<Response<SyncCalendarResponse>, Status> {
        Ok(Response::new(SyncCalendarResponse {
            status: "Sync queued".to_string(),
        }))
    }
}


#[cfg(test)]
mod native_booking_tests {
    use super::*;
    use tonic::Request;
    use ::server_ohc::app::booking_engine_service_server::BookingEngineService;
    use ::server_ohc::app::{
    ReserveTimeSlotRequest, CreateConversationalCheckoutRequest};

    #[tokio::test]
    async fn local_capacity_lock_blocks_and_releases_timeslot() {
        let locks = BookingSoftLockStore::isolated_for_tests();
        let start_time = DateTime::parse_from_rfc3339("2026-06-06T16:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        let end_time = DateTime::parse_from_rfc3339("2026-06-06T17:00:00Z")
            .unwrap()
            .with_timezone(&Utc);

        let first = locks
            .acquire_capacity_lock(
                "tenant-1",
                "service-1",
                start_time,
                end_time,
                "booking-1",
                TIMESLOT_LOCK_TTL,
            )
            .await
            .unwrap()
            .expect("first capacity hold should be acquired");

        assert!(
            locks
                .is_capacity_locked("tenant-1", "service-1", start_time, end_time)
                .await
                .unwrap()
        );
        assert!(
            locks
                .acquire_capacity_lock(
                    "tenant-1",
                    "service-1",
                    start_time,
                    end_time,
                    "booking-2",
                    TIMESLOT_LOCK_TTL,
                )
                .await
                .unwrap()
                .is_none()
        );

        assert!(locks.release(&first).await.unwrap());
        assert!(
            !locks
                .is_capacity_locked("tenant-1", "service-1", start_time, end_time)
                .await
                .unwrap()
        );
    }

    #[tokio::test]
    async fn local_inventory_locks_enforce_capacity_until_release() {
        let locks = BookingSoftLockStore::isolated_for_tests();

        let first = locks
            .acquire_inventory_lock(
                "tenant-1",
                "product-1",
                "session-1",
                1,
                INVENTORY_LOCK_TTL,
            )
            .await
            .unwrap()
            .expect("first inventory hold should be acquired");

        assert_eq!(
            locks
                .active_inventory_lock_count("tenant-1", "product-1")
                .await
                .unwrap(),
            1
        );
        assert!(
            locks
                .acquire_inventory_lock(
                    "tenant-1",
                    "product-1",
                    "session-2",
                    1,
                    INVENTORY_LOCK_TTL,
                )
                .await
                .unwrap()
                .is_none()
        );

        assert!(locks.release(&first).await.unwrap());
        assert!(
            locks
                .acquire_inventory_lock(
                    "tenant-1",
                    "product-1",
                    "session-2",
                    1,
                    INVENTORY_LOCK_TTL,
                )
                .await
                .unwrap()
                .is_some()
        );
    }

    #[tokio::test]
    async fn test_native_booking_invalid_timeslot_format() {
        let svc = NativeBookingService { redis_client: None };
        let mut req = Request::new(ReserveTimeSlotRequest {
            tenant_id: "t1".to_string(),
            customer_id: "c1".to_string(),
            product_id: "p1".to_string(),
            start_time: "invalid_time".to_string(),
            end_time: "invalid_time".to_string(),
            requires_deposit: false,
            timezone: "UTC".to_string(),
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
    #[ignore = "requires a migrated OHC_DATABASE_URL with product inventory rows"]
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

    #[tokio::test]
    async fn test_native_reserve_time_slot_with_deposit() {
        let svc = NativeBookingService { redis_client: None };
        let mut req = Request::new(ReserveTimeSlotRequest {
            tenant_id: "t1".to_string(),
            customer_id: "c1".to_string(),
            product_id: "p1".to_string(),
            start_time: "invalid_time".to_string(),
            end_time: "invalid_time".to_string(),
            requires_deposit: true,
            timezone: "UTC".to_string(),
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
}
