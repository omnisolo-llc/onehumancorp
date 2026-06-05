use sha2::Digest;
use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;

use ::server_common::Claims as ExtClaims;

#[derive(Debug, Serialize, Deserialize)]
pub struct I18nString {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FxRateResponse {
    pub from: String,
    pub to: String,
    pub rate: f64,
}

pub async fn get_translations(
    claims: ExtClaims,
    State(pool): State<Arc<PgPool>>,
    Path(locale): Path<String>,
) -> Result<Json<Vec<I18nString>>, String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    ::server_common::auth_utils::set_org_context(&mut *tx, &claims.organization_id.clone().unwrap_or("".to_string())).await.map_err(|e| e.to_string())?;

    let rows = sqlx::query(
        "SELECT key, value FROM ohc_i18n_strings
         WHERE (tenant_id = $1 OR tenant_id = 'SYSTEM') AND locale = $2"
    )
    .bind(&claims.organization_id.clone().unwrap_or("".to_string()))
    .bind(locale)
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    let translations = rows.into_iter().map(|r| {
        use sqlx::Row;
        I18nString {
            key: r.get("key"),
            value: r.get("value"),
        }
    }).collect();

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(Json(translations))
}

pub async fn get_fx_rates(
    State(pool): State<Arc<PgPool>>,
) -> Result<Json<Vec<FxRateResponse>>, String> {
    let rows = sqlx::query("SELECT from_currency, to_currency, rate FROM ohc_fx_rates")
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    let rates = rows.into_iter().map(|r| {
        use sqlx::Row;
        FxRateResponse {
            from: r.get("from_currency"),
            to: r.get("to_currency"),
            rate: r.get("rate"),
        }
    }).collect();

    Ok(Json(rates))
}


use axum::response::IntoResponse;

#[derive(Debug, Deserialize)]
pub struct TranslateRequest {
    pub source_text: String,
    pub source_lang: String,
    pub target_lang: String,
}

pub async fn translate(
    claims: ExtClaims,
    State(db): State<Arc<crate::db::DB>>,
    State(queue): State<Arc<dyn crate::queue::TaskQueue>>,
    Json(payload): Json<TranslateRequest>,
) -> axum::response::Response {
    let tenant_id = claims.organization_id.clone().unwrap_or("".to_string());
    let text_hash = format!("{:x}", sha2::Sha256::digest(payload.source_text.as_bytes()));

    let translated_text_opt: Option<String> = match &db.store {
        crate::db::DbStore::Postgres => {
            let row = sqlx::query("SELECT translated_text FROM translation_cache WHERE tenant_id = $1 AND text_hash = $2 AND target_lang = $3")
                .bind(&tenant_id)
                .bind(&text_hash)
                .bind(&payload.target_lang)
                .fetch_optional(&db.pool)
                .await
                .unwrap_or(None);
            row.map(|r| {
                use sqlx::Row;
                r.get("translated_text")
            })
        }
        crate::db::DbStore::Sqlite(pool) => {
            let row = sqlx::query("SELECT translated_text FROM translation_cache WHERE tenant_id = ? AND text_hash = ? AND target_lang = ?")
                .bind(&tenant_id)
                .bind(&text_hash)
                .bind(&payload.target_lang)
                .fetch_optional(pool)
                .await
                .unwrap_or(None);
            row.map(|r| {
                use sqlx::Row;
                r.get("translated_text")
            })
        }
    };

    if let Some(translated_text) = translated_text_opt {
        return (axum::http::StatusCode::OK, Json(serde_json::json!({
            "status": "COMPLETED",
            "translated_text": translated_text
        }))).into_response();
    }

    // Enqueue fallback job
    let job_id = uuid::Uuid::new_v4().to_string();
    let job = crate::queue::Job {
        id: job_id.clone(),
        tenant_id: tenant_id.clone(),
        parent_task_id: "".to_string(),
        job_type: "translation_task".to_string(),
        payload: serde_json::to_string(&serde_json::json!({
            "source_text": payload.source_text,
            "source_lang": payload.source_lang,
            "target_lang": payload.target_lang
        })).unwrap(),
        status: "PENDING".to_string(),
        retry_count: 0,
        max_retries: 3,
        next_retry_at: chrono::Utc::now(),
        locked_until: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    if let Err(e) = queue.enqueue(job).await {
        ::server_telemetry::record_error_signal("Failed to enqueue translation job");
        tracing::error!("Failed to enqueue translation job: {}", e);
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
            "error": "database write failed"
        }))).into_response();
    }

    (axum::http::StatusCode::ACCEPTED, Json(serde_json::json!({
        "status": "PENDING",
        "job_id": job_id
    }))).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{DB, DbStore};
    use sqlx::sqlite::SqlitePoolOptions;
    use std::sync::Arc;
    use crate::queue::{Job, TaskQueue};
    use async_trait::async_trait;

    struct MockQueue;

    #[async_trait]
    impl TaskQueue for MockQueue {
        async fn enqueue(&self, _job: Job) -> Result<(), String> {
            Ok(())
        }
        async fn enqueue_batch(&self, _jobs: Vec<Job>) -> Result<(), String> { Ok(()) }
        async fn dequeue(&self, _roles: Vec<String>) -> Result<Option<Job>, String> { Ok(None) }
        async fn complete(&self, _job_id: &str, _tenant_id: &str) -> Result<(), String> { Ok(()) }
        async fn fail(&self, _job_id: &str, _tenant_id: &str, _reason: &str) -> Result<(), String> { Ok(()) }
        async fn requeue(&self, _job: Job) -> Result<(), String> { Ok(()) }
    }

    #[tokio::test]
    async fn test_translate_endpoint() {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .expect("Failed to connect");

        sqlx::query("CREATE TABLE IF NOT EXISTS translation_cache (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, text_hash TEXT NOT NULL, source_lang TEXT NOT NULL, target_lang TEXT NOT NULL, translated_text TEXT NOT NULL, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)")
            .execute(&pool)
            .await
            .unwrap();

        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap(),
            store: DbStore::Sqlite(pool.clone()),
        });

        let claims = ExtClaims {
            sub: "test_user".to_string(),
            email: "test@example.com".to_string(),
            organization_id: Some("test_tenant".to_string()),
            exp: 9999999999,
            iat: 0,
            username: "test_user".to_string(),
            roles: vec!["admin".to_string()],
            session_id: None,
            jti: "".to_string(),
        };

        let queue: Arc<dyn TaskQueue> = Arc::new(MockQueue);

        // Test 1: Not in cache -> returns 202 Accepted and creates job
        let payload = TranslateRequest {
            source_text: "Hello".to_string(),
            source_lang: "en".to_string(),
            target_lang: "es".to_string(),
        };

        let response = translate(claims.clone(), State(db.clone()), State(queue.clone()), axum::Json(payload)).await;
        let res = response.into_response();
        assert_eq!(res.status(), axum::http::StatusCode::ACCEPTED);

        // Insert into cache directly
        let text_hash = format!("{:x}", sha2::Sha256::digest("Hello".as_bytes()));
        sqlx::query("INSERT INTO translation_cache (id, tenant_id, text_hash, source_lang, target_lang, translated_text) VALUES ('1', 'test_tenant', ?, 'en', 'es', 'Hola')")
            .bind(&text_hash)
            .execute(&pool)
            .await
            .unwrap();

        // Test 2: In cache -> returns 200 OK
        let payload2 = TranslateRequest {
            source_text: "Hello".to_string(),
            source_lang: "en".to_string(),
            target_lang: "es".to_string(),
        };
        let response2 = translate(claims, State(db), State(queue), axum::Json(payload2)).await;
        let res2 = response2.into_response();
        assert_eq!(res2.status(), axum::http::StatusCode::OK);
    }
}
