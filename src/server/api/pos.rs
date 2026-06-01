use axum::{Json, response::IntoResponse, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use server_common::Claims;
use axum::extract::Extension;
use ::ohc_builtin_agent::mesh::transport::MeshTransport;
use crate::db::get_pool;

#[derive(Serialize, Deserialize, Debug)]
pub struct TerminalSessionRequest {
    pub device_id: String,
}

#[derive(Serialize, Debug)]
pub struct TerminalSessionResponse {
    pub session_token: String,
    pub expires_at: String,
}

pub async fn create_session_handler(
    Extension(claims): Extension<Claims>,
    Json(payload): Json<TerminalSessionRequest>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_default();

    // Stub generation of a secure terminal token
    let session_token = format!("term_session_{}_{}", tenant_id, Uuid::new_v4().to_string().replace("-", ""));
    let expires_at = (chrono::Utc::now() + chrono::Duration::hours(24)).to_rfc3339();

    tracing::info!("Created stub terminal session for tenant {}, device {}", tenant_id, payload.device_id);

    (StatusCode::OK, Json(TerminalSessionResponse {
        session_token,
        expires_at,
    }))
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct PosItem {
    pub product_id: String,
    pub quantity: i32,
    pub price: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RecordTransactionRequest {
    pub session_token: String,
    pub items: Vec<PosItem>,
    pub total_amount: f64,
    pub currency: String,
    pub payment_method: String,
}

#[derive(Serialize, Debug)]
pub struct RecordTransactionResponse {
    pub success: bool,
    pub transaction_id: String,
}

pub async fn record_transaction_handler(
    State(mesh_transport): State<Arc<dyn MeshTransport>>,

    Extension(claims): Extension<Claims>,
    Json(payload): Json<RecordTransactionRequest>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_default();
    let transaction_id = format!("txn_{}", Uuid::new_v4().to_string().replace("-", ""));

    tracing::info!("Recording POS transaction {} for tenant {}", transaction_id, tenant_id);

    // Create a database transaction to insert into orders and order_items
    let mut tx = match get_pool().begin().await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Failed to start transaction: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(RecordTransactionResponse { success: false, transaction_id })).into_response();
        }
    };

    // Insert order (stubbing customer_id for now as "pos_customer")
    let order_insert = sqlx::query(
        "INSERT INTO orders (id, tenant_id, customer_id, total_amount, status) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO NOTHING"
    )
    .bind(&transaction_id)
    .bind(&tenant_id)
    .bind("pos_customer")
    .bind(payload.total_amount)
    .bind("completed")
    .execute(&mut *tx)
    .await;

    if let Err(e) = order_insert {
        tracing::error!("Failed to insert order: {}", e);
        let _ = tx.rollback().await;
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(RecordTransactionResponse { success: false, transaction_id })).into_response();
    }

    // Insert order items
    for item in &payload.items {
        let order_item_id = format!("oi_{}", Uuid::new_v4().to_string().replace("-", ""));
        let item_insert = sqlx::query(
            "INSERT INTO order_items (id, tenant_id, order_id, product_id, quantity, price) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(&order_item_id)
        .bind(&tenant_id)
        .bind(&transaction_id)
        .bind(&item.product_id)
        .bind(item.quantity)
        .bind(item.price)
        .execute(&mut *tx)
        .await;

        if let Err(e) = item_insert {
            tracing::error!("Failed to insert order item: {}", e);
            let _ = tx.rollback().await;
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(RecordTransactionResponse { success: false, transaction_id })).into_response();
        }
    }

    if let Err(e) = tx.commit().await {
         tracing::error!("Failed to commit transaction: {}", e);
         return (StatusCode::INTERNAL_SERVER_ERROR, Json(RecordTransactionResponse { success: false, transaction_id })).into_response();
    }

    // Publish mesh event for AI agents (Ops for inventory sync, Finance for analytics)
    let payload_json = serde_json::json!({
        "transactionId": transaction_id,
        "items": payload.items,
        "total": payload.total_amount,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "deviceId": "browser_pos_client",
        "agentTriggers": ["inventory-sync", "analytics", "fraud-check"],
    });

    let mesh_msg = ::ohc_builtin_agent::mesh::transport::Message {
        action: "pos.transaction.completed".to_string(),
        agent_id: "pos_system".to_string(),
        status: "success".to_string(),
        msg_id: Uuid::new_v4().to_string(),
        payload: serde_json::to_vec(&payload_json).unwrap_or_default(),
    };


    let topic = format!("tenant:{}:pos.transaction.completed", tenant_id);
    let _ = mesh_transport.publish(&topic, mesh_msg.into()).await;

    (StatusCode::OK, Json(RecordTransactionResponse {
        success: true,
        transaction_id,
    })).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, routing::post, Router};
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    // Define a dummy transport for tests
    struct DummyTransport;

    #[async_trait::async_trait]
    impl MeshTransport for DummyTransport {
        async fn acquire_lock(&self, _: &str, _: &str, _: u64) -> Result<bool, String> { Ok(true) }
        async fn release_lock(&self, _: &str, _: &str) -> Result<(), String> { Ok(()) }
        async fn register_presence(&self, _: &str, _: &str, _: u64) -> Result<(), String> { Ok(()) }
        async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> { Ok(vec![]) }
        async fn publish(&self, _topic: &str, _message: ::ohc_builtin_agent::mesh::transport::Message) -> Result<(), String> {
            Ok(())
        }
        async fn subscribe(&self, _topic: &str, _handler: Box<dyn Fn(::ohc_builtin_agent::mesh::transport::Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
            Ok(Box::new(|| {}))
        }
    }

    #[tokio::test]
    async fn test_create_session() {
        let app = Router::new()
            .route("/session", post(create_session_handler))
            .layer(axum::middleware::from_fn(|mut req: axum::extract::Request, next: axum::middleware::Next| async move {
                req.extensions_mut().insert(Claims {
                    sub: "user_1".to_string(),
                    organization_id: Some("tenant_1".to_string()),
                    exp: 0,
                    iat: 0,
                    username: "test".to_string(),
                    email: "test@example.com".to_string(),
                    session_id: Some("sid".to_string()),
                    jti: "jti".to_string(),
                    roles: vec!["admin".to_string()],
                });
                next.run(req).await
            }));

        let req_body = serde_json::to_vec(&TerminalSessionRequest {
            device_id: "test_dev".to_string(),
        }).unwrap();

        let response = app
            .oneshot(Request::builder()
                .method("POST")
                .uri("/session")
                .header("content-type", "application/json")
                .body(Body::from(req_body))
                .unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
