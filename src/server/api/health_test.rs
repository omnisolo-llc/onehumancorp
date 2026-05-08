use axum::{
    routing::get,
    Router,
};

use crate::hub::Hub;
use crate::api::health::health_handler;

#[tokio::test]
async fn test_health_handler_response() {
    let pg_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect_lazy("postgres://dummy")
        .unwrap();

    let (tx, _) = tokio::sync::mpsc::channel(100);
    let hub = std::sync::Arc::new(Hub::new(tx, pg_pool));

    let app = Router::new()
        .route("/api/health", get(health_handler))
        .with_state(hub);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let client_req = reqwest::Client::new();
    let response = client_req.get(format!("http://{}/api/health", addr)).send().await.unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let body: serde_json::Value = response.json().await.unwrap();

    assert!(body.get("mode").is_some());
    assert!(body.get("status").is_some());
    assert!(body.get("db_ping").is_some());
    assert!(body.get("sync_backlog").is_some());
    assert!(body.get("stuck_missions").is_some());
    assert!(body.get("mesh_active").is_some());
}
