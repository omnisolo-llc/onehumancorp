use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MetaOAuthCallbackQuery {
    pub code: String,
    pub state: String, // We use state to pass the tenant_id
}

pub async fn handle_meta_oauth_callback(
    State(state): State<crate::api::meta_webhook::MetaWebhookState>,
    Query(query): Query<MetaOAuthCallbackQuery>,
) -> impl IntoResponse {
    let tenant_id = query.state;
    // In a real implementation we would exchange query.code for an access token
    // via https://graph.facebook.com/v19.0/oauth/access_token
    // and fetch the phone_number_id.
    let access_token = "mock_access_token".to_string();
    let phone_number_id = format!("mock_phone_{}", tenant_id); // using a mock phone number for test

    let pool = &state.db.pool;

    // Store in meta_integrations
    match &state.db.store {
        crate::db::DbStore::Postgres => {
            let _ = sqlx::query(
                "INSERT INTO meta_integrations (phone_number_id, tenant_id, access_token) VALUES ($1, $2, $3) ON CONFLICT (phone_number_id) DO UPDATE SET access_token = EXCLUDED.access_token, tenant_id = EXCLUDED.tenant_id"
            )
            .bind(&phone_number_id)
            .bind(&tenant_id)
            .bind(&access_token)
            .execute(pool)
            .await;
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            let _ = sqlx::query(
                "INSERT INTO meta_integrations (phone_number_id, tenant_id, access_token) VALUES (?, ?, ?) ON CONFLICT (phone_number_id) DO UPDATE SET access_token = excluded.access_token, tenant_id = excluded.tenant_id"
            )
            .bind(&phone_number_id)
            .bind(&tenant_id)
            .bind(&access_token)
            .execute(sqlite_pool)
            .await;
        }
    }

    Redirect::temporary("/integrations").into_response()
}
