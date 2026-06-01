use axum::{
    extract::{Query},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct OAuthCallbackQuery {
    pub code: String,
    pub state: String,
    #[serde(flatten)]
    pub extra: HashMap<String, String>,
}

pub async fn handle_oauth_callback(
    Query(query): Query<OAuthCallbackQuery>,
) -> impl IntoResponse {
    // In Standalone mode, we receive the callback and need to route it.
    // In Cloud mode, we just process it directly.
    let state = query.state;

    // Check if this looks like a proxy request for a standalone instance
    if state.starts_with("standalone_") {
        let parts: Vec<&str> = state.splitn(3, '_').collect();
        if parts.len() == 3 {
            let tunnel_id = parts[1];

            // Validate tunnel_id to prevent path traversal or open redirect.
            // It should only contain alphanumeric characters, hyphens, and underscores.
            let is_valid_tunnel_id = !tunnel_id.is_empty() && tunnel_id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');

            if is_valid_tunnel_id {
                let actual_state = parts[2];

                // Redirect to the standalone instance via the tunnel proxy
                let tunnel_base_url = std::env::var("OHC_TUNNEL_BASE_URL")
                    .unwrap_or_else(|_| "https://tunnel.ohc.network".to_string());

                let mut redirect_url = format!("{}/{}/oauth/callback?code={}&state={}",
                    tunnel_base_url, tunnel_id, query.code, actual_state);
                for (k, v) in query.extra {
                    redirect_url.push_str(&format!("&{}={}", k, v));
                }
                return Redirect::temporary(&redirect_url).into_response();
            }
        }
    }

    // Default: Return a generic success page or redirect to local app
    "OAuth callback received. You can close this window.".into_response()
}

pub fn router() -> Router<std::sync::Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    Router::new()
        .route("/callback", get(handle_oauth_callback))
}
