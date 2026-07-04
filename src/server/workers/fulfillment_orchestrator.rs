use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;
use crate::services::booking::{BookingService, BookingTimeSlot};

pub struct FulfillmentOrchestrator;

impl FulfillmentOrchestrator {
    pub async fn process_inquiry_postgres(
        pool: &sqlx::PgPool,
        tenant_id: &str,
        customer_id: &str,
        service_id: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        _message: &str,
        message_id: &str,
        _source: &str,
        _sender_id: &str,
    ) -> Result<String, String> {
        let existing_bookings_rows = sqlx::query(
            "SELECT start_time, end_time FROM bookings WHERE tenant_id = $1 AND service_id = $2 AND status != 'cancelled'"
        )
        .bind(tenant_id)
        .bind(service_id)
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut existing_bookings: Vec<crate::services::booking::BookingTimeSlot> = Vec::new();
        for r in existing_bookings_rows {
            use sqlx::Row;
            if let (Ok(st), Ok(et)) = (r.try_get::<DateTime<Utc>, _>("start_time"), r.try_get::<DateTime<Utc>, _>("end_time")) {
                existing_bookings.push(crate::services::booking::BookingTimeSlot {
                    start_time: st,
                    end_time: et,
                });
            }
        }

        let new_slot = crate::services::booking::BookingTimeSlot {
            start_time,
            end_time,
        };

        if let Err(e) = crate::services::booking::BookingService::prevent_double_booking(&existing_bookings, &new_slot) {
            return Err(e);
        }

        let base_price_cents = 5500;
        let price_cents = crate::pricing::engine::apply_yield_management(
            pool,
            tenant_id,
            service_id,
            start_time,
            base_price_cents,
        )
        .await;

        let price = (price_cents as f64) / 100.0;
        let is_surge = price_cents > base_price_cents;

        let proposed_action = serde_json::json!({
            "feature_type": "fulfillment_draft",
            "service": service_id,
            "start_time": start_time.to_rfc3339(),
            "end_time": end_time.to_rfc3339(),
            "price": price,
            "is_surge": is_surge,
            "inbox_message_id": message_id
        });

        let agent_feed_item_id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES ($1, $2, 'triage.inquiry', $3, $4, 'PENDING_APPROVAL', NOW(), NOW())"
        )
        .bind(&agent_feed_item_id)
        .bind(tenant_id)
        .bind(serde_json::json!({
            "customer_id": customer_id
        }))
        .bind(proposed_action)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload, created_at, updated_at) VALUES ($1, $2, 'Operations', 'Fulfillment Draft', 'DRAFT', 'DraftForReview', $3, NOW(), NOW())"
        )
        .bind(&agent_feed_item_id)
        .bind(tenant_id)
        .bind(serde_json::json!({
            "feature_type": "fulfillment_draft",
            "inbox_message_id": message_id,
            "customer_id": customer_id,
        }))
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(agent_feed_item_id)
    }

    pub async fn process_inquiry_sqlite(
        pool: &sqlx::SqlitePool,
        tenant_id: &str,
        customer_id: &str,
        service_id: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        _message: &str,
        message_id: &str,
        _source: &str,
        _sender_id: &str,
    ) -> Result<String, String> {
        let existing_bookings_rows = sqlx::query(
            "SELECT start_time, end_time FROM bookings WHERE tenant_id = ? AND service_id = ? AND status != 'cancelled'"
        )
        .bind(tenant_id)
        .bind(service_id)
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut existing_bookings: Vec<crate::services::booking::BookingTimeSlot> = Vec::new();
        for r in existing_bookings_rows {
            use sqlx::Row;
            if let (Ok(st), Ok(et)) = (r.try_get::<DateTime<Utc>, _>("start_time"), r.try_get::<DateTime<Utc>, _>("end_time")) {
                existing_bookings.push(crate::services::booking::BookingTimeSlot {
                    start_time: st,
                    end_time: et,
                });
            }
        }

        let new_slot = crate::services::booking::BookingTimeSlot {
            start_time,
            end_time,
        };

        if let Err(e) = crate::services::booking::BookingService::prevent_double_booking(&existing_bookings, &new_slot) {
            return Err(e);
        }

        let base_price_cents = 5500;

        let price = (base_price_cents as f64) / 100.0;
        let is_surge = false;

        let proposed_action = serde_json::json!({
            "feature_type": "fulfillment_draft",
            "service": service_id,
            "start_time": start_time.to_rfc3339(),
            "end_time": end_time.to_rfc3339(),
            "price": price,
            "is_surge": is_surge,
            "inbox_message_id": message_id
        });

        let agent_feed_item_id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES (?, ?, 'triage.inquiry', ?, ?, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
        )
        .bind(&agent_feed_item_id)
        .bind(tenant_id)
        .bind(serde_json::json!({
            "customer_id": customer_id
        }).to_string())
        .bind(proposed_action.to_string())
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload, created_at, updated_at) VALUES (?, ?, 'Operations', 'Fulfillment Draft', 'DRAFT', 'DraftForReview', ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
        )
        .bind(&agent_feed_item_id)
        .bind(tenant_id)
        .bind(serde_json::json!({
            "feature_type": "fulfillment_draft",
            "inbox_message_id": message_id,
            "customer_id": customer_id,
        }).to_string())
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(agent_feed_item_id)
    }
}
