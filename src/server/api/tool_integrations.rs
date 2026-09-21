use crate::db::DB;
use ::server_harness::middleware::{connection_vault::ConnectionVault, usage_ledger::UsageLedger};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use hmac::{Hmac, Mac};

pub fn connection_vault(db: &DB) -> Result<ConnectionVault, String> {
    let ledger = match &db.store {
        crate::db::DbStore::Postgres => UsageLedger::Postgres(db.pool.clone()),
        crate::db::DbStore::Sqlite(pool) => UsageLedger::Sqlite(pool.clone()),
    };
    ConnectionVault::from_environment(ledger).map_err(|error| error.to_string())
}

pub async fn stripe_key_for_tenant(db: &DB, tenant: &str) -> Result<String, String> {
    if std::env::var_os("OMNISOLO_CONNECTION_KEYS").is_some() {
        // A missing/revoked tenant key must never fall back to another payer.
        return connection_vault(db)?
            .read_key(tenant, "stripe")
            .await
            .map(|key| key.to_string())
            .map_err(|error| error.to_string());
    }
    if crate::is_standalone_runtime() {
        return std::env::var("STRIPE_API_KEY")
            .map_err(|_| "Payment connection is not configured".into());
    }
    Err("A verified tenant payment connection is required".into())
}

#[derive(Clone)]
pub struct ToolIntegrationsApiState {
    pub db: Arc<DB>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
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
        "whatsapp_cloud_api" => has_api_token,
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
    State(state): State<ToolIntegrationsApiState>,
    axum::extract::Extension(user): axum::extract::Extension<::server_common::Claims>,
    Path(id): Path<String>,
    Json(payload): Json<ConnectIntegrationRequest>,
) -> impl IntoResponse {
    if !user
        .roles
        .iter()
        .any(|role| role.eq_ignore_ascii_case("owner") || role.eq_ignore_ascii_case("admin"))
    {
        return connection_response(
            StatusCode::FORBIDDEN,
            false,
            "Owner approval is required",
            "unavailable",
            false,
        )
        .into_response();
    }
    let Some(tenant_id) = user
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
    if matches!(validated.integration_id.as_str(), "openai_api" | "stripe") {
        let Some(secret) = validated.api_token.as_deref() else {
            return connection_response(
                StatusCode::BAD_REQUEST,
                false,
                "An API key is required; subscription tokens are not accepted",
                "unavailable",
                false,
            )
            .into_response();
        };
        let vault = match connection_vault(&state.db) {
            Ok(vault) => vault,
            Err(_) => {
                return connection_response(
                    StatusCode::SERVICE_UNAVAILABLE,
                    false,
                    "Connection encryption is not configured",
                    "unavailable",
                    false,
                )
                .into_response();
            }
        };
        if vault.initialize().await.is_err() {
            return connection_response(
                StatusCode::SERVICE_UNAVAILABLE,
                false,
                "Connection storage is unavailable",
                "unavailable",
                false,
            )
            .into_response();
        }
        return match vault.verify_and_store(&tenant_id,&validated.integration_id,secret).await {
            Ok(_) => connection_response(StatusCode::OK,true,"Provider API key verified and encrypted; available to supported tenant-scoped routes","verified",true).into_response(),
            Err(_) => connection_response(StatusCode::BAD_GATEWAY,false,"Provider verification failed; no connection was stored","unavailable",false).into_response(),
        };
    }
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
        crate::db::DbStore::Postgres => {
            sqlx::query_as::<_, (String, String)>(
                "SELECT id, status FROM tool_integrations WHERE tenant_id = $1",
            )
            .bind(&tenant_id)
            .fetch_all(&state.db.pool)
            .await
        }
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query_as::<_, (String, String)>(
                "SELECT id, status FROM tool_integrations WHERE tenant_id = ?",
            )
            .bind(&tenant_id)
            .fetch_all(pool)
            .await
        }
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

    let mut integrations: Vec<IntegrationInfo> = rows
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

    if let Ok(vault) = connection_vault(&state.db)
        && let Ok(verified) = vault.list(&tenant_id).await
    {
        for connection in verified {
            integrations.retain(|entry| entry.id != connection.provider);
            integrations.push(IntegrationInfo {
                id: connection.provider,
                usable: connection.state == "verified",
                status: connection.state,
            });
        }
    }
    Json(GetIntegrationsResponse {
        success: true,
        integrations,
        message: None,
    })
    .into_response()
}

async fn refresh_integration_handler(
    State(state): State<ToolIntegrationsApiState>,
    axum::extract::Extension(user): axum::extract::Extension<::server_common::Claims>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    if !user
        .roles
        .iter()
        .any(|role| role.eq_ignore_ascii_case("owner") || role.eq_ignore_ascii_case("admin"))
    {
        return connection_response(
            StatusCode::FORBIDDEN,
            false,
            "Owner approval is required",
            "unavailable",
            false,
        )
        .into_response();
    }
    let Some(tenant) = user
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
    if !matches!(id.as_str(), "openai_api" | "stripe") {
        return connection_response(
            StatusCode::NOT_IMPLEMENTED,
            false,
            "Provider revalidation is not supported",
            "unavailable",
            false,
        )
        .into_response();
    }
    let Ok(vault) = connection_vault(&state.db) else {
        return connection_response(
            StatusCode::SERVICE_UNAVAILABLE,
            false,
            "Connection encryption is not configured",
            "unavailable",
            false,
        )
        .into_response();
    };
    match vault.refresh(&tenant, &id).await {
        Ok(_) => connection_response(
            StatusCode::OK,
            true,
            "Connection reverified with the provider",
            "verified",
            true,
        )
        .into_response(),
        Err(_) => connection_response(
            StatusCode::BAD_GATEWAY,
            false,
            "Connection could not be reverified; no new authorization was granted",
            "verification_required",
            false,
        )
        .into_response(),
    }
}

async fn revoke_integration_handler(
    State(state): State<ToolIntegrationsApiState>,
    axum::extract::Extension(user): axum::extract::Extension<::server_common::Claims>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    if !user
        .roles
        .iter()
        .any(|role| role.eq_ignore_ascii_case("owner") || role.eq_ignore_ascii_case("admin"))
    {
        return connection_response(
            StatusCode::FORBIDDEN,
            false,
            "Owner approval is required",
            "unavailable",
            false,
        )
        .into_response();
    }
    let Some(tenant) = user
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
    let Ok(vault) = connection_vault(&state.db) else {
        return connection_response(
            StatusCode::SERVICE_UNAVAILABLE,
            false,
            "Connection vault unavailable",
            "unavailable",
            false,
        )
        .into_response();
    };
    match vault.revoke(&tenant, &id).await {
        Ok(()) => connection_response(
            StatusCode::OK,
            true,
            "Connection revoked for new requests; accepted provider requests may still finish",
            "revoked",
            false,
        )
        .into_response(),
        Err(_) => connection_response(
            StatusCode::SERVICE_UNAVAILABLE,
            false,
            "Unable to revoke connection",
            "unavailable",
            false,
        )
        .into_response(),
    }
}

async fn connect_google_calendar_handler(
    State(_state): State<ToolIntegrationsApiState>,
    axum::extract::Extension(user): axum::extract::Extension<::server_common::Claims>,
) -> impl IntoResponse {
    if !user
        .roles
        .iter()
        .any(|role| role.eq_ignore_ascii_case("owner") || role.eq_ignore_ascii_case("admin"))
    {
        return connection_response(StatusCode::FORBIDDEN, false, "Owner approval is required", "unavailable", false).into_response();
    }
    let Some(tenant_id) = user.organization_id.filter(|value| !value.trim().is_empty()) else {
        return connection_response(StatusCode::UNAUTHORIZED, false, "Authenticated organization required", "unavailable", false).into_response();
    };

    let client_id = match std::env::var("GOOGLE_CALENDAR_CLIENT_ID") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => return connection_response(StatusCode::SERVICE_UNAVAILABLE, false, "Google Calendar OAuth is not configured", "unavailable", false).into_response(),
    };

    let base = std::env::var("OMNISOLO_TUNNEL_BASE_URL").unwrap_or_else(|_| "https://cloud.omnisolo.co".to_string());
    let redirect_uri = std::env::var("GOOGLE_CALENDAR_REDIRECT_URI").unwrap_or_else(|_| format!("{}/api/v1/oauth/google_calendar/callback", base));

    let scope = "https://www.googleapis.com/auth/calendar.events https://www.googleapis.com/auth/calendar.readonly";
    let expiration = chrono::Utc::now().timestamp() + 3600;
    let raw_state = format!("{}:{}", tenant_id, expiration);
    let mut mac = Hmac::<sha2::Sha256>::new_from_slice(std::env::var("OMNISOLO_SECRET_KEY").unwrap_or_else(|_| "dev".into()).as_bytes()).unwrap();
    mac.update(raw_state.as_bytes());
    let state = format!("{}.{}", raw_state, hex::encode(mac.finalize().into_bytes()));

    let redirect_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&scope={}&response_type=code&access_type=offline&prompt=consent&state={}",
        urlencoding::encode(&client_id), urlencoding::encode(&redirect_uri), urlencoding::encode(scope), urlencoding::encode(&state)
    );

    Json(serde_json::json!({
        "success": true, "redirect_url": redirect_url, "message": "Redirecting to Google", "status": "pending", "usable": false
    })).into_response()
}

#[derive(Deserialize)]
pub struct OAuthCallbackQuery {
    pub code: String,
    pub state: String,
}

pub async fn google_calendar_oauth_callback_handler(
    State(state): State<ToolIntegrationsApiState>,
    axum::extract::Query(query): axum::extract::Query<OAuthCallbackQuery>,
) -> impl IntoResponse {
    let split_state: Vec<&str> = query.state.rsplitn(2, '.').collect();
    if split_state.len() != 2 {
        return (StatusCode::BAD_REQUEST, "Invalid or expired state parameter.").into_response();
    }
    let raw_state = split_state[1];
    let signature = split_state[0];

    let mut mac = Hmac::<sha2::Sha256>::new_from_slice(std::env::var("OMNISOLO_SECRET_KEY").unwrap_or_else(|_| "dev".into()).as_bytes()).unwrap();
    mac.update(raw_state.as_bytes());
    if hex::encode(mac.finalize().into_bytes()) != signature {
        return (StatusCode::BAD_REQUEST, "Invalid or expired state parameter.").into_response();
    }

    let parts: Vec<&str> = raw_state.split(':').collect();
    if parts.len() != 2 || parts[1].parse::<i64>().unwrap_or(0) < chrono::Utc::now().timestamp() {
        return (StatusCode::BAD_REQUEST, "Invalid or expired state parameter.").into_response();
    }
    let tenant_id = parts[0].to_string();

    let client_id = std::env::var("GOOGLE_CALENDAR_CLIENT_ID").unwrap_or_default();
    let client_secret = std::env::var("GOOGLE_CALENDAR_CLIENT_SECRET").unwrap_or_default();
    let base = std::env::var("OMNISOLO_TUNNEL_BASE_URL").unwrap_or_else(|_| "https://cloud.omnisolo.co".to_string());
    let redirect_uri = std::env::var("GOOGLE_CALENDAR_REDIRECT_URI").unwrap_or_else(|_| format!("{}/api/v1/oauth/google_calendar/callback", base));

    let token_url = "https://oauth2.googleapis.com/token";
    let client = reqwest::Client::new();
    let params = [
        ("client_id", client_id.as_str()), ("client_secret", client_secret.as_str()), ("code", query.code.as_str()), ("redirect_uri", redirect_uri.as_str()), ("grant_type", "authorization_code")
    ];

    let res = match client.post(token_url).form(&params).send().await {
        Ok(response) => response,
        Err(e) => {
            tracing::error!("Failed to exchange Google Calendar code: {}", e);
            return (StatusCode::BAD_GATEWAY, "Failed to exchange token").into_response();
        }
    };

    if !res.status().is_success() {
        tracing::error!("Google Calendar token exchange error {}: {}", res.status(), res.text().await.unwrap_or_default());
        return (StatusCode::BAD_GATEWAY, "Failed to exchange token").into_response();
    }

    let json_res: serde_json::Value = match res.json().await {
        Ok(j) => j,
        Err(e) => {
            tracing::error!("Failed to parse Google Calendar token response: {}", e);
            return (StatusCode::BAD_GATEWAY, "Failed to parse token response").into_response();
        }
    };

    let access_token = json_res["access_token"].as_str().unwrap_or_default().to_string();
    let mut refresh_token = json_res["refresh_token"].as_str().map(|s| s.to_string());
    let expires_in: i64 = json_res["expires_in"].as_i64().unwrap_or(3600);
    let expires_at = chrono::Utc::now() + chrono::Duration::seconds(expires_in);

    let Ok(vault) = connection_vault(&state.db) else {
        return (StatusCode::SERVICE_UNAVAILABLE, "Connection encryption is not configured").into_response();
    };
    if vault.initialize().await.is_err() {
        return (StatusCode::SERVICE_UNAVAILABLE, "Connection storage is unavailable").into_response();
    }

    if refresh_token.is_none() {
        // The vault expects a JSON encoded payload for Google Calendar secret with `access_token` and optional `refresh_token`
        if let Ok(key) = vault.read_key(&tenant_id, "google_calendar").await
            && let Ok(json) = serde_json::from_str::<serde_json::Value>(&key)
            && let Some(r) = json["refresh_token"].as_str() {
            refresh_token = Some(r.to_string());
        }
    }

    let secret_json = serde_json::json!({
        "access_token": access_token,
        "refresh_token": refresh_token,
        "expires_at": expires_at.timestamp()
    });

    match vault.verify_and_store(&tenant_id, "google_calendar", &secret_json.to_string()).await {
        Ok(_) => {
            if let crate::db::DbStore::Postgres = &state.db.store {
                let mut tx = match state.db.pool.begin().await {
                    Ok(t) => t,
                    Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
                };
                // Insert metadata only into calendar_integrations. access_token and refresh_token are left NULL
                let _ = sqlx::query(
                    "INSERT INTO calendar_integrations (id, tenant_id, provider, expires_at)
                     VALUES ($1, $2, 'google_calendar', $3)
                     ON CONFLICT (id) DO UPDATE SET expires_at = $3"
                )
                .bind(format!("{}_google_calendar", tenant_id)).bind(&tenant_id).bind(expires_at).execute(&mut *tx).await;
                let _ = sqlx::query(
                    "INSERT INTO tool_integrations (id, tenant_id, name, status, integration_code)
                     VALUES ($1, $2, 'google_calendar', 'connected', '{}')
                     ON CONFLICT (id) DO UPDATE SET status = 'connected'"
                )
                .bind(format!("{}_google_calendar", tenant_id)).bind(&tenant_id).execute(&mut *tx).await;
                let _ = tx.commit().await;
            }
        },
        Err(_) => return (StatusCode::BAD_GATEWAY, "Failed to store credentials").into_response(),
    };

    let base = std::env::var("OMNISOLO_TUNNEL_BASE_URL").unwrap_or_else(|_| "https://cloud.omnisolo.co".to_string());

    axum::response::Html(format!("<script>if(window.opener) {{ window.opener.postMessage('oauth_success', '{}'); window.close(); }}</script>OAuth callback received. You can close this window.", base)).into_response()
}

pub fn router<S: Clone + Send + Sync + 'static>(db: Arc<DB>) -> Router<S> {
    let state = ToolIntegrationsApiState { db };
    Router::new()
        .route("/", get(get_integrations_handler))
        .route("/google_calendar/connect", post(connect_google_calendar_handler))
        .route("/{id}/connect", post(connect_integration_handler))
        .route("/{id}/verify", post(refresh_integration_handler))
        .route("/{id}", axum::routing::delete(revoke_integration_handler))
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

    #[tokio::test]
    async fn test_google_calendar_auth_generates_tenant_bound_state() {
        let claims = ::server_common::Claims {
            sub: "user-1".to_string(),
            exp: 0,
            iat: 0,
            organization_id: Some("tenant_123".to_string()),
            username: "tester".to_string(),
            email: "tester@example.com".to_string(),
            roles: vec!["owner".to_string()],
            session_id: None,
            jti: "jti-1".to_string(),
        };
        let db = Arc::new(DB {
            pool: sqlx::PgPool::connect_lazy("postgres://unused:unused@127.0.0.1:1/unused").unwrap(),
            store: crate::db::DbStore::Sqlite(sqlx::SqlitePool::connect_lazy("sqlite::memory:").unwrap()),
        });
        let state = ToolIntegrationsApiState { db };
        unsafe { std::env::set_var("GOOGLE_CALENDAR_CLIENT_ID", "test_client_id"); }
        let response = connect_google_calendar_handler(axum::extract::State(state), axum::extract::Extension(claims)).await.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        let redirect_url = json["redirect_url"].as_str().unwrap();
        assert!(redirect_url.contains("state="));
        assert!(!redirect_url.contains("state=google_calendar_tenant_123")); // Signed, shouldn't be plain text
        unsafe { std::env::remove_var("GOOGLE_CALENDAR_CLIENT_ID"); }
    }

    #[tokio::test]
    async fn test_google_calendar_auth_rejects_missing_state() {
        let db = Arc::new(DB {
            pool: sqlx::PgPool::connect_lazy("postgres://unused:unused@127.0.0.1:1/unused").unwrap(),
            store: crate::db::DbStore::Sqlite(sqlx::SqlitePool::connect_lazy("sqlite::memory:").unwrap()),
        });
        let state = ToolIntegrationsApiState { db };
        let query = OAuthCallbackQuery { code: "test_code".to_string(), state: "invalid_state".to_string() };
        let response = google_calendar_oauth_callback_handler(axum::extract::State(state), axum::extract::Query(query)).await.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
    }
}
