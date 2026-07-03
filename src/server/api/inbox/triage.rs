use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

pub struct AppState {
    pub db: sqlx::PgPool,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct TriageQueueItem {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub message_id: Uuid,
    pub assigned_department: String,
    pub confidence_score: f64,
    pub status: String,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/tenant/{tenant_id}/triage_queue", get(get_triage_queue))
}

pub async fn get_triage_queue(
    State(state): State<Arc<AppState>>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<Vec<TriageQueueItem>>, axum::http::StatusCode> {

    let records = sqlx::query_as::<_, TriageQueueItem>(
        r#"
        SELECT id, tenant_id, message_id, assigned_department, confidence_score, status
        FROM triage_queue
        WHERE tenant_id = $1 AND status = 'queued'
        ORDER BY created_at DESC
        LIMIT 50
        "#
    )
    .bind(tenant_id)
    .fetch_all(&state.db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(records))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_mock_triage_queue_route() {
        let pool = sqlx::PgPool::connect_lazy("postgres://postgres:postgres@localhost:5432/ohc").unwrap();
        let state = Arc::new(AppState { db: pool });
        let app = router().with_state(state);

        let tenant_id = Uuid::new_v4();
        let req = Request::builder()
            .uri(format!("/tenant/{}/triage_queue", tenant_id))
            .body(axum::body::Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        // Just verify it doesn't 404 (route exists). It will likely return 500 without a real DB.
        assert_ne!(res.status(), axum::http::StatusCode::NOT_FOUND);
    }
}
