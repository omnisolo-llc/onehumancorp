use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::mesh;

#[tokio::test]
async fn test_broadcast_handler_returns_200() {
    let app = mesh::router();

    let listener = TcpListener::bind("127.0.0.0:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let client = reqwest::Client::new();
    let payload = json!({
        "channel": "mesh:tasks",
        "event_type": "TASK_TRANSITION",
        "data": {
            "task_id": "task_12345",
            "previous_state": "PENDING",
            "new_state": "IN_PROGRESS"
        }
    });

    let response = client
        .post(format!("http://{addr}/broadcast"))
        .json(&payload)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["status"], "success");
    assert_eq!(body["payload"]["channel"], "mesh:tasks");
}
