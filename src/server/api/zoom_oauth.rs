use axum::{
    extract::{Query, State, Request},
    response::{IntoResponse, Redirect},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};

#[derive(Clone)]
pub struct ZoomOAuthState {
    pub db: Arc<crate::db::DB>,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OAuthStateClaims {
    tenant_id: String,
    exp: usize,
}

fn create_state_token(tenant_id: &str, secret: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(15))
        .expect("valid timestamp")
        .timestamp();

    let claims = OAuthStateClaims {
        tenant_id: tenant_id.to_string(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap_or_default()
}

fn verify_state_token(token: &str, secret: &str) -> Result<String, ()> {
    let validation = Validation::default();
    let token_data = decode::<OAuthStateClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map_err(|_| ())?;

    Ok(token_data.claims.tenant_id)
}

pub async fn connect_handler(
    State(state): State<ZoomOAuthState>,
    request: Request,
) -> impl IntoResponse {
    let tenant_id = match request.extensions().get::<crate::auth::AuthInfo>() {
        Some(auth) => auth.tenant_id.clone(),
        None => return Redirect::temporary("/login"),
    };

    let state_token = create_state_token(&tenant_id, &state.client_secret);

    let auth_url = format!(
        "https://zoom.us/oauth/authorize?response_type=code&client_id={}&redirect_uri={}&state={}",
        state.client_id, state.redirect_uri, state_token
    );
    Redirect::temporary(&auth_url)
}

#[derive(Deserialize)]
pub struct CallbackQuery {
    pub code: Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
}

pub async fn callback_handler(
    State(state): State<ZoomOAuthState>,
    Query(query): Query<CallbackQuery>,
) -> impl IntoResponse {
    if let Some(_err) = query.error {
        return Redirect::temporary("/services?error=oauth_failed");
    }

    let code = match query.code {
        Some(c) => c,
        None => return Redirect::temporary("/services?error=missing_code"),
    };

    let state_token = match query.state {
        Some(s) => s,
        None => return Redirect::temporary("/services?error=missing_state"),
    };

    let tenant_id = match verify_state_token(&state_token, &state.client_secret) {
        Ok(id) => id,
        Err(_) => return Redirect::temporary("/services?error=invalid_state"),
    };

    // Exchange code for access token
    let client = reqwest::Client::new();
    let res = match client.post("https://zoom.us/oauth/token")
        .basic_auth(&state.client_id, Some(&state.client_secret))
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", code.as_str()),
            ("redirect_uri", state.redirect_uri.as_str()),
        ])
        .send().await {
        Ok(res) => res,
        Err(_) => return Redirect::temporary("/services?error=token_exchange_failed"),
    };

    if !res.status().is_success() {
        return Redirect::temporary("/services?error=token_exchange_failed");
    }

    let body = res.text().await.unwrap_or_default();
    let v: serde_json::Value = serde_json::from_str(&body).unwrap_or(serde_json::Value::Null);
    let access_token = match v.get("access_token").and_then(|t| t.as_str()) {
        Some(t) => t.to_string(),
        None => return Redirect::temporary("/services?error=no_access_token"),
    };

    // Store tokens in DB
    let res = match &state.db.store {
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query("UPDATE tenants SET zoom_token = ? WHERE tenant_id = ?")
                .bind(access_token)
                .bind(&tenant_id)
                .execute(pool)
                .await
        }
        crate::db::DbStore::Postgres => {
            sqlx::query("UPDATE tenants SET zoom_token = $1 WHERE tenant_id = $2")
                .bind(access_token)
                .bind(&tenant_id)
                .execute(&state.db.pool)
                .await
        }
    };

    match res {
        Ok(_) => Redirect::temporary("/services?success=zoom_connected"),
        Err(_) => Redirect::temporary("/services?error=db_update_failed"),
    }
}
