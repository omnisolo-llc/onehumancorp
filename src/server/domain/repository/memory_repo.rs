use std::sync::Arc;
use crate::db::{DB, DbStore};
use super::models::AgentMemory;
use sqlx::Row;

pub struct MemoryStore {
    db: Arc<DB>,
}

impl MemoryStore {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn search_memories(
        &self,
        tenant_id: &str,
        query_embedding: &[f32],
        limit: i64,
    ) -> Result<Vec<AgentMemory>, String> {
        match &self.db.store {
            DbStore::Postgres => {
                let vec_str = format!("{:?}", query_embedding);
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

                let res = sqlx::query_as::<_, AgentMemory>(
                    r#"
                    SELECT id, tenant_id, business_id, department, content, interaction_data, created_at
                    FROM agent_memories
                    WHERE tenant_id = $1
                    ORDER BY embedding <-> $2::vector
                    LIMIT $3
                    "#
                )
                .bind(tenant_id)
                .bind(&vec_str)
                .bind(limit)
                .fetch_all(&mut *tx)
                .await
                .map_err(|e| e.to_string());
                let _ = tx.commit().await;
                res
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as::<_, AgentMemory>(
                    r#"
                    SELECT id, tenant_id, business_id, department, content, interaction_data, created_at
                    FROM agent_memories
                    WHERE tenant_id = ?
                    LIMIT ?
                    "#
                )
                .bind(tenant_id)
                .bind(limit)
                .fetch_all(sqlite_pool)
                .await
                .map_err(|e| e.to_string())
            }
        }
    }

    pub async fn insert_memory(&self, memory: AgentMemory) -> Result<(), String> {
        match &self.db.store {
            DbStore::Postgres => {
                let vec_str = memory.embedding.as_ref().map(|v| format!("{:?}", v));
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                ::server_common::auth_utils::set_org_context(&mut *tx, &memory.tenant_id).await.map_err(|e| e.to_string())?;

                sqlx::query(
                    r#"
                    INSERT INTO agent_memories (
                        id, tenant_id, business_id, department, content, embedding, interaction_data, created_at
                    ) VALUES ($1, $2, $3, $4, $5, $6::vector, $7, $8)
                    "#
                )
                .bind(&memory.id)
                .bind(&memory.tenant_id)
                .bind(&memory.business_id)
                .bind(&memory.department)
                .bind(&memory.content)
                .bind(&vec_str)
                .bind(&memory.interaction_data)
                .bind(&memory.created_at)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
                let _ = tx.commit().await;
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO agent_memories (
                        id, tenant_id, business_id, department, content, interaction_data, created_at
                    ) VALUES (?, ?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(&memory.id)
                .bind(&memory.tenant_id)
                .bind(&memory.business_id)
                .bind(&memory.department)
                .bind(&memory.content)
                .bind(&memory.interaction_data)
                .bind(&memory.created_at)
                .execute(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;
    use chrono::Utc;

    async fn setup_test_db() -> Arc<DB> {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            r#"
            CREATE TABLE agent_memories (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                business_id TEXT,
                department TEXT,
                content TEXT NOT NULL,
                embedding TEXT,
                interaction_data JSON,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
            );
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        let pg_pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
            .unwrap();

        Arc::new(DB {
            pool: pg_pool,
            store: DbStore::Sqlite(pool),
        })
    }

    #[tokio::test]
    async fn test_tenant_isolation_in_memory_search() {
        let db = setup_test_db().await;
        let store = MemoryStore::new(db);

        let mem_a = AgentMemory {
            id: "mem_a".to_string(),
            tenant_id: "tenant_A".to_string(),
            business_id: None,
            department: None,
            content: "Memory for tenant A".to_string(),
            embedding: Some(vec![0.1, 0.2, 0.3]),
            interaction_data: None,
            created_at: Some(Utc::now()),
        };

        let mem_b = AgentMemory {
            id: "mem_b".to_string(),
            tenant_id: "tenant_B".to_string(),
            business_id: None,
            department: None,
            content: "Memory for tenant B".to_string(),
            embedding: Some(vec![0.4, 0.5, 0.6]),
            interaction_data: None,
            created_at: Some(Utc::now()),
        };

        store.insert_memory(mem_a).await.unwrap();
        store.insert_memory(mem_b).await.unwrap();

        let query_vec = vec![0.1, 0.2, 0.3];

        let results_a = store.search_memories("tenant_A", &query_vec, 10).await.unwrap();
        assert_eq!(results_a.len(), 1);
        assert_eq!(results_a[0].id, "mem_a");

        let results_b = store.search_memories("tenant_B", &query_vec, 10).await.unwrap();
        assert_eq!(results_b.len(), 1);
        assert_eq!(results_b[0].id, "mem_b");

        let results_c = store.search_memories("tenant_C", &query_vec, 10).await.unwrap();
        assert_eq!(results_c.len(), 0);
    }
}
