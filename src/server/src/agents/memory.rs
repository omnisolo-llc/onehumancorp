use chrono::{DateTime, Utc};
use sqlx::Row;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmbeddingRecord {
    pub id: String,
    pub organization_id: String,
    pub agent_id: String,
    pub content: String,
    pub embedding: Vec<f32>,
    pub source_type: String,
    pub created_at: DateTime<Utc>,
}

#[allow(dead_code)]
pub struct VectorRepository {
    pool: sqlx::PgPool,
}

#[allow(dead_code)]
impl VectorRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        VectorRepository { pool }
    }

    pub async fn upsert(&self, record: &EmbeddingRecord) -> Result<(), String> {
        let emb_str = serde_json::to_string(&record.embedding).map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type, created_at) \
             VALUES ($1, $2, $3, $4, $5::vector, $6, $7) \
             ON CONFLICT(id) DO UPDATE SET \
                 content=excluded.content, \
                 embedding=excluded.embedding, \
                 created_at=excluded.created_at"
        )
        .bind(&record.id)
        .bind(&record.organization_id)
        .bind(&record.agent_id)
        .bind(&record.content)
        .bind(emb_str)
        .bind(&record.source_type)
        .bind(record.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn semantic_search(&self, organization_id: &str, query_embedding: &[f32], limit: i64) -> Result<Vec<EmbeddingRecord>, String> {
        let emb_str = serde_json::to_string(query_embedding).map_err(|e| e.to_string())?;

        let rows = sqlx::query(
            "SELECT id, organization_id, COALESCE(agent_id, '') as agent_id, content, embedding::text, source_type, created_at \
             FROM consolidated_memory \
             WHERE organization_id = $1 \
             ORDER BY embedding <-> $2::vector \
             LIMIT $3"
        )
        .bind(organization_id)
        .bind(emb_str)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut results = Vec::new();
        for row in rows {
            let id: String = row.get("id");
            let organization_id: String = row.get("organization_id");
            let agent_id: String = row.get("agent_id");
            let content: String = row.get("content");
            let emb_str_res: String = row.get("embedding");
            let source_type: String = row.get("source_type");
            let created_at: DateTime<Utc> = row.get("created_at");

            let embedding: Vec<f32> = serde_json::from_str(&emb_str_res).map_err(|e| e.to_string())?;

            results.push(EmbeddingRecord {
                id,
                organization_id,
                agent_id,
                content,
                embedding,
                source_type,
                created_at,
            });
        }

        Ok(results)
    }

    pub async fn prune_stale(&self, older_than: DateTime<Utc>) -> Result<(), String> {
        sqlx::query("DELETE FROM consolidated_memory WHERE created_at < $1 AND source_type = 'TASK_SUMMARY'")
            .bind(older_than)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn delete(&self, id: &str) -> Result<(), String> {
        sqlx::query("DELETE FROM consolidated_memory WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
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
            organization_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            content: "Hello world".to_string(),
            embedding: vec![1.0, 2.0, 3.0],
            source_type: "TEXT".to_string(),
            created_at: now,
        };

        let json = serde_json::to_string(&record).unwrap();
        let deserialized: EmbeddingRecord = serde_json::from_str(&json).unwrap();

        assert_eq!(record.id, deserialized.id);
        assert_eq!(record.embedding, deserialized.embedding);
        assert_eq!(record.created_at, deserialized.created_at);
    }
}
