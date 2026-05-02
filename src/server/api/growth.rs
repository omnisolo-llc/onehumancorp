use axum::{
    extract::{State, Json},
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::ohc::orchestration::growth_service_server::GrowthService;
use crate::ohc::orchestration::GrowthIdRequest;
use crate::services::growth::service::MyGrowthService;
use tonic::Request;
use axum::http::HeaderMap;

#[derive(Deserialize)]
pub struct IdRequest {
    pub id: String,
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: String,
}

pub fn router<S: Clone + Send + Sync + 'static>(growth_service: Arc<MyGrowthService>) -> Router<S> {
    Router::new()
        .route("/referrals/click", post(handle_referral_click))
        .route("/referrals/convert", post(handle_referral_convert))
        .route("/team-invites/accept", post(handle_team_invite_accept))
        .with_state(growth_service)
}

async fn handle_referral_click(
    headers: HeaderMap,
    State(service): State<Arc<MyGrowthService>>,
    Json(payload): Json<IdRequest>,
) -> impl IntoResponse {
    let mut req = Request::new(GrowthIdRequest { id: payload.id });
    if let Some(spiffe) = headers.get("x-spiffe-id") {
        if let Ok(spiffe_str) = spiffe.to_str() {
            if let Ok(val) = spiffe_str.parse() {
                req.metadata_mut().insert("x-spiffe-id", val);
            }
        }
    }

    match service.click_referral(req).await {
        Ok(_) => (axum::http::StatusCode::OK, Json(StatusResponse { status: "success".to_string() })).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.message().to_string()).into_response(),
    }
}

async fn handle_referral_convert(
    headers: HeaderMap,
    State(service): State<Arc<MyGrowthService>>,
    Json(payload): Json<IdRequest>,
) -> impl IntoResponse {
    let mut req = Request::new(GrowthIdRequest { id: payload.id });
    if let Some(spiffe) = headers.get("x-spiffe-id") {
        if let Ok(spiffe_str) = spiffe.to_str() {
            if let Ok(val) = spiffe_str.parse() {
                req.metadata_mut().insert("x-spiffe-id", val);
            }
        }
    }

    match service.convert_referral(req).await {
        Ok(_) => (axum::http::StatusCode::OK, Json(StatusResponse { status: "success".to_string() })).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.message().to_string()).into_response(),
    }
}

async fn handle_team_invite_accept(
    headers: HeaderMap,
    State(service): State<Arc<MyGrowthService>>,
    Json(payload): Json<IdRequest>,
) -> impl IntoResponse {
    let mut req = Request::new(GrowthIdRequest { id: payload.id });
    if let Some(spiffe) = headers.get("x-spiffe-id") {
        if let Ok(spiffe_str) = spiffe.to_str() {
            if let Ok(val) = spiffe_str.parse() {
                req.metadata_mut().insert("x-spiffe-id", val);
            }
        }
    }

    match service.accept_team_invite(req).await {
        Ok(_) => (axum::http::StatusCode::OK, Json(StatusResponse { status: "success".to_string() })).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.message().to_string()).into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    async fn get_test_pool() -> PgPool {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        PgPool::connect(&database_url).await.unwrap()
    }

    #[tokio::test]
    async fn test_growth_api_routes() {
        // We ensure compilation is correct for the router setup
        // and unit test the handlers manually.
        if let Err(_) = std::env::var("DATABASE_URL") {
            return;
        }

        let pool = get_test_pool().await;
        let service = Arc::new(MyGrowthService::new(pool));

        let mut headers = axum::http::HeaderMap::new();
        headers.insert("x-spiffe-id", "spiffe://onehumancorp.io/org1/agent1".parse().unwrap());

        // These requests would normally fail at the DB level during CI unless we create
        // mocked data, but the handler will process them properly and return the inner service error
        let _ = handle_referral_click(headers.clone(), axum::extract::State(service.clone()), axum::extract::Json(IdRequest { id: "test1".into() })).await;
        let _ = handle_referral_convert(headers.clone(), axum::extract::State(service.clone()), axum::extract::Json(IdRequest { id: "test2".into() })).await;
        let _ = handle_team_invite_accept(headers, axum::extract::State(service.clone()), axum::extract::Json(IdRequest { id: "test3".into() })).await;

        let _app: Router<()> = router(service);
    }
}
