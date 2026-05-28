use axum::{
    extract::{State, Extension},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
    Json,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use ::server_common::Claims;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct VoiceTestResponse {
    pub success: bool,
}

pub fn router<S>(pool: PgPool) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/test", post(test_voice_receptionist))
        .with_state(pool)
}

async fn test_voice_receptionist(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(VoiceTestResponse { success: false })).into_response(),
    };

    let id = format!("msg_{}", chrono::Utc::now().timestamp_millis());
    let source = "Voice".to_string();
    let content = "AI Summary: Caller wants a plumbing quote. Sent booking link via SMS.\n\nTranscript: \nCustomer: Hi, I need a plumbing quote.\nAI: Hello! I can help with that. I've sent a booking link to your phone. Let me know if you need anything else.\nCustomer: Thanks, bye!".to_string();
    let status = "handled".to_string();

    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(VoiceTestResponse { success: false })).into_response(),
    };

    if let Err(_) = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(VoiceTestResponse { success: false })).into_response(),
    }

    let query = "INSERT INTO inbox_messages (id, tenant_id, source, content, status) VALUES ($1, $2, $3, $4, $5)";
    match sqlx::query(query)
        .bind(&id)
        .bind(&tenant_id)
        .bind(&source)
        .bind(&content)
        .bind(&status)
        .execute(&mut *tx)
        .await
    {
        Ok(_) => {
            let _ = tx.commit().await;
            (StatusCode::OK, Json(VoiceTestResponse { success: true })).into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(VoiceTestResponse { success: false })).into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;
    use axum::body::Body;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_voice_receptionist_success() {
        let pool = crate::db::get_pool();
        let app = router(pool.clone());
        let claims = Claims {
            organization_id: Some("test_org".to_string()),
            ..Default::default()
        };

        let req = Request::builder()
            .method("POST")
            .uri("/test")
            .extension(claims)
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_voice_receptionist_unauthorized() {
        let pool = crate::db::get_pool();
        let app = router(pool.clone());
        let claims = Claims {
            organization_id: None,
            ..Default::default()
        };

        let req = Request::builder()
            .method("POST")
            .uri("/test")
            .extension(claims)
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
