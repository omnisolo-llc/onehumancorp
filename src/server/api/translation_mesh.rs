use axum::{
    extract::{State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::common::auth::Claims;
use crate::db::DB;
use crate::queue::{QueueManager, SubAgentJob};
use sha2::{Sha256, Digest};
use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslationRequest {
    pub source_text: String,
    pub target_locale: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslationResponse {
    pub translated_text: Option<String>,
    pub source_hash: String,
    pub status: String, // "cached", "queued"
}

pub async fn get_translation(
    claims: Claims,
    State((db, queue)): State<(Arc<DB>, Arc<QueueManager>)>,
    Json(payload): Json<TranslationRequest>,
) -> Result<Json<TranslationResponse>, String> {
    let mut hasher = Sha256::new();
    hasher.update(payload.source_text.as_bytes());
    let source_hash = hex::encode(hasher.finalize());

    // Check if we have it in cache
    let cached = db.get_translation_from_cache(&claims.organization_id, &source_hash, &payload.target_locale).await.map_err(|e| e.to_string())?;

    if let Some(text) = cached {
        return Ok(Json(TranslationResponse {
            translated_text: Some(text),
            source_hash,
            status: "cached".to_string(),
        }));
    }

    // Queue job to sub-agent queue
    let job_id = Uuid::new_v4().to_string();

    let job_payload = serde_json::json!({
        "type": "translate",
        "source_text": payload.source_text,
        "target_locale": payload.target_locale,
        "source_hash": source_hash,
    });

    let job = SubAgentJob {
        id: job_id.clone(),
        tenant_id: claims.organization_id.clone(),
        parent_task_id: "".to_string(),
        payload: job_payload,
        status: "QUEUED".to_string(),
        worker_id: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    queue.enqueue(job).await.map_err(|e| e.to_string())?;

    Ok(Json(TranslationResponse {
        translated_text: None,
        source_hash,
        status: "queued".to_string(),
    }))
}
