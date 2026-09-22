use crate::api::google_calendar::GoogleCalendarTokenStorage;
use crate::integrations::google_calendar::provider::GoogleCalendarProvider;
use ::server_common::auth_utils;
use chrono::{DateTime, Utc};
use sqlx::Row;
use std::sync::Arc;
use std::time::Duration;

pub async fn run_calendar_sync_worker(redis_client: redis::Client) {
    loop {
        if let Err(e) = sync_all_calendars(&redis_client).await {
            tracing::error!("Calendar sync worker error: {}", e);
        }
        tokio::time::sleep(Duration::from_secs(300)).await;
    }
}

async fn sync_all_calendars(redis_client: &redis::Client) -> Result<(), String> {
    let pool = crate::db::get_pool();
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    let rows = sqlx::query(
        "SELECT id, tenant_id, provider, access_token, refresh_token, expires_at, sync_metadata FROM calendar_integrations",
    )
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;

    for row in rows {
        let integration_id: String = row.get("id");
        let tenant_id: String = row.get("tenant_id");
        let provider: String = row.get("provider");

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
                .arg(300)
                .arg("NX")
                .query_async(&mut conn)
                .await
                .unwrap_or(false);

            if !acquired {
                continue;
            }

            reconcile_google_calendar(&tenant_id, &integration_id).await;
        }
    }

    Ok(())
}

pub async fn reconcile_google_calendar(tenant_id: &str, integration_id: &str) {
    let pool = crate::db::get_pool();
    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(_) => return,
    };

    let row: Result<(String, Option<String>, Option<DateTime<Utc>>, serde_json::Value), sqlx::Error> = sqlx::query_as(
        "SELECT access_token, refresh_token, expires_at, sync_metadata FROM calendar_integrations WHERE id = $1 AND tenant_id = $2"
    )
    .bind(integration_id)
    .bind(tenant_id)
    .fetch_one(&mut *tx)
    .await;

    if let Ok((access_token, refresh_token, expires_at, mut sync_metadata)) = row {
        let _ = tx.commit().await;

        let token_storage = Arc::new(GoogleCalendarTokenStorage {
            db: std::sync::Arc::new(pool.clone()),
            tenant_id: tenant_id.to_string(),
        });

        let provider_client = GoogleCalendarProvider::new_with_auth(
            access_token,
            refresh_token,
            expires_at,
            Some(token_storage),
        );

        let mut sync_token = sync_metadata
            .get("next_sync_token")
            .and_then(|t| t.as_str());

        match provider_client.list_events_sync(sync_token).await {
            Ok(resp) => {
                let mut tx = pool.begin().await.unwrap();
                let _ = sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
                    .bind(tenant_id)
                    .execute(&mut *tx)
                    .await;

                for event in resp.events {
                    if let Some(summary) = &event.summary {
                        if let Some(booking_id) = summary.strip_prefix("OmniSolo Booking: ") {
                            if event.status.as_deref() == Some("cancelled") {
                                let _ = sqlx::query("UPDATE bookings SET status = 'cancelled', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2 AND status != 'cancelled'")
                                    .bind(booking_id)
                                    .bind(tenant_id)
                                    .execute(&mut *tx)
                                    .await;
                            } else {
                                if let (Some(start), Some(end)) =
                                    (event.start.as_ref(), event.end.as_ref())
                                {
                                    if let (Ok(s), Ok(e)) = (
                                        chrono::DateTime::parse_from_rfc3339(start),
                                        chrono::DateTime::parse_from_rfc3339(end),
                                    ) {
                                        let _ = sqlx::query("UPDATE bookings SET start_time = $1, end_time = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3 AND tenant_id = $4")
                                            .bind(s.with_timezone(&chrono::Utc))
                                            .bind(e.with_timezone(&chrono::Utc))
                                            .bind(booking_id)
                                            .bind(tenant_id)
                                            .execute(&mut *tx)
                                            .await;
                                    }
                                }
                            }
                        }
                    }
                }

                sync_metadata["next_sync_token"] = resp
                    .next_sync_token
                    .map(serde_json::Value::String)
                    .unwrap_or(serde_json::Value::Null);
                let _ = sqlx::query(
                    "UPDATE calendar_integrations SET sync_metadata = $1 WHERE id = $2",
                )
                .bind(&sync_metadata)
                .bind(&integration_id)
                .execute(&mut *tx)
                .await;

                let _ = tx.commit().await;
            }
            Err(e) if e == "410 Gone" => {
                let mut tx = pool.begin().await.unwrap();
                sync_metadata["next_sync_token"] = serde_json::Value::Null;
                let _ = sqlx::query(
                    "UPDATE calendar_integrations SET sync_metadata = $1 WHERE id = $2",
                )
                .bind(&sync_metadata)
                .bind(&integration_id)
                .execute(&mut *tx)
                .await;
                let _ = tx.commit().await;
            }
            Err(_) => {}
        }

        let _ = push_bookings_to_calendar(tenant_id, &provider_client).await;
    } else {
        let _ = tx.rollback().await;
    }
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

    let rows = sqlx::query(
        "SELECT id, start_time, end_time FROM bookings WHERE status = 'confirmed' AND tenant_id = $1 AND (integration_metadata->>'google_event_id' IS NULL)"
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

        if let Err(e) = provider_client
            .create_event(&summary, &start_time.to_rfc3339(), &et.to_rfc3339())
            .await
        {
            tracing::error!("Failed to create event for booking {}: {}", booking_id, e);
        } else {
            let md = serde_json::json!({"google_event_id": "created"});
            let _ = sqlx::query(
                "UPDATE bookings SET integration_metadata = $1 WHERE id = $2 AND tenant_id = $3",
            )
            .bind(md)
            .bind(&booking_id)
            .bind(tenant_id)
            .execute(&mut *tx)
            .await;
        }
    }

    let rows_cancelled = sqlx::query(
        "SELECT id, integration_metadata FROM bookings WHERE status = 'cancelled' AND tenant_id = $1 AND (integration_metadata->>'google_event_id' IS NOT NULL) AND (integration_metadata->>'cancelled' IS NULL)"
    )
    .bind(tenant_id)
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    for row in rows_cancelled {
        let booking_id: String = row.get("id");
        let md = serde_json::json!({"google_event_id": "created", "cancelled": true});
        let _ = sqlx::query(
            "UPDATE bookings SET integration_metadata = $1 WHERE id = $2 AND tenant_id = $3",
        )
        .bind(md)
        .bind(&booking_id)
        .bind(tenant_id)
        .execute(&mut *tx)
        .await;
    }

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}
