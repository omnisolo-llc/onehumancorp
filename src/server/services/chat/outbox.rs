//! At-least-once publication of committed chat events. Redis acknowledgements
//! describe active subscriptions, never a recipient's receipt or processing.
use crate::domain::repository::omnichannel_repo::{OmniChannelRepo, WidgetChatError};
use crate::redis_pool::RedisPool;
use crate::services::chat::models::ChatMessage;
use serde_json::json;
use sqlx::{FromRow, PgPool, Postgres, Transaction};
use std::time::Duration;
use tokio::sync::watch;
use uuid::Uuid;

const PUBLISH_TIMEOUT: Duration = Duration::from_millis(750);
const STEP_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_ATTEMPTS: i32 = 10;

#[derive(Debug, PartialEq, Eq)]
pub enum RelayOutcome {
    Idle,
    Published { message_id: Uuid, subscribers: u64 },
    RetryScheduled,
    Failed,
}

#[derive(FromRow)]
struct PendingEvent {
    id: String,
    tenant_id: String,
    message_id: Option<String>,
    message_tenant: Option<String>,
    retry_count: i32,
}

pub async fn relay_chat_event(
    pool: &PgPool,
    redis: &RedisPool,
    tenant_id: Uuid,
    event_id: &str,
) -> Result<RelayOutcome, sqlx::Error> {
    bounded_relay(pool, redis, Some((tenant_id, event_id))).await
}

async fn bounded_relay(
    pool: &PgPool,
    redis: &RedisPool,
    scope: Option<(Uuid, &str)>,
) -> Result<RelayOutcome, sqlx::Error> {
    tokio::time::timeout(STEP_TIMEOUT, relay_one(pool, redis, scope))
        .await
        .map_err(|_| sqlx::Error::PoolTimedOut)?
}

async fn fail_invalid_event(
    mut tx: Transaction<'_, Postgres>,
    event: &PendingEvent,
) -> Result<RelayOutcome, sqlx::Error> {
    sqlx::query("UPDATE ohc_job_queue SET status='FAILED',updated_at=clock_timestamp() WHERE id=$1 AND tenant_id=$2 AND job_type='publish_chat_event'")
        .bind(&event.id).bind(&event.tenant_id).execute(&mut *tx).await?;
    tx.commit().await?;
    Ok(RelayOutcome::Failed)
}

async fn relay_one(
    pool: &PgPool,
    redis: &RedisPool,
    scope: Option<(Uuid, &str)>,
) -> Result<RelayOutcome, sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL READ COMMITTED")
        .execute(&mut *tx)
        .await?;
    sqlx::query(
        "SELECT set_config('lock_timeout',CASE WHEN current_setting('lock_timeout')::interval=interval '0' OR current_setting('lock_timeout')::interval>interval '1 second' THEN '1s' ELSE current_setting('lock_timeout') END,true),set_config('statement_timeout',CASE WHEN current_setting('statement_timeout')::interval=interval '0' OR current_setting('statement_timeout')::interval>interval '3 seconds' THEN '3s' ELSE current_setting('statement_timeout') END,true)"
    ).execute(&mut *tx).await?;
    let event = if let Some((tenant_id, event_id)) = scope {
        server_common::auth_utils::set_org_context(&mut *tx, &tenant_id.to_string()).await?;
        sqlx::query_as::<_, PendingEvent>(
            "SELECT id,tenant_id,payload#>>'{message,id}' AS message_id,payload#>>'{message,tenant_id}' AS message_tenant,COALESCE(retry_count,0) AS retry_count FROM ohc_job_queue WHERE tenant_id=$1 AND id=$2 AND job_type='publish_chat_event' AND status='PENDING' AND next_retry_at<=clock_timestamp() FOR UPDATE SKIP LOCKED"
        ).bind(tenant_id.to_string()).bind(event_id).fetch_optional(&mut *tx).await?
    } else {
        // This is internal routing discovery, not content authority. Drop the
        // system role before reading the canonical message or publishing it.
        sqlx::query("SET LOCAL ROLE ohc_bypassrls")
            .execute(&mut *tx)
            .await?;
        sqlx::query_as::<_, PendingEvent>(
            "SELECT id,tenant_id,payload#>>'{message,id}' AS message_id,payload#>>'{message,tenant_id}' AS message_tenant,COALESCE(retry_count,0) AS retry_count FROM ohc_job_queue WHERE job_type='publish_chat_event' AND status='PENDING' AND next_retry_at<=clock_timestamp() ORDER BY next_retry_at,created_at,id LIMIT 1 FOR UPDATE SKIP LOCKED"
        ).fetch_optional(&mut *tx).await?
    };
    let Some(event) = event else {
        tx.commit().await?;
        return Ok(RelayOutcome::Idle);
    };
    let tenant_id = Uuid::parse_str(&event.tenant_id).ok();
    let message_id = event
        .message_id
        .as_deref()
        .and_then(|id| Uuid::parse_str(id).ok());
    let (Some(tenant_id), Some(message_id)) = (tenant_id, message_id) else {
        return fail_invalid_event(tx, &event).await;
    };
    if event.message_tenant.as_deref() != Some(event.tenant_id.as_str()) {
        return fail_invalid_event(tx, &event).await;
    }
    server_common::auth_utils::set_org_context(&mut *tx, &event.tenant_id).await?;
    sqlx::query("SELECT set_config('app.current_tenant_id',$1,true)")
        .bind(&event.tenant_id)
        .execute(&mut *tx)
        .await?;
    let conversation_id = sqlx::query_scalar::<_, Uuid>(
        "SELECT conversation_id FROM chat_messages WHERE id=$1 AND tenant_id=$2",
    )
    .bind(message_id)
    .bind(tenant_id)
    .fetch_optional(&mut *tx)
    .await?;
    let Some(conversation_id) = conversation_id else {
        return fail_invalid_event(tx, &event).await;
    };
    // Use the same separate ownership-checked SHARE locks as widget reads and
    // writes. Ownership changes wait while this local publication attempt runs.
    match OmniChannelRepo::lock_chat_conversation(&mut tx, tenant_id, conversation_id).await {
        Ok(_) => {}
        Err(WidgetChatError::NotFound) => return fail_invalid_event(tx, &event).await,
        Err(WidgetChatError::Database(error)) => return Err(error),
        Err(_) => {
            return Err(sqlx::Error::Protocol(
                "chat event authority unavailable".into(),
            ));
        }
    }
    // Neither a payload-supplied topic nor content can cross a tenant boundary.
    // Reconstruct from committed, still-owned canonical rows, with stored bounds.
    let message = sqlx::query_as::<_, ChatMessage>(
        "SELECT m.id,m.tenant_id,m.conversation_id,m.sender_type,m.sender_id,m.content,m.created_at,m.updated_at FROM chat_messages m JOIN chat_conversations c ON c.id=m.conversation_id AND c.tenant_id=m.tenant_id JOIN chat_inboxes i ON i.id=c.inbox_id AND i.tenant_id=c.tenant_id JOIN chat_contacts p ON p.id=c.contact_id AND p.tenant_id=c.tenant_id WHERE m.id=$1 AND m.tenant_id=$2 AND m.conversation_id=$3 AND octet_length(m.content)<=16384 AND octet_length(m.sender_type)<=32 AND (m.sender_id IS NULL OR octet_length(m.sender_id)<=4096) FOR SHARE OF m"
    ).bind(message_id).bind(tenant_id).bind(conversation_id).fetch_optional(&mut *tx).await?;
    let Some(message) = message else {
        return fail_invalid_event(tx, &event).await;
    };
    let payload = json!({"event_id":message_id,"action":"new_message","message":message});
    let topic = format!("unified:chat:{tenant_id}");
    // Queue and canonical authority locks survive this bounded network step.
    // Cancellation/restart rolls back the claim. A lost publish acknowledgement
    // may cause a retry or a late Redis effect after local locks are released;
    // subscribers must deduplicate the stable event_id.
    let published = tokio::time::timeout(PUBLISH_TIMEOUT, async {
        let mut connection = redis.get_async_connection().await?;
        redis::AsyncCommands::publish::<_, _, u64>(&mut connection, topic, payload.to_string())
            .await
    })
    .await;
    let outcome = if let Ok(Ok(subscribers)) = published
        && subscribers > 0
    {
        let publication = json!({"transport":"redis_pubsub","subscriber_count":subscribers});
        sqlx::query("UPDATE ohc_job_queue SET status='COMPLETED',payload=jsonb_set(payload,'{publication}',$3),updated_at=clock_timestamp() WHERE id=$1 AND tenant_id=$2 AND job_type='publish_chat_event'")
            .bind(&event.id).bind(&event.tenant_id).bind(sqlx::types::Json(publication)).execute(&mut *tx).await?;
        RelayOutcome::Published {
            message_id,
            subscribers,
        }
    } else {
        let attempts = event.retry_count.max(0).saturating_add(1);
        let status = if attempts >= MAX_ATTEMPTS {
            "FAILED"
        } else {
            "PENDING"
        };
        let delay_seconds = 1_i32 << attempts.min(8);
        sqlx::query("UPDATE ohc_job_queue SET status=$3,retry_count=$4,next_retry_at=clock_timestamp()+make_interval(secs=>$5),updated_at=clock_timestamp() WHERE id=$1 AND tenant_id=$2 AND job_type='publish_chat_event'")
            .bind(&event.id).bind(&event.tenant_id).bind(status).bind(attempts).bind(f64::from(delay_seconds)).execute(&mut *tx).await?;
        if status == "FAILED" {
            RelayOutcome::Failed
        } else {
            RelayOutcome::RetryScheduled
        }
    };
    tx.commit().await?;
    Ok(outcome)
}

pub async fn run_chat_outbox_worker(pool: PgPool, mut shutdown: watch::Receiver<bool>) {
    let Some(redis) = crate::redis_pool::get_redis_pool() else {
        return;
    };
    loop {
        if *shutdown.borrow() {
            return;
        }
        let result = tokio::select! {
            biased;
            changed = shutdown.changed() => {
                if changed.is_err() || *shutdown.borrow() { return; }
                continue;
            }
            result = bounded_relay(&pool, redis, None) => result,
        };
        let pause = match result {
            Ok(RelayOutcome::Idle) => true,
            Ok(_) => false,
            Err(_) => {
                tracing::warn!("Chat event relay could not acknowledge its queue operation");
                true
            }
        };
        if pause {
            tokio::select! {
                changed = shutdown.changed() => {
                    if changed.is_err() || *shutdown.borrow() { return; }
                }
                _ = tokio::time::sleep(Duration::from_millis(250)) => {}
            }
        }
    }
}
