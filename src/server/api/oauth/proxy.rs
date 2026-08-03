use axum::{
    extract::{Query},
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::Deserialize;
use std::collections::HashMap;

fn escape_html(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '&' => escaped.push_str("&amp;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#x27;"),
            _ => escaped.push(c),
        }
    }
    escaped
}


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
            // It must be a valid UUID.
            if uuid::Uuid::parse_str(tunnel_id).is_err() {
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

            if tunnel_base_url.starts_with("http://127.0.0.1:") || tunnel_base_url.starts_with("http://localhost:") {
                // allowed for local dev
            } else if !tunnel_base_url.starts_with("https://") || tunnel_base_url.len() < 10 {
                return (
                    axum::http::StatusCode::BAD_REQUEST,
                    "Invalid tunnel_base_url: must be HTTPS or localhost.",
                ).into_response();
            } else {
                let stripped = tunnel_base_url.strip_prefix("https://").unwrap_or(&tunnel_base_url);
                let host = stripped.split('/').next().unwrap_or(stripped).split(':').next().unwrap_or(stripped);
                if !host.ends_with(".ohc.network") && host != "ohc.network" && host != "localhost" && host != "127.0.0.1" {
                    return (axum::http::StatusCode::BAD_REQUEST, "Invalid tunnel_base_url host").into_response();
                }
            }

            let mut redirect_url = format!("{}/{}/oauth/callback#code={}&state={}",
                tunnel_base_url,
                urlencoding::encode(&tunnel_id),
                urlencoding::encode(&query.code),
                urlencoding::encode(&actual_state));
            for (k, v) in query.extra {
                redirect_url.push_str(&format!("&{}={}", urlencoding::encode(&k), urlencoding::encode(&v)));
            }
            let safe_redirect_url = escape_html(&redirect_url);
            let html_redirect = format!(
                r#"<!DOCTYPE html>
<html>
<head>
    <meta name="referrer" content="no-referrer" />
    <meta http-equiv="refresh" content="0; url={}" />
    <script>
        window.location.replace("{}");
    </script>
</head>
<body>Redirecting...</body>
</html>"#,
                safe_redirect_url, safe_redirect_url
            );
            return (
                axum::http::StatusCode::OK,
                [("Cache-Control", "no-store")],
                axum::response::Html(html_redirect),
            ).into_response();
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
    async fn test_valid_tunnel_id_secure_fragment_redirect() {
        let mut extra = HashMap::new();
        extra.insert("foo".to_string(), "bar".to_string());

        let query = OAuthCallbackQuery {
            code: "test_code".to_string(),
            state: "standalone_123e4567-e89b-12d3-a456-426614174000_actualState123".to_string(),
            extra,
        };

        let response = handle_oauth_callback(Query(query)).await.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::OK);

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

        // Assert that the redirect uses a fragment (#) instead of a query string (?)
        assert!(body_str.contains("tunnel.ohc.network"));
        assert!(body_str.contains("code=test_code"));
        assert!(body_str.contains("state=actualState123"));
        assert!(body_str.contains("foo=bar"));
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


    #[tokio::test]
    async fn test_xss_in_callback_is_escaped() {
        let mut extra = HashMap::new();
        extra.insert("foo".to_string(), "bar\"<script>alert(1)</script>".to_string());

        let query = OAuthCallbackQuery {
            code: "\"><img src=x onerror=alert(1)>".to_string(),
            state: "standalone_123e4567-e89b-12d3-a456-426614174000_actualState123".to_string(),
            extra,
        };

        let response = handle_oauth_callback(Query(query)).await.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::OK);

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

        assert!(!body_str.contains("<script>alert(1)</script>"));
        assert!(!body_str.contains("\"><img"));

    }

    #[tokio::test]
    async fn test_oauth_callback_extra_query_params_escaping() {
        let mut extra = HashMap::new();
        extra.insert("token".to_string(), "abc+def".to_string());

        let query = OAuthCallbackQuery {
            code: "test_code".to_string(),
            state: "standalone_123e4567-e89b-12d3-a456-426614174000_actualState123".to_string(),
            extra,
        };

        let response = handle_oauth_callback(Query(query)).await.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::OK);

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

        assert!(body_str.contains("token=abc%2Bdef"));
    }
}
