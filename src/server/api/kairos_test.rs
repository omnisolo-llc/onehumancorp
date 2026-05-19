use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use serde_json::json;
use ::server_common::Claims;
use sqlx::PgPool;
use std::sync::Arc;

use crate::api::kairos::{router, PendingAction, PendingActionsResponse, DecisionResponse};

async fn setup_db() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to DB");

    // Ensure table exists for tests
    let _ = sqlx::query(
        "CREATE TABLE IF NOT EXISTS agent_pending_actions (
            id TEXT PRIMARY KEY,
            tenant_id TEXT,
            agent_id TEXT,
            risk_level TEXT,
            payload JSONB,
            status TEXT,
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await;

    let _ = sqlx::query("DELETE FROM agent_pending_actions").execute(&pool).await;

    pool
}

fn get_claims(tenant_id: &str) -> Claims {
    Claims {
        sub: "user_1".to_string(),
        exp: 0,
        iat: 0,
        organization_id: Some(tenant_id.to_string()),
        username: "test_user".to_string(),
        email: "".to_string(),
        roles: vec![],
        session_id: Some("session_1".to_string()),
        jti: "".to_string(),
    }
}

async fn start_server(app: Router) -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    format!("http://{}", addr)
}

#[tokio::test]
async fn test_create_and_get_pending_actions() {
    if std::env::var("DATABASE_URL").is_err() {
        return;
    }
    let pool = setup_db().await;
    let tenant_id = "test_tenant";

    // Create
    let action = PendingAction {
        id: "action_1".to_string(),
        tenant_id: tenant_id.to_string(),
        agent_id: "agent_x".to_string(),
        risk_level: "HIGH".to_string(),
        payload: json!({"some": "data"}),
        status: "PENDING".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    crate::api::kairos::create_pending_action(&pool, action).await.unwrap();

    let app = router(pool.clone());
    let app = app.layer(axum::middleware::from_fn(|mut req: axum::extract::Request, next: axum::middleware::Next| async move {
        req.extensions_mut().insert(get_claims("test_tenant"));
        next.run(req).await
    }));

    let base_url = start_server(app).await;
    let client = reqwest::Client::new();

    let response = client.get(format!("{}/actions/pending", base_url)).send().await.unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let res: PendingActionsResponse = response.json().await.unwrap();

    assert_eq!(res.actions.len(), 1);
    assert_eq!(res.actions[0].id, "action_1");
    assert_eq!(res.actions[0].status, "PENDING");
}

#[tokio::test]
async fn test_approve_action() {
    if std::env::var("DATABASE_URL").is_err() {
        return;
    }
    let pool = setup_db().await;
    let tenant_id = "test_tenant_approve";

    let action = PendingAction {
        id: "action_approve".to_string(),
        tenant_id: tenant_id.to_string(),
        agent_id: "agent_x".to_string(),
        risk_level: "HIGH".to_string(),
        payload: json!({"some": "data"}),
        status: "PENDING".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    crate::api::kairos::create_pending_action(&pool, action).await.unwrap();

    let app = router(pool.clone());
    let app = app.layer(axum::middleware::from_fn(|mut req: axum::extract::Request, next: axum::middleware::Next| async move {
        req.extensions_mut().insert(get_claims("test_tenant_approve"));
        next.run(req).await
    }));

    let base_url = start_server(app).await;
    let client = reqwest::Client::new();

    let response = client.post(format!("{}/actions/approve", base_url))
        .json(&json!({"task_id": "action_approve"}))
        .send().await.unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let res: DecisionResponse = response.json().await.unwrap();
    assert!(res.success);

    let db_action = sqlx::query("SELECT status FROM agent_pending_actions WHERE id = 'action_approve'")
        .fetch_one(&pool)
        .await
        .unwrap();
    let status: String = sqlx::Row::get(&db_action, "status");
    assert_eq!(status, "APPROVED");
}

#[tokio::test]
async fn test_reject_action() {
    if std::env::var("DATABASE_URL").is_err() {
        return;
    }
    let pool = setup_db().await;
    let tenant_id = "test_tenant_reject";

    let action = PendingAction {
        id: "action_reject".to_string(),
        tenant_id: tenant_id.to_string(),
        agent_id: "agent_x".to_string(),
        risk_level: "HIGH".to_string(),
        payload: json!({"some": "data"}),
        status: "PENDING".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    crate::api::kairos::create_pending_action(&pool, action).await.unwrap();

    let app = router(pool.clone());
    let app = app.layer(axum::middleware::from_fn(|mut req: axum::extract::Request, next: axum::middleware::Next| async move {
        req.extensions_mut().insert(get_claims("test_tenant_reject"));
        next.run(req).await
    }));

    let base_url = start_server(app).await;
    let client = reqwest::Client::new();

    let response = client.post(format!("{}/actions/reject", base_url))
        .json(&json!({"task_id": "action_reject"}))
        .send().await.unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let res: DecisionResponse = response.json().await.unwrap();
    assert!(res.success);

    let db_action = sqlx::query("SELECT status FROM agent_pending_actions WHERE id = 'action_reject'")
        .fetch_one(&pool)
        .await
        .unwrap();
    let status: String = sqlx::Row::get(&db_action, "status");
    assert_eq!(status, "REJECTED");
}
