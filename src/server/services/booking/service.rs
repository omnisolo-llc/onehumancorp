use std::sync::Arc;
use crate::db::{DB, DbStore};
use crate::domain::booking::{Quote, Booking};
use uuid::Uuid;
use chrono::Utc;

pub struct BookingService {
    db: Arc<DB>,
}

impl BookingService {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn draft_quote(&self, org_id: &str, customer_id: &str, amount_cents: i64, description: &str) -> Result<Quote, String> {
        let id = Uuid::new_v4().to_string();
        let created_at_unix = Utc::now().timestamp();
        let q = "INSERT INTO quotes (id, organization_id, customer_id, amount_cents, description, status, created_at_unix) VALUES ($1, $2, $3, $4, $5, 'DRAFT', $6)";

        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(q)
                    .bind(&id)
                    .bind(org_id)
                    .bind(customer_id)
                    .bind(amount_cents)
                    .bind(description)
                    .bind(created_at_unix)
                    .execute(&self.db.pool).await
                    .map_err(|e| e.to_string())?;
            },
            DbStore::Sqlite(pool) => {
                sqlx::query(q)
                    .bind(&id)
                    .bind(org_id)
                    .bind(customer_id)
                    .bind(amount_cents)
                    .bind(description)
                    .bind(created_at_unix)
                    .execute(pool).await
                    .map_err(|e| e.to_string())?;
            }
        }

        Ok(Quote {
            id,
            organization_id: org_id.to_string(),
            customer_id: customer_id.to_string(),
            amount_cents,
            description: description.to_string(),
            status: "DRAFT".to_string(),
            created_at_unix,
        })
    }

    pub async fn approve_quote(&self, org_id: &str, quote_id: &str) -> Result<Quote, String> {
        let q = "UPDATE quotes SET status = 'APPROVED' WHERE id = $1 AND organization_id = $2 RETURNING id, organization_id, customer_id, amount_cents, description, status, created_at_unix";

        match &self.db.store {
            DbStore::Postgres => {
                use sqlx::Row;
                let row = sqlx::query(q)
                    .bind(quote_id)
                    .bind(org_id)
                    .fetch_one(&self.db.pool).await
                    .map_err(|e| e.to_string())?;

                Ok(Quote {
                    id: row.try_get("id").unwrap_or_default(),
                    organization_id: row.try_get("organization_id").unwrap_or_default(),
                    customer_id: row.try_get("customer_id").unwrap_or_default(),
                    amount_cents: row.try_get("amount_cents").unwrap_or_default(),
                    description: row.try_get("description").unwrap_or_default(),
                    status: row.try_get("status").unwrap_or_default(),
                    created_at_unix: row.try_get("created_at_unix").unwrap_or_default(),
                })
            },
            DbStore::Sqlite(pool) => {
                use sqlx::Row;
                let row = sqlx::query(q)
                    .bind(quote_id)
                    .bind(org_id)
                    .fetch_one(pool).await
                    .map_err(|e| e.to_string())?;

                Ok(Quote {
                    id: row.try_get("id").unwrap_or_default(),
                    organization_id: row.try_get("organization_id").unwrap_or_default(),
                    customer_id: row.try_get("customer_id").unwrap_or_default(),
                    amount_cents: row.try_get("amount_cents").unwrap_or_default(),
                    description: row.try_get("description").unwrap_or_default(),
                    status: row.try_get("status").unwrap_or_default(),
                    created_at_unix: row.try_get("created_at_unix").unwrap_or_default(),
                })
            }
        }
    }

    pub async fn create_booking(&self, org_id: &str, customer_id: &str, quote_id: Option<String>, start_time_unix: i64, end_time_unix: i64) -> Result<Booking, String> {
        // Prevent double booking
        let check_q = "SELECT COUNT(*) FROM bookings WHERE organization_id = $1 AND start_time_unix < $2 AND end_time_unix > $3";
        let count: i64 = match &self.db.store {
            DbStore::Postgres => {
                use sqlx::Row;
                let row = sqlx::query(check_q)
                    .bind(org_id)
                    .bind(end_time_unix)
                    .bind(start_time_unix)
                    .fetch_one(&self.db.pool).await
                    .map_err(|e| e.to_string())?;
                row.get(0)
            },
            DbStore::Sqlite(pool) => {
                use sqlx::Row;
                let row = sqlx::query(check_q)
                    .bind(org_id)
                    .bind(end_time_unix)
                    .bind(start_time_unix)
                    .fetch_one(pool).await
                    .map_err(|e| e.to_string())?;
                row.get(0)
            }
        };

        if count > 0 {
            return Err("Double booking detected".to_string());
        }

        let id = Uuid::new_v4().to_string();
        let payment_link = Some(format!("https://pay.ohc.io/{}", Uuid::new_v4()));
        let q = "INSERT INTO bookings (id, organization_id, customer_id, quote_id, start_time_unix, end_time_unix, status, payment_link) VALUES ($1, $2, $3, $4, $5, $6, 'PENDING', $7)";

        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(q)
                    .bind(&id)
                    .bind(org_id)
                    .bind(customer_id)
                    .bind(&quote_id)
                    .bind(start_time_unix)
                    .bind(end_time_unix)
                    .bind(&payment_link)
                    .execute(&self.db.pool).await
                    .map_err(|e| e.to_string())?;
            },
            DbStore::Sqlite(pool) => {
                sqlx::query(q)
                    .bind(&id)
                    .bind(org_id)
                    .bind(customer_id)
                    .bind(&quote_id)
                    .bind(start_time_unix)
                    .bind(end_time_unix)
                    .bind(&payment_link)
                    .execute(pool).await
                    .map_err(|e| e.to_string())?;
            }
        }

        Ok(Booking {
            id,
            organization_id: org_id.to_string(),
            customer_id: customer_id.to_string(),
            quote_id,
            start_time_unix,
            end_time_unix,
            status: "PENDING".to_string(),
            payment_link,
        })
    }
}
