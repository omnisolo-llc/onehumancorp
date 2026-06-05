use server_services::translation::TranslationMeshService;
use sqlx::PgPool;
use std::sync::Arc;
use serde_json::Value;

#[tokio::test]
async fn test_translate_cache_miss_enqueues_job() {
    let db_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
    let pool = match tokio::time::timeout(std::time::Duration::from_millis(5000), sqlx::PgPool::connect(&db_url)).await {
        Ok(Ok(p)) => p,
        _ => return,
    };
    let service = TranslationMeshService::new(Arc::new(pool.clone()));

    let tenant_id = "test_tenant";
    let source_text = "Hello world";
    let source_lang = "en";
    let target_lang = "ar";

    let (text, cached) = service.translate(tenant_id, source_text, source_lang, target_lang).await.unwrap();
    assert_eq!(text, "Pending translation...");
    assert!(!cached);

    let row: (String,) = sqlx::query_as("SELECT payload FROM sub_agent_queue WHERE tenant_id = $1")
        .bind(tenant_id)
        .fetch_one(&pool)
        .await
        .expect("Job should be enqueued");

    let payload: Value = serde_json::from_str(&row.0).unwrap();
    assert_eq!(payload["type"], "TRANSLATION");
    assert_eq!(payload["source_text"], source_text);
    assert_eq!(payload["target_lang"], target_lang);
}

#[tokio::test]
async fn test_translate_cache_hit() {
    let db_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
    let pool = match tokio::time::timeout(std::time::Duration::from_millis(5000), sqlx::PgPool::connect(&db_url)).await {
        Ok(Ok(p)) => p,
        _ => return,
    };
    let service = TranslationMeshService::new(Arc::new(pool.clone()));

    let tenant_id = "test_tenant";
    let source_text = "Cached text";
    let source_lang = "en";
    let target_lang = "es";

    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(source_text);
    let text_hash = hex::encode(hasher.finalize());

    sqlx::query(
        "INSERT INTO translation_cache (id, tenant_id, source_text_hash, source_lang, target_lang, translated_text)
         VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(uuid::Uuid::new_v4().to_string())
    .bind(tenant_id)
    .bind(&text_hash)
    .bind(source_lang)
    .bind(target_lang)
    .bind("Texto en caché")
    .execute(&pool)
    .await
    .unwrap();

    let (text, cached) = service.translate(tenant_id, source_text, source_lang, target_lang).await.unwrap();
    assert_eq!(text, "Texto en caché");
    assert!(cached);
}
