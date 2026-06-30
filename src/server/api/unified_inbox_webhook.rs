use crate::db::DB;
use axum::{
    extract::{Json, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DB>,
}

#[derive(Debug, Deserialize)]
pub struct UnifiedWebhookPayload {
    pub tenant_id: String,
    pub source: String,
    pub identifier: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DraftedResponse {
    pub customer_id: String,
    pub context_summary: String,
    pub draft_reply: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UnifiedThread {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: Option<String>,
    pub channel: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UnifiedMessage {
    pub id: String,
    pub tenant_id: String,
    pub thread_id: String,
    pub sender_type: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UnifiedTriageAction {
    pub id: String,
    pub tenant_id: String,
    pub thread_id: String,
    pub action_type: String,
    pub action_payload: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UnifiedFeedItem {
    pub thread: UnifiedThread,
    pub messages: Vec<UnifiedMessage>,
    pub triage_actions: Vec<UnifiedTriageAction>,
}

#[derive(Deserialize)]
pub struct LocalUiTenantQuery {
    pub tenant_id: Option<String>,
    pub tenant: Option<String>,
}

fn get_ui_tenant_id(query: &LocalUiTenantQuery) -> String {
    query
        .tenant_id
        .clone()
        .or(query.tenant.clone())
        .unwrap_or_else(|| "default".to_string())
}

pub fn router(db: Arc<DB>) -> Router {
    let state = AppState { db };
    Router::new()
        .route(
            "/api/v1/webhooks/unified_inbox",
            post(handle_unified_webhook),
        )
        .route("/api/ui/unified_inbox_feed", get(get_unified_feed))
        .with_state(state)
}

pub async fn handle_unified_webhook(
    State(state): State<AppState>,
    Json(payload): Json<UnifiedWebhookPayload>,
) -> impl IntoResponse {
    let tenant_id = &payload.tenant_id;

    if let Some(redis_client) = crate::get_redis_client() {
        if let Ok(mut conn) = redis_client.get_multiplexed_async_connection().await {
            let lock_key = format!(
                "ohc:lock:unified_inbox:{}:{}",
                tenant_id, payload.identifier
            );
            let locked: redis::RedisResult<Option<String>> = redis::cmd("SET")
                .arg(&lock_key)
                .arg("1")
                .arg("NX")
                .arg("EX")
                .arg(30)
                .query_async(&mut conn)
                .await;

            if locked.is_err() || locked.unwrap().is_none() {
                tracing::warn!(
                    "Failed to acquire lock for tenant {} identifier {}",
                    tenant_id,
                    payload.identifier
                );
            }
        }
    }

    let resolved_customer = crate::api::inbox::identity::resolve_identity(&state.db, tenant_id, &payload.source, &payload.identifier).await;
    let customer_id = resolved_customer.unwrap_or_else(|| format!("cust-{}", Uuid::new_v4()));
    let thread_id = format!("thread-{}", Uuid::new_v4());
    let message_id = format!("msg-{}", Uuid::new_v4());
    let action_id = format!("action-{}", Uuid::new_v4());

    let mut context_summary = "New customer inquiry received.".to_string();

    match &state.db.store {
        crate::db::DbStore::Postgres => {
            // Build Context Memory Graph Summary
            let recent_history = sqlx::query(
                "SELECT channel, CAST(created_at AS text) as created_at FROM unified_threads WHERE tenant_id = $1 AND customer_id = $2 ORDER BY created_at DESC LIMIT 2"
            )
            .bind(tenant_id)
            .bind(&customer_id)
            .fetch_all(&state.db.pool).await;

            if let Ok(rows) = recent_history {
                if !rows.is_empty() {
                    let mut history_str = String::from("Recent history: ");
                    let history_items: Vec<String> = rows.into_iter().map(|row| {
                        let channel: String = row.get("channel");
                        let created_at: String = row.try_get("created_at").unwrap_or_default();
                        format!("Sent {} ({})", channel, created_at)
                    }).collect();
                    history_str.push_str(&history_items.join(", "));
                    context_summary = history_str;
                }
            }

            let _ = sqlx::query("INSERT INTO unified_threads (id, tenant_id, customer_id, channel, status) VALUES ($1, $2, $3, $4, 'open') ON CONFLICT DO NOTHING")
                .bind(&thread_id)
                .bind(tenant_id)
                .bind(&customer_id)
                .bind(&payload.source)
                .execute(&state.db.pool).await;

            let _ = sqlx::query("INSERT INTO unified_messages (id, tenant_id, thread_id, sender_type, content) VALUES ($1, $2, $3, 'customer', $4)")
                .bind(&message_id)
                .bind(tenant_id)
                .bind(&thread_id)
                .bind(&payload.message)
                .execute(&state.db.pool).await;

            let draft_reply = format!(
                "Hi there! Thanks for your message: '{}'. How can we help?",
                payload.message
            );
            let action_payload = serde_json::to_string(&DraftedResponse {
                customer_id: customer_id.clone(),
                context_summary,
                draft_reply,
            })
            .unwrap();

            let _ = sqlx::query("INSERT INTO unified_triage_actions (id, tenant_id, thread_id, action_type, action_payload, status) VALUES ($1, $2, $3, 'DRAFT_REPLY', $4, 'pending')")
                .bind(&action_id)
                .bind(tenant_id)
                .bind(&thread_id)
                .bind(&action_payload)
                .execute(&state.db.pool).await;
        }
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            // Build Context Memory Graph Summary
            let recent_history = sqlx::query(
                "SELECT channel, CAST(created_at AS text) as created_at FROM unified_threads WHERE tenant_id = ? AND customer_id = ? ORDER BY created_at DESC LIMIT 2"
            )
            .bind(tenant_id)
            .bind(&customer_id)
            .fetch_all(sqlite_pool).await;

            if let Ok(rows) = recent_history {
                if !rows.is_empty() {
                    let mut history_str = String::from("Recent history: ");
                    let history_items: Vec<String> = rows.into_iter().map(|row| {
                        let channel: String = row.get("channel");
                        let created_at: String = row.try_get("created_at").unwrap_or_default();
                        format!("Sent {} ({})", channel, created_at)
                    }).collect();
                    history_str.push_str(&history_items.join(", "));
                    context_summary = history_str;
                }
            }

            let _ = sqlx::query("INSERT OR IGNORE INTO unified_threads (id, tenant_id, customer_id, channel, status) VALUES (?, ?, ?, ?, 'open')")
                .bind(&thread_id)
                .bind(tenant_id)
                .bind(&customer_id)
                .bind(&payload.source)
                .execute(sqlite_pool).await;

            let _ = sqlx::query("INSERT INTO unified_messages (id, tenant_id, thread_id, sender_type, content) VALUES (?, ?, ?, 'customer', ?)")
                .bind(&message_id)
                .bind(tenant_id)
                .bind(&thread_id)
                .bind(&payload.message)
                .execute(sqlite_pool).await;

            let draft_reply = format!(
                "Hi there! Thanks for your message: '{}'. How can we help?",
                payload.message
            );
            let action_payload = serde_json::to_string(&DraftedResponse {
                customer_id: customer_id.clone(),
                context_summary,
                draft_reply,
            })
            .unwrap();

            let _ = sqlx::query("INSERT INTO unified_triage_actions (id, tenant_id, thread_id, action_type, action_payload, status) VALUES (?, ?, ?, 'DRAFT_REPLY', ?, 'pending')")
                .bind(&action_id)
                .bind(tenant_id)
                .bind(&thread_id)
                .bind(&action_payload)
                .execute(sqlite_pool).await;
        }
    }

    (
        StatusCode::OK,
        axum::Json(serde_json::json!({"success": true, "thread_id": thread_id})),
    )
        .into_response()
}

static UI_WEBHOOK_FEED_CACHE: std::sync::OnceLock<
    ::server_utils::cache::HybridCache<Vec<UnifiedFeedItem>>,
> = std::sync::OnceLock::new();

pub async fn get_unified_feed(
    State(state): State<AppState>,
    Query(query): Query<LocalUiTenantQuery>,
) -> impl IntoResponse {
    let tenant_id = get_ui_tenant_id(&query);

    let cache_key = format!("ui_webhook_feed:{}", tenant_id);
    let cache = UI_WEBHOOK_FEED_CACHE.get_or_init(|| {
        let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
        let redis_client = match redis::Client::open(redis_url) {
            Ok(client) => Some(client),
            Err(e) => {
                tracing::warn!("Failed to initialize Redis client for UI_WEBHOOK_FEED_CACHE: {}. Falling back to in-memory cache.", e);
                None
            }
        };
        ::server_utils::cache::HybridCache::new(redis_client)
    });

    if let Some((cached, is_stale)) = cache.get_with_swr(&cache_key).await {
        if !is_stale {
            return (StatusCode::OK, axum::Json(cached)).into_response();
        }

        let state_bg = state.clone();
        let tenant_id_bg = tenant_id.clone();
        let cache_bg = cache.clone();
        let cache_key_bg = cache_key.clone();

        tokio::spawn(async move {
            let items = fetch_unified_feed_items(&state_bg, &tenant_id_bg).await;
            cache_bg
                .set(&cache_key_bg, items, std::time::Duration::from_secs(30))
                .await;
        });

        return (StatusCode::OK, axum::Json(cached)).into_response();
    }

    let feed_items = fetch_unified_feed_items(&state, &tenant_id).await;
    cache
        .set(
            &cache_key,
            feed_items.clone(),
            std::time::Duration::from_secs(30),
        )
        .await;

    (StatusCode::OK, axum::Json(feed_items)).into_response()
}

async fn fetch_unified_feed_items(state: &AppState, tenant_id: &str) -> Vec<UnifiedFeedItem> {
    let mut feed_items: Vec<UnifiedFeedItem> = vec![];

    let threads_res_mapped: Result<Vec<UnifiedThread>, sqlx::Error>;
    match &state.db.store {
        crate::db::DbStore::Postgres => {
            let res = sqlx::query("SELECT id, tenant_id, customer_id, channel, status, CAST(created_at AS text) as created_at, CAST(updated_at AS text) as updated_at FROM unified_threads WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 50")
                .bind(&tenant_id)
                .fetch_all(&state.db.pool).await;
            threads_res_mapped = res.map(|rows| {
                rows.into_iter()
                    .map(|row| UnifiedThread {
                        id: row.get("id"),
                        tenant_id: row.get("tenant_id"),
                        customer_id: row.try_get("customer_id").ok(),
                        channel: row.get("channel"),
                        status: row.get("status"),
                        created_at: row.try_get("created_at").unwrap_or_default(),
                        updated_at: row.try_get("updated_at").unwrap_or_default(),
                    })
                    .collect()
            });
        }
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            let res = sqlx::query("SELECT id, tenant_id, customer_id, channel, status, CAST(created_at AS text) as created_at, CAST(updated_at AS text) as updated_at FROM unified_threads WHERE tenant_id = ? ORDER BY created_at DESC LIMIT 50")
                .bind(&tenant_id)
                .fetch_all(sqlite_pool).await;
            threads_res_mapped = res.map(|rows| {
                rows.into_iter()
                    .map(|row| UnifiedThread {
                        id: row.get("id"),
                        tenant_id: row.get("tenant_id"),
                        customer_id: row.try_get("customer_id").ok(),
                        channel: row.get("channel"),
                        status: row.get("status"),
                        created_at: row.try_get("created_at").unwrap_or_default(),
                        updated_at: row.try_get("updated_at").unwrap_or_default(),
                    })
                    .collect()
            });
        }
    }

    if let Ok(threads_rows) = threads_res_mapped {
        if threads_rows.is_empty() {
            return feed_items;
        }

        let thread_ids: Vec<String> = threads_rows.iter().map(|t| t.id.clone()).collect();
        let mut messages_map: std::collections::HashMap<String, Vec<UnifiedMessage>> = std::collections::HashMap::new();
        let mut triage_actions_map: std::collections::HashMap<String, Vec<UnifiedTriageAction>> = std::collections::HashMap::new();

        match &state.db.store {
            crate::db::DbStore::Postgres => {
                let pool = state.db.pool.clone();
                let ids_clone = thread_ids.clone();

                let (msg_res, action_res) = tokio::join!(
                    sqlx::query("SELECT id, tenant_id, thread_id, sender_type, content, CAST(created_at AS text) as created_at FROM unified_messages WHERE thread_id = ANY($1) ORDER BY created_at ASC").bind(&thread_ids).fetch_all(&pool),
                    sqlx::query("SELECT id, tenant_id, thread_id, action_type, action_payload, status, CAST(created_at AS text) as created_at, CAST(updated_at AS text) as updated_at FROM unified_triage_actions WHERE thread_id = ANY($1)").bind(&ids_clone).fetch_all(&pool)
                );

                if let Ok(msg_rows) = msg_res {
                    for m_row in msg_rows {
                        use sqlx::Row;
                        let t_id: String = m_row.get("thread_id");
                        messages_map.entry(t_id).or_default().push(UnifiedMessage { id: m_row.get("id"), tenant_id: m_row.get("tenant_id"), thread_id: m_row.get("thread_id"), sender_type: m_row.get("sender_type"), content: m_row.get("content"), created_at: m_row.try_get("created_at").unwrap_or_default() });
                    }
                }
                if let Ok(action_rows) = action_res {
                    for a_row in action_rows {
                        use sqlx::Row;
                        let t_id: String = a_row.get("thread_id");
                        triage_actions_map.entry(t_id).or_default().push(UnifiedTriageAction { id: a_row.get("id"), tenant_id: a_row.get("tenant_id"), thread_id: a_row.get("thread_id"), action_type: a_row.get("action_type"), action_payload: a_row.try_get("action_payload").ok(), status: a_row.get("status"), created_at: a_row.try_get("created_at").unwrap_or_default(), updated_at: a_row.try_get("updated_at").unwrap_or_default() });
                    }
                }
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                let pool = sqlite_pool.clone();

                let placeholders = thread_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
                let msg_query = format!("SELECT id, tenant_id, thread_id, sender_type, content, CAST(created_at AS text) as created_at FROM unified_messages WHERE thread_id IN ({}) ORDER BY created_at ASC", placeholders);
                let action_query = format!("SELECT id, tenant_id, thread_id, action_type, action_payload, status, CAST(created_at AS text) as created_at, CAST(updated_at AS text) as updated_at FROM unified_triage_actions WHERE thread_id IN ({})", placeholders);

                let (msg_res, action_res) = tokio::join!(
                    async {
                        let mut q = sqlx::query(&msg_query);
                        for id in &thread_ids {
                            q = q.bind(id);
                        }
                        q.fetch_all(&pool).await
                    },
                    async {
                        let mut q = sqlx::query(&action_query);
                        for id in &thread_ids {
                            q = q.bind(id);
                        }
                        q.fetch_all(&pool).await
                    }
                );

                if let Ok(msg_rows) = msg_res {
                    for m_row in msg_rows {
                        use sqlx::Row;
                        let t_id: String = m_row.get("thread_id");
                        messages_map.entry(t_id).or_default().push(UnifiedMessage { id: m_row.get("id"), tenant_id: m_row.get("tenant_id"), thread_id: m_row.get("thread_id"), sender_type: m_row.get("sender_type"), content: m_row.get("content"), created_at: m_row.try_get("created_at").unwrap_or_default() });
                    }
                }
                if let Ok(action_rows) = action_res {
                    for a_row in action_rows {
                        use sqlx::Row;
                        let t_id: String = a_row.get("thread_id");
                        triage_actions_map.entry(t_id).or_default().push(UnifiedTriageAction { id: a_row.get("id"), tenant_id: a_row.get("tenant_id"), thread_id: a_row.get("thread_id"), action_type: a_row.get("action_type"), action_payload: a_row.try_get("action_payload").ok(), status: a_row.get("status"), created_at: a_row.try_get("created_at").unwrap_or_default(), updated_at: a_row.try_get("updated_at").unwrap_or_default() });
                    }
                }
            }
        }

        for thread in threads_rows {
            let thread_id = thread.id.clone();
            let messages = messages_map.remove(&thread_id).unwrap_or_default();
            let triage_actions = triage_actions_map.remove(&thread_id).unwrap_or_default();

            feed_items.push(UnifiedFeedItem {
                thread,
                messages,
                triage_actions,
            });
        }
    }

    feed_items
}
