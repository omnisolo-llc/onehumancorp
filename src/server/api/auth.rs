use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::auth::Store;
use crate::oidc::validate_oidc_token;
use chrono::Utc;

#[derive(Deserialize)]
pub struct HandshakeRequest {
    pub oauth_token: String,
}

#[derive(Serialize)]
pub struct HandshakeResponse {
    pub token: String,
    pub tenant_id: String,
    pub mode: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn auth_handshake(
    State(store): State<Arc<Store>>,
    Json(payload): Json<HandshakeRequest>,
) -> impl IntoResponse {
    let oidc_cfg = store.get_oidc_cfg();
    match validate_oidc_token(&payload.oauth_token, &oidc_cfg).await {
        Ok(claims) => {
            let tenant_id = claims.organization_id.clone().unwrap_or_else(|| "".to_string());

            // For this simplified logic we issue the token directly if the claims are valid.
            let user = crate::auth::User {
                id: claims.sub.clone(),
                username: claims.username.clone(),
                email: claims.email.clone(),
                password_hash: "".to_string(), // OIDC handled it
                roles: claims.roles.clone(),
                active: true,
                organization_id: Some(tenant_id.clone()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                oidc_subject: Some(claims.sub.clone()),
            };

            match store.issue_oauth_token(&user) {
                Ok(token) => {
                    let resp = HandshakeResponse {
                        token,
                        tenant_id,
                        mode: "cloud".to_string(),
                    };
                    (axum::http::StatusCode::OK, Json(serde_json::json!(resp))).into_response()
                }
                Err(e) => {
                    let err_resp = ErrorResponse { error: e };
                    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!(err_resp))).into_response()
                }
            }
        },
        Err(e) => {
            let err_resp = ErrorResponse { error: e };
            (axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!(err_resp))).into_response()
        }
    }
}
