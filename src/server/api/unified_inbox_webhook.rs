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
    #[serde(skip_serializing_if = "String::is_empty")]
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
    #[serde(skip_serializing_if = "String::is_empty")]
    pub tenant_id: String,
    pub thread_id: String,
    pub sender_type: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UnifiedTriageAction {
    pub id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
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
    pub mobile_optimized: Option<bool>,
}


async fn generate_draft_reply(
    tenant_id: &str,
    customer_message: &str,
    context_summary: &str,
    db: &Arc<DB>,
) -> String {
    let (business_name, industry): (String, String) = match &db.store {
        crate::db::DbStore::Postgres => sqlx::query_as(
            "SELECT name, COALESCE(industry, '') FROM tenants WHERE id = $1"
        )
        .bind(tenant_id)
        .fetch_optional(&db.pool)
        .await
        .unwrap_or(None)
        .unwrap_or_else(|| ("A business".to_string(), "".to_string())),
        crate::db::DbStore::Sqlite(sqlite_pool) => sqlx::query_as(
            "SELECT name, COALESCE(industry, '') FROM tenants WHERE id = ?"
        )
        .bind(tenant_id)
        .fetch_optional(sqlite_pool)
        .await
        .unwrap_or(None)
        .unwrap_or_else(|| ("A business".to_string(), "".to_string())),
    };

    let business_context = if industry.is_empty() {
        format!("A business named {}", business_name)
    } else {
        format!("A {} business named {}", industry, business_name)
    };

    let prompt = format!(
        "Write one concise, warm customer-service reply. Business context: {}. Customer recent history: {}. Customer message: {}",
        business_context, context_summary, customer_message
    );
    let compressed_prompt = ::server_pricing::compression::reduce_tokens(&prompt);

    let llm_res = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
        Ok("gemini") => {
            crate::minimax::LocalLLMClient::new().reason(&compressed_prompt).await
        }
        Ok("minimax") => {
            let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
            if api_key.is_empty() {
                crate::minimax::LocalLLMClient::new().reason(&compressed_prompt).await
            } else {
                crate::minimax::MinimaxClient::new(api_key).reason(&compressed_prompt).await
            }
        }
        _ => crate::minimax::LocalLLMClient::new().reason(&compressed_prompt).await,
    };

    match llm_res {
        Ok(reply) => reply,
        Err(_) => format!("Hi there! Thanks for your message: '{}'. How can we help?", customer_message),
    }
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
    let job_id = format!("job-{}", Uuid::new_v4());

    // Instead of inline LLM and database insertions, we enqueue a raw event to the AI Job Queue
    let job_payload = serde_json::json!({
        "message_id": format!("msg-{}", Uuid::new_v4()),
        "source": payload.source,
        "sender_id": payload.identifier,
        "content": payload.message,
    });

    match &state.db.store {
        crate::db::DbStore::Postgres => {
            let _ = sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'message_triage', $3, 'PENDING')")
                .bind(&job_id)
                .bind(tenant_id)
                .bind(sqlx::types::Json(&job_payload))
                .execute(&state.db.pool).await;
        }
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            let _ = sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, 'message_triage', ?, 'PENDING')")
                .bind(&job_id)
                .bind(tenant_id)
                .bind(serde_json::to_string(&job_payload).unwrap_or_else(|_| "{}".to_string()))
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
    let mobile_optimized = query.mobile_optimized.unwrap_or(false);

    let cache_key = format!("ui_webhook_feed:{}:mobile:{}", tenant_id, mobile_optimized);
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
            let items = fetch_unified_feed_items(&state_bg, &tenant_id_bg, mobile_optimized).await;
            cache_bg
                .set(&cache_key_bg, items, std::time::Duration::from_secs(30))
                .await;
        });

        return (StatusCode::OK, axum::Json(cached)).into_response();
    }

    let feed_items = fetch_unified_feed_items(&state, &tenant_id, mobile_optimized).await;
    cache
        .set(
            &cache_key,
            feed_items.clone(),
            std::time::Duration::from_secs(30),
        )
        .await;

    (StatusCode::OK, axum::Json(feed_items)).into_response()
}

async fn fetch_unified_feed_items(state: &AppState, tenant_id: &str, mobile_optimized: bool) -> Vec<UnifiedFeedItem> {
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

        for mut thread in threads_rows {
            let thread_id = thread.id.clone();
            let mut messages = messages_map.remove(&thread_id).unwrap_or_default();
            let mut triage_actions = triage_actions_map.remove(&thread_id).unwrap_or_default();

            if mobile_optimized {
                thread.tenant_id = String::new();
                for msg in &mut messages {
                    msg.tenant_id = String::new();
                }
                for action in &mut triage_actions {
                    action.tenant_id = String::new();
                }
            }

            feed_items.push(UnifiedFeedItem {
                thread,
                messages,
                triage_actions,
            });
        }
    }

    feed_items
}
