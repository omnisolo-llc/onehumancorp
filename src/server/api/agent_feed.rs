use axum::{
    extract::{Extension, State},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
    Json,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::db::{DB, DbStore};
use ::server_common::Claims;

#[derive(Serialize)]
pub struct ActionCard {
    pub id: String,
    pub tenant_id: String,
    pub agent_id: String,
    pub action_type: String,
    pub payload: serde_json::Value,
    pub status: String,
}

#[derive(Serialize)]
pub struct AgentFeedResponse {
    pub pending_actions: Vec<ActionCard>,
}

#[derive(Deserialize)]
pub struct CreateActionRequest {
    pub agent_id: String,
    pub action_type: String,
    pub payload: serde_json::Value,
}

#[derive(Serialize)]
pub struct CreateActionResponse {
    pub id: String,
}

#[derive(Clone)]
pub struct FeedState {
    pub orchestrator: Arc<DepartmentOrchestrator>,
    pub db: Arc<DB>,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>, db: Arc<DB>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let state = FeedState { orchestrator, db };
    Router::new()
        .route("/", get(list_feed).post(create_action))
        .with_state(state)
}

async fn create_action(
    State(state): State<FeedState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateActionRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(CreateActionResponse { id: "".to_string() })).into_response(),
    };

    let id = uuid::Uuid::new_v4().to_string();

    let redis_url = std::env::var("OHC_REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    if let Ok(client) = redis::Client::open(redis_url) {
        if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
            let lock_key = format!("ohc:lock:{}:agent_action:{}", tenant_id, payload.action_type);
            let acquired: bool = redis::cmd("SET")
                .arg(&lock_key)
                .arg("1")
                .arg("NX")
                .arg("EX")
                .arg(30)
                .query_async(&mut conn)
                .await
                .unwrap_or(false);

            if !acquired {
                 return (StatusCode::CONFLICT, Json(CreateActionResponse { id: "".to_string() })).into_response();
            }
        }
    }

    match &state.db.store {
        DbStore::Postgres => {
             let _ = sqlx::query("INSERT INTO agent_actions (id, tenant_id, agent_id, action_type, payload, _sync_status) VALUES ($1, $2, $3, $4, $5, 'pending')")
                .bind(&id)
                .bind(&tenant_id)
                .bind(&payload.agent_id)
                .bind(&payload.action_type)
                .bind(&payload.payload)
                .execute(&state.db.pool)
                .await;
        }
        DbStore::Sqlite(pool) => {
             let _ = sqlx::query("INSERT INTO agent_actions (id, tenant_id, agent_id, action_type, payload, _sync_status) VALUES (?, ?, ?, ?, ?, 'pending')")
                .bind(&id)
                .bind(&tenant_id)
                .bind(&payload.agent_id)
                .bind(&payload.action_type)
                .bind(serde_json::to_string(&payload.payload).unwrap_or_else(|_| "{}".to_string()))
                .execute(pool)
                .await;
        }
    }

    (StatusCode::OK, Json(CreateActionResponse { id })).into_response()
}

async fn list_feed(
    State(state): State<FeedState>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(AgentFeedResponse { pending_actions: vec![] })).into_response(),
    };

    let mut actions = Vec::new();

    match &state.db.store {
        DbStore::Postgres => {
            let rows = sqlx::query("SELECT id, tenant_id, agent_id, action_type, payload, _sync_status FROM agent_actions WHERE tenant_id = $1 AND _sync_status = 'pending'")
                .bind(&tenant_id)
                .fetch_all(&state.db.pool)
                .await;

            if let Ok(rows) = rows {
                use sqlx::Row;
                for row in rows {
                    let payload: serde_json::Value = row.get("payload");
                    actions.push(ActionCard {
                        id: row.get("id"),
                        tenant_id: row.get("tenant_id"),
                        agent_id: row.get("agent_id"),
                        action_type: row.get("action_type"),
                        payload,
                        status: row.get("_sync_status"),
                    });
                }
            } else {
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(AgentFeedResponse { pending_actions: vec![] })).into_response();
            }
        }
        DbStore::Sqlite(pool) => {
            let rows = sqlx::query("SELECT id, tenant_id, agent_id, action_type, payload, _sync_status FROM agent_actions WHERE tenant_id = ? AND _sync_status = 'pending'")
                .bind(&tenant_id)
                .fetch_all(pool)
                .await;

            if let Ok(rows) = rows {
                use sqlx::Row;
                for row in rows {
                    let payload_str: String = row.get("payload");
                    let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or(serde_json::json!({}));
                    actions.push(ActionCard {
                        id: row.get("id"),
                        tenant_id: row.get("tenant_id"),
                        agent_id: row.get("agent_id"),
                        action_type: row.get("action_type"),
                        payload,
                        status: row.get("_sync_status"),
                    });
                }
            } else {
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(AgentFeedResponse { pending_actions: vec![] })).into_response();
            }
        }
    }

    (StatusCode::OK, Json(AgentFeedResponse { pending_actions: actions })).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;
    use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
    use ohc_builtin_agent::mesh::transport::InProcessTransport;
    use crate::orchestration::mesh::CentrifugeNode;

    #[tokio::test]
    async fn test_agent_feed_list() {
        let database_url = "sqlite::memory:";
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(database_url).await.unwrap();

        let pg_pool = sqlx::PgPool::connect_lazy("postgres://localhost/dummy").unwrap();
        let db = Arc::new(crate::db::DB { pool: pg_pool, store: crate::db::DbStore::Sqlite(pool.clone()) });

        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS agent_actions (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            agent_id TEXT NOT NULL,
            interaction_id TEXT,
            action_type TEXT NOT NULL,
            payload TEXT DEFAULT '{}',
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            _sync_status TEXT DEFAULT 'pending',
            version INTEGER DEFAULT 1
        )").execute(&pool).await;

        let _ = sqlx::query("INSERT INTO agent_actions (id, tenant_id, agent_id, action_type, payload, _sync_status) VALUES ('1', 'tenant1', 'agent1', 'test_action', '{\"foo\":\"bar\"}', 'pending')").execute(&pool).await;

        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let orchestrator = Arc::new(DepartmentOrchestrator::new(db.clone(), mesh));

        let app = router(orchestrator, db)
            .layer(axum::middleware::from_fn(|mut req: axum::extract::Request, next: axum::middleware::Next| async move {
                req.extensions_mut().insert(Claims {
                    sub: "test".to_string(),
                    email: "test@example.com".to_string(),
                    organization_id: Some("tenant1".to_string()),
                    exp: 10000000000,
                    iat: 0,
                    iss: "test".to_string(),
                });
                next.run(req).await
            }));

        let response = app.oneshot(
            Request::builder()
                .uri("/")
                .method("GET")
                .body(axum::body::Body::empty())
                .unwrap()
        ).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let feed: AgentFeedResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(feed.pending_actions.len(), 1);
        assert_eq!(feed.pending_actions[0].id, "1");
        assert_eq!(feed.pending_actions[0].action_type, "test_action");
    }

    #[tokio::test]
    async fn test_agent_feed_create() {
        let database_url = "sqlite::memory:";
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(database_url).await.unwrap();

        let pg_pool = sqlx::PgPool::connect_lazy("postgres://localhost/dummy").unwrap();
        let db = Arc::new(crate::db::DB { pool: pg_pool, store: crate::db::DbStore::Sqlite(pool.clone()) });

        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS agent_actions (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            agent_id TEXT NOT NULL,
            interaction_id TEXT,
            action_type TEXT NOT NULL,
            payload TEXT DEFAULT '{}',
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            _sync_status TEXT DEFAULT 'pending',
            version INTEGER DEFAULT 1
        )").execute(&pool).await;

        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let orchestrator = Arc::new(DepartmentOrchestrator::new(db.clone(), mesh));

        let app = router(orchestrator, db.clone())
            .layer(axum::middleware::from_fn(|mut req: axum::extract::Request, next: axum::middleware::Next| async move {
                req.extensions_mut().insert(Claims {
                    sub: "test".to_string(),
                    email: "test@example.com".to_string(),
                    organization_id: Some("tenant1".to_string()),
                    exp: 10000000000,
                    iat: 0,
                    iss: "test".to_string(),
                });
                next.run(req).await
            }));

        let payload = CreateActionRequest {
            agent_id: "agent1".to_string(),
            action_type: "test_action_create".to_string(),
            payload: serde_json::json!({"foo": "baz"}),
        };

        let response = app.oneshot(
            Request::builder()
                .uri("/")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(axum::body::Body::from(serde_json::to_string(&payload).unwrap()))
                .unwrap()
        ).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let row = sqlx::query("SELECT COUNT(*) as count FROM agent_actions WHERE tenant_id = 'tenant1' AND action_type = 'test_action_create'")
            .fetch_one(&pool).await.unwrap();
        use sqlx::Row;
        let count: i32 = row.get("count");
        assert_eq!(count, 1);
    }
}
