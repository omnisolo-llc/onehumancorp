use super::*;
use serde_json::{Value, json};

async fn postgres(legacy: bool) -> sqlx::PgPool {
    let url = std::env::var("OHC_MEMORY_TEST_DATABASE_URL").expect("owned PostgreSQL is required");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&url)
        .await
        .unwrap();
    sqlx::query("CREATE EXTENSION IF NOT EXISTS vector WITH SCHEMA public")
        .execute(&pool)
        .await
        .unwrap();
    let schema = format!("memory_{}", uuid::Uuid::new_v4().simple());
    sqlx::query(&format!("CREATE SCHEMA {schema}"))
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query(&format!("SET search_path = {schema}, public"))
        .execute(&pool)
        .await
        .unwrap();
    sqlx::raw_sql(if legacy {
        include_str!("legacy.sql")
    } else {
        include_str!("active.sql")
    })
    .execute(&pool)
    .await
    .unwrap();
    pool
}

fn record(id: &str, metadata: Option<&str>) -> EmbeddingRecord {
    let now = Utc::now();
    EmbeddingRecord {
        id: id.to_owned(),
        tenant_id: "tenant-a".to_owned(),
        agent_id: "agent-a".to_owned(),
        content: "original context".to_owned(),
        embedding: vec![0.5; 1536],
        source_type: "MANUAL".to_owned(),
        created_at: now,
        last_referenced_at: now,
        reference_count: 0,
        reliability_score: 50,
        owner_override: false,
        metadata: metadata.map(str::to_owned),
    }
}

fn rich_metadata() -> String {
    json!({"namespace":"customer:東京","nested":{"receipt":[1,true,null,"quote \" and \\ slash"]},"value":12.25}).to_string()
}

async fn seed(pool: &sqlx::PgPool, record: &EmbeddingRecord) {
    sqlx::query("INSERT INTO consolidated_memory (id,tenant_id,agent_id,content,embedding,source_type,created_at,last_referenced_at,reference_count,reliability_score,owner_override,metadata) VALUES ($1,$2,$3,$4,$5::vector,$6,$7,$8,$9,$10,$11,$12)")
        .bind(&record.id).bind(&record.tenant_id).bind(&record.agent_id).bind(&record.content)
        .bind(serde_json::to_string(&record.embedding).unwrap()).bind(&record.source_type)
        .bind(record.created_at).bind(record.last_referenced_at).bind(record.reference_count)
        .bind(record.reliability_score).bind(record.owner_override)
        .bind(record.metadata.as_deref().map(|s| serde_json::from_str::<Value>(s).unwrap()))
        .execute(pool).await.unwrap();
}

fn metadata_value(record: &EmbeddingRecord) -> Option<Value> {
    record
        .metadata
        .as_deref()
        .map(|text| serde_json::from_str(text).unwrap())
}

#[tokio::test]
async fn postgres_jsonb_upsert_preserves_nested_metadata() {
    let pool = postgres(false).await;
    let repo = VectorRepository::new(pool.clone());
    let metadata = rich_metadata();
    let input = record("write", Some(&metadata));
    repo.upsert(&input)
        .await
        .expect("actual upsert must bind JSONB metadata");
    let stored: Value =
        sqlx::query_scalar("SELECT metadata FROM consolidated_memory WHERE id='write'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(stored, metadata_value(&input).unwrap());
    assert_eq!(
        repo.get_by_id("write").await.unwrap().unwrap().tenant_id,
        "tenant-a"
    );
}

#[tokio::test]
async fn postgres_jsonb_upsert_preserves_sql_null() {
    let pool = postgres(false).await;
    let repo = VectorRepository::new(pool.clone());
    repo.upsert(&record("null", None)).await.unwrap();
    let missing: bool =
        sqlx::query_scalar("SELECT metadata IS NULL FROM consolidated_memory WHERE id='null'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(missing);
    assert_eq!(
        repo.get_by_id("null").await.unwrap().unwrap().metadata,
        None
    );
}

#[tokio::test]
async fn postgres_json_null_is_distinct_from_sql_null() {
    let pool = postgres(false).await;
    let repo = VectorRepository::new(pool.clone());
    repo.upsert(&record("json-null", Some("null")))
        .await
        .unwrap();
    let missing: bool =
        sqlx::query_scalar("SELECT metadata IS NULL FROM consolidated_memory WHERE id='json-null'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(!missing);
    assert_eq!(
        metadata_value(&repo.get_by_id("json-null").await.unwrap().unwrap()),
        Some(Value::Null)
    );
}

#[tokio::test]
async fn postgres_malformed_metadata_rejects_without_mutation_or_data_echo() {
    let pool = postgres(false).await;
    let repo = VectorRepository::new(pool.clone());
    let original = record("malformed", Some("{\"before\":true}"));
    seed(&pool, &original).await;
    let mut update = original.clone();
    update.content = "must not persist".to_owned();
    update.reliability_score = 100;
    update.metadata = Some("{private-secret-invalid".to_owned());
    let error = repo
        .upsert(&update)
        .await
        .expect_err("malformed metadata must fail");
    assert!(error.to_lowercase().contains("metadata"), "{error}");
    assert!(!error.contains("private-secret-invalid"));
    let content: String =
        sqlx::query_scalar("SELECT content FROM consolidated_memory WHERE id='malformed'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(content, original.content);
    update.id = "malformed-insert".to_owned();
    assert!(repo.upsert(&update).await.is_err());
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM consolidated_memory")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 1);
}

#[tokio::test]
async fn postgres_jsonb_get_by_id_preserves_metadata() {
    let pool = postgres(false).await;
    let input = record("read", Some(&rich_metadata()));
    seed(&pool, &input).await;
    let repo = VectorRepository::new(pool);
    assert_eq!(
        metadata_value(&repo.get_by_id("read").await.unwrap().unwrap()),
        metadata_value(&input)
    );
}

#[tokio::test]
async fn postgres_jsonb_semantic_search_preserves_metadata_and_tenant() {
    let pool = postgres(false).await;
    let input = record("search", Some(&rich_metadata()));
    seed(&pool, &input).await;
    let mut other = record("other", Some("{\"private\":true}"));
    other.tenant_id = "tenant-b".to_owned();
    seed(&pool, &other).await;
    let repo = VectorRepository::new(pool.clone());
    let results = repo
        .semantic_search("tenant-a", &input.embedding, 10)
        .await
        .unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, input.id);
    assert_eq!(metadata_value(&results[0]), metadata_value(&input));
    let refs: i32 =
        sqlx::query_scalar("SELECT reference_count FROM consolidated_memory WHERE id='other'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(refs, 0);
}

#[tokio::test]
async fn postgres_jsonb_conflict_reads_preserve_both_metadata_values() {
    let pool = postgres(false).await;
    let a = record("a", Some(&rich_metadata()));
    let b = record("b", Some("{\"second\":[2,3]}"));
    seed(&pool, &a).await;
    seed(&pool, &b).await;
    let repo = VectorRepository::new(pool);
    let pairs = repo.get_conflicting_pairs().await.unwrap();
    assert_eq!(pairs.len(), 1);
    assert_eq!(metadata_value(&pairs[0].0), metadata_value(&a));
    assert_eq!(metadata_value(&pairs[0].1), metadata_value(&b));
}

#[tokio::test]
async fn postgres_get_by_id_reports_corrupt_rows_instead_of_missing() {
    let pool = postgres(false).await;
    seed(&pool, &record("corrupt", None)).await;
    sqlx::query("UPDATE consolidated_memory SET created_at=NULL WHERE id='corrupt'")
        .execute(&pool)
        .await
        .unwrap();
    let repo = VectorRepository::new(pool);
    assert!(repo.get_by_id("corrupt").await.is_err());
    assert!(repo.get_by_id("missing").await.unwrap().is_none());
}

#[tokio::test]
async fn postgres_conflict_reads_report_corruption_instead_of_no_conflict() {
    let pool = postgres(false).await;
    seed(&pool, &record("a", None)).await;
    seed(&pool, &record("b", None)).await;
    sqlx::query("UPDATE consolidated_memory SET created_at=NULL WHERE id='b'")
        .execute(&pool)
        .await
        .unwrap();
    assert!(
        VectorRepository::new(pool)
            .get_conflicting_pairs()
            .await
            .is_err()
    );
}

#[tokio::test]
async fn postgres_legacy_text_survives_active_create_and_jsonb_binding() {
    let pool = postgres(true).await;
    let input = record("legacy", Some(&rich_metadata()));
    let repo = VectorRepository::new(pool.clone());
    repo.upsert(&input).await.unwrap();
    sqlx::raw_sql(include_str!("active.sql"))
        .execute(&pool)
        .await
        .unwrap();
    let kind: String = sqlx::query_scalar(
        "SELECT pg_typeof(metadata)::text FROM consolidated_memory WHERE id='legacy'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(kind, "text");
    let mut update = input.clone();
    update.reliability_score = 99;
    update.metadata = Some("{\"after\":[true,null,4]}".to_owned());
    repo.upsert(&update).await.unwrap();
    assert_eq!(
        metadata_value(&repo.get_by_id("legacy").await.unwrap().unwrap()),
        metadata_value(&update)
    );
    repo.upsert(&record("legacy-null", None)).await.unwrap();
    assert_eq!(
        repo.get_by_id("legacy-null")
            .await
            .unwrap()
            .unwrap()
            .metadata,
        None
    );
}

#[tokio::test]
async fn postgres_owner_override_preserves_authoritative_metadata() {
    let pool = postgres(false).await;
    let mut original = record("owned", Some(&rich_metadata()));
    original.owner_override = true;
    seed(&pool, &original).await;
    let mut update = original.clone();
    update.owner_override = false;
    update.reliability_score = 100;
    update.content = "replacement".to_owned();
    update.metadata = Some("{\"replacement\":true}".to_owned());
    let repo = VectorRepository::new(pool);
    repo.upsert(&update).await.unwrap();
    let actual = repo.get_by_id("owned").await.unwrap().unwrap();
    assert_eq!(actual.content, original.content);
    assert!(actual.owner_override);
    assert_eq!(metadata_value(&actual), metadata_value(&original));
}

#[tokio::test]
async fn postgres_cross_tenant_conflict_rejects_atomically() {
    // Legacy TEXT isolates tenant-authority failure from the JSONB bind failure.
    let pool = postgres(true).await;
    let original = record("same-id", Some(&rich_metadata()));
    let repo = VectorRepository::new(pool.clone());
    repo.upsert(&original).await.unwrap();
    let before: Value = sqlx::query_scalar("SELECT row_to_json(consolidated_memory)::jsonb FROM consolidated_memory WHERE id='same-id'").fetch_one(&pool).await.unwrap();
    let mut hostile = original.clone();
    hostile.tenant_id = "tenant-b".to_owned();
    hostile.owner_override = true;
    hostile.reliability_score = 100;
    hostile.reference_count = 999;
    hostile.content = "foreign replacement".to_owned();
    hostile.metadata = Some("{\"foreign\":true}".to_owned());
    assert!(
        repo.upsert(&hostile).await.is_err(),
        "foreign conflict must not return success"
    );
    let after: Value = sqlx::query_scalar("SELECT row_to_json(consolidated_memory)::jsonb FROM consolidated_memory WHERE id='same-id'").fetch_one(&pool).await.unwrap();
    assert_eq!(before, after);
}

async fn sqlite() -> sqlx::SqlitePool {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::query("CREATE TABLE consolidated_memory (id TEXT PRIMARY KEY,tenant_id TEXT NOT NULL,agent_id TEXT,content TEXT NOT NULL,embedding TEXT,source_type TEXT NOT NULL,created_at TEXT,last_referenced_at TEXT,reference_count INTEGER,reliability_score INTEGER,owner_override BOOLEAN,metadata TEXT)").execute(&pool).await.unwrap();
    pool
}

#[tokio::test]
async fn sqlite_retains_existing_text_metadata_compatibility() {
    let pool = sqlite().await;
    let repo = VectorRepository::new_sqlite(pool);
    for (id, metadata) in [
        ("nested", Some(rich_metadata())),
        ("null", None),
        ("legacy", Some("legacy free text".to_owned())),
    ] {
        let input = record(id, metadata.as_deref());
        repo.upsert(&input).await.unwrap();
        assert_eq!(
            repo.get_by_id(id).await.unwrap().unwrap().metadata,
            metadata
        );
    }
}

#[tokio::test]
async fn sqlite_cross_tenant_conflict_rejects_atomically() {
    let pool = sqlite().await;
    let repo = VectorRepository::new_sqlite(pool);
    let original = record("same-id", Some(&rich_metadata()));
    repo.upsert(&original).await.unwrap();
    let mut hostile = original.clone();
    hostile.tenant_id = "tenant-b".to_owned();
    hostile.owner_override = true;
    hostile.content = "foreign replacement".to_owned();
    assert!(repo.upsert(&hostile).await.is_err());
    let stored = repo.get_by_id("same-id").await.unwrap().unwrap();
    assert_eq!(stored.content, original.content);
    assert_eq!(stored.metadata, original.metadata);
}

#[tokio::test]
async fn postgres_jsonb_list_recent_preserves_metadata() {
    let pool = postgres(false).await;
    let input = record("recent", Some(&rich_metadata()));
    seed(&pool, &input).await;
    let repo = VectorRepository::new(pool);
    let records = repo.list_recent("tenant-a", 10).await.unwrap();
    assert_eq!(records.len(), 1);
    assert_eq!(metadata_value(&records[0]), metadata_value(&input));
    assert!(repo.list_recent("tenant-b", 10).await.unwrap().is_empty());
}

#[tokio::test]
async fn postgres_list_recent_reports_corruption_instead_of_partial_result() {
    let pool = postgres(false).await;
    seed(&pool, &record("corrupt", None)).await;
    sqlx::query("UPDATE consolidated_memory SET created_at=NULL WHERE id='corrupt'")
        .execute(&pool)
        .await
        .unwrap();
    assert!(
        VectorRepository::new(pool)
            .list_recent("tenant-a", 10)
            .await
            .is_err()
    );
}

#[tokio::test]
async fn postgres_jsonb_binding_preserves_large_exact_numbers() {
    let pool = postgres(false).await;
    let repo = VectorRepository::new(pool.clone());
    let metadata =
        "{\"integer\":123456789012345678901234567890,\"decimal\":0.123456789012345678901234567890}";
    repo.upsert(&record("precision", Some(metadata)))
        .await
        .unwrap();
    let exact: bool = sqlx::query_scalar(
        "SELECT metadata = $1::jsonb FROM consolidated_memory WHERE id='precision'",
    )
    .bind(metadata)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(exact, "metadata numbers must not be rounded during binding");
}

async fn seed_legacy_text(pool: &sqlx::PgPool, input: &EmbeddingRecord) {
    let mut without_metadata = input.clone();
    without_metadata.metadata = None;
    seed(pool, &without_metadata).await;
    sqlx::query("UPDATE consolidated_memory SET metadata=$1 WHERE id=$2")
        .bind(&input.metadata)
        .bind(&input.id)
        .execute(pool)
        .await
        .unwrap();
}

#[tokio::test]
async fn postgres_legacy_text_writes_preserve_original_bytes() {
    let pool = postgres(true).await;
    let repo = VectorRepository::new(pool);
    for (id, value) in [
        ("plain", "  legacy free text\n東京  "),
        ("spaced-json", " { \"a\" : 1, \"b\" : [true] } \n"),
    ] {
        let input = record(id, Some(value));
        repo.upsert(&input)
            .await
            .expect("legacy TEXT remains free text");
        assert_eq!(
            repo.get_by_id(id)
                .await
                .unwrap()
                .unwrap()
                .metadata
                .as_deref(),
            Some(value)
        );
    }
}

#[tokio::test]
async fn postgres_legacy_owner_override_read_edit_write_preserves_text() {
    let pool = postgres(true).await;
    let original = record("override-text", Some("  legacy non-JSON metadata\n"));
    seed_legacy_text(&pool, &original).await;
    let repo = VectorRepository::new(pool);
    let mut updated = repo.get_by_id(&original.id).await.unwrap().unwrap();
    updated.content = "owner revision".to_owned();
    updated.owner_override = true;
    repo.upsert(&updated)
        .await
        .expect("read/edit/write must preserve legacy metadata");
    let stored = repo.get_by_id(&original.id).await.unwrap().unwrap();
    assert_eq!(stored.metadata, original.metadata);
    assert_eq!(stored.content, updated.content);
    assert!(stored.owner_override);
}

#[tokio::test]
async fn postgres_legacy_conflict_resolution_preserves_free_text() {
    let pool = postgres(true).await;
    let mut winner = record("winner", Some("legacy free text"));
    winner.owner_override = true;
    winner.reference_count = 1;
    let mut loser = record("loser", None);
    loser.reference_count = 1;
    seed_legacy_text(&pool, &winner).await;
    seed_legacy_text(&pool, &loser).await;
    let repo = VectorRepository::new(pool);
    repo.resolve_conflict(&winner, &loser)
        .await
        .expect("legacy free text is supported");
    let stored = repo.get_by_id(&winner.id).await.unwrap().unwrap();
    assert_eq!(stored.metadata, winner.metadata);
    assert_eq!(stored.reference_count, 2);
    assert!(repo.get_by_id(&loser.id).await.unwrap().is_none());
}

async fn postgres_rows(pool: &sqlx::PgPool) -> Vec<Value> {
    sqlx::query_scalar(
        "SELECT row_to_json(consolidated_memory)::jsonb FROM consolidated_memory ORDER BY id",
    )
    .fetch_all(pool)
    .await
    .unwrap()
}

#[tokio::test]
async fn postgres_conflict_write_failure_does_not_delete_loser() {
    let pool = postgres(false).await;
    let mut winner = record("winner", Some("{\"original\":true}"));
    winner.reference_count = 1;
    let mut loser = record("loser", None);
    loser.reference_count = 1;
    seed(&pool, &winner).await;
    seed(&pool, &loser).await;
    sqlx::query(
        "ALTER TABLE consolidated_memory ADD CONSTRAINT bounded_refs CHECK(reference_count < 2)",
    )
    .execute(&pool)
    .await
    .unwrap();
    let before = postgres_rows(&pool).await;
    let error = VectorRepository::new(pool.clone())
        .resolve_conflict(&winner, &loser)
        .await
        .unwrap_err();
    assert!(
        error.contains("bounded_refs"),
        "expected real winner write constraint error: {error}"
    );
    assert_eq!(postgres_rows(&pool).await, before);
}

#[tokio::test]
async fn postgres_conflict_delete_failure_rolls_back_winner_update() {
    let pool = postgres(false).await;
    let mut winner = record("winner", Some("{\"original\":true}"));
    winner.reference_count = 1;
    let mut loser = record("loser", None);
    loser.reference_count = 1;
    seed(&pool, &winner).await;
    seed(&pool, &loser).await;
    sqlx::raw_sql("CREATE FUNCTION reject_memory_delete() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'owned-fixture-delete-failure'; END $$; CREATE TRIGGER reject_memory_delete BEFORE DELETE ON consolidated_memory FOR EACH ROW EXECUTE FUNCTION reject_memory_delete();")
        .execute(&pool).await.unwrap();
    let before = postgres_rows(&pool).await;
    let error = VectorRepository::new(pool.clone())
        .resolve_conflict(&winner, &loser)
        .await
        .unwrap_err();
    assert!(error.contains("owned-fixture-delete-failure"));
    assert_eq!(postgres_rows(&pool).await, before);
}

#[tokio::test]
async fn postgres_conflict_malformed_jsonb_does_not_delete_loser() {
    let pool = postgres(false).await;
    let mut winner = record("winner", None);
    let loser = record("loser", None);
    seed(&pool, &winner).await;
    seed(&pool, &loser).await;
    let before = postgres_rows(&pool).await;
    winner.metadata = Some("invalid JSON".to_owned());
    assert!(
        VectorRepository::new(pool.clone())
            .resolve_conflict(&winner, &loser)
            .await
            .is_err()
    );
    assert_eq!(postgres_rows(&pool).await, before);
}

#[tokio::test]
async fn postgres_conflict_rejects_foreign_loser_without_mutation() {
    let pool = postgres(false).await;
    let winner = record("winner", None);
    let mut foreign = record("foreign", None);
    foreign.tenant_id = "tenant-b".to_owned();
    seed(&pool, &winner).await;
    seed(&pool, &foreign).await;
    let before = postgres_rows(&pool).await;
    // Caller data cannot authorize deleting a differently-owned persisted row.
    foreign.tenant_id = winner.tenant_id.clone();
    assert!(
        VectorRepository::new(pool.clone())
            .resolve_conflict(&winner, &foreign)
            .await
            .is_err()
    );
    assert_eq!(postgres_rows(&pool).await, before);
}

#[tokio::test]
async fn postgres_semantic_search_reports_null_timestamps() {
    for column in ["created_at", "last_referenced_at"] {
        let pool = postgres(false).await;
        let input = record("corrupt", Some("{}"));
        seed(&pool, &input).await;
        sqlx::query(&format!("UPDATE consolidated_memory SET {column}=NULL"))
            .execute(&pool)
            .await
            .unwrap();
        let result = VectorRepository::new(pool)
            .semantic_search("tenant-a", &input.embedding, 10)
            .await;
        assert!(result.is_err(), "NULL {column} must return an error");
    }
}

#[tokio::test]
async fn postgres_get_by_id_reports_null_embedding() {
    let pool = postgres(false).await;
    seed(&pool, &record("corrupt", None)).await;
    sqlx::query("UPDATE consolidated_memory SET embedding=NULL")
        .execute(&pool)
        .await
        .unwrap();
    assert!(
        VectorRepository::new(pool)
            .get_by_id("corrupt")
            .await
            .is_err()
    );
}

#[tokio::test]
async fn sqlite_conflict_write_failure_does_not_delete_loser() {
    let pool = sqlite().await;
    let repo = VectorRepository::new_sqlite(pool.clone());
    let mut winner = record("winner", Some("legacy text"));
    winner.reference_count = 1;
    let mut loser = record("loser", None);
    loser.reference_count = 1;
    repo.upsert(&winner).await.unwrap();
    repo.upsert(&loser).await.unwrap();
    sqlx::query("CREATE TRIGGER reject_merged_refs BEFORE UPDATE ON consolidated_memory WHEN NEW.reference_count >= 2 BEGIN SELECT RAISE(ABORT, 'owned-fixture-write-failure'); END")
        .execute(&pool).await.unwrap();
    let before: Vec<(String, String, i32)> =
        sqlx::query_as("SELECT id,content,reference_count FROM consolidated_memory ORDER BY id")
            .fetch_all(&pool)
            .await
            .unwrap();
    let error = repo.resolve_conflict(&winner, &loser).await.unwrap_err();
    assert!(error.contains("owned-fixture-write-failure"));
    let after: Vec<(String, String, i32)> =
        sqlx::query_as("SELECT id,content,reference_count FROM consolidated_memory ORDER BY id")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(before, after);
}
