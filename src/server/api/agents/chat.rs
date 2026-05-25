use axum::{
    extract::{Extension, State},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
    Json,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::{DepartmentType, ActionRisk};
use ::server_common::Claims;

#[derive(Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Serialize)]
pub struct ChatResponse {
    pub success: bool,
    pub department_assigned: Option<String>,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(handle_chat))
        .with_state(orchestrator)
}

async fn handle_chat(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<ChatRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(ChatResponse { success: false, department_assigned: None })).into_response(),
    };

    let msg = payload.message.to_lowercase();
    let (dept, description, payload_json) = if msg.contains("refund") {
        (
            DepartmentType::Operations,
            "Process refund request from team chat".to_string(),
            serde_json::json!({ "original_request": payload.message, "action": "refund" })
        )
    } else if msg.contains("post") || msg.contains("newsletter") || msg.contains("campaign") || msg.contains("promote") {
        (
            DepartmentType::Marketing,
            "Draft marketing content from team chat".to_string(),
            serde_json::json!({ "original_request": payload.message, "action": "create_content" })
        )
    } else if msg.contains("quote") || msg.contains("lead") || msg.contains("discount") || msg.contains("pricing") {
        (
            DepartmentType::Sales,
            "Draft sales response/quote from team chat".to_string(),
            serde_json::json!({ "original_request": payload.message, "action": "draft_quote" })
        )
    } else {
        (
            DepartmentType::Operations, // Fallback
            "General task assignment from team chat".to_string(),
            serde_json::json!({ "original_request": payload.message, "action": "general_task" })
        )
    };

    match orchestrator.execute_action(
        dept.clone(),
        description,
        tenant_id,
        ActionRisk::DraftForReview,
        payload_json,
    ).await {
        Ok(_) => (StatusCode::OK, Json(ChatResponse { success: true, department_assigned: Some(dept.to_string()) })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ChatResponse { success: false, department_assigned: None })).into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::post,
        Router,
    };
    use tower::ServiceExt;
    use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
    use crate::orchestration::mesh::CentrifugeNode;
    use ohc_builtin_agent::mesh::transport::InProcessTransport;
    use std::sync::Arc;
    use ::server_common::Claims;

    async fn create_app() -> Router {
        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let orchestrator = Arc::new(DepartmentOrchestrator::new(db, mesh));

        Router::new()
            .route("/", post(handle_chat))
            .with_state(orchestrator)
            .layer(axum::middleware::from_fn(|req: axum::extract::Request, next: axum::middleware::Next| async move {
                let mut req = req;
                req.extensions_mut().insert(Claims {
                    sub: "test".to_string(),
                    email: "test@test.com".to_string(),
                    exp: 0,
                    organization_id: Some("test_org".to_string()),
                });
                next.run(req).await
            }))
    }

    #[tokio::test]
    async fn test_chat_routing_refund() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }
        let app = create_app().await;

        let payload = serde_json::json!({
            "message": "Please refund order 123"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let resp: ChatResponse = serde_json::from_slice(&body).unwrap();
        assert!(resp.success);
        assert_eq!(resp.department_assigned, Some("operations".to_string()));
    }

    #[tokio::test]
    async fn test_chat_routing_marketing() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }
        let app = create_app().await;

        let payload = serde_json::json!({
            "message": "Draft a new newsletter for mothers day"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let resp: ChatResponse = serde_json::from_slice(&body).unwrap();
        assert!(resp.success);
        assert_eq!(resp.department_assigned, Some("marketing".to_string()));
    }

    #[tokio::test]
    async fn test_chat_routing_sales() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }
        let app = create_app().await;

        let payload = serde_json::json!({
            "message": "Give me a quote for roofing"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let resp: ChatResponse = serde_json::from_slice(&body).unwrap();
        assert!(resp.success);
        assert_eq!(resp.department_assigned, Some("sales".to_string()));
    }

    #[tokio::test]
    async fn test_chat_routing_fallback() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }
        let app = create_app().await;

        let payload = serde_json::json!({
            "message": "I need help with general stuff"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let resp: ChatResponse = serde_json::from_slice(&body).unwrap();
        assert!(resp.success);
        assert_eq!(resp.department_assigned, Some("operations".to_string()));
    }
}
