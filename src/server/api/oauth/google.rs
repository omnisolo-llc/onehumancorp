use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct GoogleConnectQuery {
    pub tenant_id: String,
}

pub async fn connect_google(
    Query(query): Query<GoogleConnectQuery>,
) -> impl IntoResponse {
    // Generate a secure state parameter (for real usage, we should sign/encrypt this)
    let state = format!("google_{}", query.tenant_id);

    // In a real app, use the actual Google OAuth endpoint and client ID
    let client_id = std::env::var("GOOGLE_CLIENT_ID").unwrap_or_else(|_| "dummy_client_id".to_string());
    let redirect_uri = std::env::var("GOOGLE_REDIRECT_URI").unwrap_or_else(|_| "http://localhost:3000/api/v1/oauth/callback".to_string());

    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope=https://www.googleapis.com/auth/business.manage&state={}&access_type=offline&prompt=consent",
        client_id, redirect_uri, state
    );

    Redirect::temporary(&auth_url)
}

pub fn router() -> Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    Router::new()
        .route("/connect", get(connect_google))
}
