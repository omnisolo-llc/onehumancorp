use sqlx::PgPool;
use crate::minimax::LocalLLMClient;

pub struct ArchIngester {
    pool: PgPool,
    client: LocalLLMClient,
}

impl ArchIngester {
    pub fn new(pool: PgPool, client: LocalLLMClient) -> Self {
        Self { pool, client }
    }

    pub async fn ingest(&self, content: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let embedding = self.client.generate_embedding(content).await.map_err(|e| e.to_string())?;

        let embedding_str = format!("[{}]", embedding.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));
        let id = uuid::Uuid::new_v4().to_string();

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        crate::utils::auth_utils::set_org_context(&mut *tx, "system").await.map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO consolidated_memory (id, tenant_id, agent_id, content, embedding, source_type, metadata)
             VALUES ($1, 'default', 'system', $2, $3::vector, 'architecture', '{\"type\": \"consolidation\"}')"
        )
        .bind(&id)
        .bind(content)
        .bind(&embedding_str)
        .execute(&mut *tx)
        .await?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(())
    }
}
