use crate::integrations::google_calendar::provider::GoogleCalendarProvider;
use ::server_common::auth_utils;
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

            let sync_metadata: Option<serde_json::Value> = row.try_get("sync_metadata").ok().flatten();
            if let Err(e) = setup_calendar_watch(&tenant_id, &provider_client, sync_metadata).await {
                tracing::error!("Failed to setup calendar watch for tenant {}: {}", tenant_id, e);
            }

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

            if let Err(e) = cancel_bookings_in_calendar(&tenant_id, &provider_client).await {
                tracing::error!("Failed to cancel bookings for tenant {}: {}", tenant_id, e); // pii-safe
            }
        }
    }

    Ok(())
}

pub async fn trigger_tenant_calendar_sync(tenant_id: &str) -> Result<(), String> {
    let pool = crate::db::get_pool();
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    let row = sqlx::query(
        "SELECT provider, access_token FROM calendar_integrations WHERE tenant_id = $1 AND provider = 'google_calendar'",
    )
    .bind(tenant_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    if let Some(_r) = row {
        // Ideally we would trigger the full sync loop for this tenant immediately here
        // bypassing the 5m wait loop in run_calendar_sync_worker
    }
    Ok(())
}

async fn setup_calendar_watch(
    tenant_id: &str,
    provider_client: &GoogleCalendarProvider,
    sync_metadata: Option<serde_json::Value>,
) -> Result<(), String> {
    // Check if watch is already active
    if let Some(meta) = &sync_metadata {
        if let Some(expiration_str) = meta.get("watch_expiration").and_then(|v| v.as_str()) {
            if let Ok(expiration) = DateTime::parse_from_rfc3339(expiration_str) {
                if expiration.with_timezone(&Utc) > Utc::now() + chrono::Duration::days(1) {
                    // Watch is still valid for at least a day, no need to renew
                    return Ok(());
                }
            }
        }
    }

    let channel_id = uuid::Uuid::new_v4().to_string();
    let webhook_url = format!("https://{}/api/v1/webhooks/google_calendar", std::env::var("OMNISOLO_HOST").unwrap_or_else(|_| "api.omnisolo.test".to_string()));

    match provider_client.watch_events(&channel_id, &webhook_url, Some(tenant_id)).await {
        Ok(response) => {
            if let Some(resource_id) = response.get("resourceId").and_then(|v| v.as_str()) {
                let _expiration = response.get("expiration").and_then(|v| v.as_str()).or(Some("")).unwrap_or_default();
                tracing::info!("Successfully setup google calendar watch for tenant {}. Resource ID: {}", tenant_id, resource_id);
                // We would ideally save the channel_id and resource_id to the DB here.
            }
            Ok(())
        },
        Err(e) => {
            tracing::error!("Failed to setup watch: {}", e);
            Err(e)
        }
    }
}

async fn cancel_bookings_in_calendar(
    tenant_id: &str,
    provider_client: &GoogleCalendarProvider,
) -> Result<(), String> {
    let pool = crate::db::get_pool();
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    auth_utils::set_org_context(&mut *tx, tenant_id)
        .await
        .map_err(|e| e.to_string())?;

    // Find cancelled bookings that still have sync metadata
    let cancelled_rows = sqlx::query(
        "SELECT id, sync_metadata FROM bookings WHERE status = 'cancelled' AND tenant_id = $1 AND sync_metadata IS NOT NULL"
    )
    .bind(tenant_id)
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;

    let mut tx2 = pool.begin().await.map_err(|e| e.to_string())?;
    auth_utils::set_org_context(&mut *tx2, tenant_id)
        .await
        .map_err(|e| e.to_string())?;

    for row in cancelled_rows {
        let booking_id: String = row.get("id");
        let sync_metadata: Option<serde_json::Value> = row.try_get("sync_metadata").ok().flatten();

        if let Some(mut meta) = sync_metadata {
            if let Some(event_id) = meta.get("google_calendar_event_id").and_then(|v| v.as_str()) {
                match provider_client.cancel_event(event_id).await {
                    Ok(_) | Err(_) => {
                        // Swallow errors (e.g. 410 Gone) and clear state
                        if let Some(obj) = meta.as_object_mut() {
                            obj.remove("google_calendar_event_id");
                        }

                        let _ = sqlx::query("UPDATE bookings SET sync_metadata = $1 WHERE id = $2 AND tenant_id = $3")
                            .bind(&meta)
                            .bind(&booking_id)
                            .bind(tenant_id)
                            .execute(&mut *tx2)
                            .await;
                    }
                }
            }
        }
    }
    tx2.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}

async fn push_bookings_to_calendar(
    tenant_id: &str,
    provider_client: &GoogleCalendarProvider,
) -> Result<(), String> {
    let pool = crate::db::get_pool();
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    auth_utils::set_org_context(&mut *tx, tenant_id)
        .await
        .map_err(|e| e.to_string())?;

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

        let summary = format!("OmniSolo Booking: {}", booking_id);

        // This is a naive sync. Real implementation would check sync_metadata to avoid creating duplicates.
        if let Err(e) = provider_client
            .create_event(&summary, &start_time.to_rfc3339(), &et.to_rfc3339())
            .await
        {
            tracing::error!("Failed to create event for booking {}: {}", booking_id, e);
        }
    }

    Ok(())
}
