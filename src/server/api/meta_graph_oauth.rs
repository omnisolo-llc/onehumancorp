use axum::{
    extract::{Query, State, Request},
    response::{IntoResponse, Redirect},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};

#[derive(Clone)]
pub struct MetaGraphOAuthState {
    pub db: Arc<crate::db::DB>,
    pub app_id: String,
    pub app_secret: String,
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
    State(state): State<MetaGraphOAuthState>,
    request: Request,
) -> impl IntoResponse {
    let tenant_id = match request.extensions().get::<crate::auth::AuthInfo>() {
        Some(auth) => auth.tenant_id.clone(),
        None => return Redirect::temporary("/login"),
    };

    let state_token = create_state_token(&tenant_id, &state.app_secret);

    let auth_url = format!(
        "https://www.facebook.com/v19.0/dialog/oauth?client_id={}&redirect_uri={}&state={}&scope=pages_messaging,pages_show_list,instagram_basic,instagram_manage_messages",
        state.app_id, state.redirect_uri, state_token
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
    State(state): State<MetaGraphOAuthState>,
    Query(query): Query<CallbackQuery>,
) -> impl IntoResponse {
    if let Some(_err) = query.error {
        return Redirect::temporary("/operations?error=oauth_failed");
    }

    let code = match query.code {
        Some(c) => c,
        None => return Redirect::temporary("/operations?error=missing_code"),
    };

    let state_token = match query.state {
        Some(s) => s,
        None => return Redirect::temporary("/operations?error=missing_state"),
    };

    let tenant_id = match verify_state_token(&state_token, &state.app_secret) {
        Ok(id) => id,
        Err(_) => return Redirect::temporary("/operations?error=invalid_state"),
    };

    // Exchange code for access token
    let token_url = format!(
        "https://graph.facebook.com/v19.0/oauth/access_token?client_id={}&redirect_uri={}&client_secret={}&code={}",
        state.app_id, state.redirect_uri, state.app_secret, code
    );

    let client = reqwest::Client::new();
    let res = match client.get(&token_url).send().await {
        Ok(res) => res,
        Err(_) => return Redirect::temporary("/operations?error=token_exchange_failed"),
    };

    if !res.status().is_success() {
        return Redirect::temporary("/operations?error=token_exchange_failed");
    }

    let body = res.text().await.unwrap_or_default();
    let v: serde_json::Value = serde_json::from_str(&body).unwrap_or(serde_json::Value::Null);
    let access_token = match v.get("access_token").and_then(|t| t.as_str()) {
        Some(t) => t.to_string(),
        None => return Redirect::temporary("/operations?error=no_access_token"),
    };

    // Store access token in DB
    let res = match &state.db.store {
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query("UPDATE tenants SET meta_graph_token = ? WHERE tenant_id = ?")
                .bind(access_token)
                .bind(&tenant_id)
                .execute(pool)
                .await
        }
        crate::db::DbStore::Postgres => {
            sqlx::query("UPDATE tenants SET meta_graph_token = $1 WHERE tenant_id = $2")
                .bind(access_token)
                .bind(&tenant_id)
                .execute(&state.db.pool)
                .await
        }
    };

    match res {
        Ok(_) => Redirect::temporary("/operations?success=meta_connected"),
        Err(_) => Redirect::temporary("/operations?error=db_update_failed"),
    }
}
