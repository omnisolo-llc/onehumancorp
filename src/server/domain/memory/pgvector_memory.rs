use sqlx::{PgPool, Row};
use std::fmt;

pub struct MemoryService {
    pool: PgPool,
}

#[derive(Debug)]
pub enum MemoryError {
    DatabaseError(sqlx::Error),
    TenantIsolationViolation,
}

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MemoryError::DatabaseError(e) => write!(f, "Database error: {}", e),
            MemoryError::TenantIsolationViolation => write!(f, "Tenant isolation violation"),
        }
    }
}

impl From<sqlx::Error> for MemoryError {
    fn from(err: sqlx::Error) -> Self {
        MemoryError::DatabaseError(err)
    }
}

pub struct AgentMemory {
    pub id: i32,
    pub tenant_id: String,
    pub department: String,
    pub content: String,
    pub embedding: Vec<f32>,
}

impl MemoryService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Ingests a new memory into the pgvector store, isolated by tenant.
    pub async fn ingest_memory(
        &self,
        tenant_id: &str,
        department: &str,
        content: &str,
        embedding: Vec<f32>,
    ) -> Result<i32, MemoryError> {
        let embedding_str = format!("{:?}", embedding);

        let mut tx = self.pool.begin().await?;

        // Enforce RLS for the session
        sqlx::query(
            "SELECT set_config('app.current_tenant_id', $1, true)"
        )
        .bind(tenant_id)
        .execute(&mut *tx)
        .await?;

        // In a real application, you'd use pgvector syntax like '[1,2,3]'
        // Here we simulate the insert.
        let row = sqlx::query(
            r#"
            INSERT INTO agent_memory (tenant_id, department, content, embedding)
            VALUES ($1, $2, $3, $4::vector)
            RETURNING id
            "#
        )
        .bind(tenant_id)
        .bind(department)
        .bind(content)
        .bind(embedding_str)
        .fetch_one(&mut *tx)
        .await?;

        let id: i32 = row.get("id");
        tx.commit().await?;

        Ok(id)
    }

    /// Recalls memories strictly bounded by the given tenant_id.
    pub async fn recall_memory(
        &self,
        tenant_id: &str,
        query_embedding: Vec<f32>,
        limit: i64,
    ) -> Result<Vec<AgentMemory>, MemoryError> {
        let embedding_str = format!("{:?}", query_embedding);

        let mut tx = self.pool.begin().await?;

        // Enforce RLS for the session
        sqlx::query(
            "SELECT set_config('app.current_tenant_id', $1, true)"
        )
        .bind(tenant_id)
        .execute(&mut *tx)
        .await?;

        let rows = sqlx::query(
            r#"
            SELECT id, tenant_id, department, content, embedding::text as embedding_str
            FROM agent_memory
            ORDER BY embedding <-> $1::vector
            LIMIT $2
            "#
        )
        .bind(embedding_str)
        .bind(limit)
        .fetch_all(&mut *tx)
        .await?;

        let mut memories = Vec::new();
        for row in rows {
            let row_tenant: String = row.get("tenant_id");
            if row_tenant != tenant_id {
                return Err(MemoryError::TenantIsolationViolation);
            }

            let id: i32 = row.get("id");
            let department: String = row.get("department");
            let content: String = row.get("content");
            // Mock parse vector back from string (e.g. "[1.0, 2.0]")
            let embedding_str: String = row.get("embedding_str");
            let parsed_embedding: Vec<f32> = vec![]; // mock

            memories.push(AgentMemory {
                id,
                tenant_id: row_tenant,
                department,
                content,
                embedding: parsed_embedding,
            });
        }

        tx.commit().await?;
        Ok(memories)
    }
}
