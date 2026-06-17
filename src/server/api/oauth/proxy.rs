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

            // Strictly validate tunnel_id to prevent Open Redirect/SSRF
            // It should only contain alphanumeric characters and hyphens.
            if tunnel_id.is_empty() || !tunnel_id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') || tunnel_id.starts_with('-') || tunnel_id.ends_with('-') {
                return (
                    axum::http::StatusCode::BAD_REQUEST,
                    "Invalid tunnel_id format.",
                )
                    .into_response();
            }

            let actual_state = parts[2];

            // Redirect to the standalone instance via the tunnel proxy
            let tunnel_base_url = std::env::var("OHC_TUNNEL_BASE_URL")
                .unwrap_or_else(|_| "https://tunnel.ohc.network".to_string());

            let mut redirect_url = format!("{}/{}/oauth/callback?code={}&state={}",
                tunnel_base_url,
                urlencoding::encode(&tunnel_id),
                urlencoding::encode(&query.code),
                urlencoding::encode(&actual_state));
            for (k, v) in query.extra {
                redirect_url.push_str(&format!("&{}={}", urlencoding::encode(&k), urlencoding::encode(&v)));
            }
            return Redirect::temporary(&redirect_url).into_response();
        }
    }

    // Default: Return a generic success page or redirect to local app
    "OAuth callback received. You can close this window.".into_response()
}

pub fn router() -> Router<std::sync::Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    Router::new()
        .route("/callback", get(handle_oauth_callback))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::Query;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_valid_tunnel_id() {
        let mut extra = HashMap::new();
        extra.insert("foo".to_string(), "bar".to_string());

        let query = OAuthCallbackQuery {
            code: "test_code".to_string(),
            state: "standalone_valid-tunnel-id-123_actualState123".to_string(),
            extra,
        };

        let response = handle_oauth_callback(Query(query)).await.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::TEMPORARY_REDIRECT);
    }

    #[tokio::test]
    async fn test_invalid_tunnel_id_path_traversal() {
        let query = OAuthCallbackQuery {
            code: "test_code".to_string(),
            state: "standalone_../etc/passwd_actualState123".to_string(),
            extra: HashMap::new(),
        };

        let response = handle_oauth_callback(Query(query)).await.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_invalid_tunnel_id_url() {
        let query = OAuthCallbackQuery {
            code: "test_code".to_string(),
            state: "standalone_http://malicious.com_actualState123".to_string(),
            extra: HashMap::new(),
        };

        let response = handle_oauth_callback(Query(query)).await.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_invalid_tunnel_id_spaces() {
        let query = OAuthCallbackQuery {
            code: "test_code".to_string(),
            state: "standalone_my tunnel id_actualState123".to_string(),
            extra: HashMap::new(),
        };

        let response = handle_oauth_callback(Query(query)).await.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_invalid_tunnel_id_empty() {
        let query = OAuthCallbackQuery {
            code: "test_code".to_string(),
            state: "standalone__actualState123".to_string(),
            extra: HashMap::new(),
        };

        let response = handle_oauth_callback(Query(query)).await.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_invalid_tunnel_id_hyphen_start() {
        let query = OAuthCallbackQuery {
            code: "test_code".to_string(),
            state: "standalone_-invalid_actualState123".to_string(),
            extra: HashMap::new(),
        };

        let response = handle_oauth_callback(Query(query)).await.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_invalid_tunnel_id_hyphen_end() {
        let query = OAuthCallbackQuery {
            code: "test_code".to_string(),
            state: "standalone_invalid-_actualState123".to_string(),
            extra: HashMap::new(),
        };

        let response = handle_oauth_callback(Query(query)).await.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
    }
}
