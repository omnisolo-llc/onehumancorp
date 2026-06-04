use axum::{
    extract::{State, Path},
    routing::{get, post},
    Json, Router,
};
use axum::extract::Extension;
use server_common::Claims;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::{Pool, Postgres};
use crate::domain::repository::brand_voice_repo::BrandVoiceRepo;
use crate::domain::repository::models::BrandVoiceProfile;

pub struct BrandVoiceState {
    pub repo: BrandVoiceRepo,
}

pub fn router(pool: Pool<Postgres>) -> Router {
    let repo = BrandVoiceRepo::new(pool);
    let state = Arc::new(BrandVoiceState { repo });

    Router::new()
        .route("/api/brand-voice", get(get_profile).post(upsert_profile))
        .route("/api/brand-voice/ab-test", post(ab_test_selection))
        .with_state(state)
}

#[derive(Serialize)]
pub struct ProfileResponse {
    pub profile: Option<BrandVoiceProfile>,
}

pub async fn get_profile(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<BrandVoiceState>>,
) -> Json<ProfileResponse> {
    let tenant_id = claims.tenant_id.unwrap_or_else(|| "system".to_string());
    let profile = state.repo.get_by_tenant_id(&tenant_id).await.unwrap_or(None);
    Json(ProfileResponse { profile })
}

#[derive(Deserialize)]
pub struct UpsertProfileRequest {
    pub tone_descriptors: serde_json::Value,
    pub vocabulary_preferences: serde_json::Value,
    pub specific_knowledge_facts: serde_json::Value,
    pub emoji_usage_level: String,
}

pub async fn upsert_profile(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<BrandVoiceState>>,
    Json(payload): Json<UpsertProfileRequest>,
) -> Json<BrandVoiceProfile> {
    let tenant_id = claims.tenant_id.unwrap_or_else(|| "system".to_string());
    let profile = BrandVoiceProfile {
        id: uuid::Uuid::new_v4(), // Will be ignored on insert/update in Postgres
        tenant_id,
        tone_descriptors: payload.tone_descriptors,
        vocabulary_preferences: payload.vocabulary_preferences,
        specific_knowledge_facts: payload.specific_knowledge_facts,
        emoji_usage_level: payload.emoji_usage_level,
        created_at: None,
        updated_at: None,
    };

    let result = state.repo.upsert(&profile).await.expect("Failed to upsert profile");
    Json(result)
}

#[derive(Deserialize)]
pub struct AbTestSelectionRequest {
    pub scenario: String,
    pub selected_option: String, // 'A' or 'B'
    pub selected_text: String,
}

pub async fn ab_test_selection(
    Extension(claims): Extension<Claims>,
    State(state): State<Arc<BrandVoiceState>>,
    Json(payload): Json<AbTestSelectionRequest>,
) -> Json<BrandVoiceProfile> {
    let tenant_id = claims.tenant_id.unwrap_or_else(|| "system".to_string());

    // In a real implementation, this would use an LLM or heuristic to
    // deduce new tone descriptors based on the selected text.
    // For now, we mock the deduction.
    let mut current_profile = state.repo.get_by_tenant_id(&tenant_id).await.unwrap_or(None).unwrap_or_else(|| BrandVoiceProfile {
        id: uuid::Uuid::new_v4(),
        tenant_id: tenant_id.clone(),
        tone_descriptors: serde_json::json!([]),
        vocabulary_preferences: serde_json::json!({}),
        specific_knowledge_facts: serde_json::json!([]),
        emoji_usage_level: "moderate".to_string(),
        created_at: None,
        updated_at: None,
    });

    // Mock logic: if they pick option A which has emojis, increase emoji usage
    if payload.selected_text.contains('✨') || payload.selected_text.contains('🍰') {
        current_profile.emoji_usage_level = "high".to_string();
        let mut descriptors = current_profile.tone_descriptors.as_array().unwrap().clone();
        if !descriptors.contains(&serde_json::json!("bubbly")) {
            descriptors.push(serde_json::json!("bubbly"));
        }
        current_profile.tone_descriptors = serde_json::Value::Array(descriptors);
    } else {
        current_profile.emoji_usage_level = "low".to_string();
        let mut descriptors = current_profile.tone_descriptors.as_array().unwrap().clone();
        if !descriptors.contains(&serde_json::json!("professional")) {
            descriptors.push(serde_json::json!("professional"));
        }
        current_profile.tone_descriptors = serde_json::Value::Array(descriptors);
    }

    let result = state.repo.upsert(&current_profile).await.expect("Failed to upsert profile after A/B test");
    Json(result)
}
