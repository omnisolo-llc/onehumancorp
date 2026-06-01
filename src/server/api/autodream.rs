use axum::{
    extract::Json,
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::autodream::AutoDreamWorker;

#[derive(Deserialize)]
pub struct QueryRequest {
    pub query_text: String,
    pub limit: Option<i32>,
}

#[derive(Deserialize)]
pub struct SyncRequest {
    pub force_reindex: Option<bool>,
}

#[derive(Serialize)]
pub struct SearchResult {
    pub memory_id: String,
    pub content: String,
    pub distance: f32,
}

#[derive(Serialize)]
pub struct QueryResponse {
    pub results: Vec<SearchResult>,
}

#[derive(Serialize)]
pub struct SyncResponse {
    pub status: String,
}

pub fn router<S: Clone + Send + Sync + 'static>(worker: Arc<AutoDreamWorker>) -> Router<S> {
    let worker_sync = worker.clone();
    let worker_query = worker.clone();

    Router::new()
        .route("/sync", post(move |payload: Option<Json<SyncRequest>>| async move {
            let _force_reindex = payload.map(|p| p.force_reindex.unwrap_or(false)).unwrap_or(false);
            match worker_sync.consolidate_epoch().await {
                Ok(_) => Json(SyncResponse { status: "success".to_string() }).into_response(),
                Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            }
        }))
        .route("/query", post(move |Json(params): Json<QueryRequest>| async move {
            if params.query_text.is_empty() {
                return (axum::http::StatusCode::BAD_REQUEST, "query_text is required".to_string()).into_response();
            }

            let limit = params.limit.unwrap_or(5);

            let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
            let client = crate::minimax::MinimaxClient::new(api_key);

            let embedding = match client.generate_embedding(&params.query_text).await {
                Ok(emb) => format!("[{}]", emb.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(",")),
                Err(e) => {
                    tracing::error!("AutoDream API: failed to generate embedding: {}", e);
                    format!("[{}]", vec!["0.0"; 1536].join(", "))
                }
            };

            match worker_query.search_memories(&embedding, limit).await {
                Ok(results) => {
                    let res = results.into_iter().map(|r| SearchResult {
                        memory_id: r.id,
                        content: r.content,
                        distance: 1.0 - (r.score as f32), // assuming score is cosine similarity, converting to distance for the spec
                    }).collect();
                    Json(QueryResponse { results: res }).into_response()
                },
                Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            }
        }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_autodream_sync_endpoint() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        let database_url = "postgres://postgres:postgres@localhost:5432/test";
        let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy(database_url)
            .unwrap();

        let db = Arc::new(crate::db::DB { pool: pool.clone(), store: crate::db::DbStore::Postgres });
        let worker = Arc::new(AutoDreamWorker::new(db));

        let app: Router<()> = router(worker);

        let req = axum::http::Request::builder()
            .method("POST")
            .uri("/sync")
            .body(axum::body::Body::empty())
            .unwrap();

        let _ = tower::ServiceExt::oneshot(app.clone(), req).await.unwrap();

        let req2 = axum::http::Request::builder()
            .method("POST")
            .uri("/query")
            .header("content-type", "application/json")
            .body(axum::body::Body::from("{\"query_text\": \"test\", \"limit\": 5}"))
            .unwrap();

        let _ = tower::ServiceExt::oneshot(app, req2).await.unwrap();
    }
}
