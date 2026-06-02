use axum::{Json, response::IntoResponse, http::StatusCode, extract::{State, Path}};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::services::localization::localization_engine::LocalizationEngine;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct TranslationResponse {
    pub value: String,
}

#[derive(Serialize)]
pub struct FxRateResponse {
    pub rate: f64,
}

#[derive(Deserialize)]
pub struct BulkTranslationsRequest {
    pub keys: Vec<String>,
}

#[derive(Serialize)]
pub struct BulkTranslationsResponse {
    pub translations: HashMap<String, String>,
}

pub async fn get_translation_handler(
    State(engine): State<Arc<LocalizationEngine>>,
    Path((tenant_id, locale, key)): Path<(String, String, String)>,
) -> impl IntoResponse {
    let cache = engine.get_cache(&tenant_id);
    let cache_lock = cache.lock().unwrap();
    if let Some(value) = cache_lock.get_translation(&locale, &key) {
        (StatusCode::OK, Json(TranslationResponse { value })).into_response()
    } else {
        (StatusCode::NOT_FOUND, "Translation not found").into_response()
    }
}

pub async fn get_fx_rate_handler(
    State(engine): State<Arc<LocalizationEngine>>,
    Path((tenant_id, currency)): Path<(String, String)>,
) -> impl IntoResponse {
    let cache = engine.get_cache(&tenant_id);
    let cache_lock = cache.lock().unwrap();
    if let Some(rate) = cache_lock.get_fx_rate(&currency) {
        (StatusCode::OK, Json(FxRateResponse { rate })).into_response()
    } else {
        (StatusCode::NOT_FOUND, "Rate not found").into_response()
    }
}

pub async fn get_bulk_translations_handler(
    State(engine): State<Arc<LocalizationEngine>>,
    Path((tenant_id, locale)): Path<(String, String)>,
    Json(payload): Json<BulkTranslationsRequest>,
) -> impl IntoResponse {
    let cache = engine.get_cache(&tenant_id);
    let cache_lock = cache.lock().unwrap();
    let mut translations = HashMap::new();
    for key in payload.keys {
        if let Some(value) = cache_lock.get_translation(&locale, &key) {
            translations.insert(key, value);
        }
    }
    (StatusCode::OK, Json(BulkTranslationsResponse { translations })).into_response()
}

pub fn router(engine: Arc<LocalizationEngine>) -> axum::Router {
    axum::Router::new()
        .route("/api/v1/localization/:tenant_id/translations/:locale/:key", axum::routing::get(get_translation_handler))
        .route("/api/v1/localization/:tenant_id/translations/:locale/bulk", axum::routing::post(get_bulk_translations_handler))
        .route("/api/v1/localization/:tenant_id/fx/:currency", axum::routing::get(get_fx_rate_handler))
        .with_state(engine)
}
