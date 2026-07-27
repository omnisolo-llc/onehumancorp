use crate::db::DB;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct ToolIntegrationsApiState {
    pub db: Arc<DB>,
}

#[derive(Deserialize, Debug)]
pub struct ConnectIntegrationRequest {
    pub bot_token: Option<String>,
    pub api_token: Option<String>,
    pub from_phone: Option<String>,
    pub integration_id: Option<String>,
    pub base_url: Option<String>,
}

#[derive(Serialize)]
pub struct ConnectIntegrationResponse {
    pub success: bool,
    pub message: String,
    pub status: String,
    pub usable: bool,
}

fn connection_response(
    status_code: StatusCode,
    success: bool,
    message: &str,
    status: &str,
    usable: bool,
) -> (StatusCode, Json<ConnectIntegrationResponse>) {
    (
        status_code,
        Json(ConnectIntegrationResponse {
            success,
            message: message.to_string(),
            status: status.to_string(),
            usable,
        }),
    )
}

#[derive(Debug)]
struct ValidatedConnectIntegration {
    integration_id: String,
    bot_token: Option<String>,
    api_token: Option<String>,
    from_phone: Option<String>,
}

fn bounded_credential(value: Option<String>, maximum: usize) -> Result<Option<String>, ()> {
    match value {
        None => Ok(None),
        Some(value) => {
            let value = value.trim();
            if value.is_empty() {
                Ok(None)
            } else if value.len() <= maximum && !value.contains('\0') {
                Ok(Some(value.to_string()))
            } else {
                Err(())
            }
        }
    }
}

fn safe_integration_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
}

fn provider_credentials_present(
    integration_id: &str,
    bot_token: Option<&str>,
    api_token: Option<&str>,
) -> bool {
    let has_bot_token = bot_token.is_some_and(|value| !value.trim().is_empty());
    let has_api_token = api_token.is_some_and(|value| !value.trim().is_empty());
    match integration_id {
        "twilio" | "whatsapp" => has_bot_token && has_api_token,
        "whatsapp_meta" => has_api_token,
        _ => has_bot_token || has_api_token,
    }
}

fn validate_connect_request(
    path_id: &str,
    payload: ConnectIntegrationRequest,
) -> Result<ValidatedConnectIntegration, ()> {
    if !safe_integration_id(path_id) {
        return Err(());
    }
    let bot_token = bounded_credential(payload.bot_token, 4096)?;
    let api_token = bounded_credential(payload.api_token, 4096)?;
    let from_phone = bounded_credential(payload.from_phone, 128)?;
    let has_required_credentials =
        provider_credentials_present(path_id, bot_token.as_deref(), api_token.as_deref());
    if !has_required_credentials {
        return Err(());
    }
    Ok(ValidatedConnectIntegration {
        integration_id: path_id.to_string(),
        bot_token,
        api_token,
        from_phone,
    })
}

pub async fn connect_integration_handler(
    State(_state): State<ToolIntegrationsApiState>,
    axum::extract::Extension(user): axum::extract::Extension<::server_common::Claims>,
    Path(id): Path<String>,
    Json(payload): Json<ConnectIntegrationRequest>,
) -> impl IntoResponse {
    let Some(_tenant_id) = user
        .organization_id
        .filter(|value| !value.trim().is_empty())
    else {
        return connection_response(
            StatusCode::UNAUTHORIZED,
            false,
            "Authenticated organization required",
            "unavailable",
            false,
        )
        .into_response();
    };
    let validated = match validate_connect_request(&id, payload) {
        Ok(validated) => validated,
        Err(()) => {
            return connection_response(
                StatusCode::BAD_REQUEST,
                false,
                "Valid provider credentials are required",
                "unavailable",
                false,
            )
            .into_response();
        }
    };
    tracing::info!(
        integration_id = %validated.integration_id,
        bot_token_supplied = validated.bot_token.is_some(),
        api_token_supplied = validated.api_token.is_some(),
        from_phone_supplied = validated.from_phone.is_some(),
        "Rejected integration credential storage because provider verification is unavailable"
    );
    // Provider verification and encrypted secret storage are not implemented by
    // this route. Never persist plaintext credentials or report a connection
    // merely because non-empty strings were submitted.
    connection_response(
        StatusCode::NOT_IMPLEMENTED,
        false,
        "Secure provider verification is not configured",
        "unavailable",
        false,
    )
    .into_response()
}

#[derive(Serialize)]
pub struct IntegrationInfo {
    pub id: String,
    pub status: String,
    pub usable: bool,
}

#[derive(Serialize)]
pub struct GetIntegrationsResponse {
    pub success: bool,
    pub integrations: Vec<IntegrationInfo>,
    pub message: Option<String>,
}

pub async fn get_integrations_handler(
    State(state): State<ToolIntegrationsApiState>,
    axum::extract::Extension(user): axum::extract::Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let Some(tenant_id) = user
        .organization_id
        .filter(|value| !value.trim().is_empty())
    else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(GetIntegrationsResponse {
                success: false,
                integrations: vec![],
                message: Some("Authenticated organization required".to_string()),
            }),
        )
            .into_response();
    };

    let rows = match &state.db.store {
        crate::db::DbStore::Postgres => sqlx::query_as::<_, (String, String)>(
            "SELECT id, status FROM tool_integrations WHERE tenant_id = $1",
        )
        .bind(&tenant_id)
        .fetch_all(&state.db.pool)
        .await,
        crate::db::DbStore::Sqlite(pool) => sqlx::query_as::<_, (String, String)>(
            "SELECT id, status FROM tool_integrations WHERE tenant_id = ?",
        )
        .bind(&tenant_id)
        .fetch_all(pool)
        .await,
    };
    let rows = match rows {
        Ok(r) => r,
        Err(error) => {
            tracing::error!(error = %error, "Failed to load tenant integrations");
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(GetIntegrationsResponse {
                    success: false,
                    integrations: vec![],
                    message: Some("Integration storage is unavailable".to_string()),
                }),
            )
                .into_response();
        }
    };

    let integrations = rows
        .into_iter()
        .map(|(id, status)| IntegrationInfo {
            status: if status == "connected" {
                "verification_required".to_string()
            } else {
                status
            },
            usable: false,
            id,
        })
        .collect();

    Json(GetIntegrationsResponse {
        success: true,
        integrations,
        message: None,
    })
    .into_response()
}

pub fn router<S: Clone + Send + Sync + 'static>(db: Arc<DB>) -> Router<S> {
    let state = ToolIntegrationsApiState { db };
    Router::new()
        .route("/", get(get_integrations_handler))
        .route("/{id}/connect", post(connect_integration_handler))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connection_validation_uses_the_path_id_and_requires_provider_credentials() {
        let payload = ConnectIntegrationRequest {
            bot_token: Some(" AC123 ".to_string()),
            api_token: Some(" secret ".to_string()),
            from_phone: Some(" +15550001111 ".to_string()),
            integration_id: Some("attacker-selected".to_string()),
            base_url: Some("https://attacker.test".to_string()),
        };
        let validated = validate_connect_request("twilio", payload).expect("valid credentials");
        assert_eq!(validated.integration_id, "twilio");
        assert_eq!(validated.bot_token.as_deref(), Some("AC123"));
        assert_eq!(validated.api_token.as_deref(), Some("secret"));
        assert_eq!(validated.from_phone.as_deref(), Some("+15550001111"));
        assert!(provider_credentials_present(
            "twilio",
            validated.bot_token.as_deref(),
            validated.api_token.as_deref(),
        ));
    }

    #[test]
    fn connection_validation_rejects_missing_credentials_and_invalid_ids() {
        let empty = ConnectIntegrationRequest {
            bot_token: Some(" ".to_string()),
            api_token: None,
            from_phone: None,
            integration_id: None,
            base_url: None,
        };
        assert!(validate_connect_request("twilio", empty).is_err());
        let cloud = ConnectIntegrationRequest {
            bot_token: None,
            api_token: Some("meta-token".to_string()),
            from_phone: None,
            integration_id: None,
            base_url: None,
        };
        assert!(validate_connect_request("whatsapp_cloud_api", cloud).is_ok());
        assert!(!provider_credentials_present("twilio", Some("sid"), None));
        assert!(!provider_credentials_present("shippo", None, None));
        let invalid = ConnectIntegrationRequest {
            bot_token: Some("bot".to_string()),
            api_token: Some("token".to_string()),
            from_phone: None,
            integration_id: None,
            base_url: None,
        };
        assert!(validate_connect_request("../twilio", invalid).is_err());
    }
}
