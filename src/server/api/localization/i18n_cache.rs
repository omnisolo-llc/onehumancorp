use axum::{
    extract::{State, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<crate::db::DB>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct I18nTranslation {
    pub id: String,
    pub tenant_id: String,
    pub language_code: String,
    pub translation_key: String,
    pub translation_value: String,
}

#[derive(Deserialize)]
pub struct UpsertTranslationRequest {
    pub tenant_id: String,
    pub language_code: String,
    pub translations: std::collections::HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct GetTranslationsQuery {
    pub tenant_id: String,
    pub language_code: String,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .with_state(state)
        .route("/api/localization/i18n", get(get_translations))
        .route("/api/localization/i18n", post(upsert_translations))
}

async fn get_translations(
    State(state): State<AppState>,
    Query(query): Query<GetTranslationsQuery>,
) -> impl IntoResponse {
    let mut tx = match state.db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };
    if let Err(e) = crate::common::auth_utils::set_org_context(&mut *tx, &query.tenant_id).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    let rows = match sqlx::query!(
        "SELECT id, tenant_id, language_code, translation_key, translation_value
         FROM local_i18n_cache
         WHERE tenant_id = $1 AND language_code = $2",
        query.tenant_id, query.language_code
    )
    .fetch_all(&mut *tx)
    .await {
        Ok(rows) => rows,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let translations: Vec<I18nTranslation> = rows.into_iter().map(|row| I18nTranslation {
        id: row.id,
        tenant_id: row.tenant_id,
        language_code: row.language_code,
        translation_key: row.translation_key,
        translation_value: row.translation_value,
    }).collect();

    (StatusCode::OK, Json(translations)).into_response()
}

async fn upsert_translations(
    State(state): State<AppState>,
    Json(payload): Json<UpsertTranslationRequest>,
) -> impl IntoResponse {
    let mut tx = match state.db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };
    if let Err(e) = crate::common::auth_utils::set_org_context(&mut *tx, &payload.tenant_id).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    for (key, value) in payload.translations {
        let id = Uuid::new_v4().to_string();
        if let Err(e) = sqlx::query!(
            "INSERT INTO local_i18n_cache (id, tenant_id, language_code, translation_key, translation_value)
             VALUES ($1, $2, $3, $4, $5)
             ON CONFLICT (tenant_id, language_code, translation_key)
             DO UPDATE SET translation_value = EXCLUDED.translation_value, updated_at = CURRENT_TIMESTAMP",
            id, payload.tenant_id, payload.language_code, key, value
        )
        .execute(&mut *tx)
        .await {
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
    }

    if let Err(e) = tx.commit().await {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    StatusCode::OK.into_response()
}
