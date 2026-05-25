use axum::{
    extract::{Query, Extension},
    response::{IntoResponse, Redirect},
    http::StatusCode,
    routing::get,
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::integrations::manychat::provider::ManychatProvider;
use crate::db::get_pool;
use crate::auth::TenantKey;

#[derive(Deserialize)]
pub struct OauthCallbackQuery {
    pub code: String,
    pub state: Option<String>,
}

#[derive(Serialize)]
pub struct OauthResponse {
    pub success: bool,
    pub error: Option<String>,
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/authorize", get(authorize_manychat))
        .route("/callback", get(manychat_callback))
}

async fn authorize_manychat(
    Extension(tenant): Extension<TenantKey>,
) -> impl IntoResponse {
    let tenant_id = tenant.org_id.to_string();

    let redirect_uri = std::env::var("MANYCHAT_REDIRECT_URI").unwrap_or_else(|_| "http://localhost:3000/api/agents/manychat/callback".to_string());

    let state = uuid::Uuid::new_v4().to_string();
    let pool = get_pool();
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
    };

    if let Err(_) = sqlx::query("INSERT INTO oauth_states (state, tenant_id, provider) VALUES ($1, $2, 'manychat')")
        .bind(&state)
        .bind(&tenant_id)
        .execute(&mut *tx)
        .await
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Database error saving state").into_response();
    }

    if let Err(_) = tx.commit().await {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Database error committing state").into_response();
    }

    let client_id = std::env::var("MANYCHAT_CLIENT_ID").unwrap_or_else(|_| "".to_string());
    let oauth_url = format!("https://manychat.com/oauth?client_id={}&redirect_uri={}&response_type=code&state={}", client_id, redirect_uri, state);

    Redirect::temporary(&oauth_url).into_response()
}

async fn manychat_callback(
    Query(query): Query<OauthCallbackQuery>,
) -> impl IntoResponse {
    let redirect_uri = std::env::var("MANYCHAT_REDIRECT_URI").unwrap_or_else(|_| "http://localhost:3000/api/agents/manychat/callback".to_string());

    let state = match query.state {
        Some(ref s) => s,
        None => return (StatusCode::BAD_REQUEST, Json(OauthResponse {
            success: false,
            error: Some("Missing state parameter".to_string()),
        })).into_response(),
    };

    let pool = get_pool();
    let tenant_id: String = match sqlx::query_scalar("SELECT tenant_id FROM oauth_states WHERE state = $1 AND provider = 'manychat'")
        .bind(&state)
        .fetch_optional(&pool).await
    {
        Ok(Some(id)) => id,
        _ => return (StatusCode::BAD_REQUEST, Json(OauthResponse {
            success: false,
            error: Some("Invalid state parameter (CSRF attempt or expired session)".to_string()),
        })).into_response(),
    };

    let provider = ManychatProvider::new("".to_string(), tenant_id.clone());

    match provider.exchange_token(&query.code, &redirect_uri).await {
        Ok(token) => {
            // Persist the token to the database associated with the tenant
            let mut tx = pool.begin().await.unwrap();
            let _ = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await;
            let _ = sqlx::query("INSERT INTO integration_credentials (tenant_id, provider, access_token) VALUES ($1, 'manychat', $2) ON CONFLICT (tenant_id, provider) DO UPDATE SET access_token = EXCLUDED.access_token")
                .bind(&tenant_id)
                .bind(&token)
                .execute(&mut *tx)
                .await;
            let _ = tx.commit().await;

            // Delete the consumed state
            let _ = sqlx::query("DELETE FROM oauth_states WHERE state = $1")
                .bind(&state)
                .execute(&pool)
                .await;

            (StatusCode::OK, Json(OauthResponse {
                success: true,
                error: None,
            })).into_response()
        },
        Err(e) => (StatusCode::BAD_REQUEST, Json(OauthResponse {
            success: false,
            error: Some(e),
        })).into_response(),
    }
}
