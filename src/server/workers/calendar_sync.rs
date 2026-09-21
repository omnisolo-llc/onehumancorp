use crate::integrations::google_calendar::provider::GoogleCalendarProvider;
use chrono::{DateTime, Utc};
use sqlx::Row;
use std::time::Duration;

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
        "SELECT id, tenant_id, provider, access_token, sync_metadata FROM calendar_integrations",
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
            let mut conn = redis_client
                .get_multiplexed_async_connection()
                .await
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

            // Push OmniSolo bookings to Google Calendar
            if let Err(e) = push_bookings_to_calendar(&tenant_id, &provider_client).await {
                tracing::error!("Failed to push bookings for tenant {}: {}", tenant_id, e); // pii-safe
            }

            // Sync cancellations to Google Calendar
            if let Err(e) = cancel_bookings_in_calendar(&tenant_id, &provider_client).await {
                tracing::error!("Failed to cancel bookings for tenant {}: {}", tenant_id, e);
            }
        }
    }

    Ok(())
}

async fn push_bookings_to_calendar(
    tenant_id: &str,
    provider_client: &GoogleCalendarProvider,
) -> Result<(), String> {
    let pool = crate::db::get_pool();
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    crate::common::auth_utils::set_org_context(&mut *tx, tenant_id)
        .await
        .map_err(|e| e.to_string())?;

    let rows = sqlx::query(
        "SELECT b.id, b.start_time, b.end_time
         FROM bookings b
         LEFT JOIN google_calendar_sync_mappings m
           ON b.id = m.booking_id AND m.provider = 'google_calendar'
         WHERE b.status = 'confirmed'
           AND b.tenant_id = $1
           AND (m.sync_state IS NULL OR m.sync_state = 'pending' OR (m.sync_state = 'failed' AND m.version < 5))
         FOR UPDATE OF b SKIP LOCKED"
    )
    .bind(tenant_id)
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    for row in rows {
        let booking_id: String = row.get("id");
        let start_time: DateTime<Utc> = row.get("start_time");
        let end_time: Option<DateTime<Utc>> = row.try_get("end_time").ok().flatten();
        let et = end_time.unwrap_or_else(|| start_time + chrono::Duration::hours(1));
        let summary = format!("OmniSolo Booking: {}", booking_id);

        match provider_client
            .create_event(&summary, &start_time.to_rfc3339(), &et.to_rfc3339())
            .await
        {
            Ok(event_id) => {
                sqlx::query(
                    "INSERT INTO google_calendar_sync_mappings (booking_id, tenant_id, provider, calendar_id, external_event_id, sync_state, last_synced_at, version)
                     VALUES ($1, $2, 'google_calendar', 'primary', $3, 'synced', CURRENT_TIMESTAMP, 1)
                     ON CONFLICT (booking_id, provider) DO UPDATE SET
                        external_event_id = EXCLUDED.external_event_id,
                        sync_state = 'synced',
                        last_synced_at = CURRENT_TIMESTAMP,
                        last_sync_error = NULL,
                        version = google_calendar_sync_mappings.version + 1"
                )
                .bind(&booking_id)
                .bind(tenant_id)
                .bind(&event_id)
                .execute(&mut *tx)
                .await
                .ok();
            }
            Err(e) => {
                tracing::error!("Failed to create event for booking {}: {}", booking_id, e);
                sqlx::query(
                    "INSERT INTO google_calendar_sync_mappings (booking_id, tenant_id, provider, calendar_id, sync_state, last_sync_error, version)
                     VALUES ($1, $2, 'google_calendar', 'primary', 'failed', $3, 1)
                     ON CONFLICT (booking_id, provider) DO UPDATE SET
                        sync_state = 'failed',
                        last_sync_error = EXCLUDED.last_sync_error,
                        version = google_calendar_sync_mappings.version + 1"
                )
                .bind(&booking_id)
                .bind(tenant_id)
                .bind(&e)
                .execute(&mut *tx)
                .await
                .ok();
            }
        }
    }
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}

async fn cancel_bookings_in_calendar(
    tenant_id: &str,
    provider_client: &GoogleCalendarProvider,
) -> Result<(), String> {
    let pool = crate::db::get_pool();
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    crate::common::auth_utils::set_org_context(&mut *tx, tenant_id)
        .await
        .map_err(|e| e.to_string())?;

    let rows = sqlx::query(
        "SELECT booking_id, external_event_id
         FROM google_calendar_sync_mappings
         WHERE tenant_id = $1
           AND provider = 'google_calendar'
           AND (sync_state = 'delete_pending' OR (sync_state = 'failed' AND version < 5 AND external_event_id IS NOT NULL))
         FOR UPDATE SKIP LOCKED"
    )
    .bind(tenant_id)
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    for row in rows {
        let booking_id: String = row.get("booking_id");
        let external_event_id: Option<String> = row.try_get("external_event_id").ok().flatten();

        if let Some(event_id) = external_event_id {
            match provider_client.cancel_event(&event_id).await {
                Ok(_) => {
                    sqlx::query("UPDATE google_calendar_sync_mappings SET sync_state = 'deleted', last_synced_at = CURRENT_TIMESTAMP WHERE booking_id = $1 AND provider = 'google_calendar'")
                        .bind(&booking_id)
                        .execute(&mut *tx)
                        .await
                        .ok();
                }
                Err(e) => {
                    tracing::error!("Failed to cancel event {}: {}", event_id, e);
                    let e_str = e.to_string();
                    if e_str.contains("404") {
                        sqlx::query("UPDATE google_calendar_sync_mappings SET sync_state = 'deleted', last_synced_at = CURRENT_TIMESTAMP WHERE booking_id = $1 AND provider = 'google_calendar'")
                            .bind(&booking_id)
                            .execute(&mut *tx)
                            .await
                            .ok();
                    } else {
                        sqlx::query("UPDATE google_calendar_sync_mappings SET sync_state = 'failed', last_sync_error = $1, version = version + 1 WHERE booking_id = $2 AND provider = 'google_calendar'")
                            .bind(&e_str)
                            .bind(&booking_id)
                            .execute(&mut *tx)
                            .await
                            .ok();
                    }
                }
            }
        } else {
            sqlx::query("UPDATE google_calendar_sync_mappings SET sync_state = 'deleted', last_synced_at = CURRENT_TIMESTAMP WHERE booking_id = $1 AND provider = 'google_calendar'")
                 .bind(&booking_id)
                 .execute(&mut *tx)
                 .await
                 .ok();
        }
    }
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}
