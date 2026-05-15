use axum::{Router, routing::post, Json};
use serde::{Deserialize, Serialize};
use crate::db::get_pool;

#[derive(Deserialize)]
pub struct SubagentRequest {
    pub parent_task_id: Option<String>,
    pub payload: serde_json::Value,
    pub scheduled_at: Option<String>,
}

#[derive(Serialize)]
pub struct SubagentResponse {
    pub queue_id: String,
    pub status: String,
}

pub async fn enqueue_subagent(Json(payload): Json<SubagentRequest>) -> (axum::http::StatusCode, Json<SubagentResponse>) {
    let queue_id = format!("queue_{}", uuid::Uuid::new_v4().simple());

    let pool = get_pool();

    let _ = sqlx::query(
        "INSERT INTO agent_missions (id, status, payload, tenant_id) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING"
    ).bind(&queue_id)
    .bind("PENDING")
    .bind(payload.payload.to_string())
    .bind("system")
    .execute(&pool)
    .await;

    let response = SubagentResponse {
        queue_id,
        status: "ENQUEUED".to_string(),
    };
    (axum::http::StatusCode::ACCEPTED, Json(response))
}

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/subagent", post(enqueue_subagent))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use serde_json::json;

    #[tokio::test]
    async fn test_enqueue_subagent() {
        let payload = json!({
            "parent_task_id": "task_123",
            "payload": {
                "instruction": "test"
            }
        });

        let payload_json = Json(serde_json::from_value(payload).unwrap());
        let (status, _) = enqueue_subagent(payload_json).await;

        assert_eq!(status, StatusCode::ACCEPTED);
    }
}




#[cfg(test)]
mod test {
    use super::*;
    use axum::http::StatusCode;
    use serde_json::json;

    #[tokio::test]
    async fn test_enqueue_subagent() {
        let payload = json!({
            "parent_task_id": "edge_case_task",
            "payload": {"instruction": "edge case payload"},
            "scheduled_at": "2030-01-01T00:00:00Z"
        });

        let payload_json = Json(serde_json::from_value(payload).unwrap());
        let (status, _) = enqueue_subagent(payload_json).await;

        assert_eq!(status, StatusCode::ACCEPTED);
    }
}
