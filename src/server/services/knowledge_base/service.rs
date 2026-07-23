use sqlx::{PgPool, Row};
use uuid::Uuid;
use server_common::auth_utils;
use crate::services::knowledge_base::types::SearchResult;

pub struct KnowledgeBaseService {
    pool: PgPool,
}

impl KnowledgeBaseService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn ingest_document(
        &self,
        tenant_id: &str,
        title: &str,
        content: &str,
        source_type: &str,
        metadata: Option<serde_json::Value>,
    ) -> Result<String, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

        let doc_id = Uuid::new_v4().to_string();

        sqlx::query(
            "INSERT INTO knowledge_base_documents (id, tenant_id, title, content, source_type, metadata)
             VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(&doc_id)
        .bind(tenant_id)
        .bind(title)
        .bind(content)
        .bind(source_type)
        .bind(metadata)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let chunks = self.chunk_text(content);

        for (i, chunk_content) in chunks.into_iter().enumerate() {
            let chunk_id = Uuid::new_v4().to_string();
            // Call local embedding
            let embedding = crate::minimax::LocalLLMClient::new().generate_embedding(&chunk_content).await.unwrap_or_else(|_| vec![0.0; 1536]);
            let emb_str = if embedding.is_empty() {
                None
            } else {
                Some(format!("[{}]", embedding.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(",")))
            };

            sqlx::query(
                "INSERT INTO knowledge_base_document_chunks (id, tenant_id, document_id, content, embedding, chunk_index)
                 VALUES ($1, $2, $3, $4, $5::vector, $6)"
            )
            .bind(&chunk_id)
            .bind(tenant_id)
            .bind(&doc_id)
            .bind(&chunk_content)
            .bind(emb_str)
            .bind(i as i32)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(doc_id)
    }

    pub async fn search(
        &self,
        tenant_id: &str,
        query: &str,
        limit: i64,
    ) -> Result<Vec<SearchResult>, String> {
        let query_embedding = crate::minimax::LocalLLMClient::new().generate_embedding(query).await.unwrap_or_else(|_| vec![0.0; 1536]);

        if query_embedding.is_empty() {
            return Ok(vec![]);
        }

        let emb_str = format!("[{}]", query_embedding.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

        let rows = sqlx::query(
            "SELECT document_id, id as chunk_id, content, embedding <=> $1::vector as distance
             FROM knowledge_base_document_chunks
             WHERE tenant_id = $2
             ORDER BY embedding <=> $1::vector ASC
             LIMIT $3"
        )
        .bind(emb_str)
        .bind(tenant_id)
        .bind(limit)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let mut results = Vec::new();
        for row in rows {
            results.push(SearchResult {
                document_id: row.get("document_id"),
                chunk_id: row.get("chunk_id"),
                content: row.get("content"),
                distance: row.get("distance"),
            });
        }

        Ok(results)
    }

    fn chunk_text(&self, text: &str) -> Vec<String> {
        text.split("\n\n")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }
}
