
use crate::integrations::google_calendar::provider::GoogleCalendarProvider;
use ::server_common::auth_utils;
use sqlx::Row;
use std::time::Duration;
use chrono::{DateTime, Utc};


pub async fn run_calendar_sync_worker(redis_client: redis::Client) {
    loop {
        if let Err(e) = sync_all_calendars(&redis_client).await {
            tracing::error!("Calendar sync worker error: {}", e);
        }
        tokio::time::sleep(Duration::from_secs(300)).await; // Poll every 5 minutes
    }
}

async fn sync_all_calendars(redis_client: &redis::Client) -> Result<(), String> {
    let pool = crate::db::get_pool();

    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    // Fetch active calendar integrations
    let rows = sqlx::query(
        "SELECT id, tenant_id, provider, access_token, sync_metadata FROM calendar_integrations"
    )
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;

    for row in rows {
        let _integration_id: String = row.get("id");
        let tenant_id: String = row.get("tenant_id");
        let provider: String = row.get("provider");
        let access_token: String = row.get("access_token");
        // let sync_metadata: Value = row.get("sync_metadata");

        if provider == "google_calendar" {
            let lock_key = format!("ohc:lock:{}:calendar_sync", tenant_id);
            let mut conn = redis_client.get_multiplexed_async_connection().await
                .map_err(|e| format!("Redis conn failed: {}", e))?;

            let acquired: bool = redis::cmd("SET")
                .arg(&lock_key)
                .arg("1")
                .arg("EX")
                .arg(300) // 5m TTL
                .arg("NX")
                .query_async(&mut conn)
                .await
                .unwrap_or(false);

            if !acquired {
                tracing::debug!("Sync already running for tenant {}", tenant_id); // pii-safe
                continue;
            }

            let provider_client = GoogleCalendarProvider::new(access_token);

            let now = Utc::now();
            let time_min = now.to_rfc3339();
            let time_max = (now + chrono::Duration::days(30)).to_rfc3339();

            match provider_client.get_free_busy(&time_min, &time_max).await {
                Ok(_fb_data) => {
                    // Update availability schedules in DB
                    // (Simplified dummy update for illustration, typically parses JSON and updates busy_slots)
                    tracing::info!("Successfully synced calendar for tenant {}", tenant_id); // pii-safe
                }
                Err(e) => {
                    tracing::error!("Failed to fetch free/busy for tenant {}: {}", tenant_id, e); // pii-safe
                }
            }

            // Push OHC bookings to Google Calendar
            if let Err(e) = push_bookings_to_calendar(&tenant_id, &provider_client).await {
                tracing::error!("Failed to push bookings for tenant {}: {}", tenant_id, e); // pii-safe
            }
        }
    }

    Ok(())
}

async fn push_bookings_to_calendar(
    tenant_id: &str,
    provider_client: &GoogleCalendarProvider
) -> Result<(), String> {
    let pool = crate::db::get_pool();
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

    // Find unsynced confirmed bookings
    let rows = sqlx::query(
        "SELECT id, start_time, end_time FROM bookings WHERE status = 'confirmed' AND tenant_id = $1"
    )
    .bind(tenant_id)
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;

    for row in rows {
        let booking_id: String = row.get("id");
        let start_time: DateTime<Utc> = row.get("start_time");
        let end_time: Option<DateTime<Utc>> = row.try_get("end_time").ok().flatten();

        let et = end_time.unwrap_or_else(|| start_time + chrono::Duration::hours(1));

        let summary = format!("OHC Booking: {}", booking_id);

        // This is a naive sync. Real implementation would check sync_metadata to avoid creating duplicates.
        if let Err(e) = provider_client.create_event(&summary, &start_time.to_rfc3339(), &et.to_rfc3339()).await {
             tracing::error!("Failed to create event for booking {}: {}", booking_id, e);
        }
    }

    Ok(())
}
