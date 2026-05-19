use axum::{
    extract::{Query, State, Request},
    response::{IntoResponse, Redirect},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};

#[derive(Clone)]
pub struct GoogleCalendarOAuthState {
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
    State(state): State<GoogleCalendarOAuthState>,
    request: Request,
) -> impl IntoResponse {
    let tenant_id = match request.extensions().get::<crate::auth::AuthInfo>() {
        Some(auth) => auth.org_id.clone(),
        None => return Redirect::temporary("/login"),
    };

    let state_token = create_state_token(&tenant_id, &state.client_secret);

    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope=https://www.googleapis.com/auth/calendar&access_type=offline&prompt=consent&state={}",
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
    State(state): State<GoogleCalendarOAuthState>,
    Query(query): Query<CallbackQuery>,
) -> impl IntoResponse {
    if let Some(_err) = query.error {
        return Redirect::temporary("/sales?error=oauth_failed");
    }

    let code = match query.code {
        Some(c) => c,
        None => return Redirect::temporary("/sales?error=missing_code"),
    };

    let state_token = match query.state {
        Some(s) => s,
        None => return Redirect::temporary("/sales?error=missing_state"),
    };

    let tenant_id = match verify_state_token(&state_token, &state.client_secret) {
        Ok(id) => id,
        Err(_) => return Redirect::temporary("/sales?error=invalid_state"),
    };

    // Exchange code for access token
    let client = reqwest::Client::new();
    let res = match client.post("https://oauth2.googleapis.com/token")
        .form(&[
            ("client_id", state.client_id.as_str()),
            ("client_secret", state.client_secret.as_str()),
            ("code", code.as_str()),
            ("grant_type", "authorization_code"),
            ("redirect_uri", state.redirect_uri.as_str()),
        ])
        .send().await {
        Ok(res) => res,
        Err(_) => return Redirect::temporary("/sales?error=token_exchange_failed"),
    };

    if !res.status().is_success() {
        return Redirect::temporary("/sales?error=token_exchange_failed");
    }

    let body = res.text().await.unwrap_or_default();
    let v: serde_json::Value = serde_json::from_str(&body).unwrap_or(serde_json::Value::Null);
    let access_token = match v.get("access_token").and_then(|t| t.as_str()) {
        Some(t) => t.to_string(),
        None => return Redirect::temporary("/sales?error=no_access_token"),
    };
    let refresh_token = v.get("refresh_token").and_then(|t| t.as_str()).unwrap_or("").to_string();

    // Store tokens in DB
    let res = match &state.db.store {
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query("UPDATE tenants SET google_calendar_token = ?, google_calendar_refresh = ? WHERE tenant_id = ?")
                .bind(access_token)
                .bind(refresh_token)
                .bind(&tenant_id)
                .execute(pool)
                .await.map(|_| ())
        }
        crate::db::DbStore::Postgres => {
            sqlx::query("UPDATE tenants SET google_calendar_token = $1, google_calendar_refresh = $2 WHERE tenant_id = $3")
                .bind(access_token)
                .bind(refresh_token)
                .bind(&tenant_id)
                .execute(&state.db.pool)
                .await.map(|_| ())
        }
    };

    match res {
        Ok(_) => Redirect::temporary("/sales?success=google_calendar_connected"),
        Err(_) => Redirect::temporary("/sales?error=db_update_failed"),
    }
}
