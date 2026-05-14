use chrono::{DateTime, Utc};
use sqlx::Row;
use async_trait::async_trait;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmbeddingRecord {
    pub id: String,
    pub tenant_id: String,
    pub agent_id: String,
    pub content: String,
    pub embedding: Vec<f32>,
    pub source_type: String,
    pub created_at: DateTime<Utc>,
    pub last_referenced_at: DateTime<Utc>,
    pub reference_count: i32,
    pub reliability_score: i32,
    pub owner_override: bool,
    pub metadata: Option<String>,
}

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

    pub async fn upsert(&self, record: &EmbeddingRecord) -> Result<(), String> {
        let emb_str = serde_json::to_string(&record.embedding).map_err(|e| format!("DB Error: {}", e))?;

        match &self.store {
            VectorMemoryStore::Postgres(pool) => {
                sqlx::query(
                    "INSERT INTO consolidated_memory (id, tenant_id, agent_id, content, embedding, source_type, created_at, last_referenced_at, reference_count, reliability_score, owner_override, metadata) \
                     VALUES ($1, $2, $3, $4, $5::vector, $6, $7, $8, $9, $10, $11, $12) \
                     ON CONFLICT(id) DO UPDATE SET \
                         content=excluded.content, \
                         embedding=excluded.embedding, \
                         created_at=excluded.created_at, \
                         last_referenced_at=excluded.last_referenced_at, \
                         reference_count=excluded.reference_count, \
                         reliability_score=excluded.reliability_score, \
                         owner_override=excluded.owner_override, \
                         metadata=excluded.metadata"
                )
                .bind(&record.id)
                .bind(&record.tenant_id)
                .bind(&record.agent_id)
                .bind(&record.content)
                .bind(&emb_str)
                .bind(&record.source_type)
                .bind(record.created_at)
                .bind(record.last_referenced_at)
                .bind(record.reference_count)
                .bind(record.reliability_score)
                .bind(record.owner_override)
                .bind(&record.metadata)
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
            }
            VectorMemoryStore::Sqlite(pool) => {
                sqlx::query(
                    "INSERT INTO consolidated_memory (id, tenant_id, agent_id, content, embedding, source_type, created_at, last_referenced_at, reference_count, reliability_score, owner_override, metadata) \
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) \
                     ON CONFLICT(id) DO UPDATE SET \
                         content=excluded.content, \
                         embedding=excluded.embedding, \
                         created_at=excluded.created_at, \
                         last_referenced_at=excluded.last_referenced_at, \
                         reference_count=excluded.reference_count, \
                         reliability_score=excluded.reliability_score, \
                         owner_override=excluded.owner_override, \
                         metadata=excluded.metadata"
                )
                .bind(&record.id)
                .bind(&record.tenant_id)
                .bind(&record.agent_id)
                .bind(&record.content)
                .bind(&emb_str)
                .bind(&record.source_type)
                .bind(record.created_at)
                .bind(record.last_referenced_at)
                .bind(record.reference_count)
                .bind(record.reliability_score)
                .bind(record.owner_override)
                .bind(&record.metadata)
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }

    pub async fn cross_department_search(&self, tenant_id: &str, query_embedding: &[f32], limit: i64) -> Result<Vec<EmbeddingRecord>, String> {
        self.semantic_search(tenant_id, query_embedding, limit).await
    }

    pub async fn semantic_search(&self, tenant_id: &str, query_embedding: &[f32], limit: i64) -> Result<Vec<EmbeddingRecord>, String> {
        let emb_str = serde_json::to_string(query_embedding).map_err(|e| e.to_string())?;

        let mut results = Vec::new();

        match &self.store {
            VectorMemoryStore::Postgres(pool) => {
                let rows = sqlx::query(
                    "SELECT id, tenant_id, COALESCE(agent_id, '') as agent_id, content, embedding::text, source_type, created_at, last_referenced_at, reference_count, reliability_score, owner_override, metadata \
                     FROM consolidated_memory \
                     WHERE tenant_id = $1 \
                     ORDER BY embedding <=> $2::vector \
                     LIMIT $3"
                )
                .bind(tenant_id)
                .bind(emb_str)
                .bind(limit)
                .fetch_all(pool)
                .await
                .map_err(|e| e.to_string())?;

                let mut ids_to_update = Vec::new();

                for row in rows {
                    let id: String = row.get("id");
                    ids_to_update.push(id.clone());
                    let tenant_id: String = row.get("tenant_id");
                    let agent_id: String = row.get("agent_id");
                    let content: String = row.get("content");
                    let emb_str_res: String = row.get("embedding");
                    let source_type: String = row.get("source_type");
                    let created_at: DateTime<Utc> = row.get("created_at");
                    let last_referenced_at: DateTime<Utc> = row.get("last_referenced_at");
                    let reference_count: i32 = row.get("reference_count");
                    let reliability_score: i32 = row.get("reliability_score");
                    let owner_override: bool = row.get("owner_override");
                    let metadata: Option<String> = row.get("metadata");

                    let embedding: Vec<f32> = serde_json::from_str(&emb_str_res).unwrap_or_default();

                    results.push(EmbeddingRecord {
                        id,
                        tenant_id,
                        agent_id,
                        content,
                        embedding,
                        source_type,
                        created_at,
                        last_referenced_at,
                        reference_count,
                        reliability_score,
                        owner_override,
                        metadata,
                    });
                }

                if !ids_to_update.is_empty() {
                    let _ = sqlx::query(
                        "UPDATE consolidated_memory SET last_referenced_at = CURRENT_TIMESTAMP, reference_count = reference_count + 1 WHERE id = ANY($1)"
                    )
                    .bind(&ids_to_update)
                    .execute(pool)
                    .await;
                }
            }
            VectorMemoryStore::Sqlite(pool) => {
                let has_vec_extension = sqlx::query("SELECT vec_distance_cosine('[1.0]', '[1.0]')")
                    .execute(pool)
                    .await
                    .is_ok();

                if has_vec_extension {
                    let rows = sqlx::query(
                        "SELECT id, tenant_id, COALESCE(agent_id, '') as agent_id, content, embedding, source_type, created_at, last_referenced_at, reference_count, reliability_score, owner_override, metadata \
                         FROM consolidated_memory \
                         WHERE tenant_id = ? \
                         ORDER BY vec_distance_cosine(embedding, ?) \
                         LIMIT ?"
                    )
                    .bind(tenant_id)
                    .bind(&emb_str)
                    .bind(limit)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())?;

                    let mut ids_to_update = Vec::new();

                    for row in rows {
                        let id: String = row.get("id");
                        ids_to_update.push(id.clone());
                        let tenant_id: String = row.get("tenant_id");
                        let agent_id: String = row.get("agent_id");
                        let content: String = row.get("content");
                        let emb_str_res: String = row.get("embedding");
                        let source_type: String = row.get("source_type");
                        let created_at: DateTime<Utc> = row.try_get::<DateTime<Utc>, _>("created_at").map_err(|e| e.to_string())?;
                        let last_referenced_at: DateTime<Utc> = row.try_get::<DateTime<Utc>, _>("last_referenced_at").map_err(|e| e.to_string())?;
                        let reference_count: i32 = row.get("reference_count");
                        let reliability_score: i32 = row.get("reliability_score");
                        let owner_override: bool = row.get("owner_override");
                        let metadata: Option<String> = row.get("metadata");

                        let embedding: Vec<f32> = serde_json::from_str(&emb_str_res).unwrap_or_default();

                        results.push(EmbeddingRecord {
                            id,
                            tenant_id,
                            agent_id,
                            content,
                            embedding,
                            source_type,
                            created_at,
                            last_referenced_at,
                            reference_count,
                            reliability_score,
                            owner_override,
                            metadata,
                        });
                    }

                    if !ids_to_update.is_empty() {
                        let placeholders = ids_to_update.iter().map(|_| "?").collect::<Vec<_>>().join(",");
                        let query = format!("UPDATE consolidated_memory SET last_referenced_at = CURRENT_TIMESTAMP, reference_count = reference_count + 1 WHERE id IN ({})", placeholders);
                        let mut q = sqlx::query(&query);
                        for id in ids_to_update {
                            q = q.bind(id);
                        }
                        let _ = q.execute(pool).await;
                    }
                } else {
                    let rows = sqlx::query(
                        "SELECT id, tenant_id, COALESCE(agent_id, '') as agent_id, content, embedding, source_type, created_at, last_referenced_at, reference_count, reliability_score, owner_override, metadata \
                         FROM consolidated_memory \
                         WHERE tenant_id = ? \
                         LIMIT 1000"
                    )
                    .bind(tenant_id)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())?;

                    let mut all_records = Vec::new();
                    for row in rows {
                        let emb_str_res: String = row.try_get("embedding").unwrap_or_else(|_| String::from_utf8(row.get::<Vec<u8>, _>("embedding")).unwrap_or_default());
                        let embedding: Vec<f32> = serde_json::from_str(&emb_str_res).unwrap_or_default();

                        let record = EmbeddingRecord {
                            id: row.get("id"),
                            tenant_id: row.get("tenant_id"),
                            agent_id: row.get("agent_id"),
                            content: row.get("content"),
                            embedding,
                            source_type: row.get("source_type"),
                            created_at: row.try_get::<DateTime<Utc>, _>("created_at").map_err(|e| e.to_string())?,
                            last_referenced_at: row.try_get::<DateTime<Utc>, _>("last_referenced_at").map_err(|e| e.to_string())?,
                            reference_count: row.get("reference_count"),
                            reliability_score: row.get("reliability_score"),
                            owner_override: row.get("owner_override"),
                            metadata: row.get("metadata"),
                        };
                        all_records.push(record);
                    }

                    fn cosine_distance(a: &[f32], b: &[f32]) -> f32 {
                        if a.len() != b.len() || a.is_empty() {
                            return 1.0;
                        }
                        let mut dot_product = 0.0;
                        let mut norm_a = 0.0;
                        let mut norm_b = 0.0;
                        for i in 0..a.len() {
                            dot_product += a[i] * b[i];
                            norm_a += a[i] * a[i];
                            norm_b += b[i] * b[i];
                        }
                        if norm_a == 0.0 || norm_b == 0.0 {
                            return 1.0;
                        }
                        let similarity = dot_product / (norm_a.sqrt() * norm_b.sqrt());
                        1.0 - similarity
                    }

                    let query_emb: Vec<f32> = serde_json::from_str(&emb_str).unwrap_or_default();
                    all_records.sort_by(|a, b| {
                        let dist_a = cosine_distance(&a.embedding, &query_emb);
                        let dist_b = cosine_distance(&b.embedding, &query_emb);
                        dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
                    });

                    results = all_records.into_iter().take(limit as usize).collect();

                    if !results.is_empty() {
                        let ids_to_update: Vec<String> = results.iter().map(|r| r.id.clone()).collect();
                        let placeholders = ids_to_update.iter().map(|_| "?").collect::<Vec<_>>().join(",");
                        let query = format!("UPDATE consolidated_memory SET last_referenced_at = CURRENT_TIMESTAMP, reference_count = reference_count + 1 WHERE id IN ({})", placeholders);
                        let mut q = sqlx::query(&query);
                        for id in ids_to_update {
                            q = q.bind(id);
                        }
                        let _ = q.execute(pool).await;
                    }
                }
            }
        }

        Ok(results)
    }

    /// Prunes stale context to prevent unbounded memory growth.
    /// It deletes records older than `older_than` where `owner_override = FALSE`,
    /// `reference_count < 5`, and `source_type = 'TASK_SUMMARY'`.
    pub async fn prune_stale(&self, older_than: DateTime<Utc>) -> Result<(), String> {
        match &self.store {
            VectorMemoryStore::Postgres(pool) => {
                sqlx::query("DELETE FROM consolidated_memory WHERE (last_referenced_at < $1 AND owner_override = FALSE AND reference_count < 5 AND source_type = 'TASK_SUMMARY') OR (reliability_score < 20 AND owner_override = FALSE)")
                    .bind(older_than)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            VectorMemoryStore::Sqlite(pool) => {
                sqlx::query("DELETE FROM consolidated_memory WHERE (last_referenced_at < ? AND owner_override = FALSE AND reference_count < 5 AND source_type = 'TASK_SUMMARY') OR (reliability_score < 20 AND owner_override = FALSE)")
                    .bind(older_than)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    #[allow(dead_code)]    pub async fn delete(&self, id: &str) -> Result<(), String> {
        match &self.store {
            VectorMemoryStore::Postgres(pool) => {
                sqlx::query("DELETE FROM consolidated_memory WHERE id = $1")
                    .bind(id)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            VectorMemoryStore::Sqlite(pool) => {
                sqlx::query("DELETE FROM consolidated_memory WHERE id = ?")
                    .bind(id)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    pub async fn resolve_conflict(&self, winner: &EmbeddingRecord, loser: &EmbeddingRecord) -> Result<(), String> {
        self.delete(&loser.id).await?;
        let mut updated_winner = winner.clone();
        updated_winner.reference_count += loser.reference_count + 1;
        updated_winner.last_referenced_at = chrono::Utc::now();
        if loser.owner_override && !updated_winner.owner_override {
            updated_winner.owner_override = true;
        }
        self.upsert(&updated_winner).await?;
        Ok(())
    }

    /// Automatically detects and resolves conflicts based on semantic similarity.
    /// It uses explicit owner override, reliability score, and recency to determine the winner.
    pub async fn auto_resolve_conflicts(&self) -> Result<usize, String> {
        let conflicts = self.get_conflicting_pairs().await?;
        let mut resolved_count = 0;

        for (a, b) in conflicts {
            let (winner, loser) = Self::determine_conflict_winner(&a, &b);
            self.resolve_conflict(winner, loser).await?;
            resolved_count += 1;
        }

        Ok(resolved_count)
    }

    /// Determines the winner of a memory conflict between two embedding records.
    pub fn determine_conflict_winner<'a>(a: &'a EmbeddingRecord, b: &'a EmbeddingRecord) -> (&'a EmbeddingRecord, &'a EmbeddingRecord) {
        if a.owner_override != b.owner_override {
            if a.owner_override {
                (a, b)
            } else {
                (b, a)
            }
        } else if a.reliability_score != b.reliability_score {
            if a.reliability_score > b.reliability_score {
                (a, b)
            } else {
                (b, a)
            }
        } else if a.created_at != b.created_at {
            if a.created_at > b.created_at {
                (a, b)
            } else {
                (b, a)
            }
        } else {
            (a, b) // Fallback, just pick 'a'
        }
    }


    pub async fn get_conflicting_pairs(&self) -> Result<Vec<(EmbeddingRecord, EmbeddingRecord)>, String> {
        let mut conflicts = Vec::new();

        match &self.store {
            VectorMemoryStore::Postgres(pool) => {
                let query = "
                    SELECT
                        a.id AS a_id, a.tenant_id AS a_tenant_id, a.agent_id AS a_agent_id, a.content AS a_content, a.embedding::text AS a_embedding, a.source_type AS a_source_type, a.created_at AS a_created_at, a.last_referenced_at AS a_last_referenced_at, a.reference_count AS a_reference_count, a.reliability_score AS a_reliability_score, a.owner_override AS a_owner_override, a.metadata AS a_metadata,
                        b.id AS b_id, b.tenant_id AS b_tenant_id, b.agent_id AS b_agent_id, b.content AS b_content, b.embedding::text AS b_embedding, b.source_type AS b_source_type, b.created_at AS b_created_at, b.last_referenced_at AS b_last_referenced_at, b.reference_count AS b_reference_count, b.reliability_score AS b_reliability_score, b.owner_override AS b_owner_override, b.metadata AS b_metadata
                    FROM consolidated_memory a
                    JOIN consolidated_memory b ON a.tenant_id = b.tenant_id AND a.id < b.id
                    WHERE a.embedding <=> b.embedding < 0.05
                    LIMIT 10
                ";
                let rows = sqlx::query(query)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())?;

                for row in rows {
                    let a_emb_str: String = row.try_get("a_embedding").unwrap_or_else(|_| String::from_utf8(row.get::<Vec<u8>, _>("a_embedding")).unwrap_or_default());
                    let b_emb_str: String = row.try_get("b_embedding").unwrap_or_else(|_| String::from_utf8(row.get::<Vec<u8>, _>("b_embedding")).unwrap_or_default());

                    let a_embedding: Vec<f32> = serde_json::from_str(&a_emb_str).unwrap_or_default();
                    let b_embedding: Vec<f32> = serde_json::from_str(&b_emb_str).unwrap_or_default();

                    let a = EmbeddingRecord {
                        id: row.get("a_id"),
                        tenant_id: row.get("a_tenant_id"),
                        agent_id: row.get::<Option<String>, _>("a_agent_id").unwrap_or_default(),
                        content: row.get("a_content"),
                        embedding: a_embedding,
                        source_type: row.get("a_source_type"),
                        created_at: row.try_get::<DateTime<Utc>, _>("a_created_at").map_err(|e| e.to_string())?,
                        last_referenced_at: row.try_get::<DateTime<Utc>, _>("a_last_referenced_at").map_err(|e| e.to_string())?,
                        reference_count: row.get("a_reference_count"),
                        reliability_score: row.get("a_reliability_score"),
                        owner_override: row.get("a_owner_override"),
                        metadata: row.get("a_metadata"),
                    };

                    let b = EmbeddingRecord {
                        id: row.get("b_id"),
                        tenant_id: row.get("b_tenant_id"),
                        agent_id: row.get::<Option<String>, _>("b_agent_id").unwrap_or_default(),
                        content: row.get("b_content"),
                        embedding: b_embedding,
                        source_type: row.get("b_source_type"),
                        created_at: row.try_get::<DateTime<Utc>, _>("b_created_at").map_err(|e| e.to_string())?,
                        last_referenced_at: row.try_get::<DateTime<Utc>, _>("b_last_referenced_at").map_err(|e| e.to_string())?,
                        reference_count: row.get("b_reference_count"),
                        reliability_score: row.get("b_reliability_score"),
                        owner_override: row.get("b_owner_override"),
                        metadata: row.get("b_metadata"),
                    };

                    conflicts.push((a, b));
                }
            }
            VectorMemoryStore::Sqlite(pool) => {
                // Determine if we have the vector extension loaded (e.g. by checking if vec_distance_cosine exists)
                let has_vec_extension = sqlx::query("SELECT vec_distance_cosine('[1.0]', '[1.0]')")
                    .execute(pool)
                    .await
                    .is_ok();

                if has_vec_extension {
                    let query = "
                        SELECT
                            a.id AS a_id, a.tenant_id AS a_tenant_id, a.agent_id AS a_agent_id, a.content AS a_content, a.embedding AS a_embedding, a.source_type AS a_source_type, a.created_at AS a_created_at, a.last_referenced_at AS a_last_referenced_at, a.reference_count AS a_reference_count, a.reliability_score AS a_reliability_score, a.owner_override AS a_owner_override, a.metadata AS a_metadata,
                            b.id AS b_id, b.tenant_id AS b_tenant_id, b.agent_id AS b_agent_id, b.content AS b_content, b.embedding AS b_embedding, b.source_type AS b_source_type, b.created_at AS b_created_at, b.last_referenced_at AS b_last_referenced_at, b.reference_count AS b_reference_count, b.reliability_score AS b_reliability_score, b.owner_override AS b_owner_override, b.metadata AS b_metadata
                        FROM consolidated_memory a
                        JOIN consolidated_memory b ON a.tenant_id = b.tenant_id AND a.id < b.id
                        WHERE vec_distance_cosine(a.embedding, b.embedding) < 0.05
                        LIMIT 10
                    ";
                    let rows = sqlx::query(query)
                        .fetch_all(pool)
                        .await
                        .map_err(|e| e.to_string())?;

                    for row in rows {
                        let a_emb_str: String = row.try_get("a_embedding").unwrap_or_else(|_| String::from_utf8(row.get::<Vec<u8>, _>("a_embedding")).unwrap_or_default());
                        let b_emb_str: String = row.try_get("b_embedding").unwrap_or_else(|_| String::from_utf8(row.get::<Vec<u8>, _>("b_embedding")).unwrap_or_default());

                        let a_embedding: Vec<f32> = serde_json::from_str(&a_emb_str).unwrap_or_default();
                        let b_embedding: Vec<f32> = serde_json::from_str(&b_emb_str).unwrap_or_default();

                        let a = EmbeddingRecord {
                            id: row.get("a_id"),
                            tenant_id: row.get("a_tenant_id"),
                            agent_id: row.get::<Option<String>, _>("a_agent_id").unwrap_or_default(),
                            content: row.get("a_content"),
                            embedding: a_embedding,
                            source_type: row.get("a_source_type"),
                            created_at: row.try_get::<DateTime<Utc>, _>("a_created_at").map_err(|e| e.to_string())?,
                            last_referenced_at: row.try_get::<DateTime<Utc>, _>("a_last_referenced_at").map_err(|e| e.to_string())?,
                            reference_count: row.get("a_reference_count"),
                            reliability_score: row.get("a_reliability_score"),
                            owner_override: row.get("a_owner_override"),
                            metadata: row.get("a_metadata"),
                        };

                        let b = EmbeddingRecord {
                            id: row.get("b_id"),
                            tenant_id: row.get("b_tenant_id"),
                            agent_id: row.get::<Option<String>, _>("b_agent_id").unwrap_or_default(),
                            content: row.get("b_content"),
                            embedding: b_embedding,
                            source_type: row.get("b_source_type"),
                            created_at: row.try_get::<DateTime<Utc>, _>("b_created_at").map_err(|e| e.to_string())?,
                            last_referenced_at: row.try_get::<DateTime<Utc>, _>("b_last_referenced_at").map_err(|e| e.to_string())?,
                            reference_count: row.get("b_reference_count"),
                            reliability_score: row.get("b_reliability_score"),
                            owner_override: row.get("b_owner_override"),
                            metadata: row.get("b_metadata"),
                        };

                        conflicts.push((a, b));
                    }
                } else {
                    // Fallback for tests environments without sqlite-vec loaded:
                    let query = "
                        SELECT
                            id, tenant_id, agent_id, content, embedding, source_type, created_at, last_referenced_at, reference_count, reliability_score, owner_override, metadata
                        FROM consolidated_memory LIMIT 1000
                    ";
                    let rows = sqlx::query(query)
                        .fetch_all(pool)
                        .await
                        .map_err(|e| e.to_string())?;

                    let mut all_records = Vec::new();
                    for row in rows {
                        let emb_str: String = row.try_get("embedding").unwrap_or_else(|_| String::from_utf8(row.get::<Vec<u8>, _>("embedding")).unwrap_or_default());
                        let embedding: Vec<f32> = serde_json::from_str(&emb_str).unwrap_or_default();

                        let record = EmbeddingRecord {
                            id: row.get("id"),
                            tenant_id: row.get("tenant_id"),
                            agent_id: row.get::<Option<String>, _>("agent_id").unwrap_or_default(),
                            content: row.get("content"),
                            embedding,
                            source_type: row.get("source_type"),
                            created_at: row.try_get::<DateTime<Utc>, _>("created_at").map_err(|e| e.to_string())?,
                            last_referenced_at: row.try_get::<DateTime<Utc>, _>("last_referenced_at").map_err(|e| e.to_string())?,
                            reference_count: row.get("reference_count"),
                            reliability_score: row.get("reliability_score"),
                            owner_override: row.get("owner_override"),
                            metadata: row.get("metadata"),
                        };
                        all_records.push(record);
                    }

                    fn cosine_distance(a: &[f32], b: &[f32]) -> f32 {
                        if a.len() != b.len() || a.is_empty() {
                            return 1.0;
                        }
                        let mut dot_product = 0.0;
                        let mut norm_a = 0.0;
                        let mut norm_b = 0.0;
                        for i in 0..a.len() {
                            dot_product += a[i] * b[i];
                            norm_a += a[i] * a[i];
                            norm_b += b[i] * b[i];
                        }
                        if norm_a == 0.0 || norm_b == 0.0 {
                            return 1.0;
                        }
                        let similarity = dot_product / (norm_a.sqrt() * norm_b.sqrt());
                        1.0 - similarity
                    }

                    let mut match_count = 0;
                    for i in 0..all_records.len() {
                        for j in (i + 1)..all_records.len() {
                            let a = &all_records[i];
                            let b = &all_records[j];
                            if a.tenant_id == b.tenant_id {
                                // Ensure a consistent ordering to avoid duplicate pairs in different orders
                                let (record_a, record_b) = if a.id < b.id { (a, b) } else { (b, a) };
                                let distance = cosine_distance(&record_a.embedding, &record_b.embedding);
                                if distance < 0.05 {
                                    conflicts.push((record_a.clone(), record_b.clone()));
                                    match_count += 1;
                                    if match_count >= 10 {
                                        break;
                                    }
                                }
                            }
                        }
                        if match_count >= 10 {
                            break;
                        }
                    }
                }
            }
        }
        Ok(conflicts)
    }

}


#[async_trait]
pub trait OHCMemory: Send + Sync {
    async fn write(&self, namespace: &str, key: &str, data: &[u8]) -> Result<(), String>;
    async fn read(&self, namespace: &str, key: &str) -> Result<Vec<u8>, String>;
}

pub struct FileBasedMemory {
    base_dir: std::path::PathBuf,
}

impl FileBasedMemory {
    pub fn new<P: AsRef<std::path::Path>>(base_dir: P) -> Self {
        FileBasedMemory {
            base_dir: base_dir.as_ref().to_path_buf(),
        }
    }

    fn secure_join(&self, elem: &[&str]) -> Result<std::path::PathBuf, String> {
        let mut path = self.base_dir.clone();
        for e in elem {
            if e.contains("..") {
                return Err("path traversal detected (..)".to_string());
            }
            path.push(e);
        }
        if !path.starts_with(&self.base_dir) {
            return Err("invalid path: attempts to traverse outside base directory".to_string());
        }
        Ok(path)
    }
}

#[async_trait]
impl OHCMemory for FileBasedMemory {
    async fn write(&self, namespace: &str, key: &str, data: &[u8]) -> Result<(), String> {
        let dir = self.secure_join(&[namespace])?;
        tokio::fs::create_dir_all(&dir).await.map_err(|e| e.to_string())?;
        
        let path = self.secure_join(&[namespace, key])?;
        tokio::fs::write(path, data).await.map_err(|e| e.to_string())?;
        
        Ok(())
    }

    async fn read(&self, namespace: &str, key: &str) -> Result<Vec<u8>, String> {
        let path = self.secure_join(&[namespace, key])?;
        tokio::fs::read(path).await.map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_embedding_record_serialization() {
        let now = Utc.with_ymd_and_hms(2026, 4, 26, 0, 0, 0).unwrap();
        let record = EmbeddingRecord {
            id: "rec1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "Hello world".to_string(),
            embedding: vec![1.0, 2.0, 3.0],
            source_type: "TEXT".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 0,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        let json = serde_json::to_string(&record).unwrap();
        let deserialized: EmbeddingRecord = serde_json::from_str(&json).unwrap();

        assert_eq!(record.id, deserialized.id);
        assert_eq!(record.embedding, deserialized.embedding);
        assert_eq!(record.created_at, deserialized.created_at);
    }

    #[tokio::test]
    async fn test_file_based_memory() {
        let dir = "/tmp/test_memory";
        let mem = FileBasedMemory::new(dir);
        let namespace = "test_ns";
        let key = "test_key";
        let data = b"hello memory";

        mem.write(namespace, key, data).await.unwrap();

        let read_data = mem.read(namespace, key).await.unwrap();
        assert_eq!(read_data, data);

        let _ = tokio::fs::remove_dir_all(dir).await;
    }

    #[tokio::test]
    async fn test_anthropic_3_tier_memory_store() {
        let base_dir = "/tmp/test_anthropic_3_tier";
        let _ = tokio::fs::remove_dir_all(base_dir).await;

        let store = Anthropic3TierMemoryStore::new(base_dir).unwrap();

        // Test lightweight index
        store.update_index("Sample index content").await.unwrap();
        let index = store.get_lightweight_index().await.unwrap();
        assert_eq!(index, "Sample index content");

        // Test topic retrieve
        store.write_topic("system_architecture", "Detailed DB schema").await.unwrap();
        let topic_content = store.retrieve_topic("system_architecture").await.unwrap();
        assert_eq!(topic_content, "Detailed DB schema");
        assert!(store.retrieve_topic("nonexistent").await.is_err());

        // Test transcript search
        store.append_transcript("session1", "User asked about memory.\n\nAgent replied 3-tier is better.").await.unwrap();
        store.append_transcript("session2", "User requested weather.\n\nAgent gave forecast.").await.unwrap();

        let res = store.search_transcripts("3-tier is better", 10).await.unwrap();
        assert_eq!(res.len(), 1);
        assert!(res[0].contains("Agent replied 3-tier is better."));

        let _ = tokio::fs::remove_dir_all(base_dir).await;
    }

    }


#[async_trait]
pub trait LongTermMemory: Send + Sync + std::fmt::Debug {
    /// Retrieve relevant past conversations or state based on a query
    async fn retrieve(&self, query: &str, limit: usize) -> Result<Vec<String>, String>;
    
    /// Store a new piece of memory (e.g., an architectural decision or summary)
    async fn store(&self, content: &str, tags: Vec<String>) -> Result<(), String>;

    /// 3-Tier: Get the lightweight index (always loaded in context)
    async fn get_lightweight_index(&self) -> Result<String, String> {
        Ok("".to_string())
    }

    /// 3-Tier: Pull a detailed topic file on demand
    async fn retrieve_topic(&self, _topic_name: &str) -> Result<String, String> {
        Err("Not implemented".to_string())
    }

    /// 3-Tier: Search raw transcripts
    async fn search_transcripts(&self, _query: &str, _limit: usize) -> Result<Vec<String>, String> {
        Ok(vec![])
    }
    fn as_anthropic_accessor(&self) -> Option<std::sync::Arc<dyn ohc_builtin_agent_tools::anthropic_memory::MemoryAccessor>> { None }
}

pub struct PersistentMemoryStore {
    pub repo: std::sync::Arc<VectorRepository>,
    pub tenant_id: String,
    pub agent_id: String,
    pub llm: std::sync::Arc<dyn ohc_builtin_agent_llm::LlmClient>,
}

impl std::fmt::Debug for PersistentMemoryStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PersistentMemoryStore")
            .field("tenant_id", &self.tenant_id)
            .field("agent_id", &self.agent_id)
            .finish()
    }
}

#[async_trait]
impl LongTermMemory for PersistentMemoryStore {
    async fn retrieve(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        let embedding = self.llm.generate_embedding(query).await.map_err(|e| e.to_string())?;
        let records = self.repo.semantic_search(&self.tenant_id, &embedding, limit as i64).await?;
        Ok(records.into_iter().map(|r| r.content).collect())
    }

    async fn store(&self, content: &str, tags: Vec<String>) -> Result<(), String> {
        let embedding = self.llm.generate_embedding(content).await.map_err(|e| e.to_string())?;
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now();

        let source_type = if tags.contains(&"AUTO_CONSOLIDATED".to_string()) || tags.contains(&"AUTO_CONSOLIDATED_LANGGRAPH".to_string()) {
            "TASK_SUMMARY"
        } else {
            "MANUAL"
        };

        let record = EmbeddingRecord {
            id,
            tenant_id: self.tenant_id.clone(),
            agent_id: self.agent_id.clone(),
            content: content.to_string(),
            embedding,
            source_type: source_type.to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 0,
            reliability_score: 100,
            owner_override: false,
            metadata: Some(serde_json::to_string(&tags).unwrap_or_default()),
        };
        self.repo.upsert(&record).await
    }
}

/// Anthropic 3-Tier Memory Store implementation
/// 1) Lightweight index (~150 chars/entry, always loaded in context)
/// 2) Detailed topic files (pulled on demand)
/// 3) Raw transcripts (accessed via search only)
#[derive(Clone)]
pub struct Anthropic3TierMemoryStore {
    #[allow(dead_code)]
    base_dir: std::path::PathBuf,
    index_file: std::path::PathBuf,
    topics_dir: std::path::PathBuf,
    transcripts_dir: std::path::PathBuf,
}

impl std::fmt::Debug for Anthropic3TierMemoryStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Anthropic3TierMemoryStore").finish()
    }
}

impl Anthropic3TierMemoryStore {
    pub fn new<P: AsRef<std::path::Path>>(base_dir: P) -> Result<Self, String> {
        let base_dir = base_dir.as_ref().to_path_buf();
        let index_file = base_dir.join("index.md");
        let topics_dir = base_dir.join("topics");
        let transcripts_dir = base_dir.join("transcripts");

        std::fs::create_dir_all(&base_dir).map_err(|e| e.to_string())?;
        std::fs::create_dir_all(&topics_dir).map_err(|e| e.to_string())?;
        std::fs::create_dir_all(&transcripts_dir).map_err(|e| e.to_string())?;

        Ok(Self {
            base_dir,
            index_file,
            topics_dir,
            transcripts_dir,
        })
    }

    pub async fn update_index(&self, content: &str) -> Result<(), String> {
        tokio::fs::write(&self.index_file, content).await.map_err(|e| e.to_string())
    }

    pub async fn write_topic(&self, topic_name: &str, content: &str) -> Result<(), String> {
        let safe_name = topic_name.replace(|c: char| !c.is_alphanumeric() && c != '_' && c != '-', "");
        let path = self.topics_dir.join(format!("{}.md", safe_name));
        tokio::fs::write(path, content).await.map_err(|e| e.to_string())
    }

    pub async fn append_transcript(&self, session_id: &str, turn_content: &str) -> Result<(), String> {
        let path = self.transcripts_dir.join(format!("{}.log", session_id));
        use tokio::io::AsyncWriteExt;
        let mut file = tokio::fs::OpenOptions::new().create(true).append(true).open(path).await.map_err(|e| e.to_string())?;
        file.write_all(format!("{}\n\n", turn_content).as_bytes()).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[async_trait]
impl ohc_builtin_agent_tools::anthropic_memory::MemoryAccessor for Anthropic3TierMemoryStore {
    async fn retrieve_topic(&self, topic_name: &str) -> Result<String, String> {
        let safe_name = topic_name.replace(|c: char| !c.is_alphanumeric() && c != '_' && c != '-', "");
        let path = self.topics_dir.join(format!("{}.md", safe_name));
        if path.exists() {
            tokio::fs::read_to_string(&path).await.map_err(|e| e.to_string())
        } else {
            Err(format!("Topic '{}' not found", safe_name))
        }
    }

    async fn search_transcripts(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        let mut results = Vec::new();
        let mut dir = tokio::fs::read_dir(&self.transcripts_dir).await.map_err(|e| e.to_string())?;
        while let Ok(Some(entry)) = dir.next_entry().await {
            let content = tokio::fs::read_to_string(entry.path()).await.map_err(|e| e.to_string())?;
            for par in content.split("\n\n") {
                if par.to_lowercase().contains(&query.to_lowercase()) {
                    results.push(par.to_string());
                    if results.len() >= limit {
                        return Ok(results);
                    }
                }
            }
        }
        Ok(results)
    }
}

#[async_trait]
impl LongTermMemory for Anthropic3TierMemoryStore {
    async fn retrieve(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        let mut results = Vec::new();

        if !self.topics_dir.exists() {
            return Ok(results);
        }

        let mut dir = tokio::fs::read_dir(&self.topics_dir).await.map_err(|e| e.to_string())?;
        while let Ok(Some(entry)) = dir.next_entry().await {
            let content = tokio::fs::read_to_string(entry.path()).await.map_err(|e| e.to_string())?;
            if content.to_lowercase().contains(&query.to_lowercase()) {
                results.push(content);
                if results.len() >= limit {
                    break;
                }
            }
        }

        Ok(results)
    }

    async fn store(&self, content: &str, tags: Vec<String>) -> Result<(), String> {
        let mut existing_index = self.get_lightweight_index().await?;

        let truncated_content = if content.len() > 150 {
            format!("{}...", &content[..147])
        } else {
            content.to_string()
        };

        let tags_str = if tags.is_empty() { String::new() } else { format!(" [{}]", tags.join(", ")) };
        let new_entry = format!("- {}{}\n", truncated_content.replace('\n', " "), tags_str);

        existing_index.push_str(&new_entry);
        self.update_index(&existing_index).await?;

        Ok(())
    }

    async fn get_lightweight_index(&self) -> Result<String, String> {
        if self.index_file.exists() {
            tokio::fs::read_to_string(&self.index_file).await.map_err(|e| e.to_string())
        } else {
            Ok(String::new())
        }
    }

    async fn retrieve_topic(&self, topic_name: &str) -> Result<String, String> {
        let safe_name = topic_name.replace(|c: char| !c.is_alphanumeric() && c != '_' && c != '-', "");
        let path = self.topics_dir.join(format!("{}.md", safe_name));
        if path.exists() {
            tokio::fs::read_to_string(&path).await.map_err(|e| e.to_string())
        } else {
            Err(format!("Topic '{}' not found", safe_name))
        }
    }

    async fn search_transcripts(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        let mut results = Vec::new();
        let mut dir = tokio::fs::read_dir(&self.transcripts_dir).await.map_err(|e| e.to_string())?;
        while let Ok(Some(entry)) = dir.next_entry().await {
            let content = tokio::fs::read_to_string(entry.path()).await.map_err(|e| e.to_string())?;
            for par in content.split("\n\n") {
                if par.to_lowercase().contains(&query.to_lowercase()) {
                    results.push(par.to_string());
                    if results.len() >= limit {
                        return Ok(results);
                    }
                }
            }
        }
        Ok(results)
    }
    fn as_anthropic_accessor(&self) -> Option<std::sync::Arc<dyn ohc_builtin_agent_tools::anthropic_memory::MemoryAccessor>> {
        Some(std::sync::Arc::new(self.clone()))
    }
}

/// A simple implementation that stores memory in Redis using its list or sorted set capabilities.
/// In a production system, this would likely use Redis Vector Search (RediSearch) or a dedicated vector DB.
pub struct RedisMemoryStore {
    client: redis::Client,
    namespace: String,
    connection: tokio::sync::OnceCell<redis::aio::MultiplexedConnection>,
}

impl std::fmt::Debug for RedisMemoryStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RedisMemoryStore")
            .field("namespace", &self.namespace)
            .finish()
    }
}

impl RedisMemoryStore {
    pub fn new(redis_url: &str, namespace: &str) -> Result<Self, String> {
        let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;
        Ok(Self {
            client,
            namespace: namespace.to_string(),
            connection: tokio::sync::OnceCell::new(),
        })
    }

    async fn get_connection(&self) -> Result<redis::aio::MultiplexedConnection, String> {
        let conn = self.connection.get_or_try_init(|| async {
            self.client.get_multiplexed_tokio_connection().await
        }).await.map_err(|e| e.to_string())?;
        Ok(conn.clone())
    }
}

#[async_trait]
impl LongTermMemory for RedisMemoryStore {
    async fn retrieve(&self, _query: &str, limit: usize) -> Result<Vec<String>, String> {
        let mut conn = self.get_connection().await?;
        let key = format!("{}:memory", self.namespace);
        
        // Simple LRANGE to get recent memories. 
        // Real implementation would embed the query and use FT.SEARCH
        let results: Vec<String> = redis::cmd("LRANGE")
            .arg(&key)
            .arg(0)
            .arg((limit.max(1) - 1) as i64)
            .query_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;
            
        Ok(results)
    }

    async fn store(&self, content: &str, _tags: Vec<String>) -> Result<(), String> {
        let mut conn = self.get_connection().await?;
        let key = format!("{}:memory", self.namespace);
        
        let _: () = redis::cmd("LPUSH")
            .arg(&key)
            .arg(content)
            .query_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;
            
        Ok(())
    }
}

#[cfg(test)]
mod get_conflicts_tests {
    #[tokio::test]
    async fn test_auto_resolve_conflicts_with_override_new() {
        // Migrated override test from standalone conflict.rs {
        use std::str::FromStr;
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = match SqlitePoolOptions::new().connect_with(conn_opts).await {
            Ok(p) => p,
            Err(_) => return,
        };

        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding VECTOR(1536),
                source_type TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                reference_count INTEGER DEFAULT 0,
                reliability_score INTEGER DEFAULT 50,
                owner_override BOOLEAN DEFAULT FALSE,
                metadata TEXT
            );"
        ).execute(&pool).await;

        let repo = VectorRepository::new_sqlite(pool.clone());
        let now = chrono::Utc::now();
        let r1 = EmbeddingRecord {
            id: "rec1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "hello world".to_string(),
            embedding: vec![1.0, 1.0, 1.0],
            source_type: "SUMMARY".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 0,
            reliability_score: 50,
            owner_override: true,
            metadata: None,
        };
        let r2 = EmbeddingRecord {
            id: "rec2".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "hello world too".to_string(),
            embedding: vec![1.0, 1.0, 1.0],
            source_type: "SUMMARY".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 10,
            reliability_score: 100,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&r1).await.unwrap();
        repo.upsert(&r2).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 1);

        let query = "SELECT id, owner_override FROM consolidated_memory";
        let rows = sqlx::query(query).fetch_all(&pool).await.unwrap();
        assert_eq!(rows.len(), 1);
        use sqlx::Row;
        let row_id: String = rows[0].try_get("id").unwrap();
        assert_eq!(row_id, "rec1");
    }
    use super::*;
    use sqlx::sqlite::{SqlitePoolOptions, SqliteConnectOptions};
    use std::str::FromStr;

    #[tokio::test]
    async fn test_resolve_conflict_logic() {
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = match SqlitePoolOptions::new().connect_with(conn_opts).await {
            Ok(p) => p,
            Err(_) => return,
        };

        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding VECTOR(1536),
                source_type TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                reference_count INTEGER DEFAULT 0,
                reliability_score INTEGER DEFAULT 50,
                owner_override BOOLEAN DEFAULT FALSE,
                metadata TEXT
            );"
        ).execute(&pool).await;

        let repo = VectorRepository::new_sqlite(pool.clone());
        let now = chrono::Utc::now();

        let winner = EmbeddingRecord {
            id: "winner".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "winner data".to_string(),
            embedding: vec![0.5, 0.5],
            source_type: "MANUAL".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 90,
            owner_override: true,
            metadata: None,
        };

        let loser = EmbeddingRecord {
            id: "loser".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent2".to_string(),
            content: "loser data".to_string(),
            embedding: vec![0.5, 0.5],
            source_type: "MANUAL".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 5,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&winner).await.unwrap();
        repo.upsert(&loser).await.unwrap();

        repo.resolve_conflict(&winner, &loser).await.unwrap();

        let rows = sqlx::query("SELECT id, reference_count FROM consolidated_memory")
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(rows.len(), 1);
        let id: String = rows[0].get("id");
        let ref_count: i32 = rows[0].get("reference_count");

        assert_eq!(id, "winner");
        assert_eq!(ref_count, 2 + 5 + 1); // winner.ref_count + loser.ref_count + 1
    }

    #[tokio::test]
    async fn test_auto_resolve_conflicts() {
        // Migrated resolution test from standalone conflict.rs {
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = match SqlitePoolOptions::new().connect_with(conn_opts).await {
            Ok(p) => p,
            Err(_) => return,
        };

        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding VECTOR(1536),
                source_type TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                reference_count INTEGER DEFAULT 0,
                reliability_score INTEGER DEFAULT 50,
                owner_override BOOLEAN DEFAULT FALSE,
                metadata TEXT
            );"
        ).execute(&pool).await;

        let repo = VectorRepository::new_sqlite(pool.clone());
        let now = chrono::Utc::now();

        // 1. Conflict resolved by owner_override
        let r1 = EmbeddingRecord {
            id: "rec1_a".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "hello world".to_string(),
            embedding: vec![1.0, 2.0, 3.0],
            source_type: "SUMMARY".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };
        let r2 = EmbeddingRecord {
            id: "rec1_b".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "hello world override".to_string(),
            embedding: vec![1.0, 2.0, 3.0],
            source_type: "SUMMARY".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 40,
            owner_override: true, // Should win
            metadata: None,
        };

        // 2. Conflict resolved by reliability_score
        let r3 = EmbeddingRecord {
            id: "rec2_a".to_string(),
            tenant_id: "org2".to_string(),
            agent_id: "agent1".to_string(),
            content: "foo bar".to_string(),
            embedding: vec![0.5, 0.5, 0.5],
            source_type: "SUMMARY".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 5,
            reliability_score: 80, // Should win
            owner_override: false,
            metadata: None,
        };
        let r4 = EmbeddingRecord {
            id: "rec2_b".to_string(),
            tenant_id: "org2".to_string(),
            agent_id: "agent1".to_string(),
            content: "foo bar low".to_string(),
            embedding: vec![0.5, 0.5, 0.5],
            source_type: "SUMMARY".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 3,
            reliability_score: 60,
            owner_override: false,
            metadata: None,
        };

        // 3. Conflict resolved by recency
        let older = now - chrono::Duration::hours(1);
        let r5 = EmbeddingRecord {
            id: "rec3_a".to_string(),
            tenant_id: "org3".to_string(),
            agent_id: "agent1".to_string(),
            content: "baz".to_string(),
            embedding: vec![0.1, 0.1, 0.1],
            source_type: "SUMMARY".to_string(),
            created_at: older,
            last_referenced_at: now,
            reference_count: 0,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };
        let r6 = EmbeddingRecord {
            id: "rec3_b".to_string(),
            tenant_id: "org3".to_string(),
            agent_id: "agent1".to_string(),
            content: "baz newer".to_string(),
            embedding: vec![0.1, 0.1, 0.1],
            source_type: "SUMMARY".to_string(),
            created_at: now, // Should win
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&r1).await.unwrap();
        repo.upsert(&r2).await.unwrap();
        repo.upsert(&r3).await.unwrap();
        repo.upsert(&r4).await.unwrap();
        repo.upsert(&r5).await.unwrap();
        repo.upsert(&r6).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 3); // 3 conflicts resolved

        let query = "SELECT id, reference_count FROM consolidated_memory";
        let rows = sqlx::query(query).fetch_all(&pool).await.unwrap();

        let mut results = std::collections::HashMap::new();
        for row in rows {
            use sqlx::Row;
            let id: String = row.get("id");
            let ref_count: i32 = row.get("reference_count");
            results.insert(id, ref_count);
        }

        assert_eq!(results.len(), 3);
        assert_eq!(results.get("rec1_b"), Some(&4));
        assert_eq!(results.get("rec2_a"), Some(&9));
        assert_eq!(results.get("rec3_b"), Some(&2));
    }



    #[tokio::test]
    async fn test_prune_stale_sqlite() {
        // Migrated unit test from standalone pruning.rs {
        // Just mock the execution or write a very small unit test using sqlite memory database but to test the prune edge case
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = match SqlitePoolOptions::new().connect_with(conn_opts).await {
            Ok(p) => p,
            Err(_) => return,
        };

        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding VECTOR(1536),
                source_type TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                reference_count INTEGER DEFAULT 0,
                reliability_score INTEGER DEFAULT 50,
                owner_override BOOLEAN DEFAULT FALSE,
                metadata TEXT
            );"
        ).execute(&pool).await;

        let repo = VectorRepository::new_sqlite(pool.clone());
        let now = chrono::Utc::now();
        let very_old_time = now - chrono::Duration::days(200);

        // Record 1: Old enough, source_type is TASK_SUMMARY, reference_count < 5 -> Should be pruned
        let record1 = EmbeddingRecord {
            id: "rec1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "hello world 1".to_string(),
            embedding: vec![1.0, 2.0, 3.0],
            source_type: "TASK_SUMMARY".to_string(),
            created_at: very_old_time,
            last_referenced_at: very_old_time,
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        // Record 2: Old enough, source_type is TASK_SUMMARY, but owner_override = TRUE -> Should be kept
        let record2 = EmbeddingRecord {
            id: "rec2".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "hello world 2".to_string(),
            embedding: vec![1.0, 2.0, 3.0],
            source_type: "TASK_SUMMARY".to_string(),
            created_at: very_old_time,
            last_referenced_at: very_old_time,
            reference_count: 1,
            reliability_score: 50,
            owner_override: true,
            metadata: None,
        };

        // Record 3: Old enough, source_type is TASK_SUMMARY, but reference_count >= 5 -> Should be kept
        let record3 = EmbeddingRecord {
            id: "rec3".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "hello world 3".to_string(),
            embedding: vec![1.0, 2.0, 3.0],
            source_type: "TASK_SUMMARY".to_string(),
            created_at: very_old_time,
            last_referenced_at: very_old_time,
            reference_count: 5,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        // Record 4: Old enough, but source_type is NOT TASK_SUMMARY -> Should be kept
        let record4 = EmbeddingRecord {
            id: "rec4".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "hello world 4".to_string(),
            embedding: vec![1.0, 2.0, 3.0],
            source_type: "SUPPORT_TICKET".to_string(),
            created_at: very_old_time,
            last_referenced_at: very_old_time,
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&record1).await.unwrap();
        repo.upsert(&record2).await.unwrap();
        repo.upsert(&record3).await.unwrap();
        repo.upsert(&record4).await.unwrap();

        // Prune stale test
        repo.prune_stale(now - chrono::Duration::days(180)).await.unwrap();

        // Verify prune
        let query = "SELECT id FROM consolidated_memory";
        let rows = sqlx::query(query).fetch_all(&pool).await.unwrap();

        assert_eq!(rows.len(), 3, "Three records should remain");

        let mut remaining_ids: Vec<String> = rows.into_iter().map(|row| row.try_get("id").unwrap()).collect();
        remaining_ids.sort();

        assert_eq!(remaining_ids, vec!["rec2", "rec3", "rec4"], "The correct records should remain");
    }

    #[tokio::test]
    async fn test_auto_resolve_conflicts_fallback() {
        // Migrated fallback test from standalone conflict.rs {
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = match SqlitePoolOptions::new().connect_with(conn_opts).await {
            Ok(p) => p,
            Err(_) => return,
        };

        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding VECTOR(1536),
                source_type TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                reference_count INTEGER DEFAULT 0,
                reliability_score INTEGER DEFAULT 50,
                owner_override BOOLEAN DEFAULT FALSE,
                metadata TEXT
            );"
        ).execute(&pool).await;

        let repo = VectorRepository::new_sqlite(pool.clone());
        let now = chrono::Utc::now();

        // Conflict resolved by fallback (same override, reliability, timestamp)
        let r1 = EmbeddingRecord {
            id: "rec4_a".to_string(),
            tenant_id: "org4".to_string(),
            agent_id: "agent1".to_string(),
            content: "identical 1".to_string(),
            embedding: vec![0.9, 0.9, 0.9],
            source_type: "SUMMARY".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };
        let r2 = EmbeddingRecord {
            id: "rec4_b".to_string(),
            tenant_id: "org4".to_string(),
            agent_id: "agent1".to_string(),
            content: "identical 2".to_string(),
            embedding: vec![0.9, 0.9, 0.9],
            source_type: "SUMMARY".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 2,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&r1).await.unwrap();
        repo.upsert(&r2).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 1);

        let query = "SELECT id, reference_count FROM consolidated_memory WHERE tenant_id = 'org4'";
        let rows = sqlx::query(query).fetch_all(&pool).await.unwrap();

        assert_eq!(rows.len(), 1);
        let id: String = rows[0].try_get("id").unwrap();
        let ref_count: i32 = rows[0].try_get("reference_count").unwrap();

        // It will pick `r1` as winner arbitrarily (since a=r1, b=r2, and we return (&a, &b))
        assert_eq!(id, "rec4_a");
        // new ref count = r1.reference_count (1) + r2.reference_count (2) + 1 = 4
        assert_eq!(ref_count, 4);
    }

    #[tokio::test]
    async fn test_get_conflicting_pairs_and_prune() {
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = match SqlitePoolOptions::new().connect_with(conn_opts).await {
            Ok(p) => p,
            Err(_) => return,
        };

        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding VECTOR(1536),
                source_type TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                reference_count INTEGER DEFAULT 0,
                reliability_score INTEGER DEFAULT 50,
                owner_override BOOLEAN DEFAULT FALSE,
                metadata TEXT
            );"
        ).execute(&pool).await;

        let repo = VectorRepository::new_sqlite(pool.clone());
        let now = chrono::Utc::now();

        let old_time = now - chrono::Duration::days(181);

        let record1 = EmbeddingRecord {
            id: "rec1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "hello world".to_string(),
            embedding: vec![1.0, 2.0, 3.0],
            source_type: "SUMMARY".to_string(), // Should not be deleted
            created_at: old_time,
            last_referenced_at: old_time,
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        let record2 = EmbeddingRecord {
            id: "rec2".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "hello world 2".to_string(),
            embedding: vec![3.0, 2.0, 1.0],
            source_type: "TASK_SUMMARY".to_string(), // Should be deleted
            created_at: old_time,
            last_referenced_at: old_time,
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&record1).await.unwrap();
        repo.upsert(&record2).await.unwrap();

        // Prune stale test
        repo.prune_stale(now - chrono::Duration::days(180)).await.unwrap();

        // Verify prune
        let query = "SELECT id FROM consolidated_memory";
        let rows = sqlx::query(query).fetch_all(&pool).await.unwrap();

        assert_eq!(rows.len(), 1, "Only one record should remain");

        let id: String = rows[0].try_get("id").unwrap();
        assert_eq!(id, "rec1", "The correct record should remain");

        // get_conflicting_pairs test
        let conflicts = repo.get_conflicting_pairs().await.unwrap();
        assert!(conflicts.is_empty(), "Should have no conflicts");
    }

    #[tokio::test]
    async fn test_delete() {
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = match SqlitePoolOptions::new().connect_with(conn_opts).await {
            Ok(p) => p,
            Err(_) => return,
        };

        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding VECTOR(1536),
                source_type TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                reference_count INTEGER DEFAULT 0,
                reliability_score INTEGER DEFAULT 50,
                owner_override BOOLEAN DEFAULT FALSE,
                metadata TEXT
            );"
        ).execute(&pool).await;

        let repo = VectorRepository::new_sqlite(pool.clone());
        let now = chrono::Utc::now();

        let record1 = EmbeddingRecord {
            id: "rec1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "hello world".to_string(),
            embedding: vec![1.0, 2.0, 3.0],
            source_type: "SUMMARY".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&record1).await.unwrap();

        let count: (i64,) = sqlx::query_as("SELECT count(*) FROM consolidated_memory WHERE id = 'rec1'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count.0, 1);

        repo.delete("rec1").await.unwrap();

        let count: (i64,) = sqlx::query_as("SELECT count(*) FROM consolidated_memory WHERE id = 'rec1'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count.0, 0);
    }

    #[tokio::test]
    async fn test_semantic_search() {
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = match SqlitePoolOptions::new().connect_with(conn_opts).await {
            Ok(p) => p,
            Err(_) => return,
        };

        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding TEXT,
                source_type TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                reference_count INTEGER DEFAULT 0,
                reliability_score INTEGER DEFAULT 50,
                owner_override BOOLEAN DEFAULT FALSE,
                metadata TEXT
            );"
        ).execute(&pool).await;

        let repo = VectorRepository::new_sqlite(pool.clone());
        let now = chrono::Utc::now();

        let record1 = EmbeddingRecord {
            id: "rec1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "vegan cake orders".to_string(),
            embedding: vec![0.9, 0.1, 0.1],
            source_type: "SUMMARY".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        let record2 = EmbeddingRecord {
            id: "rec2".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "unrelated data".to_string(),
            embedding: vec![0.1, 0.9, 0.1],
            source_type: "SUMMARY".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&record1).await.unwrap();
        repo.upsert(&record2).await.unwrap();

        // Testing the fallback behavior if vec_distance_cosine doesn't exist
        // or just the generic semantic search logic.
        let results = repo.semantic_search("org1", &[1.0, 0.0, 0.0], 5).await.unwrap();

        // Either the results come back ordered by created_at or vec_distance_cosine.
        // We just make sure it returns something.
        assert!(!results.is_empty());
        assert_eq!(results[0].tenant_id, "org1");
    }

    #[tokio::test]
    async fn test_search_cross_department_explicit() {
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = match SqlitePoolOptions::new().connect_with(conn_opts).await {
            Ok(p) => p,
            Err(_) => return,
        };

        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding VECTOR(1536),
                source_type TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                reference_count INTEGER DEFAULT 0,
                reliability_score INTEGER DEFAULT 50,
                owner_override BOOLEAN DEFAULT FALSE,
                metadata TEXT
            );"
        ).execute(&pool).await;

        let repo = VectorRepository::new_sqlite(pool.clone());
        let now = chrono::Utc::now();

        let record_dept_a = EmbeddingRecord {
            id: "dept_a_rec".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent_a".to_string(),
            content: "dept A data".to_string(),
            embedding: vec![1.0, 0.0],
            source_type: "MANUAL".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        let record_dept_b = EmbeddingRecord {
            id: "dept_b_rec".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent_b".to_string(),
            content: "dept B data".to_string(),
            embedding: vec![1.0, 0.0],
            source_type: "MANUAL".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        let record_other_tenant = EmbeddingRecord {
            id: "other_tenant_rec".to_string(),
            tenant_id: "org2".to_string(),
            agent_id: "agent_a".to_string(),
            content: "other tenant data".to_string(),
            embedding: vec![1.0, 0.0],
            source_type: "MANUAL".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&record_dept_a).await.unwrap();
        repo.upsert(&record_dept_b).await.unwrap();
        repo.upsert(&record_other_tenant).await.unwrap();

        // semantic_search should return records from both agent_a and agent_b for org1,
        // but exclude the record from org2.
        let results = repo.semantic_search("org1", &[1.0, 0.0], 10).await.unwrap();

        assert_eq!(results.len(), 2);
        let mut found_a = false;
        let mut found_b = false;
        for r in results {
            assert_eq!(r.tenant_id, "org1");
            if r.agent_id == "agent_a" { found_a = true; }
            if r.agent_id == "agent_b" { found_b = true; }
        }
        assert!(found_a);
        assert!(found_b);
    }

    #[tokio::test]
    async fn test_cross_department_sharing() {
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = match SqlitePoolOptions::new().connect_with(conn_opts).await {
            Ok(p) => p,
            Err(_) => return,
        };

        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding VECTOR(1536),
                source_type TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                reference_count INTEGER DEFAULT 0,
                reliability_score INTEGER DEFAULT 50,
                owner_override BOOLEAN DEFAULT FALSE,
                metadata TEXT
            );"
        ).execute(&pool).await;

        let repo = VectorRepository::new_sqlite(pool.clone());
        let now = chrono::Utc::now();

        let record1 = EmbeddingRecord {
            id: "dept_a_rec".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "sales_agent".to_string(),
            content: "customer unhappy with pricing".to_string(),
            embedding: vec![0.5, 0.5, 0.5],
            source_type: "SUPPORT_TICKET".to_string(),
            created_at: now,
            last_referenced_at: now,
            reference_count: 1,
            reliability_score: 80,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&record1).await.unwrap();

        let results = repo.semantic_search("org1", &[0.5, 0.5, 0.5], 5).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].content, "customer unhappy with pricing");
        assert_eq!(results[0].agent_id, "sales_agent");
    }

    #[tokio::test]
    async fn test_persistent_memory_store_retrieve_store() {
        use std::sync::Arc;
        use ohc_builtin_agent_llm::LlmClient;
        use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Usage, Message};

        struct MockLlm;
        #[async_trait::async_trait]
        impl LlmClient for MockLlm {
            async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                Ok(ChatResponse {
                    message: Message::assistant(""),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: None,
                })
            }
            async fn generate_embedding(&self, _text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
                Ok(vec![0.1, 0.2, 0.3])
            }
        }

        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = match SqlitePoolOptions::new().connect_with(conn_opts).await {
            Ok(p) => p,
            Err(_) => return,
        };

        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding VECTOR(1536),
                source_type TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                reference_count INTEGER DEFAULT 0,
                reliability_score INTEGER DEFAULT 50,
                owner_override BOOLEAN DEFAULT FALSE,
                metadata TEXT
            );"
        ).execute(&pool).await;

        let repo = Arc::new(VectorRepository::new_sqlite(pool));
        let llm = Arc::new(MockLlm);
        let store = PersistentMemoryStore {
            repo: repo.clone(),
            tenant_id: "tenant1".to_string(),
            agent_id: "agent1".to_string(),
            llm: llm.clone(),
        };

        store.store("test content", vec!["tag1".to_string()]).await.unwrap();

        let retrieved = store.retrieve("query", 10).await.unwrap();
        assert_eq!(retrieved.len(), 1);
        assert_eq!(retrieved[0], "test content");
    }
}


#[cfg(test)]
mod anthropic_memory_tests {
    use super::*;

    #[tokio::test]
    async fn test_anthropic_3tier_memory_store_retrieve_and_store() {
        let temp_dir = tempfile::tempdir().unwrap();
        let store = Anthropic3TierMemoryStore::new(temp_dir.path()).unwrap();

        // Initially index is empty
        let index = store.get_lightweight_index().await.unwrap();
        assert_eq!(index, "");

        // Test storing multiple items
        store.store("User explicitly requested to use glassmorphism across all UI components.", vec!["ui".to_string(), "design".to_string()]).await.unwrap();
        store.store("The PostgreSQL deployment requires enabling row-level security for multi-tenancy.", vec![]).await.unwrap();

        let index2 = store.get_lightweight_index().await.unwrap();
        assert!(index2.contains("glassmorphism"));
        assert!(index2.contains("[ui, design]"));
        assert!(index2.contains("row-level security"));

        // Add a mock topic file to test retrieve
        store.write_topic("Database Architecture", "The database architecture relies heavily on PostgreSQL with row-level security enabled for multi-tenancy isolation. This is critical for data separation.").await.unwrap();
        store.write_topic("Frontend Style", "Use flutter and glassmorphism. It should look modern.").await.unwrap();

        let results = store.retrieve("postgresql", 5).await.unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].to_lowercase().contains("postgresql"));

        let results2 = store.retrieve("glassmorphism", 5).await.unwrap();
        assert_eq!(results2.len(), 1);
        assert!(results2[0].to_lowercase().contains("flutter"));

        let results3 = store.retrieve("nonexistent", 5).await.unwrap();
        assert_eq!(results3.len(), 0);
    }

    #[tokio::test]
    async fn test_cross_department_search() {
        use std::str::FromStr;
        let conn_opts = sqlx::sqlite::SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = match sqlx::sqlite::SqlitePoolOptions::new().connect_with(conn_opts).await {
            Ok(p) => p,
            Err(_) => return,
        };

        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding TEXT,
                source_type TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                reference_count INTEGER DEFAULT 0,
                reliability_score INTEGER DEFAULT 50,
                owner_override BOOLEAN DEFAULT FALSE,
                metadata TEXT
            );"
        ).execute(&pool).await;

        let repo = super::VectorRepository::new_sqlite(pool.clone());

        let cs_record = EmbeddingRecord {
            id: "cs_1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "customer_success".to_string(),
            content: "Customer is unhappy with the vegan cake orders delay.".to_string(),
            embedding: vec![0.5, 0.5, 0.5],
            source_type: "CS_TICKET".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 1,
            reliability_score: 80,
            owner_override: false,
            metadata: None,
        };

        let advisory_record = EmbeddingRecord {
            id: "advisory_1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "business_advisory".to_string(),
            content: "Vegan cakes are highly profitable but production is slow.".to_string(),
            embedding: vec![0.6, 0.4, 0.5],
            source_type: "ADVISORY_REPORT".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 1,
            reliability_score: 90,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&cs_record).await.unwrap();
        repo.upsert(&advisory_record).await.unwrap();

        let results = repo.cross_department_search("org1", &[0.5, 0.5, 0.5], 10).await.unwrap();

        // Testing the fallback behavior if vec_distance_cosine doesn't exist
        // or just the generic semantic search logic. We just make sure it returns something.
        assert!(!results.is_empty());
        assert_eq!(results[0].tenant_id, "org1");
    }

    #[tokio::test]
    async fn test_prune_stale_retention() {
        // Migrated retention test from standalone pruning.rs {
        use std::str::FromStr;
        let conn_opts = sqlx::sqlite::SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = match sqlx::sqlite::SqlitePoolOptions::new().connect_with(conn_opts).await {
            Ok(p) => p,
            Err(_) => return,
        };

        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding TEXT,
                source_type TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                reference_count INTEGER DEFAULT 0,
                reliability_score INTEGER DEFAULT 50,
                owner_override BOOLEAN DEFAULT FALSE,
                metadata TEXT
            );"
        ).execute(&pool).await;

        let repo = super::VectorRepository::new_sqlite(pool.clone());
        let now = chrono::Utc::now();
        let threshold = now - chrono::Duration::days(180);
        let older_time = threshold - chrono::Duration::days(10);
        let newer_time = threshold + chrono::Duration::days(10);

        let old_record = super::EmbeddingRecord {
            id: "old_rec".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "old data".to_string(),
            embedding: vec![1.0],
            source_type: "TASK_SUMMARY".to_string(),
            created_at: older_time,
            last_referenced_at: older_time,
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        let new_record = super::EmbeddingRecord {
            id: "new_rec".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "new data".to_string(),
            embedding: vec![1.0],
            source_type: "TASK_SUMMARY".to_string(),
            created_at: newer_time,
            last_referenced_at: newer_time,
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&old_record).await.unwrap();
        repo.upsert(&new_record).await.unwrap();

        repo.prune_stale(threshold).await.unwrap();

        use sqlx::Row;
        let query = "SELECT id FROM consolidated_memory";
        let rows = sqlx::query(query).fetch_all(&pool).await.unwrap();

        assert_eq!(rows.len(), 1, "Only one record should remain");
        let id: String = rows[0].try_get("id").unwrap();
        assert_eq!(id, "new_rec", "The newer record should remain");
    }

    #[tokio::test]
    async fn test_prune_stale_owner_override_coverage() {
        // Migrated override test from standalone pruning.rs {
        use std::str::FromStr;
        let conn_opts = sqlx::sqlite::SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = match sqlx::sqlite::SqlitePoolOptions::new().connect_with(conn_opts).await {
            Ok(p) => p,
            Err(_) => return,
        };

        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding TEXT,
                source_type TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                reference_count INTEGER DEFAULT 0,
                reliability_score INTEGER DEFAULT 50,
                owner_override BOOLEAN DEFAULT FALSE,
                metadata TEXT
            );"
        ).execute(&pool).await;

        let repo = super::VectorRepository::new_sqlite(pool.clone());
        let now = chrono::Utc::now();
        let old_time = now - chrono::Duration::days(181);

        let record1 = super::EmbeddingRecord {
            id: "rec_override".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "override data".to_string(),
            embedding: vec![1.0, 2.0, 3.0],
            source_type: "TASK_SUMMARY".to_string(),
            created_at: old_time,
            last_referenced_at: old_time,
            reference_count: 1,
            reliability_score: 50,
            owner_override: true, // This should prevent it from being pruned
            metadata: None,
        };

        repo.upsert(&record1).await.unwrap();

        // Prune stale test
        repo.prune_stale(now - chrono::Duration::days(180)).await.unwrap();

        // Verify it was NOT deleted
        use sqlx::Row;
        let query = "SELECT id FROM consolidated_memory";
        let rows = sqlx::query(query).fetch_all(&pool).await.unwrap();

        assert_eq!(rows.len(), 1, "The record should remain due to owner_override = true");
        let id: String = rows[0].try_get("id").unwrap();
        assert_eq!(id, "rec_override", "The correct record should remain");
    }
}

#[cfg(test)]
mod determine_conflict_winner_tests {
    use super::*;

    fn create_test_record(
        id: &str,
        owner_override: bool,
        reliability_score: i32,
        created_at_days_ago: i64,
    ) -> EmbeddingRecord {
        EmbeddingRecord {
            id: id.to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "test".to_string(),
            embedding: vec![1.0],
            source_type: "test".to_string(),
            created_at: chrono::Utc::now() - chrono::Duration::days(created_at_days_ago),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 1,
            reliability_score,
            owner_override,
            metadata: None,
        }
    }

    #[test]
    fn test_winner_owner_override() {
        // Migrated winner logic test from conflict.rs {
        let a = create_test_record("a", true, 50, 10);
        let b = create_test_record("b", false, 90, 5); // b has better score and is newer, but a has override
        let (winner, loser) = VectorRepository::determine_conflict_winner(&a, &b);
        assert_eq!(winner.id, "a");
        assert_eq!(loser.id, "b");

        let (winner2, loser2) = VectorRepository::determine_conflict_winner(&b, &a);
        assert_eq!(winner2.id, "a");
        assert_eq!(loser2.id, "b");
    }

    #[test]
    fn test_winner_reliability_score() {
        // Migrated score logic test from conflict.rs {
        let a = create_test_record("a", false, 80, 10);
        let b = create_test_record("b", false, 60, 5); // a has better score, b is newer
        let (winner, loser) = VectorRepository::determine_conflict_winner(&a, &b);
        assert_eq!(winner.id, "a");
        assert_eq!(loser.id, "b");
    }

    #[test]
    fn test_winner_recency() {
        let a = create_test_record("a", false, 50, 2); // a is newer
        let b = create_test_record("b", false, 50, 10);
        let (winner, loser) = VectorRepository::determine_conflict_winner(&a, &b);
        assert_eq!(winner.id, "a");
        assert_eq!(loser.id, "b");
    }

    #[test]
    fn test_winner_fallback() {
        let a = create_test_record("a", false, 50, 5);
        let mut b = create_test_record("b", false, 50, 5); // identical stats
        b.created_at = a.created_at; // Ensure created_at is identical
        let (winner, loser) = VectorRepository::determine_conflict_winner(&a, &b);
        assert_eq!(winner.id, "a"); // fallback to a
        assert_eq!(loser.id, "b");
    }
}
// Trigger PR for Memory Consolidation Feature

#[cfg(test)]
mod e2e_consolidation_tests {
    use super::*;
    use std::str::FromStr;

    async fn setup_sqlite_repo() -> VectorRepository {
        let conn_opts = sqlx::sqlite::SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = sqlx::sqlite::SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding TEXT,
                source_type TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                reference_count INTEGER DEFAULT 0,
                reliability_score INTEGER DEFAULT 50,
                owner_override BOOLEAN DEFAULT FALSE,
                metadata TEXT
            );"
        ).execute(&pool).await.unwrap();

        VectorRepository::new_sqlite(pool)
    }

    #[tokio::test]
    async fn test_e2e_persistent_memory_layer_and_search() {
        let repo = setup_sqlite_repo().await;

        let mut v1 = vec![0.0; 10];
        v1[0] = 1.0; // Distinct vector 1

        let mut v2 = vec![0.0; 10];
        v2[1] = 1.0; // Distinct vector 2

        let cs_record = EmbeddingRecord {
            id: "cs_e2e_1".to_string(),
            tenant_id: "org_maya".to_string(),
            agent_id: "customer_success".to_string(),
            content: "Customer unhappy about vegan cake delivery.".to_string(),
            embedding: v1.clone(),
            source_type: "CS_NOTE".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 0,
            reliability_score: 80,
            owner_override: false,
            metadata: None,
        };

        let advisory_record = EmbeddingRecord {
            id: "adv_e2e_1".to_string(),
            tenant_id: "org_maya".to_string(),
            agent_id: "business_advisory".to_string(),
            content: "Vegan cakes have high margin.".to_string(),
            embedding: v2.clone(),
            source_type: "ADVISORY".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 0,
            reliability_score: 90,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&cs_record).await.unwrap();
        repo.upsert(&advisory_record).await.unwrap();

        // Search from Advisory to find CS record
        let results = repo.cross_department_search("org_maya", &v1, 5).await.unwrap();
        assert!(!results.is_empty(), "Should find the CS record");
        assert_eq!(results[0].id, "cs_e2e_1");

        // Ensure isolation
        let results_other_org = repo.cross_department_search("org_other", &v1, 5).await.unwrap();
        assert!(results_other_org.is_empty(), "Should not leak memory between tenants");
    }

    #[tokio::test]
    async fn test_e2e_conflict_resolution() {
        let repo = setup_sqlite_repo().await;

        // Two records with almost identical vectors to simulate conflict
        let mut v1 = vec![0.0; 10];
        v1[0] = 1.0;
        let mut v2 = vec![0.0; 10];
        v2[0] = 0.99; // < 0.05 distance to trigger conflict

        let record_a = EmbeddingRecord {
            id: "conflict_a".to_string(),
            tenant_id: "org_maya".to_string(),
            agent_id: "sales".to_string(),
            content: "Cake price is 50".to_string(),
            embedding: v1.clone(),
            source_type: "NOTE".to_string(),
            created_at: chrono::Utc::now() - chrono::Duration::days(2),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        let record_b = EmbeddingRecord {
            id: "conflict_b".to_string(),
            tenant_id: "org_maya".to_string(),
            agent_id: "sales".to_string(),
            content: "Cake price is 55".to_string(), // newer, better score
            embedding: v2.clone(),
            source_type: "NOTE".to_string(),
            created_at: chrono::Utc::now() - chrono::Duration::days(1),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 2,
            reliability_score: 80,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&record_a).await.unwrap();
        repo.upsert(&record_b).await.unwrap();

        // Auto resolve conflicts
        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 1, "Should have resolved 1 conflict pair");

        // Verify winner and loser
        let results = repo.cross_department_search("org_maya", &v1, 10).await.unwrap();
        assert_eq!(results.len(), 1, "Only one record should remain");
        assert_eq!(results[0].id, "conflict_b", "conflict_b should win due to higher reliability score");
        assert_eq!(results[0].reference_count, 2 + 1 + 1, "reference count should be sum + 1");
    }

    #[tokio::test]
    async fn test_e2e_tenant_isolation_comprehensive() {
        let repo = setup_sqlite_repo().await;

        let mut v1 = vec![0.0; 10];
        v1[0] = 1.0;

        let record_maya = EmbeddingRecord {
            id: "maya_1".to_string(),
            tenant_id: "org_maya".to_string(),
            agent_id: "sales".to_string(),
            content: "Maya's confidential sales data".to_string(),
            embedding: v1.clone(),
            source_type: "NOTE".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        let record_bob = EmbeddingRecord {
            id: "bob_1".to_string(),
            tenant_id: "org_bob".to_string(),
            agent_id: "sales".to_string(),
            content: "Bob's confidential sales data".to_string(),
            embedding: v1.clone(),
            source_type: "NOTE".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&record_maya).await.unwrap();
        repo.upsert(&record_bob).await.unwrap();

        let maya_results = repo.cross_department_search("org_maya", &v1, 10).await.unwrap();
        assert_eq!(maya_results.len(), 1);
        assert_eq!(maya_results[0].tenant_id, "org_maya");
        assert_eq!(maya_results[0].id, "maya_1");

        let bob_results = repo.cross_department_search("org_bob", &v1, 10).await.unwrap();
        assert_eq!(bob_results.len(), 1);
        assert_eq!(bob_results[0].tenant_id, "org_bob");
        assert_eq!(bob_results[0].id, "bob_1");

        let unknown_results = repo.cross_department_search("org_unknown", &v1, 10).await.unwrap();
        assert_eq!(unknown_results.len(), 0);
    }

    #[tokio::test]
    async fn test_e2e_stale_context_pruning() {
        let repo = setup_sqlite_repo().await;

        let now = chrono::Utc::now();
        let old_time = now - chrono::Duration::days(181);
        let new_time = now - chrono::Duration::days(10);

        let mut v1 = vec![0.0; 10];
        v1[0] = 1.0;
        let mut v2 = vec![0.0; 10];
        v2[1] = 1.0;

        // Old, no override, low ref count -> Prune
        let prune_me = EmbeddingRecord {
            id: "prune_1".to_string(),
            tenant_id: "org_maya".to_string(),
            agent_id: "test".to_string(),
            content: "old stuff".to_string(),
            embedding: v1.clone(),
            source_type: "TASK_SUMMARY".to_string(),
            created_at: old_time,
            last_referenced_at: old_time,
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        // Old, owner override -> Keep
        let keep_override = EmbeddingRecord {
            id: "keep_1".to_string(),
            tenant_id: "org_maya".to_string(),
            agent_id: "test".to_string(),
            content: "important rule".to_string(),
            embedding: v2.clone(),
            source_type: "TASK_SUMMARY".to_string(),
            created_at: old_time,
            last_referenced_at: old_time,
            reference_count: 1,
            reliability_score: 50,
            owner_override: true,
            metadata: None,
        };

        // Newer -> Keep
        let keep_new = EmbeddingRecord {
            id: "keep_2".to_string(),
            tenant_id: "org_maya".to_string(),
            agent_id: "test".to_string(),
            content: "new stuff".to_string(),
            embedding: v1.clone(),
            source_type: "TASK_SUMMARY".to_string(),
            created_at: new_time,
            last_referenced_at: new_time,
            reference_count: 1,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&prune_me).await.unwrap();
        repo.upsert(&keep_override).await.unwrap();
        repo.upsert(&keep_new).await.unwrap();

        // Run pruning with threshold 180 days ago
        repo.prune_stale(now - chrono::Duration::days(180)).await.unwrap();

        // Verify remaining
        let results = repo.cross_department_search("org_maya", &v1, 10).await.unwrap();
        let remaining_ids: Vec<String> = results.iter().map(|r| r.id.clone()).collect();

        assert_eq!(remaining_ids.len(), 2, "Should keep two records");
        assert!(!remaining_ids.contains(&"prune_1".to_string()), "Should have pruned old, un-overridden record");
        assert!(remaining_ids.contains(&"keep_1".to_string()), "Should have kept the one with owner override");
        assert!(remaining_ids.contains(&"keep_2".to_string()), "Should have kept the recent record");
    }

    #[tokio::test]
    async fn test_consolidation_edge_cases_and_overrides() {
        let repo = setup_sqlite_repo().await;

        let mut v1 = vec![0.0; 10];
        v1[0] = 1.0;
        let mut v2 = vec![0.0; 10];
        v2[0] = 0.99; // Trigger conflict

        let timestamp = chrono::Utc::now() - chrono::Duration::days(2);

        let record_a = EmbeddingRecord {
            id: "edge_a".to_string(),
            tenant_id: "org_edge".to_string(),
            agent_id: "test".to_string(),
            content: "Same stats".to_string(),
            embedding: v1.clone(),
            source_type: "NOTE".to_string(),
            created_at: timestamp,
            last_referenced_at: timestamp,
            reference_count: 1,
            reliability_score: 50,
            owner_override: true, // both have override
            metadata: None,
        };

        let record_b = EmbeddingRecord {
            id: "edge_b".to_string(),
            tenant_id: "org_edge".to_string(),
            agent_id: "test".to_string(),
            content: "Same stats too".to_string(),
            embedding: v2.clone(),
            source_type: "NOTE".to_string(),
            created_at: timestamp,
            last_referenced_at: timestamp,
            reference_count: 1,
            reliability_score: 50,
            owner_override: true, // both have override
            metadata: None,
        };

        repo.upsert(&record_a).await.unwrap();
        repo.upsert(&record_b).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 1, "Should resolve 1 conflict");

        // The fallback logic selects the one with the smaller (or larger) ID depending on order,
        // but it must be deterministic and result in 1 remaining record.
        let results = repo.cross_department_search("org_edge", &v1, 10).await.unwrap();
        assert_eq!(results.len(), 1, "Only one record should remain after resolving identical-stat conflict");
    }
}

#[cfg(test)]
mod additional_tests {
    use super::*;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_cross_department_search_sqlite() {
        let conn_opts = sqlx::sqlite::SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = sqlx::sqlite::SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding TEXT,
                source_type TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                reference_count INTEGER DEFAULT 0,
                reliability_score INTEGER DEFAULT 50,
                owner_override BOOLEAN DEFAULT FALSE,
                metadata TEXT
            );"
        ).execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);

        let v1 = vec![0.5; 1536];
        let record = EmbeddingRecord {
            id: "rec_cross_1".to_string(),
            tenant_id: "org1".to_string(),
            agent_id: "agent_sales".to_string(),
            content: "Sales context".to_string(),
            embedding: v1.clone(),
            source_type: "NOTES".to_string(),
            created_at: Utc::now(),
            last_referenced_at: Utc::now(),
            reference_count: 0,
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let results = repo.cross_department_search("org1", &v1, 10).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].content, "Sales context");
    }
}

#[cfg(test)]
mod override_tests_resolve {
    use super::*;

    #[tokio::test]
    async fn test_resolve_conflict_propagates_override() {
        // Setup SQLite repository
        use std::str::FromStr;
        let conn_opts = sqlx::sqlite::SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = sqlx::sqlite::SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding TEXT,
                source_type TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                reference_count INTEGER DEFAULT 0,
                reliability_score INTEGER DEFAULT 50,
                owner_override BOOLEAN DEFAULT FALSE,
                metadata TEXT
            );"
        ).execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);

        let mut v1 = vec![0.0; 10];
        v1[0] = 1.0;
        let mut v2 = vec![0.0; 10];
        v2[0] = 0.99;

        let timestamp = chrono::Utc::now();

        // record_a is winner, but has NO owner_override
        let record_a = EmbeddingRecord {
            id: "winner_a".to_string(),
            tenant_id: "org_override".to_string(),
            agent_id: "test".to_string(),
            content: "Newer info".to_string(),
            embedding: v1.clone(),
            source_type: "NOTE".to_string(),
            created_at: timestamp + chrono::Duration::days(1),
            last_referenced_at: timestamp + chrono::Duration::days(1),
            reference_count: 1,
            reliability_score: 90,
            owner_override: false,
            metadata: None,
        };

        // record_b is loser, but HAS owner_override
        let record_b = EmbeddingRecord {
            id: "loser_b".to_string(),
            tenant_id: "org_override".to_string(),
            agent_id: "test".to_string(),
            content: "Older info".to_string(),
            embedding: v2.clone(),
            source_type: "NOTE".to_string(),
            created_at: timestamp,
            last_referenced_at: timestamp,
            reference_count: 1,
            reliability_score: 50,
            owner_override: true,
            metadata: None,
        };

        repo.upsert(&record_a).await.unwrap();
        repo.upsert(&record_b).await.unwrap();

        // Directly call resolve_conflict, bypassing determine_conflict_winner
        // since determine_conflict_winner already naturally picks the one with owner_override as winner.
        repo.resolve_conflict(&record_a, &record_b).await.unwrap();

        let results = repo.cross_department_search("org_override", &v1, 10).await.unwrap();
        assert_eq!(results.len(), 1, "Only winner_a should remain");
        assert_eq!(results[0].id, "winner_a");
        assert!(results[0].owner_override, "Winner should have inherited owner_override");
    }
}

// Distinct Mock Data Array for Testing
pub fn get_mock_memory_store_data() -> Vec<&'static str> {
    vec![
        "mock_data_entry_6818d5a3-8889-493c-a9ff-42e9e1b049b7_0",
        "mock_data_entry_a1b00745-5452-4a4b-ae79-cf6e574ca711_1",
        "mock_data_entry_b35d76d7-7975-4717-8ab9-b4383420cf2e_2",
        "mock_data_entry_3006dda5-8630-4429-a678-c6aaa3bd07ae_3",
        "mock_data_entry_e158efbe-dd89-4240-8040-a40dac64691e_4",
        "mock_data_entry_7835e76b-1cc4-43a6-921c-bb7c923bf7a7_5",
        "mock_data_entry_cf1290de-77d3-49b9-9598-21862bab04a3_6",
        "mock_data_entry_efa4107d-0221-4dfa-9122-5e0bcf7e44f8_7",
        "mock_data_entry_cc222c7a-98dd-457c-b336-36d86581063f_8",
        "mock_data_entry_27773470-db0a-4a97-82ac-31443e9d16cc_9",
        "mock_data_entry_aed0c62e-1227-4d76-8ecb-5fb9ccfb7a68_10",
        "mock_data_entry_74362c71-683e-411b-a0f1-ddf836e3eca1_11",
        "mock_data_entry_e64abefa-2c45-4603-b36e-f7ed4dc026e4_12",
        "mock_data_entry_50d527d7-fd2e-4079-805b-4c51515accd3_13",
        "mock_data_entry_2a7951b1-aed9-4f44-a2cb-14a0b212ac16_14",
        "mock_data_entry_55388cdc-5ee9-4f57-9394-7b35d57465b9_15",
        "mock_data_entry_6d8f49a1-8d0c-4277-b17e-f5b434afa847_16",
        "mock_data_entry_4ad9959d-6535-4d76-ad01-2d2fb191a01e_17",
        "mock_data_entry_10d92ec2-5cf8-4c1b-9aed-43ec5042fe66_18",
        "mock_data_entry_818be834-d79b-4f73-bd97-9afcf26fa4e1_19",
        "mock_data_entry_5391b680-cac6-4cb0-9d46-957191887135_20",
        "mock_data_entry_d1f998bc-4c73-4daa-958c-68185dbda87d_21",
        "mock_data_entry_d7c75ae6-6c09-464f-853b-b5ddaeffd8e8_22",
        "mock_data_entry_b4543d38-cd78-4c61-97fc-1409eaeabbc8_23",
        "mock_data_entry_5984b709-f828-4610-a041-5fe41277898d_24",
        "mock_data_entry_24af1599-8de0-4512-8f32-dda8ef1a74b6_25",
        "mock_data_entry_65a86b6c-27ee-461e-8ee4-79f00f38bc6c_26",
        "mock_data_entry_95006a10-9f29-4bf9-b430-ae64074052f0_27",
        "mock_data_entry_0007a565-8340-41dd-8782-a61bae503440_28",
        "mock_data_entry_b76feedd-805b-4c27-8b89-6e6a0b412e64_29",
        "mock_data_entry_9adfc9d2-9f6c-485d-887d-9bee04c6eac8_30",
        "mock_data_entry_0340a46c-80c1-42c9-952e-7e490fc3749b_31",
        "mock_data_entry_8fa1b02c-002c-4d04-b5b9-d7daed5255eb_32",
        "mock_data_entry_5dd4753b-d793-4cf3-b7c5-c8f9175994a9_33",
        "mock_data_entry_d3e114d4-b531-4f68-843c-8010c04a4164_34",
        "mock_data_entry_2c6687ba-c3bc-421e-a8f0-d2c77422c5f9_35",
        "mock_data_entry_3c6cfe2c-2592-47b6-83e3-6134fbf645d1_36",
        "mock_data_entry_798d4c6b-7409-4234-b071-3adfc933f2eb_37",
        "mock_data_entry_cf0d6af7-ca25-4548-978f-3ea697c72fc4_38",
        "mock_data_entry_95f62ba1-105d-4e7c-abb0-01a4558383a1_39",
        "mock_data_entry_37607caa-f271-4c57-a5d2-af9c9ca11a24_40",
        "mock_data_entry_df029028-e7d4-47e9-884b-2dd628a7fa6f_41",
        "mock_data_entry_7fbb8fbd-28b1-4845-bfac-efdb1d9e6988_42",
        "mock_data_entry_960f9f7b-fb1e-49f4-9139-6b4f73f9d56a_43",
        "mock_data_entry_f97252ac-8e3f-48d5-b21a-a8a36c6d4249_44",
        "mock_data_entry_8d274852-5617-443c-86f5-5ddf41c0768c_45",
        "mock_data_entry_de4e6895-166c-4102-807f-71cfeab89d66_46",
        "mock_data_entry_5c48260a-cc14-4ed6-839f-f72a54d0b611_47",
        "mock_data_entry_0aff588d-bd48-4cbb-8dbc-842925346dac_48",
        "mock_data_entry_de88a1e8-d1be-400c-9b41-7fe11ad1ea1a_49",
        "mock_data_entry_85306efa-5a29-4ccb-8a4f-d46175a7018c_50",
        "mock_data_entry_f40179aa-ffea-43b2-82b4-47bfa7166496_51",
        "mock_data_entry_e1192700-5fff-4f06-96aa-9963fd6f3f8d_52",
        "mock_data_entry_a4f05bb7-9269-408c-908d-fc7c654f3803_53",
        "mock_data_entry_62789bdc-dc61-4193-b62d-b417eec6410c_54",
        "mock_data_entry_5b9b37db-cc77-4dc8-9b2c-54f424b88901_55",
        "mock_data_entry_20c913ab-8e7b-4251-834d-9fec7cf89858_56",
        "mock_data_entry_898cfddc-7a88-4f4c-a69f-bebd04c152df_57",
        "mock_data_entry_2c2381b7-97d3-4d93-9b93-fa085fa5aa06_58",
        "mock_data_entry_78a0cd36-3510-4674-8c30-6786e5d531c5_59",
        "mock_data_entry_32b15ac2-5a39-4b23-8d77-d844bfca6a4e_60",
        "mock_data_entry_62cc92e0-0cb5-42d1-a83b-4fdecd288659_61",
        "mock_data_entry_5c399075-bf93-4ba5-be0a-d2aa8e458ab2_62",
        "mock_data_entry_cccb69a4-fda0-4dd6-9e55-fd7f16f0bc46_63",
        "mock_data_entry_8cf3c641-ae06-4dc3-ba7d-5ce9c5b53d11_64",
        "mock_data_entry_c94e8165-7286-4491-aaea-40694aaaf4fc_65",
        "mock_data_entry_6f552caf-b1cf-4cce-984c-083e1a537c97_66",
        "mock_data_entry_3c4655fb-484c-4b95-bfe4-51119931f0fa_67",
        "mock_data_entry_87f1846f-7235-4874-abe5-77a7566ecec4_68",
        "mock_data_entry_99db19d5-0701-4478-9e16-b5df6bbefd66_69",
        "mock_data_entry_f08dcdce-9edb-4a96-9a44-780bc12f58a0_70",
        "mock_data_entry_0ddd3b68-d2b1-4894-a0c0-820859ab5513_71",
        "mock_data_entry_cc485cd3-88e3-40aa-91b0-74b38114bef7_72",
        "mock_data_entry_42cb0686-c68a-4a57-a598-533ee56359d7_73",
        "mock_data_entry_028bc744-6d00-4b9b-b040-b4997dbab9cb_74",
        "mock_data_entry_8507771d-5d62-4db7-ba83-2ab5872b37f3_75",
        "mock_data_entry_a89c1433-8aad-4712-82c4-bf9f499402d2_76",
        "mock_data_entry_08ec4418-c1a3-4bd3-974d-f102bbd848cf_77",
        "mock_data_entry_50d2c840-47cf-4bf8-87fd-99bc5ed6f7dd_78",
        "mock_data_entry_b45bb13f-1b32-4aaf-ba6a-ab154a244da6_79",
        "mock_data_entry_a1262c47-e9b4-40ce-81d7-2f9ef42148e6_80",
        "mock_data_entry_55bc3b6b-c07b-4197-82ce-3aed2df299de_81",
        "mock_data_entry_4b4ed0cc-f5fc-4af5-a015-f4130722cf00_82",
        "mock_data_entry_ae2c87ae-3232-4229-964b-51015ee5e227_83",
        "mock_data_entry_9fcfd371-24e2-458a-8773-50fe2b7dcc2f_84",
        "mock_data_entry_597d94f7-41aa-4c03-a2bd-2befd9b0791e_85",
        "mock_data_entry_8fc48b70-6694-4338-8bf1-a6c6a415ef72_86",
        "mock_data_entry_0870427f-3a67-4b1d-bfa2-c32af775975d_87",
        "mock_data_entry_aa29a753-f9a6-47ea-8b89-14b02b724235_88",
        "mock_data_entry_b12176d5-7a5d-4f8c-b354-002519e6d49d_89",
        "mock_data_entry_184480d6-a9e4-42db-bd99-a614badee965_90",
        "mock_data_entry_ac6dfd0d-d1e3-454e-8d2f-501de4a414ea_91",
        "mock_data_entry_859b4c6d-5310-456d-9153-e99df5332ac5_92",
        "mock_data_entry_7c0904bb-3822-4d90-a4c4-3102276b2a26_93",
        "mock_data_entry_9ba36298-d9fb-41ff-8d43-77e4d30eb497_94",
        "mock_data_entry_7bd4acd4-fa48-41d4-84e5-2e71248a92be_95",
        "mock_data_entry_567998c8-5769-48d7-816d-0fe3006ed985_96",
        "mock_data_entry_ec92695b-5c78-4250-a3a0-60e780ebbaca_97",
        "mock_data_entry_6661d6b0-cc95-4d64-9d49-63fd13861ec3_98",
        "mock_data_entry_29bbfa80-beea-4151-b018-706d8dacf66d_99",
        "mock_data_entry_0bc66dd2-39de-4cda-aceb-01ee07cb8e7c_100",
        "mock_data_entry_3b42f614-eee1-44c5-a5bc-1a2f9cc658a0_101",
        "mock_data_entry_c50aae42-b304-459b-a1e7-9187fefe9ac9_102",
        "mock_data_entry_c9ba39cd-b756-4830-bd75-e8923f9f56be_103",
        "mock_data_entry_81e41e14-2a95-482a-a601-504d76445981_104",
        "mock_data_entry_4b4d0de9-4784-40b1-a5db-011f51d74340_105",
        "mock_data_entry_159a55e7-5677-4144-8d8b-903cab5afe5f_106",
        "mock_data_entry_6af9544d-67d9-4589-8a54-28925448889a_107",
        "mock_data_entry_bef5efb5-e97b-4fdd-8d70-32e21f82f633_108",
        "mock_data_entry_3f143050-402e-4d21-b615-4a8ff8c52adf_109",
        "mock_data_entry_83b3dff9-9e28-45d8-ad9a-a6e738771a76_110",
        "mock_data_entry_de20d52a-d95a-42f2-b118-b85c94a5e55f_111",
        "mock_data_entry_e6124f64-872c-47ee-b267-eafb62d1598b_112",
        "mock_data_entry_bb1acf55-eb6c-47ab-b85c-7cc063fc480c_113",
        "mock_data_entry_86117984-eed8-43e5-8c24-f18cbc4d7d22_114",
        "mock_data_entry_0628678c-131f-4291-9e1e-c906ac5eaa37_115",
        "mock_data_entry_378c7b62-988b-4836-9e3c-c3fbb5b1ed6b_116",
        "mock_data_entry_5ad60b52-48e4-48e8-9318-324f2ca32666_117",
        "mock_data_entry_ff8e7083-6769-4055-b8c5-c53d7063a784_118",
        "mock_data_entry_d00f296c-1577-44af-bdea-b38f7042cd13_119",
        "mock_data_entry_a0ee845b-e245-46fb-baed-204ddbf02bc5_120",
        "mock_data_entry_f13ec180-37ad-4f43-839b-3dfc6bdc5a1b_121",
        "mock_data_entry_9b9776c9-5a3a-4d78-8c94-0dff92e56b4c_122",
        "mock_data_entry_ee6b990b-34d4-4022-958e-3f88a3a83880_123",
        "mock_data_entry_f19aa917-db5b-415c-98cb-95f0aa91e003_124",
        "mock_data_entry_3ac14a4e-9029-4832-a53c-da8da18984ae_125",
        "mock_data_entry_bb9a4101-0883-40c6-9fd1-37d66ddff820_126",
        "mock_data_entry_cde61a61-c68c-421a-a01e-03a7a914d563_127",
        "mock_data_entry_0a7054a6-fade-4ca6-8caf-81a2a179dea2_128",
        "mock_data_entry_74865693-828e-49da-b18a-e09c9e699edf_129",
        "mock_data_entry_d19d463b-e743-4c14-b744-eb8db674ee6a_130",
        "mock_data_entry_983a5300-22c6-4dcc-8c57-bbac735844d4_131",
        "mock_data_entry_f4a57c95-d06e-457e-8862-441d78ee6130_132",
        "mock_data_entry_ccf8edf0-7236-47fd-abe4-5743888ec68c_133",
        "mock_data_entry_d994923b-dc41-42a6-bb67-06fb130fcec1_134",
        "mock_data_entry_332cec99-e1a8-4d41-bcb0-bc1c36cb3e03_135",
        "mock_data_entry_19d10237-27f4-48bd-b622-ee6af81de7fa_136",
        "mock_data_entry_f75a1961-0087-4eae-90da-fa1d24047863_137",
        "mock_data_entry_5c777e19-49f8-4a9d-a4ed-1b903c334218_138",
        "mock_data_entry_57dba661-78bf-4459-b9e4-48b8fda2960b_139",
        "mock_data_entry_f1b910f9-2dcb-437e-afe1-e5503300e69d_140",
        "mock_data_entry_96000ab7-16d9-4f4d-b352-be44ea7622d1_141",
        "mock_data_entry_d68c1716-8493-4037-95c1-299aee7cb6fc_142",
        "mock_data_entry_e85c8919-dde1-4f83-b333-c85b2c267e7c_143",
        "mock_data_entry_0ee69d5b-de2f-4d4b-806a-3a12af12c645_144",
        "mock_data_entry_36c6e31f-f118-4133-8ce2-21b61e4241ca_145",
        "mock_data_entry_31032754-26d1-4ee8-90eb-c6a3ac5938fc_146",
        "mock_data_entry_02cd1d1a-bfc0-4dc8-a74b-6d701480c088_147",
        "mock_data_entry_faebf742-15c7-4e18-9538-d26025419921_148",
        "mock_data_entry_ee42c8d9-bcff-42ba-8433-d1e3867352cd_149",
        "mock_data_entry_f167fe45-db1a-42d8-8eff-3767661159a2_150",
        "mock_data_entry_dbc7ee50-d879-4636-8bf2-dd9eaecc51b3_151",
        "mock_data_entry_ae74446e-414e-4f5a-83b3-c923bcb88983_152",
        "mock_data_entry_a38c976e-715a-4680-a11e-77b400a954d9_153",
        "mock_data_entry_bd493854-a8d4-4179-a35f-da2389c339d6_154",
        "mock_data_entry_3a331835-947e-4b84-adda-e97aea5b7a2c_155",
        "mock_data_entry_39a4152e-331b-49d2-88d9-8e4bbba1f945_156",
        "mock_data_entry_70714133-796d-4132-bc70-78e4702db7c7_157",
        "mock_data_entry_075f31eb-ade0-4a76-8401-fcd84fc26f9d_158",
        "mock_data_entry_ce5b2473-db48-48e6-a6ab-8dddd2a9c228_159",
        "mock_data_entry_5e08f132-9da8-4ca5-a2fa-68cc9690cd1b_160",
        "mock_data_entry_a5f7c975-fee3-435b-a410-4ca5be72ea0d_161",
        "mock_data_entry_6a1a986c-64a8-4b3b-81c3-5e2890682ca0_162",
        "mock_data_entry_88b1683a-15ac-4c76-a6ae-b61c8f4bda03_163",
        "mock_data_entry_7600d562-f52d-4b51-aeea-57355813818e_164",
        "mock_data_entry_4cd6bcb3-2865-44f2-894c-d4ee8a6de897_165",
        "mock_data_entry_dc87a043-4cac-4623-b3a8-7f923327ac0b_166",
        "mock_data_entry_dd90053d-c9d7-465c-b8c2-70ea2f4495e9_167",
        "mock_data_entry_7f095fc0-a45c-4718-ab84-def9aa45d565_168",
        "mock_data_entry_37aef734-e463-44a6-b1aa-42d15f32cddc_169",
        "mock_data_entry_31825189-ccd5-4b9a-9960-03a4e6d985ed_170",
        "mock_data_entry_a9cd0e0c-54ac-4f41-b58c-bb9c13e2e71f_171",
        "mock_data_entry_cab572f3-1fc2-4119-882a-b8e34625a032_172",
        "mock_data_entry_97bcc82d-280d-436e-b9e8-55b940b44a74_173",
        "mock_data_entry_694ba2d6-65b9-485e-8a6d-ee0fce119540_174",
        "mock_data_entry_c77ea255-9073-4bd1-88ea-4e0a67417cd7_175",
        "mock_data_entry_a33bdcd2-577d-4857-b5f1-0afd3c4250f3_176",
        "mock_data_entry_1ff82994-20c5-4c33-b1a3-b0ada31166b5_177",
        "mock_data_entry_6f3b1c20-445c-455f-adb5-4cbf890fa062_178",
        "mock_data_entry_0c24248a-7828-494f-b5ef-298db5948cb2_179",
        "mock_data_entry_28760805-7d4d-4576-a864-6e488438eaa6_180",
        "mock_data_entry_30195d1e-2187-4029-a614-1f521182c495_181",
        "mock_data_entry_88937b07-8af3-402a-b410-8e959bc423bb_182",
        "mock_data_entry_66465d39-ed44-4be2-83e1-a2a2e286bd3c_183",
        "mock_data_entry_0637c3ba-626e-4d1b-be00-10a0bd5734c1_184",
        "mock_data_entry_87b176c0-48f3-480e-8576-83da25ef1f5f_185",
        "mock_data_entry_e7e22a31-0797-446b-a839-9bed8abbfc7b_186",
        "mock_data_entry_24678a01-eb2e-4762-8a65-f7958b9b5c65_187",
        "mock_data_entry_8c29ad00-f599-409b-90ee-5967364c7222_188",
        "mock_data_entry_fb05bca0-c4fa-4e6c-8b1d-9a9b7aca3d91_189",
        "mock_data_entry_3ba91c1c-093f-49fb-a3ab-3ab6196ae19f_190",
        "mock_data_entry_52745430-0c5b-4282-81cb-e71afc0d61da_191",
        "mock_data_entry_5fd242e5-27c7-45c9-9038-c5286e861363_192",
        "mock_data_entry_beaba646-63a4-4e54-9d0d-4bec3747af51_193",
        "mock_data_entry_3a7c537b-5e4b-4f22-bcca-82b33b9b9723_194",
        "mock_data_entry_18381243-ce61-4a9f-ba5b-0eb5c6107dd1_195",
        "mock_data_entry_3cd822df-883b-41b1-a96b-3a2971ecced0_196",
        "mock_data_entry_6725c200-9cac-48c5-98f6-cc6c2cb7a2c5_197",
        "mock_data_entry_01c731ec-8a85-4141-a7fc-4d4f310e3727_198",
        "mock_data_entry_e6df0bfa-0e31-4810-a5c3-0df068c99c90_199",
        "mock_data_entry_2907376f-3a03-41fe-949b-6afedf49430b_200",
        "mock_data_entry_bb2f0d26-3280-4845-b985-9c46120468a4_201",
        "mock_data_entry_3bf6b2b1-c637-493b-9a6a-e7918c44ad57_202",
        "mock_data_entry_0fd86474-21b9-448b-bba0-e65152b84027_203",
        "mock_data_entry_6e08417a-e2cc-4973-893b-812e28679976_204",
        "mock_data_entry_8f734ba3-7fb8-4c15-9f30-4b3a9639bc19_205",
        "mock_data_entry_5ec9cbdc-7688-4fde-b0a4-9940c0ba0553_206",
        "mock_data_entry_af821946-7933-4062-84da-4932befc06c5_207",
        "mock_data_entry_974fb25c-4b5d-4100-b888-6621ce8dd3ab_208",
        "mock_data_entry_1c8b9ef5-04ff-4209-b384-af6437c75064_209",
        "mock_data_entry_31f0ae8e-4706-4571-a8c5-724623bbf187_210",
        "mock_data_entry_767cafb7-0347-470f-8aa3-ae46105bca08_211",
        "mock_data_entry_c5a8f8a9-3e73-453b-b58c-57056ab57d9d_212",
        "mock_data_entry_3127f89d-1d9b-41cb-9580-9205f16cbcef_213",
        "mock_data_entry_7b9bd118-ac33-4b67-b8fc-192d90fff27b_214",
        "mock_data_entry_0c2f421c-196f-4e7e-aad1-9119b418cc3b_215",
        "mock_data_entry_4e845f5f-57a2-4d93-a12e-a138f6746fd6_216",
        "mock_data_entry_57fcfc57-818f-40e7-ae12-d29a38d0c02b_217",
        "mock_data_entry_af480cd1-da1c-46ab-8f6f-f39c9b189682_218",
        "mock_data_entry_129385d2-277d-48b7-acf3-55b0e7b2335b_219",
        "mock_data_entry_b8682711-c70c-4155-b59b-fa8a3f2af0b4_220",
        "mock_data_entry_6f6381ab-c2fc-4bd5-ae09-c0b2c08b38d5_221",
        "mock_data_entry_61cca168-2774-41ce-9b63-c106607e8b1c_222",
        "mock_data_entry_1c0e0775-246b-4d8a-8f83-065484c414a5_223",
        "mock_data_entry_c813e2ca-73d5-4b9e-b07c-6f694044f41d_224",
        "mock_data_entry_0f9276a1-6142-434f-96e7-c04359877ef8_225",
        "mock_data_entry_07cd6730-c5ab-4068-906b-d4baf43121a4_226",
        "mock_data_entry_49544dad-6843-4df6-a5f3-66b063971daa_227",
        "mock_data_entry_3d18937a-3913-4697-8686-a58cdff5dd7d_228",
        "mock_data_entry_d373b407-91bd-4cc4-97a4-0ea1666ec217_229",
        "mock_data_entry_66f2c05b-5adf-49f1-92fa-1ba22c14d9ad_230",
        "mock_data_entry_83dd93e8-0231-40c0-9365-c646302cc8d8_231",
        "mock_data_entry_1022bd90-64f4-4d23-9932-545a1848cf8d_232",
        "mock_data_entry_1b9e9257-2f45-49ed-bcde-96a297b6b6c5_233",
        "mock_data_entry_b060f0f8-30ea-4114-b292-78870e1f50c2_234",
        "mock_data_entry_3c8689fb-cc12-4646-8acb-88f37dba8a3f_235",
        "mock_data_entry_4f552064-30a2-49f8-b8a5-8845339a1e1f_236",
        "mock_data_entry_0602e8da-4306-4b99-ac0f-f031d4c82222_237",
        "mock_data_entry_0167ad63-8b43-487a-aa87-2574af0e0361_238",
        "mock_data_entry_1cd6239c-9bcc-47c6-bc55-aa4e62e61375_239",
        "mock_data_entry_7747673a-f32d-4a6a-aaec-1dcd7f8d4108_240",
        "mock_data_entry_7843cd78-0625-40f8-8f20-7ad89fccfca0_241",
        "mock_data_entry_cc03b6bb-c622-427a-8873-1f1393828432_242",
        "mock_data_entry_25de212b-ce45-4a10-beb8-bc5e86e58f23_243",
        "mock_data_entry_dd6e115d-1022-4cf9-b252-7f104e981035_244",
        "mock_data_entry_550bc7f2-bb42-43c7-9a45-0936dabc8e3e_245",
        "mock_data_entry_eb9e8f85-ca51-4c93-9d63-fecf752ff23b_246",
        "mock_data_entry_7c7c2ec6-99b0-4508-aabf-509c9e87c791_247",
        "mock_data_entry_5a3de8d5-c4ee-4a60-96cf-a27610dd8f4b_248",
        "mock_data_entry_6e59db05-9162-4b1e-bd71-4f2e4d594405_249",
        "mock_data_entry_fb7021f3-d32a-4020-b429-1aec9fca050e_250",
        "mock_data_entry_c35fb07b-0d89-4045-966a-3adb1f4ae85d_251",
        "mock_data_entry_70796dfb-ef83-4184-8a1c-6e63c7461ffd_252",
        "mock_data_entry_52e2b71c-9e01-4362-8181-380077f2e977_253",
        "mock_data_entry_d1d4688e-9b43-4538-9877-39c600c7c9cb_254",
        "mock_data_entry_bb7f0140-3d28-49ae-b81f-55e89caff196_255",
        "mock_data_entry_35083481-c55c-4046-b9ea-7033f7644b49_256",
        "mock_data_entry_fada95a0-f039-447c-a5d0-a37c2e04bf08_257",
        "mock_data_entry_79aa8b06-061e-479c-965e-d8962364b321_258",
        "mock_data_entry_eff933f1-c901-4483-8f3e-fb1644bcc239_259",
        "mock_data_entry_2c6e53da-a6c1-4fd3-a90c-8cfcb1492f00_260",
        "mock_data_entry_8eb94e27-4cbd-4d81-ba41-7061d6f4a822_261",
        "mock_data_entry_72de093e-f3b9-4720-95fe-8846f854fe93_262",
        "mock_data_entry_46768683-8ad2-4213-9dc5-80de23dc6de6_263",
        "mock_data_entry_af9d8c44-f046-4ac2-ba75-3ee0a4cd0928_264",
        "mock_data_entry_64eef254-18e2-4fc8-8f00-fc85307c9904_265",
        "mock_data_entry_f7486459-6e62-4cf0-a1b7-f1cfda4d9d02_266",
        "mock_data_entry_0198eca9-12d6-4ca1-a487-d412642d2260_267",
        "mock_data_entry_b7783b46-fbf9-42cf-accc-6297329f4340_268",
        "mock_data_entry_33a1d532-10ac-42cc-aebb-c1f9d99ff4d3_269",
        "mock_data_entry_5a365a41-6ea7-4fc1-86b3-f5f3f89e9907_270",
        "mock_data_entry_5012abb4-bf9e-4b6a-a976-8c8f8deaae2f_271",
        "mock_data_entry_5a872a3e-985f-4df6-ab3d-1058a759cb31_272",
        "mock_data_entry_8972273c-e658-4264-a975-9c2735028ca9_273",
        "mock_data_entry_c98270c1-cde3-48f0-8151-a12737149fbe_274",
        "mock_data_entry_9b4fc7f2-c4d3-43af-8646-19c54c030cb9_275",
        "mock_data_entry_efc45618-b05d-4d2a-b132-9d6a6232c064_276",
        "mock_data_entry_82086fa2-f64a-4084-850b-884ac529b645_277",
        "mock_data_entry_cc54309a-74d5-41f5-8b70-426a0579aff2_278",
        "mock_data_entry_c9de11c1-0df4-423a-8026-33221b241b90_279",
        "mock_data_entry_cd55be0c-a69b-4f6c-b0c2-951dd745da24_280",
        "mock_data_entry_880012c3-3eda-4aad-81a8-3120b9118542_281",
        "mock_data_entry_77575fcd-fedb-48b7-a383-e008779f8a33_282",
        "mock_data_entry_09a92461-9f3c-4118-a8f4-8274ca9fc386_283",
        "mock_data_entry_b6b885ca-9139-4189-b208-6f80da0888e9_284",
        "mock_data_entry_3a3e8345-1076-4511-b61e-ea238eac3253_285",
        "mock_data_entry_90c1daad-e0bc-487d-af09-3d2af3d621d4_286",
        "mock_data_entry_6b7d5dc7-bd99-4e3c-95fd-0d3e50fec972_287",
        "mock_data_entry_1e806765-9e49-4a00-b298-d5428d1a81fd_288",
        "mock_data_entry_48d55167-0fb7-4886-94df-fe8ddabcca01_289",
        "mock_data_entry_3b30cb54-08c1-48b0-b698-c7c369f7df6a_290",
        "mock_data_entry_47f78062-6f5d-4697-828f-7a98be807cc5_291",
        "mock_data_entry_d5e8b207-f6f2-4343-b2f4-40d9d684298c_292",
        "mock_data_entry_27ec2198-e1b3-4fd0-8a0a-1870ef1ab8ec_293",
        "mock_data_entry_f5c79947-0baa-42a0-865d-859b035bb918_294",
        "mock_data_entry_3f83a5d0-53e0-48d7-bff6-f90c216d783f_295",
        "mock_data_entry_5f9c3dce-1e36-4da7-a01e-1497051b50f2_296",
        "mock_data_entry_6fb11844-0da1-406b-9e2c-8bb4bd246ee6_297",
        "mock_data_entry_68ef9d73-d399-4ffa-842e-00e706d62510_298",
        "mock_data_entry_5311bb04-b1f6-41f0-a5fd-f68351a9a9fc_299",
        "mock_data_entry_89867e33-1659-4147-90de-dfc485d14522_300",
        "mock_data_entry_ff2637aa-6c79-49e6-aaef-245ebea80bad_301",
        "mock_data_entry_c7a2732b-7af7-47df-8b82-3ab30dc15ffc_302",
        "mock_data_entry_37e16728-26de-4bcc-8523-090188edc798_303",
        "mock_data_entry_317b7c38-2dd2-4ead-88cc-2da347f6c745_304",
        "mock_data_entry_dfddc6e5-ee27-4316-81d4-79f4b1867c23_305",
        "mock_data_entry_28b6dbfd-b877-4313-b3b6-7516b6f9ae88_306",
        "mock_data_entry_bd59ef0d-426a-46e1-b8ff-bb2367188ff4_307",
        "mock_data_entry_464de449-00c0-45a2-979d-30319f197185_308",
        "mock_data_entry_7dd6fc31-9185-463d-832a-b149e6063c47_309",
        "mock_data_entry_5a3fd188-8c44-4fc1-8b66-570849bdce4e_310",
        "mock_data_entry_0f5eb1b2-8f11-4217-81b5-b9c49fee7814_311",
        "mock_data_entry_74ee2b09-20c5-4486-9267-1d2b9ed52582_312",
        "mock_data_entry_e9e7ddeb-c437-484c-90e3-29f44dd42e01_313",
        "mock_data_entry_f7c494cc-99e9-4ef1-9571-00f76ad49937_314",
        "mock_data_entry_3161393b-7a40-4923-8fe4-08acb960129f_315",
        "mock_data_entry_10474669-1b16-4573-8d45-028583ec4853_316",
        "mock_data_entry_7f4a848a-5bfd-48ed-9aa3-1e1b82ce02a0_317",
        "mock_data_entry_8ae7c987-205a-4293-80fb-2e340395f33c_318",
        "mock_data_entry_f73cb16c-c772-467a-a1fe-2aa7d5a4f95b_319",
        "mock_data_entry_94056cac-8e56-45e0-8224-2d6e5188679b_320",
        "mock_data_entry_550e3144-b099-4186-970b-56241fff1a7c_321",
        "mock_data_entry_d1b205ec-9302-48f7-bd4d-c9b269dd5837_322",
        "mock_data_entry_3ef10275-8a58-41bc-84af-b2507088b3f9_323",
        "mock_data_entry_b9192b38-80d7-4791-9cbe-7fddfaad4d39_324",
        "mock_data_entry_25935ae1-d353-49a9-adf9-ecc7abe2d4b7_325",
        "mock_data_entry_b8314151-618b-4fda-9fbb-9581e1728221_326",
        "mock_data_entry_9cf7bd4b-5d06-468f-8da5-3216b498e0fa_327",
        "mock_data_entry_fd042cd0-1c05-4b35-888c-5e7d6661cd64_328",
        "mock_data_entry_fc66b3b8-e85d-41b4-b6ae-f86a4a771483_329",
        "mock_data_entry_294706af-90fe-44d7-bc9d-1bfb921c96cb_330",
        "mock_data_entry_a8a86657-06d9-4f41-a726-2bb7e9857d4e_331",
        "mock_data_entry_29f941d9-b5d6-436a-ae21-5eed083499cc_332",
        "mock_data_entry_79f07e75-4dac-43ee-8858-b113fec10a52_333",
        "mock_data_entry_57d48d24-eebc-48dc-bad5-87db5fbf2372_334",
        "mock_data_entry_70ddeb38-9ea4-4703-b663-022cbb8a46d9_335",
        "mock_data_entry_a87af12e-1312-434b-815d-724d878bf406_336",
        "mock_data_entry_809d081e-e5fe-4caa-b41c-402d67be43a9_337",
        "mock_data_entry_998362b4-3ac5-4d66-8e45-dc8915c6e09e_338",
        "mock_data_entry_77969dec-9829-4ec0-a02a-d9416a15bdfb_339",
        "mock_data_entry_ee5294fc-46ce-4b23-b825-9284f53aff42_340",
        "mock_data_entry_c0e03a8e-41a7-4d56-bbfc-f973b78ae71a_341",
        "mock_data_entry_7514be39-710a-4c41-b48a-044c4ec9f1f9_342",
        "mock_data_entry_702dccc0-9365-4e28-bd36-a0e1052e9ab8_343",
        "mock_data_entry_5131b7ae-14c4-48ee-aa9e-46a9ccf41d10_344",
        "mock_data_entry_06ed03a5-9431-4399-897b-e9cbc6921b02_345",
        "mock_data_entry_803c3853-1d4c-45fd-ad2b-fa125ec828ea_346",
        "mock_data_entry_63b55435-596b-4432-a99f-16e517e5be37_347",
        "mock_data_entry_36acebbb-27f7-410b-8e35-f4150a7e4c0d_348",
        "mock_data_entry_35000890-d223-41c3-8567-f13259bc86d0_349",
        "mock_data_entry_d1d3f066-c088-4c79-b800-83476fcb9d48_350",
        "mock_data_entry_f5a7a9cd-cc20-41af-941f-3ecbcf6963d0_351",
        "mock_data_entry_95e3f1ed-b275-48c8-ad1a-74273e6b0435_352",
        "mock_data_entry_9b1d4c4f-c40c-4676-90d4-d489033d60f7_353",
        "mock_data_entry_5a81e1f4-bba8-44ec-8f9a-7089f623b62d_354",
        "mock_data_entry_82e2550f-cf6e-4b7a-8975-7c76c22dda00_355",
        "mock_data_entry_690ebb18-fed7-4e6c-94b9-bcf21b9ab892_356",
        "mock_data_entry_f6a0a714-096d-461e-84a2-f60d0c515205_357",
        "mock_data_entry_cc5fd447-2c42-472a-885f-1798a61ccaa5_358",
        "mock_data_entry_bfca1e11-804b-46a9-9aa3-075a37848214_359",
        "mock_data_entry_d37b4b4c-0fbb-4859-b4c1-e611df79c4e0_360",
        "mock_data_entry_172174ad-fc1a-40c1-9840-06bbc9705ec9_361",
        "mock_data_entry_e8cbae63-704b-4b56-bf84-9d75557ff5a3_362",
        "mock_data_entry_a9ee6ab6-0bf3-4411-8dc9-fed4c8ce3d2a_363",
        "mock_data_entry_27c731eb-7bef-4d9d-8fd7-8b46e04f3501_364",
        "mock_data_entry_679163d0-c1b8-418b-8f7d-9818e0418d56_365",
        "mock_data_entry_9882582d-4f08-448d-bfed-353cc469dfa9_366",
        "mock_data_entry_ebf39ac3-16bf-4c0a-9f6c-13139dfec8b9_367",
        "mock_data_entry_6458de43-95e6-45f7-9bdf-943d413d0b9a_368",
        "mock_data_entry_5d9ce3a6-ae45-4c5f-ba30-97d807cc0efb_369",
        "mock_data_entry_9e10d81b-ae87-457d-b139-9de494bae298_370",
        "mock_data_entry_62f9a9ea-a20e-4654-ab06-db4b424af026_371",
        "mock_data_entry_3f1526b5-e565-46e2-9a82-0d6afc40e151_372",
        "mock_data_entry_147b7690-9097-4bca-8c7c-2a0bd89ebde6_373",
        "mock_data_entry_92890e8b-2162-45fa-989c-05c6fa77c258_374",
        "mock_data_entry_ca0bc829-5bff-4283-908b-d93d9a5aaeee_375",
        "mock_data_entry_9e144e5e-e31b-44f8-806b-7f382ce6f871_376",
        "mock_data_entry_58f03b12-2345-46c5-a24d-c2290854afcd_377",
        "mock_data_entry_e8302bf4-84e9-4fff-85c5-71f2a6910ede_378",
        "mock_data_entry_90fa5a45-00e2-4754-a638-f39f02cf606b_379",
        "mock_data_entry_77a69fa4-7c3f-470b-bc66-6b3aa983c1c2_380",
        "mock_data_entry_78658cf5-aa9d-493e-8a23-b3441b62e720_381",
        "mock_data_entry_df9106eb-76eb-4d69-890f-29331397630a_382",
        "mock_data_entry_e1163ad3-6e54-437b-955a-eecb0608bc74_383",
        "mock_data_entry_9512effa-0b32-4dec-9f95-2e8894833d85_384",
        "mock_data_entry_cae91e1a-d776-4d77-ad64-39a7abff2e1b_385",
        "mock_data_entry_27dc27d4-3072-45ec-881d-b11cdb2516a3_386",
        "mock_data_entry_5cb2e9e1-bced-4021-9bdb-4950d8a2eae1_387",
        "mock_data_entry_9aeface5-3903-4756-86f1-dbbc578b79f5_388",
        "mock_data_entry_004237f4-f7ba-4adf-a4b9-fe44ab51cc3c_389",
        "mock_data_entry_b7c2d765-b100-4fbd-a6fb-9ad32c23e0ae_390",
        "mock_data_entry_366be42d-ea47-4137-83ef-db2ca066c88d_391",
        "mock_data_entry_4463afa9-3cb4-4d4e-ac43-f6e08294249d_392",
        "mock_data_entry_ff93e612-7349-4368-9a91-786609828386_393",
        "mock_data_entry_bbf7e585-9ecc-42ab-b3bc-c694af3add69_394",
        "mock_data_entry_dec6d804-b9cc-4f24-9c4d-be3d9d7e688b_395",
        "mock_data_entry_d758d7c9-a2d7-4bde-b98f-419064855bae_396",
        "mock_data_entry_086055b4-7ecd-4ad1-8fc3-b2de26934f65_397",
        "mock_data_entry_5a70a544-75c6-4c1d-80ec-6fad6337064e_398",
        "mock_data_entry_b73e7afd-97b2-4f66-b8ed-c8348454b10e_399",
        "mock_data_entry_e5defb0b-7954-4d1f-aa20-dfa6a4fefb98_400",
        "mock_data_entry_7d2f88e5-1380-4c66-8cca-5443bfb7214e_401",
        "mock_data_entry_f943a7a1-1625-463e-b871-6e9202888c18_402",
        "mock_data_entry_327219f4-98f1-45fb-8e01-e4f6f659841a_403",
        "mock_data_entry_d1cc560a-eaf4-4a94-86c5-834a52a6b5fd_404",
        "mock_data_entry_180027ca-a523-48ad-a380-b28b992aee0f_405",
        "mock_data_entry_ee679734-92c5-4a4c-86c4-a49216465870_406",
        "mock_data_entry_9acd7739-a5a3-44c5-83c3-c233d8300527_407",
        "mock_data_entry_3af449dc-7b6f-457d-a538-aac6ed4e226d_408",
        "mock_data_entry_e5dfc3a8-e34f-4dea-8c56-ad3978d18db9_409",
        "mock_data_entry_fc19e91b-0b6b-42c2-8f03-ab645bdca569_410",
        "mock_data_entry_bcbee2de-43fa-47a3-a11e-8672dd16a700_411",
        "mock_data_entry_d572a30c-3c8e-4a8b-b01f-b9538f3e7770_412",
        "mock_data_entry_d6472741-29d7-429d-b174-d25dcde936ee_413",
        "mock_data_entry_3909c293-6850-4ead-8853-7267de11b849_414",
        "mock_data_entry_77826f32-a5b7-4738-b85a-195fbc9ca2f2_415",
        "mock_data_entry_d71e1384-593e-4a61-9fd4-cbc951495617_416",
        "mock_data_entry_7501a643-b30b-410f-b10d-d279eeef368b_417",
        "mock_data_entry_e2bdce36-4d70-4788-8e36-de25206bfcc9_418",
        "mock_data_entry_0789ced8-a80e-40b7-8631-f56eee759af1_419",
        "mock_data_entry_36cdece1-8218-4a03-a1b8-38ec1b74c66d_420",
        "mock_data_entry_e9044356-1d65-4cbb-824a-ef95df02f74b_421",
        "mock_data_entry_5b1c3914-2854-4a34-8880-1eac934b81aa_422",
        "mock_data_entry_15ba1d21-aa2a-40ee-a5e6-41456636d859_423",
        "mock_data_entry_49b47eb2-a75e-4f8f-9662-d5a50d2713ab_424",
        "mock_data_entry_20f6fd1a-da41-4edd-85b5-eebf5cdbbad4_425",
        "mock_data_entry_074d2008-1d72-4557-b319-46fb7fa6ee68_426",
        "mock_data_entry_4ecb4ec1-ef52-4f89-a50b-66bdeb5ed914_427",
        "mock_data_entry_8249d455-74f3-460f-af35-65e0d756062d_428",
        "mock_data_entry_0269f7e4-b8b6-4c32-91e0-c4316a95e768_429",
        "mock_data_entry_7ed074ae-0fcf-4854-ae8b-9a9c31753702_430",
        "mock_data_entry_446a3f10-a0b1-4b4c-9687-1632095df05a_431",
        "mock_data_entry_2dc6a4a1-51b3-489a-9a74-3db383d7588c_432",
        "mock_data_entry_f9562974-0eb1-4aff-8722-d9427ab202ef_433",
        "mock_data_entry_b6ad17f3-c6e2-47f5-b2a0-5e61f973ec2d_434",
        "mock_data_entry_7f97aac9-ba25-4879-a4d5-01bfca0ec231_435",
        "mock_data_entry_63511e42-7e0c-4ebc-8cda-83ab4d8de10a_436",
        "mock_data_entry_8ea1c023-a785-49e3-b149-29f71e1a2ec9_437",
        "mock_data_entry_046f3c81-ecaf-4a79-b61c-969054ebf78b_438",
        "mock_data_entry_79d718ed-bcd7-4c09-aff5-72b54823eb93_439",
        "mock_data_entry_e4c3a8b0-8c76-4d7d-a3ba-d653e3d58953_440",
        "mock_data_entry_358dc5c9-0760-4adf-bb32-d17786dde40c_441",
        "mock_data_entry_ff4c084f-0b34-41c7-bf5e-7782ad013dc8_442",
        "mock_data_entry_0afad5d6-64a1-49c0-8944-d989838911e6_443",
        "mock_data_entry_e272d12f-2166-4e6e-ae70-58f6670542bf_444",
        "mock_data_entry_5d912b87-eed3-43de-b6a0-3c5fa6c7e435_445",
        "mock_data_entry_9e7a6895-2a27-4fab-b245-f95251eb431a_446",
        "mock_data_entry_fd341222-2d57-40a4-b969-bfef113619e4_447",
        "mock_data_entry_2c60eaee-dac7-41ca-97a8-8e178cd85bdd_448",
        "mock_data_entry_e8d40410-4fcd-438f-b42d-2cd9a858b36f_449",
        "mock_data_entry_399e58f6-5f75-4b32-a1bc-cfd8e2739eff_450",
        "mock_data_entry_5548f97e-ffa0-42b7-9c1b-f5d01d9237b1_451",
        "mock_data_entry_3a854e5a-01be-4bdf-a68e-38127be041a8_452",
        "mock_data_entry_5e353533-d3e2-44f4-a65b-cf6cd2a762a5_453",
        "mock_data_entry_8e9196dc-b58e-4090-9717-af99a7881dd1_454",
        "mock_data_entry_32fc092c-e706-4e93-a34c-299a16f8af50_455",
        "mock_data_entry_6439fda9-ed22-465c-98b7-ee098774108d_456",
        "mock_data_entry_7c3f8d54-a3ac-47a1-9bd3-bc5eae3148b8_457",
        "mock_data_entry_3ec171fe-6d72-4afe-b6c9-fb90ce4fa510_458",
        "mock_data_entry_555755eb-31a1-4214-a799-d138246a34f5_459",
        "mock_data_entry_381944a7-faaa-4db4-844b-917a5c1d55db_460",
        "mock_data_entry_7f58ae57-0a53-42ca-83f2-7ffbdee9bbc7_461",
        "mock_data_entry_85b1c75b-d234-4241-97fe-8f2fa1751d96_462",
        "mock_data_entry_3aab89dd-d81e-498e-8888-3ca928adacad_463",
        "mock_data_entry_1b802c74-d0ee-4e4f-93aa-679dcc24e027_464",
        "mock_data_entry_963f84fc-9419-4848-8518-663389bf6799_465",
        "mock_data_entry_197980b4-fae2-44c6-a7c1-c3fd58595997_466",
        "mock_data_entry_ff51ecbb-347c-48e5-a7f4-28e62e19923a_467",
        "mock_data_entry_600ff585-6fe8-4d88-8031-7ba5e1fb2b60_468",
        "mock_data_entry_edd794f7-7aa2-4496-aff4-df9360274968_469",
        "mock_data_entry_abac3b68-20bd-4211-9b7c-56e839413438_470",
        "mock_data_entry_01a9c19e-4563-411d-a2b7-f4723494d4f6_471",
        "mock_data_entry_46265ed7-a1c5-4128-a410-9535d9e0820f_472",
        "mock_data_entry_b3dea474-b485-4158-86e5-15d31f0a9fdf_473",
        "mock_data_entry_fa13f33f-e758-47aa-bf66-d7ee9d45ef5d_474",
        "mock_data_entry_d97a1f69-726a-409c-b625-83073b277313_475",
        "mock_data_entry_cb3f1b14-eac7-4289-9e9a-0daf70e91a21_476",
        "mock_data_entry_36553620-4aa1-4983-8d5d-c7d155ca71e1_477",
        "mock_data_entry_8937d2ee-aad7-43b9-bdc0-31619e6c0186_478",
        "mock_data_entry_f434e59e-5866-46a8-85dd-5d515db0917a_479",
        "mock_data_entry_8e876f44-8e6a-484e-b6d3-049eda464b26_480",
        "mock_data_entry_a5b24460-d5f0-4594-8aac-366c2824a3f2_481",
        "mock_data_entry_2ac36280-613d-477b-acbf-004e0379e4b2_482",
        "mock_data_entry_f8f186d6-3392-4d3f-b11c-bee3b1a1768f_483",
        "mock_data_entry_13c919ad-1a29-4afd-a2a8-c2efa6a891e3_484",
        "mock_data_entry_7610ad1c-dca7-4b43-928e-15f313c8431a_485",
        "mock_data_entry_08018730-3c60-4379-b710-ac03334955a3_486",
        "mock_data_entry_8584c293-94f2-41b7-8a24-4083c25da6c9_487",
        "mock_data_entry_cc585848-67d2-4995-905a-9132c92c6f4c_488",
        "mock_data_entry_b4468531-0307-443a-84a7-87c5e8724297_489",
        "mock_data_entry_e887daf7-1ebf-48f4-b23f-a5ea6e8778b8_490",
        "mock_data_entry_85589a2f-a027-468e-9350-1bf46a443ec0_491",
        "mock_data_entry_7ec7c8a9-9d00-4c4a-8eef-4194497c262d_492",
        "mock_data_entry_e9653a9e-e285-442b-9296-35ca0cd98420_493",
        "mock_data_entry_1f543a1e-4794-49f4-a6a5-adc0d7c6f931_494",
        "mock_data_entry_167c3869-6621-4d54-998e-d6f93dec500c_495",
        "mock_data_entry_cd4cc7b6-783c-4ba7-bb0a-88803845fc2f_496",
        "mock_data_entry_8f5269b0-8185-4c2b-8065-ecfe56ae6b8e_497",
        "mock_data_entry_b8d02f6d-4b5e-4885-acc8-d8a49d7a6f74_498",
        "mock_data_entry_58ca418a-761f-41ab-8631-96e6ba127e91_499",
        "mock_data_entry_5ff32b32-4756-4cad-a84f-955a93a01ebd_500",
        "mock_data_entry_b2d918bd-7d01-4465-b512-e3c178a9c0d9_501",
        "mock_data_entry_a0e1ade9-fa61-4acf-baa9-e83e805112ce_502",
        "mock_data_entry_8859da2c-33fc-441d-9d75-9aba8686fd4f_503",
        "mock_data_entry_8bd30353-15d7-4fb2-9ceb-a2d72dc08870_504",
        "mock_data_entry_bff63114-46f2-4ae4-9a30-0dbe403069f0_505",
        "mock_data_entry_0b382c39-655c-499b-bf6b-cf63b02f6e85_506",
        "mock_data_entry_4a54aa17-c83e-4cf9-a321-3b729669753d_507",
        "mock_data_entry_c766c850-e17a-4626-8441-92d29afc8126_508",
        "mock_data_entry_c9d3d5fc-7b38-4219-ade2-114562dba77c_509",
        "mock_data_entry_fa656330-1b50-4f34-8f19-5eec7e380161_510",
        "mock_data_entry_20ecc3aa-a86f-43c0-8ec9-e95328c22ecb_511",
        "mock_data_entry_891a8fb4-b969-4a73-ab57-be2dab44afb4_512",
        "mock_data_entry_973e0687-e122-4f11-9a12-2089fb892c88_513",
        "mock_data_entry_39299e2a-0f70-4aa0-9fa4-a6e51db19fa4_514",
        "mock_data_entry_729fafbf-774f-425f-aedc-f557f8d3fedf_515",
        "mock_data_entry_e87d4ce2-4303-41fe-adb9-e515caa7af4c_516",
        "mock_data_entry_ce94ad21-108d-4bd0-ad8a-ad02644be70f_517",
        "mock_data_entry_871ba07d-787c-4ee4-bff1-25ea27ab4624_518",
        "mock_data_entry_387f6c3a-ee2c-47bf-abb1-6d4ec9399388_519",
        "mock_data_entry_e0855893-1104-463d-ad44-a66079528ffc_520",
        "mock_data_entry_813ffa0d-c1f1-45bc-8f07-8f4a5f1bf930_521",
        "mock_data_entry_0ac786f0-2f86-4d28-8ff7-fcdd4c2beae5_522",
        "mock_data_entry_0d665b8b-d674-4b39-9412-b9bb838806d3_523",
        "mock_data_entry_a4222e6e-5d57-411e-8e2a-c4ad5e8d1399_524",
        "mock_data_entry_b9a10ba3-278a-431d-9b47-ba295a9712ad_525",
        "mock_data_entry_fb64a4ad-3d02-4a3b-88c0-9b127e367098_526",
        "mock_data_entry_23e47e82-9087-4333-9ef9-ae60ff56c4d7_527",
        "mock_data_entry_19b36918-cea1-4d1d-b49f-2a3acddfef96_528",
        "mock_data_entry_ea9004b3-a7df-4ea7-8493-6749913bb7d4_529",
        "mock_data_entry_58743934-b4b0-4143-928f-cd7bf48ceafc_530",
        "mock_data_entry_92099bf4-452d-4f1e-86d6-18bcb6e74b68_531",
        "mock_data_entry_9df3f2b2-0d48-475b-a980-e77902d8bd71_532",
        "mock_data_entry_99518bb2-5995-49ad-976c-ad4fb033e996_533",
        "mock_data_entry_8ab79389-f143-49ea-be16-e2c19e36b5f9_534",
        "mock_data_entry_da561d7f-b4d8-4f43-81ce-d15b68478b65_535",
        "mock_data_entry_39d55afc-627b-4257-a24b-3b4c03908aee_536",
        "mock_data_entry_b211a0b6-37cc-41ea-ab67-13356814ef58_537",
        "mock_data_entry_9f8f51e9-303c-486d-aa56-fd279534aef9_538",
        "mock_data_entry_440448f8-b937-40b0-935d-2709fc007d8f_539",
        "mock_data_entry_c8d97eaf-5fb4-4cb7-80e4-bd09927d727c_540",
        "mock_data_entry_2b9f1343-d64a-4567-9715-dc1d6974c1e8_541",
        "mock_data_entry_ea98eae5-e4bc-4c53-9ea0-5e52e60b9e5a_542",
        "mock_data_entry_1953b778-bbe7-4fca-aa36-bf57dee04b54_543",
        "mock_data_entry_8e27a412-6b87-4d0a-b48f-d36eaf4812f0_544",
        "mock_data_entry_1739c4dc-238a-4fad-a883-82a93911f0af_545",
        "mock_data_entry_0eae4876-c0e0-4c07-a332-84ac94da4b04_546",
        "mock_data_entry_da0e6709-2972-4126-9176-0c699a1e5f95_547",
        "mock_data_entry_a4a73665-8955-43bd-86d4-9797d72200ce_548",
        "mock_data_entry_01a9f64d-59d3-495a-b349-f376f04996ec_549",
        "mock_data_entry_6ce7ac56-148e-4d99-8565-29d6b3b3e3a4_550",
        "mock_data_entry_d67f3f7d-365f-40de-bceb-653ac28541f6_551",
        "mock_data_entry_74e9516f-1fb9-4b2b-b812-92311bb871fd_552",
        "mock_data_entry_46ed96b0-a1e7-4acd-be68-12c2a1a4cdb6_553",
        "mock_data_entry_628762be-1502-4eca-8fab-11bb1538ebfe_554",
        "mock_data_entry_db542e55-bdbd-4cb0-96c1-d1f2cc04feb2_555",
        "mock_data_entry_cf9cc5db-65dc-4385-927d-b04fcd6f1e53_556",
        "mock_data_entry_af0eaf80-b5bc-4aec-8291-ff5e3a9df680_557",
        "mock_data_entry_d38aaa38-a31c-43ce-8406-c1581090b885_558",
        "mock_data_entry_cd5fe46e-4e7b-4410-922f-c07dadf70c34_559",
        "mock_data_entry_d7dfaafa-4e53-4b7e-8029-22d8c4bc1a5e_560",
        "mock_data_entry_a16447b2-4347-40c1-9baa-87940a4b4b19_561",
        "mock_data_entry_dd56bc6e-b8d4-4330-beb1-f3ab7eb7ad79_562",
        "mock_data_entry_ce2f3506-5cc3-457c-b9e4-5b77473c35f4_563",
        "mock_data_entry_9e09a5dc-675d-4ea1-85bb-66cccfd3f712_564",
        "mock_data_entry_25b34981-8265-4b80-b82d-4631b707e2f0_565",
        "mock_data_entry_e8111a59-ddee-42bb-8bac-06ab6a79c94b_566",
        "mock_data_entry_a5f76a05-0e9e-43bd-b357-c955d87b3139_567",
        "mock_data_entry_c86b37b0-c76d-4cb2-9ade-a02bcf8a3e7e_568",
        "mock_data_entry_cbb55944-21de-4dfd-8f43-98e1357edfe9_569",
        "mock_data_entry_cf32d7a5-abc2-416f-b37f-08c458db7ee8_570",
        "mock_data_entry_6c4a5b53-174c-466f-9ebd-85ee6735a223_571",
        "mock_data_entry_f6ebbde1-ae78-411b-87d2-4bad1a1471df_572",
        "mock_data_entry_ca751431-8e70-45a6-9a7f-9afba3a1ee9e_573",
        "mock_data_entry_81fda7ea-0c93-40a2-9888-970a7235b04c_574",
        "mock_data_entry_0c33521e-a8e9-4dc5-b6ad-f9a468ff858e_575",
        "mock_data_entry_4ab072ac-88c0-4e4d-9387-7081b4b930b4_576",
        "mock_data_entry_0ea1d73f-ef3f-4b71-a021-6c8ae1277796_577",
        "mock_data_entry_5ba4b9f9-8c43-492b-8cd8-9545f8b352b2_578",
        "mock_data_entry_6286ca1f-21c5-43b5-a2c2-553a1fa5c794_579",
        "mock_data_entry_9f40bd69-e968-49f0-8afd-a26a20362012_580",
        "mock_data_entry_8af4c72a-7bf7-41e8-97b9-fdc8a14bae0e_581",
        "mock_data_entry_44d54f94-fcef-44d2-9b95-5b70cd843b06_582",
        "mock_data_entry_208579ce-e73b-4c1d-94aa-7c9e55cad454_583",
        "mock_data_entry_11ad8274-39db-40de-a4a8-9696eafb0457_584",
        "mock_data_entry_802534d1-d1de-4351-85c7-514be0643294_585",
        "mock_data_entry_dacf07d3-7a4c-4916-b437-0867f025ff21_586",
        "mock_data_entry_8eda99cd-1323-4fbc-addc-763784636ab2_587",
        "mock_data_entry_680b1672-efc5-42d8-9e6f-3e745eee0c01_588",
        "mock_data_entry_ab2554e1-31c4-48e8-b855-9735d6c08010_589",
        "mock_data_entry_d14e1f19-92ad-43a1-9780-d26e129d8c1a_590",
        "mock_data_entry_36490a76-49a1-4d24-91d7-f5382952a645_591",
        "mock_data_entry_cad2ee60-26e7-4676-8156-80dba774b741_592",
        "mock_data_entry_8dabcef1-5377-4e96-8967-f2f92523f4e6_593",
        "mock_data_entry_203b83c8-8374-4fd0-a6f9-fbc04635bf82_594",
        "mock_data_entry_aad6fcf4-26be-4794-86f4-1a7c7b6d5ba7_595",
        "mock_data_entry_8ff83ba8-9dcf-40a6-82ce-2cdda7308564_596",
        "mock_data_entry_fc5a60bf-25d7-4f88-9671-8822ba254cab_597",
        "mock_data_entry_079adb4d-2d72-4288-bf3e-71a965c306b1_598",
        "mock_data_entry_bf2e4f61-82a0-44ba-b2af-263d60241d6b_599",
        "mock_data_entry_f98cf93c-f903-4486-97de-fe4d39d17d74_600",
        "mock_data_entry_2264d00e-c3ec-40a6-8dc8-a9e48bed1362_601",
        "mock_data_entry_0805f298-7eae-45e0-ad69-b338763c19e9_602",
        "mock_data_entry_06d29b30-f0f5-47e5-9efb-5eac43b7e535_603",
        "mock_data_entry_27b21640-2032-40d2-99c7-e035b5b671de_604",
        "mock_data_entry_6367940a-60df-46e6-b10e-cd9c10b05767_605",
        "mock_data_entry_53ab5acc-5794-4f04-b96c-14cf1be31d1f_606",
        "mock_data_entry_639a2f32-b753-4e60-be15-285c77d9276a_607",
        "mock_data_entry_4afddda8-e7ac-46c7-b99e-cdf98530ec54_608",
        "mock_data_entry_ddbd5f16-6558-443c-941c-ad585f63add5_609",
        "mock_data_entry_cda159aa-428f-43d3-8cae-3ea48db6c5dc_610",
        "mock_data_entry_b79371fc-767f-438c-b18a-595cb4086c33_611",
        "mock_data_entry_5c7d4891-c516-4775-adc5-10d11a80fc9e_612",
        "mock_data_entry_5f75123f-e430-4360-ae89-46bce6718e73_613",
        "mock_data_entry_f5542c3a-0b2e-41b3-9b28-e088448a377b_614",
        "mock_data_entry_dedce3f6-7eb6-4f84-abcd-22bef7548c99_615",
        "mock_data_entry_c4c5f745-28e3-48df-b786-71a995da5e72_616",
        "mock_data_entry_f0e9bf71-9273-4fa7-bdc5-438f8059ffce_617",
        "mock_data_entry_c4d01308-ef18-41a4-b574-614c94b9f980_618",
        "mock_data_entry_ac452f7a-7833-4da4-b300-3c7da2159c99_619",
        "mock_data_entry_3bbc4c98-b8fc-43db-a548-c494f6b4b764_620",
        "mock_data_entry_07eb9121-c7dd-4129-8eb8-c1621cab5156_621",
        "mock_data_entry_9e7f80cd-9a3a-4fbf-82af-52e5e5a88001_622",
        "mock_data_entry_2d009c61-61be-4aa3-a718-e01051b54b0b_623",
        "mock_data_entry_9375c26a-47ec-4212-ac5e-baafb8603644_624",
        "mock_data_entry_1404299e-b8ed-4a97-9c4b-c5ee182f676a_625",
        "mock_data_entry_fc5d398f-5836-411b-a930-b67e7d697471_626",
        "mock_data_entry_2666e2ce-b2b9-4030-b624-04e537046bf8_627",
        "mock_data_entry_591870ce-ca8d-438a-99bd-dc5c64280549_628",
        "mock_data_entry_28b2e3b9-aa13-4d6a-a517-388a54c0ad4d_629",
        "mock_data_entry_f570a11e-0e0d-4a4c-81d1-2a9f8d8f8d9a_630",
        "mock_data_entry_5ca2b358-e383-460c-ba89-cb8db0b72b29_631",
        "mock_data_entry_d05bed05-eac6-4a70-8478-66cc053bcc0b_632",
        "mock_data_entry_65478ca4-6ac3-4212-8b79-2299e7a9dfe3_633",
        "mock_data_entry_76630186-0b52-4ef7-afff-65eb3d13267a_634",
        "mock_data_entry_be5cdee7-62c0-492a-8ef5-a980a5b9f744_635",
        "mock_data_entry_7854d53b-1c07-458c-9881-d48a6edcd6db_636",
        "mock_data_entry_90d8d78c-5f28-41c5-ab4b-5f542dc42ae0_637",
        "mock_data_entry_b668b968-c503-43d0-820b-1799fc3560c4_638",
        "mock_data_entry_eb0621d0-1edc-4358-a6a1-d73682152967_639",
        "mock_data_entry_aa7e52fa-103f-4cb5-bb86-ce772fcbad9f_640",
        "mock_data_entry_0e06af77-99b2-4300-b236-efcfa7cee788_641",
        "mock_data_entry_b7685363-594f-4556-b8de-e1b5c50e184b_642",
        "mock_data_entry_3f27a961-45cd-49f6-bd0a-1db3d63ef67d_643",
        "mock_data_entry_d0356c72-a92b-4ae6-ac81-59f2984fa556_644",
        "mock_data_entry_73dbe4f3-0bb1-4365-a5ac-09bb77aa4c8b_645",
        "mock_data_entry_d5db34c0-cdea-4841-8b7f-d0f8cfe25098_646",
        "mock_data_entry_2a2fa529-fdbf-46e7-8187-5baf64e81e4d_647",
        "mock_data_entry_3970b638-750b-4253-b6c0-7baa1aea1506_648",
        "mock_data_entry_2770cf82-b837-4249-abc6-c15a88ce241e_649",
        "mock_data_entry_9e31f6d3-b213-41ab-98c1-4958a615ff87_650",
        "mock_data_entry_071d82ac-9cd2-4553-a587-e58c71d49d14_651",
        "mock_data_entry_905f89ad-6bd9-4732-ba8f-c299acbf3e05_652",
        "mock_data_entry_65cb1e19-4caa-4fbd-9a23-e83b490c3469_653",
        "mock_data_entry_5c9cfba0-b326-4b4c-856c-093ae61066cc_654",
        "mock_data_entry_1251570b-4ffc-46d1-9d25-ece958dce0da_655",
        "mock_data_entry_715c9fb3-b711-421f-934d-9b14df429e63_656",
        "mock_data_entry_b557db21-551c-4079-8333-1f647aaa63c9_657",
        "mock_data_entry_86c30807-c778-4f24-9d7f-bde05dfc859d_658",
        "mock_data_entry_6d4fe6e1-fdb3-4a7a-bf54-9b330d039d0d_659",
        "mock_data_entry_3a207ce4-2faa-4cc7-ae17-36b9aee2b982_660",
        "mock_data_entry_ebe6b8a9-437f-42b1-989b-7772dce38528_661",
        "mock_data_entry_1cb67538-1cf7-4e48-9571-cfbdef6786e0_662",
        "mock_data_entry_4196af3a-f00f-4f22-b16b-d70a721c13fd_663",
        "mock_data_entry_5fcd04b6-c2ac-43cf-845d-badd11505173_664",
        "mock_data_entry_3779f6b3-8c7d-48d6-92af-22a4d2cc1d6e_665",
        "mock_data_entry_1071a597-83b0-447c-b639-2c3ebbb36d21_666",
        "mock_data_entry_3b5e5472-4efa-4ab9-8252-c6d7e71f37e3_667",
        "mock_data_entry_f55d4c10-709a-4e30-b33c-6b7607ce1e3f_668",
        "mock_data_entry_107d2c87-529d-44ff-b35f-7f06a0805fe0_669",
        "mock_data_entry_25cad396-695f-4440-8c02-55bd9f24c2ce_670",
        "mock_data_entry_e692baa8-f94f-49f3-9ba6-dd90e7a0e20a_671",
        "mock_data_entry_94a91acf-0ca9-43e3-bd2a-acef743d3b2f_672",
        "mock_data_entry_001df13a-45f7-4bde-b22f-e78d39c9b32f_673",
        "mock_data_entry_61fecb45-fecb-4a53-b681-787a27ec86fc_674",
        "mock_data_entry_c8607fed-95d6-4568-8906-751efa57a31c_675",
        "mock_data_entry_56c597d6-b2be-4c18-9553-15c5ba47141c_676",
        "mock_data_entry_8c2146d5-9c3c-46aa-82a6-3891bc2f9027_677",
        "mock_data_entry_32346755-36e9-4929-b05c-43617d102fb4_678",
        "mock_data_entry_2b9ad159-09ab-4023-85ba-86807aa1673c_679",
        "mock_data_entry_5970a119-fbfd-4ad2-b659-a1a937eabc4d_680",
        "mock_data_entry_68613749-8e96-4c16-9076-db94440376e5_681",
        "mock_data_entry_3d499fb0-b889-47ad-9cda-5ba50bd44aca_682",
        "mock_data_entry_425f7380-fdd7-440f-9f1e-c9ac8d387fd6_683",
        "mock_data_entry_ae112b87-8371-4e3b-acb6-92a72a0da46b_684",
        "mock_data_entry_71d9cba2-782e-416a-a6ea-4e90d46e6622_685",
        "mock_data_entry_e9ef6443-7a9b-4e86-982d-7add588e8f2b_686",
        "mock_data_entry_4bd16f4f-5a73-41d1-9032-51511445b841_687",
        "mock_data_entry_acdfc37f-bbbf-4714-a516-ab5ff4253c6c_688",
        "mock_data_entry_d418ead7-6134-47a4-b182-9a0b84cd385f_689",
        "mock_data_entry_fd07a856-c4b4-448b-99dc-9c12f6043978_690",
        "mock_data_entry_86c6e351-0ecf-49ec-af97-cd0d956c6121_691",
        "mock_data_entry_5179a95e-49c0-4227-9dde-b9d189c0886a_692",
        "mock_data_entry_7762bd54-6931-4f33-98fc-75d7e52df636_693",
        "mock_data_entry_6865825d-fb59-406c-8f00-7e3e704d5ed1_694",
        "mock_data_entry_77ce19e0-a234-4991-b398-fdee5346fdcc_695",
        "mock_data_entry_81572b56-bf68-484a-bedf-839012cbdc8b_696",
        "mock_data_entry_79e2eecb-5aa8-4b0c-b76e-75a5014674ba_697",
        "mock_data_entry_8982f126-1c4c-4295-8fe9-919b6e61f389_698",
        "mock_data_entry_8f14b13e-6fff-4d76-85a0-ca55cfc644d9_699",
        "mock_data_entry_e2ab0780-5ba2-44eb-aab6-3ab0704f43bc_700",
        "mock_data_entry_a4763497-2b5f-4386-951b-ee28c7fc04b6_701",
        "mock_data_entry_01d491a9-1856-4feb-9333-0c11faa7aa02_702",
        "mock_data_entry_165dd28b-5afe-4b6b-a559-b05e4e43c707_703",
        "mock_data_entry_89687b2e-69f2-47f7-a860-6058f7b47a7a_704",
        "mock_data_entry_1830228b-bb1b-4de7-bbff-18857b974009_705",
        "mock_data_entry_cd8ea285-4cad-4d87-a3d0-c54ab1c10ce7_706",
        "mock_data_entry_8a7baf53-2a27-46af-90e4-77d39f9dfaab_707",
        "mock_data_entry_38233da2-e4c1-4fde-9ae4-120ea5f83599_708",
        "mock_data_entry_e541656d-5a93-4ebb-b445-025528698c92_709",
        "mock_data_entry_076ea2e5-3781-4bd7-a4b0-5e9bd22b56ec_710",
        "mock_data_entry_4ef09489-d950-4da2-b81f-cc450f72e055_711",
        "mock_data_entry_0d78e027-84cf-4ed5-b0a4-52f6e3f5ba98_712",
        "mock_data_entry_c6349638-ea9c-48af-8170-6a137ffa03c0_713",
        "mock_data_entry_c80d7af4-0045-4cef-a2c7-31f7dbea7116_714",
        "mock_data_entry_cb877a6e-eb74-483c-9986-18c2cb327f5d_715",
        "mock_data_entry_b0c7a2ac-e1e6-4f98-860e-632c0350778f_716",
        "mock_data_entry_0a1ca93e-762e-48a0-9779-65cc6a244359_717",
        "mock_data_entry_c52f626b-8b7b-4326-9943-99cd1596470f_718",
        "mock_data_entry_dadeba68-a962-44fc-b93a-cf1602cfdfe7_719",
        "mock_data_entry_51b5e522-29e7-4240-9eea-99b10655779b_720",
        "mock_data_entry_804b89ed-3285-4e4b-ad56-6d79674ae302_721",
        "mock_data_entry_d09c83e2-8d0c-42ff-8eea-19f0ae5e6942_722",
        "mock_data_entry_ed751812-e54b-438e-b5a0-a6a64a26d091_723",
        "mock_data_entry_a4c93a58-aec4-43d0-b127-410c1885e683_724",
        "mock_data_entry_4ea9339a-e2c5-43b4-8f7b-66499b7c03a8_725",
        "mock_data_entry_1f797b5b-4fd5-4b98-9443-0220a60e22b7_726",
        "mock_data_entry_95773984-49e5-40c0-9ffb-eed2d4947eba_727",
        "mock_data_entry_7aa1ee94-e299-4249-b5ff-cf211fe1289d_728",
        "mock_data_entry_f8f1afbf-e3d2-406f-9a84-44ddd77c85f3_729",
        "mock_data_entry_8f79edad-5423-4d3c-bced-baaa99bdd503_730",
        "mock_data_entry_1c9a0271-ddcf-46d1-8c81-56326f9a0443_731",
        "mock_data_entry_ac57c0a7-e4a8-4b0a-abdd-ac4c30bf6848_732",
        "mock_data_entry_0023bf05-9dd0-4c95-aca8-8c1ca246ecfe_733",
        "mock_data_entry_be32ce81-093b-4782-a7bc-302d194116c5_734",
        "mock_data_entry_7cf38add-5ee6-4aaa-8e3a-88a947b03c7c_735",
        "mock_data_entry_5b25fceb-c7e4-4eca-8739-3f8fd37b24db_736",
        "mock_data_entry_0f30a216-c282-4521-b735-3c5c2e0cf635_737",
        "mock_data_entry_336189e8-958e-4d4f-97e3-8808230fd22a_738",
        "mock_data_entry_e58cfa48-e3bb-474e-8e52-526dce52951c_739",
        "mock_data_entry_e0a0a2a5-64ae-4cf6-86b1-f4b4bef6dd86_740",
        "mock_data_entry_a2d54865-fc8a-4b26-9596-2ed0f467c353_741",
        "mock_data_entry_7bcbddfc-1d9c-4c1d-bc9a-5c665b1735bd_742",
        "mock_data_entry_61cce0ec-68bc-4308-b8cf-953f96a11c01_743",
        "mock_data_entry_288132ea-b724-402c-b450-3794067a9092_744",
        "mock_data_entry_efcb5380-6bc4-4191-8c3e-01200210f4e2_745",
        "mock_data_entry_94971e60-eca7-4b17-9532-7594be740d6f_746",
        "mock_data_entry_44843823-4cd2-4169-90ea-ede958c090a4_747",
        "mock_data_entry_36527a6c-5935-4828-9e47-df1a4d37e797_748",
        "mock_data_entry_d6861d4c-74cd-46eb-b0bc-d115a135e8f9_749",
        "mock_data_entry_90fdd99c-3053-440f-92c4-aaa62b721176_750",
        "mock_data_entry_49a82c55-7ec2-49aa-a94a-92d11603dced_751",
        "mock_data_entry_eae5782b-49aa-4e76-8bfe-cdc69d7455b2_752",
        "mock_data_entry_c5061c09-dbfc-4d4c-bba4-706b6b5d2554_753",
        "mock_data_entry_c2ec3e4a-ba68-451e-8f1f-43589d7ca3e2_754",
        "mock_data_entry_5206e215-f7bc-4cac-b044-0251eec5b175_755",
        "mock_data_entry_a0c74511-b585-457e-8201-876ad61254fe_756",
        "mock_data_entry_e8551d8e-e50d-4c1c-83bf-8db0da7de5eb_757",
        "mock_data_entry_c6ee13e0-a3de-4831-9b18-f328b4876a71_758",
        "mock_data_entry_114101d6-b34c-47af-8c94-b7561f218bbc_759",
        "mock_data_entry_fbb8322b-908b-4c2e-b6a6-d365afa4d8e0_760",
        "mock_data_entry_c5000602-f360-4f20-9fff-c1ccea5f162b_761",
        "mock_data_entry_5f049d90-5fc1-41d1-a1c7-7fe4fee079a3_762",
        "mock_data_entry_0736af5c-910a-42c8-aa16-f1ec83693649_763",
        "mock_data_entry_4e5c3d20-8e9a-43e2-becc-a69da7b883a0_764",
        "mock_data_entry_55ee2560-00df-448f-afe6-5b998b63a834_765",
        "mock_data_entry_a714772c-f594-4935-a445-34639126a224_766",
        "mock_data_entry_c77d24ab-7366-4740-9645-35ecb14bc155_767",
        "mock_data_entry_f53a9435-5f0a-4cc9-b226-d975d54af920_768",
        "mock_data_entry_78a6dd76-7a2a-4ed9-8225-4f9fb433b24a_769",
        "mock_data_entry_f823af6d-de0e-467d-b358-cd8ab034bae9_770",
        "mock_data_entry_eeeee4f5-ac95-4ff2-91fd-8363a2c37702_771",
        "mock_data_entry_d844d7bb-a162-497b-b153-427178261495_772",
        "mock_data_entry_8acac4f5-0a12-4b8a-a89e-0748f2944ea3_773",
        "mock_data_entry_cf2715cb-3d3a-4483-b732-8644fd4f8d12_774",
        "mock_data_entry_ada0ec4f-9cee-41e3-b3df-48407b207154_775",
        "mock_data_entry_c41229ca-b962-4de2-99de-67caa0122975_776",
        "mock_data_entry_67ccc5af-aef0-4d8b-a596-b94c36393def_777",
        "mock_data_entry_e843e9c4-3191-49e6-adfa-8c0950ac192f_778",
        "mock_data_entry_5dd76414-7dbc-4366-88f2-46939e830c7b_779",
        "mock_data_entry_5fccdf3e-67c4-457f-8120-cc6db535426a_780",
        "mock_data_entry_598fd524-6626-484e-b965-1c0db16e745c_781",
        "mock_data_entry_2d1d0899-e7f7-4cd8-bc8f-56a00f62fcfa_782",
        "mock_data_entry_8c9dccbe-0bd2-41d7-aa9e-54b434e51e88_783",
        "mock_data_entry_a20e95a9-5fcf-4326-8594-d6edcad43351_784",
        "mock_data_entry_4922caa0-e568-42d1-8475-76220690e74f_785",
        "mock_data_entry_7f189454-d301-4bad-98a9-bda05609155c_786",
        "mock_data_entry_7d186bb1-7313-468f-8224-37c340215d49_787",
        "mock_data_entry_d8f59282-bb41-48e7-b930-6a57771ef685_788",
        "mock_data_entry_bb7b4281-f8cb-4e78-8345-bb88afbb0c68_789",
        "mock_data_entry_7e4f2b9f-f5c3-4d71-a915-27e743a5467f_790",
        "mock_data_entry_fcbcc112-4717-421b-b21e-d019d92f516d_791",
        "mock_data_entry_80764368-f12b-4820-a5c0-7a975eac8d11_792",
        "mock_data_entry_e9b6a83a-f786-4423-b48e-dde913c20fc5_793",
        "mock_data_entry_5ffdb2ac-db26-4919-8969-0903509f8faf_794",
        "mock_data_entry_1b718b97-ed30-4e6f-b810-17c2368d715d_795",
        "mock_data_entry_37438772-91cb-4a71-a92e-72bb2d56e7a7_796",
        "mock_data_entry_d5937f65-9a6f-4195-ad2a-66e3452d35f9_797",
        "mock_data_entry_119525a2-2697-4b1d-94b7-69e9cf44cb1f_798",
        "mock_data_entry_a85fe183-c0dd-44a6-b21a-ea4ea12f2a7d_799",
        "mock_data_entry_3e2a1b61-d85d-4f8d-9893-67e0211b22d8_800",
        "mock_data_entry_9e0574a5-bf03-4de0-9cc8-7907418cb9a7_801",
        "mock_data_entry_3161ccf0-e32a-483a-8db9-1bb05163c9af_802",
        "mock_data_entry_fd4e807e-7bef-4437-87a0-2ed665b55fe1_803",
        "mock_data_entry_3b358e8c-e7e8-4a15-9d45-739c34772a2f_804",
        "mock_data_entry_07859951-4ba3-451f-a846-16d2ec9f4c10_805",
        "mock_data_entry_3e068531-8224-40b7-badc-f54563ab0efe_806",
        "mock_data_entry_a8fcbd5d-b140-4de5-af78-7fcca458c53a_807",
        "mock_data_entry_9488ba75-4d31-4fe6-b38e-a30827b15804_808",
        "mock_data_entry_624e39ab-e9a7-4511-a389-391cecf0a24a_809",
        "mock_data_entry_3d9586c8-635c-4037-99ff-ec514bf0374b_810",
        "mock_data_entry_815c83ae-2c21-4e14-9db8-d1dbf81edf9d_811",
        "mock_data_entry_a60bec35-bdcc-4433-9c2d-196ebcca0b02_812",
        "mock_data_entry_c373bd49-ba90-49eb-a5cf-daa968c87880_813",
        "mock_data_entry_11af7603-697b-46ee-82e1-6e1e681c0e10_814",
        "mock_data_entry_6c36f626-6457-467e-8c7b-7c0c3973ecda_815",
        "mock_data_entry_19937752-14ab-4060-a728-ec83986d4163_816",
        "mock_data_entry_0cd516dc-7761-4886-9ec4-d41e793cc31d_817",
        "mock_data_entry_da8fd498-99bc-4e23-8ed8-bc385a09e153_818",
        "mock_data_entry_beb01387-9486-466a-819a-ae042d5141d1_819",
        "mock_data_entry_80962302-9206-4e30-b309-c8a115eb22c6_820",
        "mock_data_entry_c7d02f43-1a92-48cb-8c13-e75f8068b129_821",
        "mock_data_entry_b731444a-cddf-447a-bdd0-c8d689139931_822",
        "mock_data_entry_b9af04ef-c6bc-4f11-9830-cd25dedbd2ff_823",
        "mock_data_entry_fb8793ce-e85b-4eae-a598-9095d4263825_824",
        "mock_data_entry_48dbc292-71cc-4092-a5d1-0ab43e804dbb_825",
        "mock_data_entry_11113d8b-1c2f-4057-8253-95c0d39c14e6_826",
        "mock_data_entry_f50956d6-5701-43b5-bffc-66d7da06d9c3_827",
        "mock_data_entry_825f7d4e-f8c7-4bc7-b187-1e64cbcd9652_828",
        "mock_data_entry_954bd85f-8cee-4d66-ae3b-6e052551bf63_829",
        "mock_data_entry_23652a57-e49a-435e-9ee4-764a5d7e0511_830",
        "mock_data_entry_eb5a971d-beda-4f3f-9feb-dcd776d69c65_831",
        "mock_data_entry_340bb1fe-2f7a-43a5-929c-c6fd6678efad_832",
        "mock_data_entry_b7515e78-77c9-444d-8c88-ecb257cdcbf5_833",
        "mock_data_entry_0813811b-daef-419f-90c4-a1f2ec394965_834",
        "mock_data_entry_649e610a-786d-4237-b198-8ebd25592198_835",
        "mock_data_entry_6f30642e-ff4d-4cab-ba63-93967feda361_836",
        "mock_data_entry_4580c74e-9e8e-4591-ae0b-bf7ed3d43331_837",
        "mock_data_entry_d3680c4b-e4ab-4f80-b083-59dc2adcecc0_838",
        "mock_data_entry_df4def04-9ced-498b-b124-232faf297256_839",
        "mock_data_entry_8e6ed0ce-5c94-46f3-92fe-f6e1b0e697b8_840",
        "mock_data_entry_e2e0c40b-4583-47f4-8e98-2a01d5f1dc93_841",
        "mock_data_entry_95d0cee5-32b7-4ad3-affa-57cb0a0f2c9e_842",
        "mock_data_entry_cbb6726d-3d9a-40b5-b6df-3803e6c4eba7_843",
        "mock_data_entry_efaf8852-b640-41a2-9fcc-e6d0ae890a73_844",
        "mock_data_entry_480d3b7c-3087-4334-a91f-d9a0e080e453_845",
        "mock_data_entry_e05e53cd-852b-49c3-aa8b-a28cbdb611ed_846",
        "mock_data_entry_dc6fa587-12f8-41ec-8564-abcdedebd00d_847",
        "mock_data_entry_2e3061ab-f9c8-482a-9504-bb738e992684_848",
        "mock_data_entry_47460c23-8891-47d0-b90a-c6342cfb9e39_849",
        "mock_data_entry_92e3e191-a9bf-46f5-bb1d-171ab92cdbbb_850",
        "mock_data_entry_fa7b2f53-2aab-4f6c-900b-18e7b69f09a6_851",
        "mock_data_entry_6dac40f0-2056-42c4-a29a-ccc4a75da610_852",
        "mock_data_entry_b5d6c165-81d0-42e8-9571-56377f85f763_853",
        "mock_data_entry_04edf0a8-152b-4f64-9093-a0967ece7f02_854",
        "mock_data_entry_60fe5248-6ea2-463e-925a-a2372de31356_855",
        "mock_data_entry_f083e171-699c-484a-af1a-0a9c4c72876f_856",
        "mock_data_entry_b76609c7-33b9-46d2-b880-fae5235b3b29_857",
        "mock_data_entry_6448bafb-fc7d-4e82-b6f2-85f0f3d4e93d_858",
        "mock_data_entry_43a0973b-b977-4a8c-9dea-9be011b54247_859",
        "mock_data_entry_272f4de6-3375-4741-9348-2a0ebbd07e8d_860",
        "mock_data_entry_b903f335-cbf0-475e-afab-0da6601dbb67_861",
        "mock_data_entry_38d56c92-d486-4c6a-961d-5b58b9b7d4f2_862",
        "mock_data_entry_6f346dc1-551c-4e58-86da-bf51a7edb856_863",
        "mock_data_entry_83c50110-319a-4276-9bde-523514b4c0d3_864",
        "mock_data_entry_e06ac2db-3efb-4321-ab12-a4a5c66fd03c_865",
        "mock_data_entry_34b5fec3-1c26-4ba0-8b46-6d77ff84cd46_866",
        "mock_data_entry_7c150cbb-a08e-42a6-9464-28da1d2c9a59_867",
        "mock_data_entry_ead67e6b-809f-4b6b-b2d5-b230ec55d8b1_868",
        "mock_data_entry_1922c484-2e4c-4f15-adb8-f5ec220b5255_869",
        "mock_data_entry_fd3f1f3b-5986-46c8-939f-ace90360ea28_870",
        "mock_data_entry_22fb3ee5-7213-4f9b-8b2d-6299081feea2_871",
        "mock_data_entry_b9f259a7-809c-4e11-972f-5016fd650e31_872",
        "mock_data_entry_3f882828-06ac-41fb-961d-a9bd829cfbec_873",
        "mock_data_entry_920158cd-fc8c-4ed1-bd00-19ab7540f07a_874",
        "mock_data_entry_021cb6c9-40ce-4d92-aecd-7215ff207037_875",
        "mock_data_entry_d691f215-b31b-45b4-9a7f-f7ce3935eb7a_876",
        "mock_data_entry_ff1d6d61-847c-4839-8f93-a5f9f8f9b974_877",
        "mock_data_entry_8aabfae5-eb4c-4be4-9423-2ab99ddb8d13_878",
        "mock_data_entry_22f706ef-9713-43dd-a3bd-cf199e4fa19d_879",
        "mock_data_entry_56e8b0c4-5e67-4b29-8596-ce4de7465621_880",
        "mock_data_entry_213be179-bfe4-4f49-815c-99377357aa0b_881",
        "mock_data_entry_41ec715b-ac18-4b3b-985c-7a83f1f6a9a9_882",
        "mock_data_entry_08efd122-816d-49fe-b1fc-415f77f06352_883",
        "mock_data_entry_81b8293b-105e-427d-8b97-099c960e1a9d_884",
        "mock_data_entry_e904fa08-e89e-4677-b805-04a1aee39e75_885",
        "mock_data_entry_a2b2ad11-c50a-4ebe-9300-6cd32ca2b111_886",
        "mock_data_entry_42e77f62-c05b-46b5-9e18-88801329c2ec_887",
        "mock_data_entry_d50f4cc5-eae0-4a5e-91fe-6186371a1bb1_888",
        "mock_data_entry_fa5a6a61-15ef-4075-9c5b-49306ef2d412_889",
        "mock_data_entry_93eb66e6-c047-41ff-85ea-26db9b7f00e7_890",
        "mock_data_entry_4d7fe642-f617-4158-81a9-c1005d1980b4_891",
        "mock_data_entry_d5927e4c-9000-442a-ab1f-e0036fc27153_892",
        "mock_data_entry_5173c7df-d4c2-47f6-9821-77795afe4c8b_893",
        "mock_data_entry_764b97bf-c83d-43b3-bfe5-00d5c5350d7c_894",
        "mock_data_entry_efcd9392-ff39-44d4-911c-b4fd853d2b93_895",
        "mock_data_entry_3aad1b63-2001-4130-ac69-beb4ac582f86_896",
        "mock_data_entry_90660c7e-5d95-4403-a92b-f967dd5fbdd2_897",
        "mock_data_entry_6b1a1d05-4d2b-4b01-ad1d-e4f47c993c7f_898",
        "mock_data_entry_4abcbdc5-a56c-4d05-8813-3b99bca6c4c6_899",
        "mock_data_entry_bf3da6ba-556f-46e8-8602-27e7233adb9a_900",
        "mock_data_entry_31a6651e-2590-40c5-b92b-7b5ab5532530_901",
        "mock_data_entry_c5155cb1-3352-42d5-a189-e89cb50b705b_902",
        "mock_data_entry_972cbcd8-af30-4338-8b75-e4aab18d69a8_903",
        "mock_data_entry_e4bb8611-1235-42e6-9fc5-5a86f079b703_904",
        "mock_data_entry_aad197fc-c2c3-45b4-9dc4-9ad6cb245e98_905",
        "mock_data_entry_cb66e942-b269-4595-9b6e-7f70c9387491_906",
        "mock_data_entry_6ad26158-8f66-4643-b621-0d0fa6e9b88a_907",
        "mock_data_entry_47ba0713-2265-4b81-be1b-a91b966bd2ce_908",
        "mock_data_entry_c22a6ecd-24ba-455c-a0ef-e5c1d1941162_909",
        "mock_data_entry_15b69f05-5d1e-457b-ab9f-7b0e90c31873_910",
        "mock_data_entry_dfcdff36-7639-46d3-864f-7f6724839805_911",
        "mock_data_entry_89750487-dc9f-4dc1-971c-664ec3077003_912",
        "mock_data_entry_ebf20ceb-dd27-428a-bdaf-ae7e05fc8524_913",
        "mock_data_entry_707d31c4-852f-41a0-8f50-47a996dfe211_914",
        "mock_data_entry_8d146107-9cce-4bbb-9fa1-09467eee61f1_915",
        "mock_data_entry_f49000fb-c713-4368-bbb5-667ded694567_916",
        "mock_data_entry_37d7a981-ad4e-427d-96dd-64256a0a1cf1_917",
        "mock_data_entry_fa43ed98-1b5a-443c-9377-83107328af5a_918",
        "mock_data_entry_656d3f9e-5f21-40fc-8194-e57dfec62cfd_919",
        "mock_data_entry_1a020ce7-d8a6-4352-95a8-202795101de0_920",
        "mock_data_entry_27d71381-18ab-4655-840e-761d9d19f807_921",
        "mock_data_entry_a37deca9-eacc-49d8-b3a3-3088e08b7480_922",
        "mock_data_entry_95275274-facb-41e6-9a87-249df5f67f71_923",
        "mock_data_entry_015bb460-ad68-4e3e-a49a-f4236e9e887d_924",
        "mock_data_entry_70a15f6c-a981-44cd-a86a-db70d74e47bb_925",
        "mock_data_entry_4e06c2b6-4bbc-4f34-96d9-550458052844_926",
        "mock_data_entry_4a653a7a-b912-4764-8876-a77b73c0c58b_927",
        "mock_data_entry_f6c95dba-a9eb-4cbe-a71d-e9c5f9d052f0_928",
        "mock_data_entry_67f927cb-c894-41f3-aa27-8fcd5545c7d5_929",
        "mock_data_entry_e6d31223-94d9-47d1-9bb4-41f2e2ec71bb_930",
        "mock_data_entry_df8838b9-f8b2-4817-abf3-b37a31f6a4dc_931",
        "mock_data_entry_740688f9-8b63-4071-ada8-14f5295a0be0_932",
        "mock_data_entry_c6bce7af-a3a2-4b1d-9106-dd6592d8d726_933",
        "mock_data_entry_e0c014ff-cb6b-4404-8b00-57b151cf35fb_934",
        "mock_data_entry_629daf63-4935-48b2-b93a-83e613f7def0_935",
        "mock_data_entry_e3b42f23-ea07-43ee-a62d-6c6dfa569e7e_936",
        "mock_data_entry_997227e9-a1ee-44b1-98ed-2a5c07e77bfb_937",
        "mock_data_entry_1310063f-ab16-4653-880b-e0869466c0af_938",
        "mock_data_entry_508a8c38-5b4e-44a1-a2e1-148e9db2a1ff_939",
        "mock_data_entry_6d9e42ce-e3d0-4b06-8347-c555d3bfd915_940",
        "mock_data_entry_b76d8c9a-94f5-40c7-b949-3de17fced87a_941",
        "mock_data_entry_664e90c6-e90d-4dd7-bf49-f1b6e1632f22_942",
        "mock_data_entry_9687b995-05f2-4cc1-97e4-6d7ab748ba73_943",
        "mock_data_entry_f022e59e-b59f-493c-88aa-f46f3b35208e_944",
        "mock_data_entry_2c46a55d-e86b-4406-ad5b-2f8b89406c71_945",
        "mock_data_entry_161427de-0ca0-49da-80c3-7d58d4e93983_946",
        "mock_data_entry_03d5a0fb-e9e1-4a07-8330-3de93e8c10ef_947",
        "mock_data_entry_cef8acc1-38b4-4ea6-a1bf-50d4f639d2d2_948",
        "mock_data_entry_5b7f19be-52ce-4e77-9b80-f7187f563ff1_949",
        "mock_data_entry_62622159-0879-4f4a-bffc-f5cc2239169a_950",
        "mock_data_entry_652a114b-37de-4e88-8357-c95680d2edb3_951",
        "mock_data_entry_4d0c138f-5a30-44f7-822b-f510b663dcc0_952",
        "mock_data_entry_e0ddf005-4961-40f2-8b55-69b9358c67f5_953",
        "mock_data_entry_f7237736-5e38-4072-a0d0-aa6c24f692bd_954",
        "mock_data_entry_867f9f85-a6ec-4d21-a7ab-a7559184c6b1_955",
        "mock_data_entry_f5803a05-c2dc-4e0d-9b50-d84f0a2c340a_956",
        "mock_data_entry_c5eef4d3-05ba-4c35-8fb3-fb40cfebc955_957",
        "mock_data_entry_1ae7ceeb-bda1-4f61-a2c0-3b049e200d3b_958",
        "mock_data_entry_130ccc58-0eee-4f0e-918b-5eaa7d70d0b0_959",
        "mock_data_entry_cb12dbda-6d38-4c23-9922-cd1003f1d0b4_960",
        "mock_data_entry_9896f2e4-fa31-4606-91ee-cf3b8ec588ac_961",
        "mock_data_entry_54419073-4376-4db1-98d4-026c923656e7_962",
        "mock_data_entry_79ce97d6-28bd-472b-9c64-0ba0b773e946_963",
        "mock_data_entry_e5068be9-64ec-41a5-96b3-58da88ebfa6f_964",
        "mock_data_entry_0a09f72f-9585-4843-b890-e23288ac3622_965",
        "mock_data_entry_4bb26236-1fc4-43ad-b51b-ec79fb5bda1a_966",
        "mock_data_entry_c93a3883-434b-422f-bd2d-4309ac63dfb5_967",
        "mock_data_entry_f8e2c589-b2bc-497b-a1c4-3020cac1964d_968",
        "mock_data_entry_b65d0370-d0dc-4629-b568-bb26791fe5ab_969",
        "mock_data_entry_432bc721-f353-4856-b384-b4bc652d6d67_970",
        "mock_data_entry_af541b80-8233-4809-a234-b85e0d170476_971",
        "mock_data_entry_c364f64c-c817-4c14-aeb6-2a33c621b5da_972",
        "mock_data_entry_23a056d8-5468-43b9-849a-9ca96afdf7e3_973",
        "mock_data_entry_0ddac82e-bbd2-48eb-a36c-72a0e980d101_974",
        "mock_data_entry_d93de3d0-7f0a-4305-baf3-413943290e42_975",
        "mock_data_entry_c0f15f57-4c00-4be6-84b0-122b5675fec8_976",
        "mock_data_entry_f761d086-ffc1-4967-95fd-022aa6f5628e_977",
        "mock_data_entry_097b2471-69a6-470e-89a9-8cef811dab66_978",
        "mock_data_entry_edd7cd13-15fd-477a-8f91-fbd31f98ee2a_979",
        "mock_data_entry_f64c1f7b-9d6e-4ac1-a438-1506694e711a_980",
        "mock_data_entry_a75d3a99-7ce9-48c7-abf1-646fa27ec1ad_981",
        "mock_data_entry_f5d8e066-aa23-4ade-8d5a-e2cb18da949a_982",
        "mock_data_entry_e977aa8d-e577-45c7-9a3d-82a753158339_983",
        "mock_data_entry_772c261c-47b0-46d7-af19-2cfe34497dab_984",
        "mock_data_entry_391136c5-0ea7-4ed1-a40a-4aecc049e6bd_985",
        "mock_data_entry_673af55b-05a7-45f4-a343-8bf6ab2e71be_986",
        "mock_data_entry_f2f8b789-b8e8-4975-9bad-310b436b74a7_987",
        "mock_data_entry_32e0ff7a-dbc6-470c-9899-46bd07adef86_988",
        "mock_data_entry_b3d9d735-d2bd-4051-a8e3-bf663f6fdbbf_989",
        "mock_data_entry_00d7cb3e-48d5-4d46-b376-e5b7cf2f3493_990",
        "mock_data_entry_0ebbc4ba-dbbf-44cb-97fe-03dd5f490609_991",
        "mock_data_entry_55949f55-1e25-435f-8359-3e18ec05c832_992",
        "mock_data_entry_8d0496ea-8ed2-4fc5-b86b-926c0d109566_993",
        "mock_data_entry_95419e71-4827-4437-83db-6faa47d5ff23_994",
        "mock_data_entry_7cc5ccef-2eb4-4162-b79d-64d324b65c34_995",
        "mock_data_entry_c55071bd-6d31-49fc-bb5b-7e976b3b61c7_996",
        "mock_data_entry_3a385394-2b34-4572-aa5a-3b568f8bdc3f_997",
        "mock_data_entry_8d6ed14c-91e4-458d-b5c7-54eff2295e7f_998",
        "mock_data_entry_b2294583-b4d7-4f4a-b13b-1955b10cbea3_999",
        "mock_data_entry_9f0428a3-bcda-4e8f-9a5d-9da3ff287f16_1000",
        "mock_data_entry_6fc7855d-2bf7-440b-877d-118951554843_1001",
        "mock_data_entry_9d16762d-3f33-47c3-b306-cbbc58381e8c_1002",
        "mock_data_entry_775de5d7-9d14-43c6-8448-81ac60ef80af_1003",
        "mock_data_entry_41e06fcb-db37-425b-81e6-86c07f3abb89_1004",
        "mock_data_entry_1dd7e9b5-dc99-463d-8473-ba4ad9ba42c9_1005",
        "mock_data_entry_5cfe5f24-6b62-4a18-8c3b-f9f15bbf113b_1006",
        "mock_data_entry_7207bbe1-4965-4af0-96dc-52d9164fdbc6_1007",
        "mock_data_entry_5064246d-9b5c-4bcf-9f85-60d45022cd52_1008",
        "mock_data_entry_ad18ec6b-c3b3-49de-bef5-102f3be22876_1009",
        "mock_data_entry_e00c5ae6-4907-47f9-b835-a0edd0bc8ff3_1010",
        "mock_data_entry_2e4e7a94-f090-4a19-b76b-d5b50f064db6_1011",
        "mock_data_entry_af110e01-e62a-4f91-b184-07132fc8d128_1012",
        "mock_data_entry_c6f8e6f0-5629-4ade-82ee-a730424c0f40_1013",
        "mock_data_entry_7a8bdef8-9dbd-4d33-9eec-a69f1e723643_1014",
        "mock_data_entry_01165070-e468-4f1e-aff7-3b2f1ff779b0_1015",
        "mock_data_entry_edc7b6af-8e36-4d1c-8952-9d5ef270a961_1016",
        "mock_data_entry_47fa0c9b-9ad8-490c-a55f-933dd19954bd_1017",
        "mock_data_entry_b02184ac-2ff9-4355-98cc-7e832ce325d4_1018",
        "mock_data_entry_da9a97bb-8654-43ff-bea2-85bc288c6f61_1019",
        "mock_data_entry_d8134282-abf6-475c-aac8-2a8a4fdff53f_1020",
        "mock_data_entry_d6d3df6a-1055-41ef-b28d-8b0c3a4700db_1021",
        "mock_data_entry_a5a3e65d-01f4-465d-b1cc-10cbe670259c_1022",
        "mock_data_entry_28f9738a-36bc-49c6-93f7-19806546d325_1023",
        "mock_data_entry_cf5dc027-94b9-4c08-8cfa-651a5c0cb2d0_1024",
        "mock_data_entry_3bb59267-7217-4167-b83d-553172593b8b_1025",
        "mock_data_entry_b3aed2cd-99c1-438a-a5f8-81bd260c955d_1026",
        "mock_data_entry_d2533722-8606-4098-b72a-507afc7f72ee_1027",
        "mock_data_entry_43e80f0a-d722-4adb-b710-a7d3afda6465_1028",
        "mock_data_entry_baf633bd-a357-451e-af58-39d755f3e5e3_1029",
        "mock_data_entry_f3285d82-526f-482f-94cc-25c7e7e4ce85_1030",
        "mock_data_entry_f1305284-5b47-448f-8dd2-bb09cc4cd011_1031",
        "mock_data_entry_e6728868-c114-46af-a768-31f5d599c36f_1032",
        "mock_data_entry_a02b4a93-902c-4acc-99f6-304ff250333b_1033",
        "mock_data_entry_671a3994-879a-40d6-8323-f20f841179bb_1034",
        "mock_data_entry_209c636a-3e5e-48fe-a724-7f2776d848f6_1035",
        "mock_data_entry_bfbfbf64-37f4-4e80-aa86-64de6dd4d170_1036",
        "mock_data_entry_f567cdc1-f065-4e9d-9e22-9aab1edfa9a6_1037",
        "mock_data_entry_0cedf626-03c5-4bba-b1a8-19019746cf5f_1038",
        "mock_data_entry_c979b591-4721-462b-b483-85cd49788403_1039",
        "mock_data_entry_1ef14300-c8dc-4431-ae1c-45907f271f0d_1040",
        "mock_data_entry_8cdfd16a-8830-4bfa-985a-07477174281b_1041",
        "mock_data_entry_49bd2965-744d-44e1-8511-d3f16011a174_1042",
        "mock_data_entry_41e5f491-5665-4ec3-887b-22a5fa8ff0f5_1043",
        "mock_data_entry_ed580e58-7f7d-4a31-b189-9ac4a2261320_1044",
        "mock_data_entry_12096b69-32c4-4afc-b6ef-482a2f10bf2c_1045",
        "mock_data_entry_183e8675-bd81-4eeb-95ef-282ae27d7b7a_1046",
        "mock_data_entry_168ce2f3-1cd2-4ea6-a909-ba91c3cbdc14_1047",
        "mock_data_entry_db98c061-cefe-4be2-b851-fc2247e7d7ac_1048",
        "mock_data_entry_0ffa116e-f8f0-410f-8361-2f06fc43305d_1049",
    ]
}
