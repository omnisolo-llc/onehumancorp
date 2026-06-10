use std::sync::Arc;
use uuid::Uuid;

use crate::api::agents::translation::{translate_inbox_message_with_llm, generate_inbox_draft_reply, InboxTranslation};
use crate::orchestration::departments::types::DepartmentType;
use crate::orchestration::departments::types::ActionRisk;

pub async fn process_omnichannel_message(db: &Arc<crate::db::DB>, orchestrator: &Arc<crate::orchestration::departments::orchestrator::DepartmentOrchestrator>, tenant_id: String, source: String, sender_id: String, text: String) {
    let target_language = "English";

    let translation = match translate_inbox_message_with_llm(&tenant_id, &source, &text, target_language).await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Translation failed: {}", e);
            InboxTranslation {
                translated_content: text.clone(),
                source_language: Some("Unknown".to_string()),
                target_language: target_language.to_string(),
                original_content: text.clone(),
            }
        }
    };

    let draft_reply = match generate_inbox_draft_reply(&tenant_id, &source, &translation).await {
        Ok(d) => d,
        Err(e) => {
            tracing::error!("Failed to generate draft reply: {}", e);
            "Thanks for reaching out! We will review this and get back to you soon.".to_string()
        }
    };

    let inbox_id = Uuid::new_v4().to_string();
    let pool = &db.pool;

    let insert_result = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query(
                "INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, source_language, target_language, draft_reply, status, sender_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'unread', $9, NOW(), NOW())"
            )
            .bind(&inbox_id)
            .bind(&tenant_id)
            .bind(&source)
            .bind(&translation.original_content)
            .bind(&translation.translated_content)
            .bind(&translation.source_language)
            .bind(&translation.target_language)
            .bind(&draft_reply)
            .bind(&sender_id)
            .execute(pool)
            .await.map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query(
                "INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, source_language, target_language, draft_reply, status, sender_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'unread', ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
            )
            .bind(&inbox_id)
            .bind(&tenant_id)
            .bind(&source)
            .bind(&translation.original_content)
            .bind(&translation.translated_content)
            .bind(&translation.source_language)
            .bind(&translation.target_language)
            .bind(&draft_reply)
            .bind(&sender_id)
            .execute(sqlite_pool)
            .await.map(|_| ())
        }
    };

    if let Err(e) = insert_result {
        tracing::error!("Failed to insert into omni_inbox_messages: {}", e);
    }

    let _ = orchestrator.execute_action(
        DepartmentType::CustomerSuccess,
        format!("New {} message from {} (Language: {:?})", source, tenant_id, translation.source_language),
        tenant_id.clone(),
        ActionRisk::DraftForReview,
        serde_json::json!({
            "source": source.clone(),
            "message": translation.translated_content.clone(),
            "original_content": translation.original_content.clone(),
            "translated_from_language": translation.source_language.clone(),
            "draft_reply": draft_reply.clone(),
            "inbox_message_id": inbox_id.clone(),
            "sender_id": sender_id.clone(),
        }),
    ).await;

    let event = crate::orchestration::departments::types::DepartmentEvent {
        id: Uuid::new_v4().to_string(),
        tenant_id: tenant_id.clone(),
        event_type: "tenant.omnichannel.message.received".to_string(),
        payload: serde_json::json!({
            "source": source,
            "message": translation.translated_content,
            "original_message": translation.original_content,
            "translated_from_language": translation.source_language,
            "generated_response": draft_reply,
            "feature_type": "ambassador_reply",
            "sender_id": sender_id,
            "inbox_message_id": inbox_id,
        }),
    };

    let orchestrator_clone = orchestrator.clone();
    tokio::spawn(async move {
        let _ = orchestrator_clone.dispatch_event(event).await;
    });
}
