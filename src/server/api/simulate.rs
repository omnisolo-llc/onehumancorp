use axum::{extract::State, response::IntoResponse, Json};
use std::sync::Arc;
use crate::hub::Hub;
use server_ohc::orchestration::Message;
use serde_json::json;

pub async fn simulate_order_handler(State(hub): State<Arc<Hub>>) -> impl IntoResponse {
    let _ = hub.publish(Message {
        id: "msg_simulate".to_string(),
        from_agent: "system".to_string(),
        to_agent: "system".to_string(),
        r#type: "system".to_string(),
        content: "system:order_received".to_string(),
        occurred_at_unix: chrono::Utc::now().timestamp(),
        meeting_id: "".to_string(),
    });

    Json(json!({ "status": "ok" }))
}

pub fn router(hub: Arc<Hub>) -> axum::Router<Arc<Hub>> {
    axum::Router::new()
        .route("/order", axum::routing::post(simulate_order_handler))
        .with_state(hub)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_simulate_order_handler() {
        assert!(true);
    }
}
