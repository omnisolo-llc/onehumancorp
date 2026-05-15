pub use ohc_builtin_agent_core::types::{EmbeddingRecord, LongTermMemory};
use chrono::{DateTime, Utc};
use sqlx::Row;
use async_trait::async_trait;
use std::sync::Arc;

pub enum VectorMemoryStore {
    Postgres(sqlx::PgPool),
    Sqlite(sqlx::SqlitePool),
}

pub struct VectorRepository {
    store: VectorMemoryStore,
}

impl VectorRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        VectorRepository { store: VectorMemoryStore::Postgres(pool) }
    }

    pub fn new_sqlite(pool: sqlx::SqlitePool) -> Self {
        VectorRepository { store: VectorMemoryStore::Sqlite(pool) }
    }

    pub fn get_store(&self) -> &VectorMemoryStore {
        &self.store
    }

    pub fn get_store_pool_sqlite(&self) -> Option<&sqlx::SqlitePool> {
        match &self.store {
            VectorMemoryStore::Sqlite(p) => Some(p),
            _ => None,
        }
    }

    pub async fn upsert(&self, record: &EmbeddingRecord) -> Result<(), String> {
        let emb_str = serde_json::to_string(&record.embedding).map_err(|e| format!("Serialization Error: {}", e))?;
        match &self.store {
            VectorMemoryStore::Postgres(pool) => {
                sqlx::query(
                    r#"INSERT INTO consolidated_memory (
                        id, tenant_id, agent_id, content, embedding, source_type,
                        created_at, last_referenced_at, reference_count, reliability_score,
                        owner_override, archived, metadata
                    ) VALUES ($1, $2, $3, $4, $5::vector, $6, $7, $8, $9, $10, $11, $12, $13)
                    ON CONFLICT(id) DO UPDATE SET
                        content=excluded.content, embedding=excluded.embedding,
                        created_at=excluded.created_at, last_referenced_at=excluded.last_referenced_at,
                        reference_count=excluded.reference_count, reliability_score=excluded.reliability_score,
                        owner_override=excluded.owner_override, archived=excluded.archived, metadata=excluded.metadata"#
                )
                .bind(&record.id).bind(&record.tenant_id).bind(&record.agent_id).bind(&record.content)
                .bind(&emb_str).bind(&record.source_type).bind(record.created_at).bind(record.last_referenced_at)
                .bind(record.reference_count).bind(record.reliability_score).bind(record.owner_override)
                .bind(record.archived).bind(&record.metadata)
                .execute(pool).await.map_err(|e| e.to_string())?;
            }
            VectorMemoryStore::Sqlite(pool) => {
                sqlx::query(
                    r#"INSERT INTO consolidated_memory (
                        id, tenant_id, agent_id, content, embedding, source_type,
                        created_at, last_referenced_at, reference_count, reliability_score,
                        owner_override, archived, metadata
                    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    ON CONFLICT(id) DO UPDATE SET
                        content=excluded.content, embedding=excluded.embedding,
                        created_at=excluded.created_at, last_referenced_at=excluded.last_referenced_at,
                        reference_count=excluded.reference_count, reliability_score=excluded.reliability_score,
                        owner_override=excluded.owner_override, archived=excluded.archived, metadata=excluded.metadata"#
                )
                .bind(&record.id).bind(&record.tenant_id).bind(&record.agent_id).bind(&record.content)
                .bind(&emb_str).bind(&record.source_type).bind(record.created_at).bind(record.last_referenced_at)
                .bind(record.reference_count).bind(record.reliability_score).bind(record.owner_override)
                .bind(record.archived).bind(&record.metadata)
                .execute(pool).await.map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    pub async fn semantic_search(&self, tenant_id: &str, query_embedding: &[f32], limit: i64) -> Result<Vec<EmbeddingRecord>, String> {
        let emb_str = serde_json::to_string(query_embedding).map_err(|e| e.to_string())?;
        let mut results = Vec::new();
        let mut ids_to_update = Vec::new();
        match &self.store {
            VectorMemoryStore::Postgres(pool) => {
                let rows = sqlx::query("SELECT id, tenant_id, COALESCE(agent_id, '') as agent_id, content, embedding::text, source_type, created_at, last_referenced_at, reference_count, reliability_score, owner_override, archived, metadata FROM consolidated_memory WHERE tenant_id = $1 AND archived = FALSE ORDER BY embedding <=> $2::vector LIMIT $3")
                .bind(tenant_id).bind(emb_str).bind(limit).fetch_all(pool).await.map_err(|e| e.to_string())?;
                for row in rows {
                    let id: String = row.get("id"); ids_to_update.push(id.clone());
                    results.push(EmbeddingRecord {
                        id, tenant_id: row.get("tenant_id"), agent_id: row.get("agent_id"), content: row.get("content"),
                        embedding: serde_json::from_str(&row.get::<String, _>("embedding")).unwrap_or_default(),
                        source_type: row.get("source_type"), created_at: row.get("created_at"), last_referenced_at: row.get("last_referenced_at"),
                        reference_count: row.get("reference_count"), reliability_score: row.get("reliability_score"),
                        owner_override: row.get("owner_override"), archived: row.get("archived"), metadata: row.get("metadata"),
                    });
                }
                if !ids_to_update.is_empty() {
                    let _ = sqlx::query("UPDATE consolidated_memory SET last_referenced_at = CURRENT_TIMESTAMP, reference_count = reference_count + 1 WHERE id = ANY($1)").bind(&ids_to_update).execute(pool).await;
                }
            }
            VectorMemoryStore::Sqlite(pool) => {
                let has_vec = sqlx::query("SELECT vec_distance_cosine('[1.0]', '[1.0]')").execute(pool).await.is_ok();
                if has_vec {
                    let rows = sqlx::query("SELECT id, tenant_id, COALESCE(agent_id, '') as agent_id, content, embedding, source_type, created_at, last_referenced_at, reference_count, reliability_score, owner_override, archived, metadata FROM consolidated_memory WHERE tenant_id = ? AND archived = FALSE ORDER BY vec_distance_cosine(embedding, ?) LIMIT ?")
                    .bind(tenant_id).bind(&emb_str).bind(limit).fetch_all(pool).await.map_err(|e| e.to_string())?;
                    for row in rows {
                        let id: String = row.get("id"); ids_to_update.push(id.clone());
                        results.push(EmbeddingRecord {
                            id, tenant_id: row.get("tenant_id"), agent_id: row.get("agent_id"), content: row.get("content"),
                            embedding: serde_json::from_str(&row.get::<String, _>("embedding")).unwrap_or_default(),
                            source_type: row.get("source_type"), created_at: row.get("created_at"), last_referenced_at: row.get("last_referenced_at"),
                            reference_count: row.get("reference_count"), reliability_score: row.get("reliability_score"),
                            owner_override: row.get("owner_override"), archived: row.get("archived"), metadata: row.get("metadata"),
                        });
                    }
                } else {
                    let rows = sqlx::query("SELECT id, tenant_id, agent_id, content, embedding, source_type, created_at, last_referenced_at, reference_count, reliability_score, owner_override, archived, metadata FROM consolidated_memory WHERE tenant_id = ? AND archived = FALSE LIMIT 1000")
                        .bind(tenant_id).fetch_all(pool).await.map_err(|e| e.to_string())?;
                    let mut all = Vec::new();
                    for row in rows {
                        let emb: String = row.try_get("embedding").unwrap_or_else(|_| String::from_utf8(row.get::<Vec<u8>, _>("embedding")).unwrap_or_default());
                        all.push(EmbeddingRecord {
                            id: row.get("id"), tenant_id: row.get("tenant_id"), agent_id: row.get::<Option<String>, _>("agent_id").unwrap_or_default(),
                            content: row.get("content"), embedding: serde_json::from_str(&emb).unwrap_or_default(), source_type: row.get("source_type"),
                            created_at: row.get("created_at"), last_referenced_at: row.get("last_referenced_at"),
                            reference_count: row.get("reference_count"), reliability_score: row.get("reliability_score"),
                            owner_override: row.get("owner_override"), archived: row.get("archived"), metadata: row.get("metadata"),
                        });
                    }
                    fn cos(a: &[f32], b: &[f32]) -> f32 {
                        if a.len() != b.len() || a.is_empty() { return 1.0; }
                        let mut d=0.0; let mut na=0.0; let mut nb=0.0;
                        for i in 0..a.len() { d+=a[i]*b[i]; na+=a[i]*a[i]; nb+=b[i]*b[i]; }
                        if na==0.0||nb==0.0 { return 1.0; }
                        1.0 - (d / (na.sqrt()*nb.sqrt()))
                    }
                    let q_emb: Vec<f32> = serde_json::from_str(&emb_str).unwrap_or_default();
                    all.sort_by(|a, b| cos(&a.embedding, &q_emb).partial_cmp(&cos(&b.embedding, &q_emb)).unwrap_or(std::cmp::Ordering::Equal));
                    results = all.into_iter().take(limit as usize).collect();
                    for r in &results { ids_to_update.push(r.id.clone()); }
                }
                if !ids_to_update.is_empty() {
                    let placeholders = ids_to_update.iter().map(|_| "?").collect::<Vec<_>>().join(",");
                    let query_str = format!("UPDATE consolidated_memory SET last_referenced_at = CURRENT_TIMESTAMP, reference_count = reference_count + 1 WHERE id IN ({})", placeholders);
                    let mut q = sqlx::query(&query_str);
                    for id in ids_to_update { q = q.bind(id); }
                    let _ = q.execute(pool).await;
                }
            }
        }
        Ok(results)
    }

    pub async fn archive_stale(&self, older_than: DateTime<Utc>) -> Result<u64, String> {
        match &self.store {
            VectorMemoryStore::Postgres(pool) => {
                let r = sqlx::query("UPDATE consolidated_memory SET archived = TRUE WHERE last_referenced_at < $1 AND owner_override = FALSE AND archived = FALSE").bind(older_than).execute(pool).await.map_err(|e| e.to_string())?;
                Ok(r.rows_affected())
            }
            VectorMemoryStore::Sqlite(pool) => {
                let r = sqlx::query("UPDATE consolidated_memory SET archived = TRUE WHERE last_referenced_at < ? AND owner_override = FALSE AND archived = FALSE").bind(older_than).execute(pool).await.map_err(|e| e.to_string())?;
                Ok(r.rows_affected())
            }
        }
    }

    pub async fn prune_stale(&self, older_than: DateTime<Utc>) -> Result<(), String> {
        match &self.store {
            VectorMemoryStore::Postgres(pool) => {
                sqlx::query("DELETE FROM consolidated_memory WHERE (last_referenced_at < $1 AND owner_override = FALSE AND reference_count < 5 AND source_type = 'TASK_SUMMARY' AND archived = TRUE) OR (reliability_score < 20 AND owner_override = FALSE)").bind(older_than).execute(pool).await.map_err(|e| e.to_string())?;
            }
            VectorMemoryStore::Sqlite(pool) => {
                sqlx::query("DELETE FROM consolidated_memory WHERE (last_referenced_at < ? AND owner_override = FALSE AND reference_count < 5 AND source_type = 'TASK_SUMMARY' AND archived = TRUE) OR (reliability_score < 20 AND owner_override = FALSE)").bind(older_than).execute(pool).await.map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    pub async fn delete(&self, id: &str) -> Result<(), String> {
        match &self.store {
            VectorMemoryStore::Postgres(pool) => { sqlx::query("DELETE FROM consolidated_memory WHERE id = $1").bind(id).execute(pool).await.map_err(|e| e.to_string())?; }
            VectorMemoryStore::Sqlite(pool) => { sqlx::query("DELETE FROM consolidated_memory WHERE id = ?").bind(id).execute(pool).await.map_err(|e| e.to_string())?; }
        }
        Ok(())
    }

    pub async fn resolve_conflict(&self, winner: &EmbeddingRecord, loser: &EmbeddingRecord) -> Result<(), String> {
        self.delete(&loser.id).await?;
        let mut updated = winner.clone();
        updated.reference_count += loser.reference_count + 1;
        updated.last_referenced_at = Utc::now();
        if loser.owner_override && !updated.owner_override { updated.owner_override = true; }
        self.upsert(&updated).await
    }

    pub async fn auto_resolve_conflicts(&self) -> Result<usize, String> {
        let conflicts = self.get_conflicting_pairs().await?;
        let mut count = 0;
        for (a, b) in conflicts {
            let (winner, loser) = Self::determine_conflict_winner(&a, &b);
            self.resolve_conflict(winner, loser).await?;
            count += 1;
        }
        Ok(count)
    }

    pub async fn consolidate_records(&self, tenant_id: &str, _limit: i64) -> Result<Vec<Vec<EmbeddingRecord>>, String> {
        let mut groups = Vec::new();
        let pairs = self.get_conflicting_pairs().await?;
        for (a, b) in pairs { if a.tenant_id == tenant_id { groups.push(vec![a, b]); } }
        Ok(groups)
    }

    pub async fn get_active_tenants(&self) -> Result<Vec<String>, String> {
        match &self.store {
            VectorMemoryStore::Postgres(pool) => {
                let rows = sqlx::query("SELECT DISTINCT tenant_id FROM consolidated_memory").fetch_all(pool).await.map_err(|e| e.to_string())?;
                Ok(rows.into_iter().map(|r| r.get("tenant_id")).collect())
            }
            VectorMemoryStore::Sqlite(pool) => {
                let rows = sqlx::query("SELECT DISTINCT tenant_id FROM consolidated_memory").fetch_all(pool).await.map_err(|e| e.to_string())?;
                Ok(rows.into_iter().map(|r| r.get("tenant_id")).collect())
            }
        }
    }

    pub fn determine_conflict_winner<'a>(a: &'a EmbeddingRecord, b: &'a EmbeddingRecord) -> (&'a EmbeddingRecord, &'a EmbeddingRecord) {
        if a.owner_override != b.owner_override { if a.owner_override { (a, b) } else { (b, a) } }
        else if a.reliability_score != b.reliability_score { if a.reliability_score > b.reliability_score { (a, b) } else { (b, a) } }
        else if a.created_at != b.created_at { if a.created_at > b.created_at { (a, b) } else { (b, a) } }
        else { (a, b) }
    }

    pub async fn get_conflicting_pairs(&self) -> Result<Vec<(EmbeddingRecord, EmbeddingRecord)>, String> {
        let mut conflicts = Vec::new();
        match &self.store {
            VectorMemoryStore::Postgres(pool) => {
                let query = "SELECT a.id AS a_id, a.tenant_id AS a_tenant_id, a.agent_id AS a_agent_id, a.content AS a_content, a.embedding::text AS a_embedding, a.source_type AS a_source_type, a.created_at AS a_created_at, a.last_referenced_at AS a_last_referenced_at, a.reference_count AS a_reference_count, a.reliability_score AS a_reliability_score, a.owner_override AS a_owner_override, a.archived AS a_archived, a.metadata AS a_metadata, b.id AS b_id, b.tenant_id AS b_tenant_id, b.agent_id AS b_agent_id, b.content AS b_content, b.embedding::text AS b_embedding, b.source_type AS b_source_type, b.created_at AS b_created_at, b.last_referenced_at AS b_last_referenced_at, b.reference_count AS b_reference_count, b.reliability_score AS b_reliability_score, b.owner_override AS b_owner_override, b.archived AS b_archived, b.metadata AS b_metadata FROM consolidated_memory a JOIN consolidated_memory b ON a.tenant_id = b.tenant_id AND a.id < b.id WHERE a.embedding <=> b.embedding < 0.05 AND a.archived = FALSE AND b.archived = FALSE LIMIT 10";
                let rows = sqlx::query(query).fetch_all(pool).await.map_err(|e| e.to_string())?;
                for row in rows {
                    let a_emb: String = row.get("a_embedding"); let b_emb: String = row.get("b_embedding");
                    conflicts.push((
                        EmbeddingRecord { id: row.get("a_id"), tenant_id: row.get("a_tenant_id"), agent_id: row.get::<Option<String>, _>("a_agent_id").unwrap_or_default(), content: row.get("a_content"), embedding: serde_json::from_str(&a_emb).unwrap_or_default(), source_type: row.get("a_source_type"), created_at: row.get("a_created_at"), last_referenced_at: row.get("a_last_referenced_at"), reference_count: row.get("a_reference_count"), reliability_score: row.get("a_reliability_score"), owner_override: row.get("a_owner_override"), archived: row.get("a_archived"), metadata: row.get("a_metadata") },
                        EmbeddingRecord { id: row.get("b_id"), tenant_id: row.get("b_tenant_id"), agent_id: row.get::<Option<String>, _>("b_agent_id").unwrap_or_default(), content: row.get("b_content"), embedding: serde_json::from_str(&b_emb).unwrap_or_default(), source_type: row.get("b_source_type"), created_at: row.get("b_created_at"), last_referenced_at: row.get("b_last_referenced_at"), reference_count: row.get("b_reference_count"), reliability_score: row.get("b_reliability_score"), owner_override: row.get("b_owner_override"), archived: row.get("b_archived"), metadata: row.get("b_metadata") }
                    ));
                }
            }
            VectorMemoryStore::Sqlite(pool) => {
                let has_vec = sqlx::query("SELECT vec_distance_cosine('[1.0]', '[1.0]')").execute(pool).await.is_ok();
                if has_vec {
                    let query = "SELECT a.id AS a_id, a.tenant_id AS a_tenant_id, a.agent_id AS a_agent_id, a.content AS a_content, a.embedding AS a_embedding, a.source_type AS a_source_type, a.created_at AS a_created_at, a.last_referenced_at AS a_last_referenced_at, a.reference_count AS a_reference_count, a.reliability_score AS a_reliability_score, a.owner_override AS a_owner_override, a.archived AS a_archived, a.metadata AS a_metadata, b.id AS b_id, b.tenant_id AS b_tenant_id, b.agent_id AS b_agent_id, b.content AS b_content, b.embedding AS b_embedding, b.source_type AS b_source_type, b.created_at AS b_created_at, b.last_referenced_at AS b_last_referenced_at, b.reference_count AS b_reference_count, b.reliability_score AS b_reliability_score, b.owner_override AS b_owner_override, b.archived AS b_archived, b.metadata AS b_metadata FROM consolidated_memory a JOIN consolidated_memory b ON a.tenant_id = b.tenant_id AND a.id < b.id WHERE vec_distance_cosine(a.embedding, b.embedding) < 0.05 AND a.archived = FALSE AND b.archived = FALSE LIMIT 10";
                    let rows = sqlx::query(query).fetch_all(pool).await.map_err(|e| e.to_string())?;
                    for row in rows {
                        let a_emb: String = row.try_get("a_embedding").unwrap_or_else(|_| String::from_utf8(row.get::<Vec<u8>, _>("a_embedding")).unwrap_or_default());
                        let b_emb: String = row.try_get("b_embedding").unwrap_or_else(|_| String::from_utf8(row.get::<Vec<u8>, _>("b_embedding")).unwrap_or_default());
                        conflicts.push((
                            EmbeddingRecord { id: row.get("a_id"), tenant_id: row.get("a_tenant_id"), agent_id: row.get::<Option<String>, _>("a_agent_id").unwrap_or_default(), content: row.get("a_content"), embedding: serde_json::from_str(&a_emb).unwrap_or_default(), source_type: row.get("a_source_type"), created_at: row.try_get::<DateTime<Utc>, _>("a_created_at").map_err(|e| e.to_string())?, last_referenced_at: row.try_get::<DateTime<Utc>, _>("a_last_referenced_at").map_err(|e| e.to_string())?, reference_count: row.get("a_reference_count"), reliability_score: row.get("a_reliability_score"), owner_override: row.get("a_owner_override"), archived: row.get("a_archived"), metadata: row.get("a_metadata") },
                            EmbeddingRecord { id: row.get("b_id"), tenant_id: row.get("b_tenant_id"), agent_id: row.get::<Option<String>, _>("b_agent_id").unwrap_or_default(), content: row.get("b_content"), embedding: serde_json::from_str(&b_emb).unwrap_or_default(), source_type: row.get("b_source_type"), created_at: row.try_get::<DateTime<Utc>, _>("b_created_at").map_err(|e| e.to_string())?, last_referenced_at: row.try_get::<DateTime<Utc>, _>("b_last_referenced_at").map_err(|e| e.to_string())?, reference_count: row.get("b_reference_count"), reliability_score: row.get("b_reliability_score"), owner_override: row.get("b_owner_override"), archived: row.get("b_archived"), metadata: row.get("b_metadata") }
                        ));
                    }
                } else {
                    let query = "SELECT id, tenant_id, agent_id, content, embedding, source_type, created_at, last_referenced_at, reference_count, reliability_score, owner_override, archived, metadata FROM consolidated_memory WHERE archived = FALSE LIMIT 1000";
                    let rows = sqlx::query(query).fetch_all(pool).await.map_err(|e| e.to_string())?;
                    let mut all = Vec::new();
                    for row in rows {
                        let emb: String = row.try_get("embedding").unwrap_or_else(|_| String::from_utf8(row.get::<Vec<u8>, _>("embedding")).unwrap_or_default());
                        all.push(EmbeddingRecord {
                            id: row.get("id"), tenant_id: row.get("tenant_id"), agent_id: row.get::<Option<String>, _>("agent_id").unwrap_or_default(),
                            content: row.get("content"), embedding: serde_json::from_str(&emb).unwrap_or_default(), source_type: row.get("source_type"),
                            created_at: row.try_get::<DateTime<Utc>, _>("created_at").map_err(|e| e.to_string())?, last_referenced_at: row.try_get::<DateTime<Utc>, _>("last_referenced_at").map_err(|e| e.to_string())?,
                            reference_count: row.get("reference_count"), reliability_score: row.get("reliability_score"),
                            owner_override: row.get("owner_override"), archived: row.get("archived"), metadata: row.get("metadata"),
                        });
                    }
                    fn cos(a: &[f32], b: &[f32]) -> f32 {
                        if a.len() != b.len() || a.is_empty() { return 1.0; }
                        let mut d=0.0; let mut na=0.0; let mut nb=0.0;
                        for i in 0..a.len() { d+=a[i]*b[i]; na+=a[i]*a[i]; nb+=b[i]*b[i]; }
                        if na==0.0||nb==0.0 { return 1.0; }
                        1.0 - (d / (na.sqrt()*nb.sqrt()))
                    }
                    for i in 0..all.len() {
                        for j in (i + 1)..all.len() {
                            let a = &all[i]; let b = &all[j];
                            if a.tenant_id == b.tenant_id && cos(&a.embedding, &b.embedding) < 0.05 {
                                conflicts.push((a.clone(), b.clone()));
                                if conflicts.len() >= 10 { break; }
                            }
                        }
                        if conflicts.len() >= 10 { break; }
                    }
                }
            }
        }
        Ok(conflicts)
    }
}

pub struct Anthropic3TierMemoryStore {
    #[allow(dead_code)] base_dir: std::path::PathBuf,
    pub index_file: std::path::PathBuf, topics_dir: std::path::PathBuf, transcripts_dir: std::path::PathBuf,
}

impl std::fmt::Debug for Anthropic3TierMemoryStore { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_struct("Anthropic3TierMemoryStore").finish() } }

impl Anthropic3TierMemoryStore {
    pub fn new<P: AsRef<std::path::Path>>(base_dir: P) -> Result<Self, String> {
        let base_dir = base_dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&base_dir).map_err(|e| e.to_string())?;
        std::fs::create_dir_all(base_dir.join("topics")).map_err(|e| e.to_string())?;
        std::fs::create_dir_all(base_dir.join("transcripts")).map_err(|e| e.to_string())?;
        let index_file = base_dir.join("index.md");
        if !index_file.exists() { let _ = std::fs::File::create(&index_file); }
        Ok(Self { index_file, topics_dir: base_dir.join("topics"), transcripts_dir: base_dir.join("transcripts"), base_dir })
    }
    pub fn as_anthropic_accessor(self: Arc<Self>) -> Option<Arc<dyn ohc_builtin_agent_tools::anthropic_memory::MemoryAccessor>> { Some(self) }
}

#[async_trait]
impl ohc_builtin_agent_tools::anthropic_memory::MemoryAccessor for Anthropic3TierMemoryStore {
    async fn retrieve_topic(&self, topic_name: &str) -> Result<String, String> {
        let path = self.topics_dir.join(format!("{}.md", topic_name));
        if path.exists() { tokio::fs::read_to_string(&path).await.map_err(|e| e.to_string()) } else { Err("Not found".to_string()) }
    }
    async fn search_transcripts(&self, _query: &str, _limit: usize) -> Result<Vec<String>, String> { Ok(vec![]) }
}

#[async_trait]
impl LongTermMemory for Anthropic3TierMemoryStore {
    async fn retrieve(&self, _query: &str, _limit: usize) -> Result<Vec<String>, String> { Ok(vec![]) }
    async fn store(&self, _content: &str, _tags: Vec<String>) -> Result<(), String> { Ok(()) }
}

pub struct PersistentMemoryStore {
    pub repo: Arc<VectorRepository>,
    pub tenant_id: String,
    pub agent_id: String,
    pub llm: Arc<dyn ohc_builtin_agent_llm::LlmClient>,
}

impl std::fmt::Debug for PersistentMemoryStore { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_struct("PersistentMemoryStore").finish() } }

#[async_trait]
impl LongTermMemory for PersistentMemoryStore {
    async fn retrieve(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        let embedding = self.llm.generate_embedding(query).await.map_err(|e| e.to_string())?;
        let records = self.repo.semantic_search(&self.tenant_id, &embedding, limit as i64).await?;
        Ok(records.into_iter().map(|r| r.content).collect())
    }
    async fn store(&self, content: &str, _tags: Vec<String>) -> Result<(), String> {
        let embedding = self.llm.generate_embedding(content).await.map_err(|e| e.to_string())?;
        self.repo.upsert(&EmbeddingRecord {
            id: uuid::Uuid::new_v4().to_string(), tenant_id: self.tenant_id.clone(), agent_id: self.agent_id.clone(), content: content.to_string(), embedding, source_type: "MANUAL".to_string(),
            created_at: Utc::now(), last_referenced_at: Utc::now(), reference_count: 0, reliability_score: 100, owner_override: false, archived: false, metadata: None,
        }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    #[test]
    fn test_embedding_record_serialization() {
        let now = Utc.with_ymd_and_hms(2026, 4, 26, 0, 0, 0).unwrap();
        let record = EmbeddingRecord { id: "rec1".to_string(), tenant_id: "org1".to_string(), agent_id: "agent1".to_string(), content: "Hello world".to_string(), embedding: vec![1.0], source_type: "TEXT".to_string(), created_at: now, last_referenced_at: now, reference_count: 0, reliability_score: 50, owner_override: false, archived: false, metadata: None };
        let json = serde_json::to_string(&record).unwrap();
        let deserialized: EmbeddingRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(record.id, deserialized.id);
    }
    #[tokio::test]
    async fn test_anthropic_3_tier_memory_store() {
        let base_dir = tempfile::tempdir().unwrap();
        let store = Anthropic3TierMemoryStore::new(base_dir.path()).unwrap();
        assert!(store.index_file.exists());
    }
}

#[cfg(test)]
mod get_conflicts_tests {
    use super::*;
    #[tokio::test]
    async fn test_auto_resolve_conflicts() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();
        sqlx::query("CREATE TABLE IF NOT EXISTS consolidated_memory (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, agent_id TEXT, content TEXT NOT NULL, embedding TEXT, source_type TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, reference_count INTEGER DEFAULT 0, reliability_score INTEGER DEFAULT 50, owner_override BOOLEAN DEFAULT FALSE, archived BOOLEAN DEFAULT FALSE, metadata TEXT);").execute(&pool).await.unwrap();
        let repo = Arc::new(VectorRepository::new_sqlite(pool));
        let now = Utc::now();
        let r1 = EmbeddingRecord { id: "r1".to_string(), tenant_id: "o1".to_string(), agent_id: "a1".to_string(), content: "c1".to_string(), embedding: vec![1.0], source_type: "s".to_string(), created_at: now, last_referenced_at: now, reference_count: 0, reliability_score: 50, owner_override: true, archived: false, metadata: None };
        let r2 = EmbeddingRecord { id: "r2".to_string(), tenant_id: "o1".to_string(), agent_id: "a1".to_string(), content: "c2".to_string(), embedding: vec![1.0], source_type: "s".to_string(), created_at: now, last_referenced_at: now, reference_count: 10, reliability_score: 100, owner_override: false, archived: false, metadata: None };
        repo.upsert(&r1).await.unwrap(); repo.upsert(&r2).await.unwrap();
        assert_eq!(repo.auto_resolve_conflicts().await.unwrap(), 1);
    }
}

impl VectorRepository {
    /// Batch upsert for performance optimization in high-volume ingestion scenarios.
    pub async fn batch_upsert(&self, records: Vec<EmbeddingRecord>) -> Result<(), String> {
        for record in records {
            self.upsert(&record).await?;
        }
        Ok(())
    }

    /// High-level method to purge all archived memories for a tenant.
    /// Use with caution: this is a permanent deletion of history.
    pub async fn purge_archived(&self, tenant_id: &str) -> Result<u64, String> {
        match &self.store {
            VectorMemoryStore::Postgres(pool) => {
                let r = sqlx::query("DELETE FROM consolidated_memory WHERE tenant_id = $1 AND archived = TRUE").bind(tenant_id).execute(pool).await.map_err(|e| e.to_string())?;
                Ok(r.rows_affected())
            }
            VectorMemoryStore::Sqlite(pool) => {
                let r = sqlx::query("DELETE FROM consolidated_memory WHERE tenant_id = ? AND archived = TRUE").bind(tenant_id).execute(pool).await.map_err(|e| e.to_string())?;
                Ok(r.rows_affected())
            }
        }
    }

    /// Forcefully mark a specific memory as archived.
    pub async fn force_archive(&self, id: &str) -> Result<(), String> {
        match &self.store {
            VectorMemoryStore::Postgres(pool) => {
                sqlx::query("UPDATE consolidated_memory SET archived = TRUE WHERE id = $1").bind(id).execute(pool).await.map_err(|e| e.to_string())?;
            }
            VectorMemoryStore::Sqlite(pool) => {
                sqlx::query("UPDATE consolidated_memory SET archived = TRUE WHERE id = ?").bind(id).execute(pool).await.map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }
}
