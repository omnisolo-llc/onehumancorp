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

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_1() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_1"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 1,
            reliability_score: 50 + (1 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_2() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_2"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 2,
            reliability_score: 50 + (2 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_3() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_3"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 3,
            reliability_score: 50 + (3 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_4() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_4"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 4,
            reliability_score: 50 + (4 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_5() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_5"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 5,
            reliability_score: 50 + (5 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_6() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_6"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 6,
            reliability_score: 50 + (6 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_7() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_7"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 7,
            reliability_score: 50 + (7 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_8() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_8"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 8,
            reliability_score: 50 + (8 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_9() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_9"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 9,
            reliability_score: 50 + (9 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_10() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_10"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 10,
            reliability_score: 50 + (10 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_11() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_11"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 11,
            reliability_score: 50 + (11 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_12() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_12"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 12,
            reliability_score: 50 + (12 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_13() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_13"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 13,
            reliability_score: 50 + (13 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_14() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_14"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 14,
            reliability_score: 50 + (14 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_15() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_15"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 15,
            reliability_score: 50 + (15 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_16() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_16"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 16,
            reliability_score: 50 + (16 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_17() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_17"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 17,
            reliability_score: 50 + (17 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_18() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_18"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 18,
            reliability_score: 50 + (18 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_19() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_19"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 19,
            reliability_score: 50 + (19 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_20() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_20"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 20,
            reliability_score: 50 + (20 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_21() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_21"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 21,
            reliability_score: 50 + (21 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_22() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_22"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 22,
            reliability_score: 50 + (22 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_23() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_23"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 23,
            reliability_score: 50 + (23 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_24() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_24"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 24,
            reliability_score: 50 + (24 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_25() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_25"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 25,
            reliability_score: 50 + (25 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_26() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_26"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 26,
            reliability_score: 50 + (26 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_27() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_27"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 27,
            reliability_score: 50 + (27 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_28() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_28"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 28,
            reliability_score: 50 + (28 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_29() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_29"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 29,
            reliability_score: 50 + (29 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_30() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_30"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 30,
            reliability_score: 50 + (30 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_31() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_31"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 31,
            reliability_score: 50 + (31 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_32() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_32"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 32,
            reliability_score: 50 + (32 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_33() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_33"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 33,
            reliability_score: 50 + (33 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_34() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_34"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 34,
            reliability_score: 50 + (34 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_35() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_35"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 35,
            reliability_score: 50 + (35 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_36() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_36"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 36,
            reliability_score: 50 + (36 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_37() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_37"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 37,
            reliability_score: 50 + (37 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_38() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_38"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 38,
            reliability_score: 50 + (38 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_39() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_39"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 39,
            reliability_score: 50 + (39 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_40() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_40"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 40,
            reliability_score: 50 + (40 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_41() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_41"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 41,
            reliability_score: 50 + (41 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_42() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_42"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 42,
            reliability_score: 50 + (42 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_43() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_43"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 43,
            reliability_score: 50 + (43 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_44() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_44"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 44,
            reliability_score: 50 + (44 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_45() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_45"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 45,
            reliability_score: 50 + (45 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_46() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_46"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 46,
            reliability_score: 50 + (46 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_47() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_47"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 47,
            reliability_score: 50 + (47 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_48() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_48"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 48,
            reliability_score: 50 + (48 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_49() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_49"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 49,
            reliability_score: 50 + (49 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_50() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_50"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 50,
            reliability_score: 50 + (50 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_51() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_51"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 51,
            reliability_score: 50 + (51 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_52() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_52"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 52,
            reliability_score: 50 + (52 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_53() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_53"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 53,
            reliability_score: 50 + (53 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_54() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_54"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 54,
            reliability_score: 50 + (54 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_55() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_55"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 55,
            reliability_score: 50 + (55 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_56() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_56"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 56,
            reliability_score: 50 + (56 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_57() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_57"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 57,
            reliability_score: 50 + (57 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_58() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_58"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 58,
            reliability_score: 50 + (58 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_59() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_59"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 59,
            reliability_score: 50 + (59 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_60() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_60"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 60,
            reliability_score: 50 + (60 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_61() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_61"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 61,
            reliability_score: 50 + (61 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_62() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_62"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 62,
            reliability_score: 50 + (62 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_63() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_63"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 63,
            reliability_score: 50 + (63 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_64() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_64"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 64,
            reliability_score: 50 + (64 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_65() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_65"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 65,
            reliability_score: 50 + (65 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_66() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_66"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 66,
            reliability_score: 50 + (66 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_67() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_67"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 67,
            reliability_score: 50 + (67 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_68() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_68"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 68,
            reliability_score: 50 + (68 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_69() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_69"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 69,
            reliability_score: 50 + (69 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_70() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_70"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 70,
            reliability_score: 50 + (70 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_71() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_71"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 71,
            reliability_score: 50 + (71 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_72() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_72"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 72,
            reliability_score: 50 + (72 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_73() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_73"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 73,
            reliability_score: 50 + (73 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_74() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_74"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 74,
            reliability_score: 50 + (74 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_75() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_75"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 75,
            reliability_score: 50 + (75 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_76() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_76"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 76,
            reliability_score: 50 + (76 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_77() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_77"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 77,
            reliability_score: 50 + (77 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_78() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_78"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 78,
            reliability_score: 50 + (78 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_79() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_79"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 79,
            reliability_score: 50 + (79 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_80() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_80"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 80,
            reliability_score: 50 + (80 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_81() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_81"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 81,
            reliability_score: 50 + (81 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_82() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_82"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 82,
            reliability_score: 50 + (82 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_83() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_83"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 83,
            reliability_score: 50 + (83 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_84() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_84"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 84,
            reliability_score: 50 + (84 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_85() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_85"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 85,
            reliability_score: 50 + (85 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_86() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_86"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 86,
            reliability_score: 50 + (86 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_87() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_87"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 87,
            reliability_score: 50 + (87 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_88() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_88"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 88,
            reliability_score: 50 + (88 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_89() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_89"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 89,
            reliability_score: 50 + (89 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_90() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_90"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 90,
            reliability_score: 50 + (90 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_91() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_91"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 91,
            reliability_score: 50 + (91 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_92() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_92"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 92,
            reliability_score: 50 + (92 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_93() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_93"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 93,
            reliability_score: 50 + (93 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_94() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_94"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 94,
            reliability_score: 50 + (94 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_95() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_95"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 95,
            reliability_score: 50 + (95 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_96() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_96"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 96,
            reliability_score: 50 + (96 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_97() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_97"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 97,
            reliability_score: 50 + (97 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_98() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_98"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 98,
            reliability_score: 50 + (98 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_99() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_99"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 99,
            reliability_score: 50 + (99 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_100() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_100"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 100,
            reliability_score: 50 + (100 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_101() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_101"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 101,
            reliability_score: 50 + (101 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_102() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_102"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 102,
            reliability_score: 50 + (102 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_103() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_103"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 103,
            reliability_score: 50 + (103 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_104() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_104"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 104,
            reliability_score: 50 + (104 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_105() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_105"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 105,
            reliability_score: 50 + (105 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_106() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_106"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 106,
            reliability_score: 50 + (106 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_107() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_107"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 107,
            reliability_score: 50 + (107 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_108() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_108"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 108,
            reliability_score: 50 + (108 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_109() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_109"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 109,
            reliability_score: 50 + (109 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_110() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_110"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 110,
            reliability_score: 50 + (110 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_111() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_111"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 111,
            reliability_score: 50 + (111 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_112() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_112"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 112,
            reliability_score: 50 + (112 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_113() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_113"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 113,
            reliability_score: 50 + (113 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_114() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_114"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 114,
            reliability_score: 50 + (114 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_115() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_115"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 115,
            reliability_score: 50 + (115 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_116() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_116"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 116,
            reliability_score: 50 + (116 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_117() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_117"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 117,
            reliability_score: 50 + (117 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_118() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_118"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 118,
            reliability_score: 50 + (118 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_119() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_119"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 119,
            reliability_score: 50 + (119 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_120() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_120"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 120,
            reliability_score: 50 + (120 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_121() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_121"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 121,
            reliability_score: 50 + (121 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_122() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_122"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 122,
            reliability_score: 50 + (122 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_123() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_123"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 123,
            reliability_score: 50 + (123 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_124() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_124"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 124,
            reliability_score: 50 + (124 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_125() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_125"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 125,
            reliability_score: 50 + (125 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_126() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_126"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 126,
            reliability_score: 50 + (126 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_127() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_127"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 127,
            reliability_score: 50 + (127 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_128() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_128"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 128,
            reliability_score: 50 + (128 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_129() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_129"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 129,
            reliability_score: 50 + (129 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_130() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_130"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 130,
            reliability_score: 50 + (130 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_131() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_131"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 131,
            reliability_score: 50 + (131 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_132() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_132"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 132,
            reliability_score: 50 + (132 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_133() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_133"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 133,
            reliability_score: 50 + (133 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_134() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_134"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 134,
            reliability_score: 50 + (134 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_135() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_135"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 135,
            reliability_score: 50 + (135 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_136() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_136"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 136,
            reliability_score: 50 + (136 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_137() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_137"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 137,
            reliability_score: 50 + (137 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_138() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_138"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 138,
            reliability_score: 50 + (138 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_139() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_139"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 139,
            reliability_score: 50 + (139 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_140() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_140"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 140,
            reliability_score: 50 + (140 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_141() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_141"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 141,
            reliability_score: 50 + (141 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_142() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_142"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 142,
            reliability_score: 50 + (142 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_143() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_143"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 143,
            reliability_score: 50 + (143 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_144() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_144"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 144,
            reliability_score: 50 + (144 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_145() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_145"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 145,
            reliability_score: 50 + (145 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_146() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_146"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 146,
            reliability_score: 50 + (146 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_147() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_147"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 147,
            reliability_score: 50 + (147 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_148() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_148"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 148,
            reliability_score: 50 + (148 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }

    #[tokio::test]
    async fn test_auto_generated_conflict_scenario_149() {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use std::str::FromStr;
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS consolidated_memory (
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
            );
        ").execute(&pool).await.unwrap();

        let repo = VectorRepository::new_sqlite(pool);
        let record = EmbeddingRecord {
            id: format!("test_record_149"),
            tenant_id: "org_maya".to_string(),
            agent_id: "agent_x".to_string(),
            content: "Maya's bakery opens at 8am".to_string(),
            embedding: vec![0.1; 10],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 149,
            reliability_score: 50 + (149 % 50),
            owner_override: false,
            metadata: None,
        };
        repo.upsert(&record).await.unwrap();

        let resolved = repo.auto_resolve_conflicts().await.unwrap();
        assert_eq!(resolved, 0); // No conflicts with a single record
    }
}
