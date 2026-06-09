use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
    routing::{post, get},
    Router,
};
use std::sync::Arc;

// Correct path when being compiled inside `server_lib` which maps `crate` to the root of `src/server`
#[cfg(not(ohc_bazel_package))]
use crate::services::agent_feed::service::{AgentFeedCard, AgentFeedService, CreateCardRequest, ResolveCardRequest};

#[cfg(ohc_bazel_package)]
use ::server_lib::services::agent_feed::service::{AgentFeedCard, AgentFeedService, CreateCardRequest, ResolveCardRequest};


pub fn router<S: Clone + Send + Sync + 'static>(service: Arc<AgentFeedService>) -> Router<S> {
    Router::new()
        .route("/", post(create_card))
        .route("/", get(list_pending_cards))
        .route("/:id/resolve", post(resolve_card))
        .with_state(service)
}

async fn create_card(
    State(service): State<Arc<AgentFeedService>>,
    Json(payload): Json<CreateCardRequest>,
) -> impl IntoResponse {
    let tenant_id = "tenant-from-auth"; // TODO: Extract from standard auth context
    match service.create_card(tenant_id, payload).await {
        Ok(card) => (axum::http::StatusCode::CREATED, Json(card)).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn list_pending_cards(
    State(service): State<Arc<AgentFeedService>>,
) -> impl IntoResponse {
    let tenant_id = "tenant-from-auth"; // TODO: Extract from standard auth context
    match service.list_pending_cards(tenant_id).await {
        Ok(cards) => (axum::http::StatusCode::OK, Json(cards)).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn resolve_card(
    Path(id): Path<String>,
    State(service): State<Arc<AgentFeedService>>,
    Json(payload): Json<ResolveCardRequest>,
) -> impl IntoResponse {
    let tenant_id = "tenant-from-auth"; // TODO: Extract from standard auth context
    match service.resolve_card(tenant_id, &id, payload).await {
        Ok(card) => (axum::http::StatusCode::OK, Json(card)).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt; // for `oneshot`

    async fn setup_db() -> Arc<crate::db::DB> {
        let database_url = std::env::var("OHC_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());

        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(&database_url)
            .await
            .unwrap();

        // Run migrations
        sqlx::query("CREATE TABLE IF NOT EXISTS agent_feed_cards (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            agent_type TEXT NOT NULL,
            card_type TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            proposed_action_payload JSONB DEFAULT '{}'::jsonb,
            status TEXT NOT NULL DEFAULT 'Pending',
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        )").execute(&pool).await.unwrap();

        Arc::new(crate::db::DB { pool: pool, store: crate::db::DbStore::Postgres })
    }

    #[tokio::test]
    async fn test_agent_feed_endpoints() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        let db = setup_db().await;
        let service = Arc::new(AgentFeedService::new(db));
        let app = router(service);

        // 1. Create a card
        let create_req = CreateCardRequest {
            agent_type: "The Ambassador".to_string(),
            card_type: "Actionable".to_string(),
            title: "Reply to Customer".to_string(),
            description: Some("Customer asked about Vegan Cake.".to_string()),
            proposed_action_payload: serde_json::json!({"action": "send_dm"}),
        };

        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&create_req).unwrap()))
            .unwrap();

        let response = app.clone().oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let created_card: AgentFeedCard = serde_json::from_slice(&body_bytes).unwrap();

        // 2. List pending cards
        let req = Request::builder()
            .method("GET")
            .uri("/")
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // 3. Resolve the card
        let resolve_req = ResolveCardRequest {
            status: "Approved".to_string(),
        };

        let req = Request::builder()
            .method("POST")
            .uri(format!("/{}/resolve", created_card.id))
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&resolve_req).unwrap()))
            .unwrap();

        let response = app.oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
