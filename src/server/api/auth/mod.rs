use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct HandshakeRequest {
    pub credentials: Option<String>,
    pub oauth_token: Option<String>,
    pub mode: Option<String>,
    pub tenant_id: Option<String>,
}

#[derive(Serialize)]
pub struct HandshakeResponse {
    pub token: String,
    pub tenant_id: String,
    pub mode: String,
}

pub async fn handshake_handler(
    State(store): State<Arc<crate::auth::Store>>,
    Json(payload): Json<HandshakeRequest>,
) -> axum::response::Result<Json<HandshakeResponse>, (axum::http::StatusCode, String)> {
    // Basic handshake implementation for Thin Client connection
    if std::env::var("OHC_REQUIRE_SPIFFE").is_ok() {
        // Handshake not supported when strict SPIFFE is required
        return Err((axum::http::StatusCode::UNAUTHORIZED, "SPIFFE mTLS authentication is strictly required.".to_string()));
    }


    let (username, password) = if let Some(creds) = payload.credentials {
        let parts: Vec<&str> = creds.split(':').collect();
        if parts.len() >= 2 {
            let username = parts[0].to_string();
            let password = parts[1..].join(":");
            (username, password)
        } else {
            return Err((axum::http::StatusCode::BAD_REQUEST, "Invalid credentials format".to_string()));
        }
    } else {
        return Err((axum::http::StatusCode::BAD_REQUEST, "Missing credentials".to_string()));
    };

    // For a real implementation, you'd extract org_id from the context or the request
    let org_id = payload.tenant_id.unwrap_or_default();
    if ::server_config::get().multitenant && org_id.trim().is_empty() {
        return Err((axum::http::StatusCode::BAD_REQUEST, "tenant_id is required in cloud mode".to_string()));
    }

    match store.authenticate(&username, &password, org_id.clone()) {
        Ok(user) => {
            match store.issue_token(&user) {
                Ok(token) => {
                    let tenant_id = user.organization_id.unwrap_or_else(|| "default".to_string());
                    Ok(Json(HandshakeResponse {
                        token,
                        tenant_id,
                        mode: if ::server_config::get().multitenant { "cloud".to_string() } else { "standalone".to_string() },
                    }))
                }
                Err(e) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, e)),
            }
        }
        Err(e) => Err((axum::http::StatusCode::UNAUTHORIZED, e)),
    }
}
