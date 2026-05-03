use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
    routing::{post, get},
    Router,
};
use axum::extract::Extension;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct EscalateRequest {
    pub local_id: String,
    pub payload: serde_json::Value,
}

#[derive(Serialize, Debug)]
pub struct EscalateResponse {
    pub cloud_id: String,
    pub status: String,
}

#[derive(Serialize, Debug)]
pub struct StatusResponse {
    pub cloud_id: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub fn router<S: Clone + Send + Sync + 'static>(state: AppState) -> Router<S> {
    Router::new()
        .route("/escalate", post(escalate_mission))
        .route("/:cloud_id/status", get(mission_status))
        .with_state(state)
}

async fn escalate_mission(
    State(state): State<AppState>,
    Extension(claims): Extension<crate::auth::Claims>,
    Json(req): Json<EscalateRequest>,
) -> impl IntoResponse {
    let cloud_id = Uuid::new_v4().to_string();

    let org_id = claims.organization_id.unwrap_or_else(|| "system".to_string());

    // Insert into cloud `agent_missions`
    let result = sqlx::query(
        "INSERT INTO agent_missions (id, status, payload, organization_id, created_at, updated_at)
         VALUES ($1, 'PENDING', $2, $3, NOW(), NOW())"
    )
    .bind(&cloud_id)
    .bind(&req.payload)
    .bind(&org_id)
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => Json(EscalateResponse {
            cloud_id,
            status: "ACCEPTED".to_string(),
        }).into_response(),
        Err(e) => {
            eprintln!("Failed to insert escalated mission: {}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to escalate mission").into_response()
        }
    }
}

async fn mission_status(
    State(state): State<AppState>,
    Extension(claims): Extension<crate::auth::Claims>,
    Path(cloud_id): Path<String>,
) -> impl IntoResponse {
    let org_id = claims.organization_id.unwrap_or_else(|| "system".to_string());

    let result = sqlx::query(
        "SELECT status FROM agent_missions WHERE id = $1 AND organization_id = $2"
    )
    .bind(&cloud_id)
    .bind(&org_id)
    .fetch_optional(&state.pool)
    .await;

    match result {
        Ok(Some(row)) => {
            let status: String = sqlx::Row::get(&row, "status");
            Json(StatusResponse {
                cloud_id,
                status,
                result: None, // Simplified for now, real implementation might fetch result payload
            }).into_response()
        },
        Ok(None) => {
            (axum::http::StatusCode::NOT_FOUND, "Mission not found").into_response()
        },
        Err(e) => {
            eprintln!("Failed to fetch mission status: {}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch status").into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        middleware,
    };
    use axum::extract::Request as AxumRequest;
    use tower::ServiceExt;

    async fn mock_auth_middleware(
        mut req: AxumRequest,
        next: axum::middleware::Next,
    ) -> axum::response::Response {
        let claims = crate::auth::Claims {
            sub: "test_user".to_string(),
            username: "test_user".to_string(),
            email: "test_user@example.com".to_string(),
            roles: vec![],
            organization_id: Some("system".to_string()),
            session_id: None,
            iat: 0,
            exp: 9999999999,
            jti: "test_jti".to_string(),
        };
        req.extensions_mut().insert(claims);
        next.run(req).await
    }

    #[tokio::test]
    async fn test_escalate_and_status() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let database_url = std::env::var("DATABASE_URL").unwrap();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; Ok(true) }) })
            .acquire_timeout(Duration::from_millis(50))
            .before_acquire(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("SET app.current_tenant = 'system'").await?; Ok(true) }) }).connect_lazy(&database_url)
            .unwrap();

        let state = AppState { pool: pool.clone() };

        let app = router::<()>(state.clone())
            .layer(middleware::from_fn(mock_auth_middleware));

        // Test escalate
        let payload = serde_json::json!({
            "local_id": "test_local_id",
            "payload": {
                "role": "test",
                "task": "test",
            }
        });

        let req = Request::builder()
            .method("POST")
            .uri("/escalate")
            .header("content-type", "application/json")
            .body(Body::from(payload.to_string()))
            .unwrap();

        let response = app.clone().oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();

        let cloud_id = body["cloud_id"].as_str().unwrap().to_string();

        // Test status
        let req = Request::builder()
            .method("GET")
            .uri(format!("/{}/status", cloud_id))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
