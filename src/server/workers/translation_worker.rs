use sqlx::{PgPool, Executor};
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use ohc_builtin_agent_core::types::{ChatRequest, Message, Role};
use ohc_builtin_agent::llm::LlmClient;

pub async fn run_translation_worker(pool: Arc<PgPool>, llm: Arc<dyn LlmClient>) {
    loop {
        if let Err(e) = process_translation_jobs(&pool, &llm).await {
            tracing::error!("Translation worker error: {}", e);
        }
        sleep(Duration::from_secs(5)).await;
    }
}

pub async fn process_translation_jobs(pool: &PgPool, llm: &Arc<dyn LlmClient>) -> Result<(), String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    let job_opt = sqlx::query(
        r#"
        UPDATE ohc_job_queue
        SET status = 'PROCESSING',
            locked_until = CURRENT_TIMESTAMP + INTERVAL '5 minutes',
            updated_at = CURRENT_TIMESTAMP
        WHERE id = (
            SELECT id FROM ohc_job_queue
            WHERE job_type = 'translation_batch'
              AND status = 'PENDING'
              AND next_retry_at <= CURRENT_TIMESTAMP
            ORDER BY next_retry_at ASC
            FOR UPDATE SKIP LOCKED
            LIMIT 1
        )
        RETURNING id, tenant_id, payload, retry_count
        "#,
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    let job_row = match job_opt {
        Some(row) => row,
        None => {
            let _ = tx.rollback().await;
            return Ok(());
        }
    };

    use sqlx::Row;
    let job_id: String = job_row.get("id");
    let tenant_id: String = job_row.get("tenant_id");
    let payload: serde_json::Value = job_row.get("payload");
    let retry_count: i32 = job_row.get("retry_count");

    let text_hash: String = payload.get("text_hash").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let text: String = payload.get("text").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let locale: String = payload.get("locale").and_then(|v| v.as_str()).unwrap_or("").to_string();

    if text.is_empty() || locale.is_empty() {
        sqlx::query("UPDATE ohc_job_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(&job_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        tx.commit().await.map_err(|e| e.to_string())?;
        return Ok(());
    }

    // Call LLM
    let prompt = format!("Translate the following text to locale '{}':\n\n{}", locale, text);
    let req = ChatRequest {
        model: "gpt-4o".to_string(),
        system: "You are a professional translator. Only output the translated text, nothing else.".to_string(),
        messages: vec![Message {
            role: Role::User,
            content: prompt,
            tool_calls: vec![],
            tool_results: vec![],
            response_id: None,
            previous_response_id: None,
        }],
        tools: vec![],
        max_tokens: 2048,
        temperature: 0.0,
    };

    let result = llm.chat(req).await;

    match result {
        Ok(res) => {
            let translated_text = res.message.content.trim().to_string();
            ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| e.to_string())?;
            let id = uuid::Uuid::new_v4().to_string();
            let _ = sqlx::query(
                "INSERT INTO translation_cache (id, tenant_id, text_hash, locale, translated_text)
                 VALUES ($1, $2, $3, $4, $5)
                 ON CONFLICT (tenant_id, text_hash, locale) DO UPDATE
                 SET translated_text = EXCLUDED.translated_text, updated_at = CURRENT_TIMESTAMP"
            )
            .bind(&id)
            .bind(&tenant_id)
            .bind(&text_hash)
            .bind(&locale)
            .bind(&translated_text)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

            sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                .bind(&job_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
        }
        Err(_) => {
            let next_retry = chrono::Utc::now() + chrono::Duration::seconds(2_i64.pow(retry_count as u32) * 10);
            sqlx::query(
                "UPDATE ohc_job_queue
                 SET status = 'PENDING',
                     retry_count = retry_count + 1,
                     next_retry_at = $1,
                     updated_at = CURRENT_TIMESTAMP
                 WHERE id = $2"
            )
            .bind(next_retry)
            .bind(&job_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }
    }

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}
