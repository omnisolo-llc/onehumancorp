use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use crate::common::auth::Claims;

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslationPreferences {
    pub primary_language: String,
    pub enabled_languages: Vec<String>,
    pub auto_translate: bool,
}

pub async fn get_translation_preferences(
    axum::extract::Extension(claims): axum::extract::Extension<::server_common::auth::Claims>,
    State(pool): State<Arc<PgPool>>,
) -> Result<Json<TranslationPreferences>, String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    ::server_common::auth_utils::set_org_context(&mut *tx, &claims.organization_id).await.map_err(|e| e.to_string())?;

    let row = sqlx::query!(
        "SELECT primary_language, enabled_languages, auto_translate FROM merchant_translation_preferences WHERE tenant_id = $1",
        claims.organization_id
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;

    if let Some(r) = row {
        Ok(Json(TranslationPreferences {
            primary_language: r.primary_language,
            enabled_languages: r.enabled_languages.into_iter().map(|s| s).collect(),
            auto_translate: r.auto_translate,
        }))
    } else {
        Ok(Json(TranslationPreferences {
            primary_language: "en".to_string(),
            enabled_languages: vec![],
            auto_translate: true,
        }))
    }
}

pub async fn update_translation_preferences(
    axum::extract::Extension(claims): axum::extract::Extension<::server_common::auth::Claims>,
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<TranslationPreferences>,
) -> Result<Json<TranslationPreferences>, String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    ::server_common::auth_utils::set_org_context(&mut *tx, &claims.organization_id).await.map_err(|e| e.to_string())?;

    let row = sqlx::query!(
        r#"
        INSERT INTO merchant_translation_preferences (tenant_id, primary_language, enabled_languages, auto_translate)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (tenant_id) DO UPDATE SET
            primary_language = EXCLUDED.primary_language,
            enabled_languages = EXCLUDED.enabled_languages,
            auto_translate = EXCLUDED.auto_translate,
            updated_at = CURRENT_TIMESTAMP
        RETURNING primary_language, enabled_languages, auto_translate
        "#,
        claims.organization_id,
        payload.primary_language,
        &payload.enabled_languages,
        payload.auto_translate
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;

    Ok(Json(TranslationPreferences {
        primary_language: row.primary_language,
        enabled_languages: row.enabled_languages.into_iter().map(|s| s).collect(),
        auto_translate: row.auto_translate,
    }))
}

pub fn router(pool: Arc<PgPool>) -> axum::Router {
    axum::Router::new()
        .route("/", axum::routing::get(get_translation_preferences).put(update_translation_preferences))
        .with_state(pool)
}
