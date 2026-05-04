use axum::{
    extract::Json,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::services::growth::invites::InviteTracker;

#[derive(Serialize, Deserialize)]
pub struct CreateTeamInviteRequest {
    pub team_id: String,
    pub inviter_id: String,
    pub invitee_id: String,
}

#[derive(Serialize)]
pub struct CreateTeamInviteResponse {
    pub status: String,
}

#[derive(Serialize)]
pub struct GetTeamInvitesCountResponse {
    pub count: i64,
}

#[derive(Serialize)]
pub struct GetTotalInvitesCountResponse {
    pub count: i64,
}

pub fn router<S: Clone + Send + Sync + 'static>(tracker: Arc<InviteTracker>) -> Router<S> {
    let tracker_post = tracker.clone();
    let tracker_get_team = tracker.clone();
    let tracker_get_total = tracker.clone();

    Router::new()
        .route("/team-invites", post(move |req_parts: axum::http::request::Parts, Json(req): Json<CreateTeamInviteRequest>| async move {
            let tenant_id = match req_parts.extensions.get::<crate::auth::Claims>() {
                Some(claims) => match &claims.organization_id {
                    Some(id) => id.clone(),
                    None => return (axum::http::StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
                },
                None => return (axum::http::StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
            };
            match tracker_post.record_invite(&tenant_id, &req.team_id, &req.inviter_id, &req.invitee_id).await {
                Ok(_) => Json(CreateTeamInviteResponse { status: "success".to_string() }).into_response(),
                Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
            }
        }))
        .route("/team-invites/count/:team_id", get(move |req_parts: axum::http::request::Parts, axum::extract::Path(team_id): axum::extract::Path<String>| async move {
            let tenant_id = match req_parts.extensions.get::<crate::auth::Claims>() {
                Some(claims) => match &claims.organization_id {
                    Some(id) => id.clone(),
                    None => return (axum::http::StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
                },
                None => return (axum::http::StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
            };
            match tracker_get_team.get_team_invites_count(&tenant_id, &team_id).await {
                Ok(count) => Json(GetTeamInvitesCountResponse { count }).into_response(),
                Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
            }
        }))
        .route("/team-invites/total", get(move |req_parts: axum::http::request::Parts| async move {
            let tenant_id = match req_parts.extensions.get::<crate::auth::Claims>() {
                Some(claims) => match &claims.organization_id {
                    Some(id) => id.clone(),
                    None => return (axum::http::StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
                },
                None => return (axum::http::StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
            };
            match tracker_get_total.get_total_invites_count(&tenant_id).await {
                Ok(count) => Json(GetTotalInvitesCountResponse { count }).into_response(),
                Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
            }
        }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::services::growth::invites::InviteRepository;
    use axum::http::Request;
    use tower::ServiceExt;
    #[tokio::test]
    async fn test_growth_team_invites_endpoints() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let database_url = "postgres://postgres:postgres@localhost:5432/test";
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy(database_url)
            .unwrap();

        // Ensure table exists for test
        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS team_invites (id VARCHAR PRIMARY KEY, tenant_id VARCHAR NOT NULL, team_id VARCHAR NOT NULL, inviter_id VARCHAR NOT NULL, invitee_id VARCHAR NOT NULL, status VARCHAR NOT NULL, created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP)")
            .execute(&pool)
            .await;

        let repo = Arc::new(InviteRepository::new(pool.clone()));
        let tracker = Arc::new(InviteTracker::new(repo));

        let app: Router<()> = router(tracker);

        let body = serde_json::to_string(&CreateTeamInviteRequest {
            team_id: "team_test".to_string(),
            inviter_id: "inviter1".to_string(),
            invitee_id: "invitee1".to_string(),
        }).unwrap();

        let mut req = Request::builder()
            .method("POST")
            .uri("/team-invites")
            .header("content-type", "application/json")
            .body(axum::body::Body::from(body))
            .unwrap();
        req.extensions_mut().insert(crate::auth::Claims {
            sub: "test".to_string(),
            exp: 0,
            iat: 0,
            jti: "test".to_string(),
            username: "test".to_string(),
            session_id: None,
            roles: vec!["user".to_string()],
            organization_id: Some("test_tenant".to_string()),
            email: "test@test.com".to_string(),
        });

        let res = tower::ServiceExt::oneshot(app.clone(), req).await.unwrap();
        assert_eq!(res.status(), axum::http::StatusCode::OK);

        let mut req = Request::builder()
            .method("GET")
            .uri("/team-invites/count/team_test")
            .body(axum::body::Body::empty())
            .unwrap();
        req.extensions_mut().insert(crate::auth::Claims {
            sub: "test".to_string(),
            exp: 0,
            iat: 0,
            jti: "test".to_string(),
            username: "test".to_string(),
            session_id: None,
            roles: vec!["user".to_string()],
            organization_id: Some("test_tenant".to_string()),
            email: "test@test.com".to_string(),
        });

        let res = tower::ServiceExt::oneshot(app.clone(), req).await.unwrap();
        assert_eq!(res.status(), axum::http::StatusCode::OK);
    }
}
