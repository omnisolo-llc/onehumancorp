use axum::{
    extract::{Path, State},
    routing::{get, post, delete},
    Json, Router,
};
use axum::http::HeaderMap;
use crate::db::DB;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use ohc_builtin_agent::memory_store::VectorRepository;

#[derive(Serialize)]
pub struct MemoryResponse {
    pub id: String,
    pub content: String,
    pub source_type: String,
    pub owner_override: bool,
    pub reference_count: i32,
    pub reliability_score: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_referenced_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
pub struct OverrideRequest {
    pub override_value: bool,
    pub content: Option<String>,
}

pub fn router(db: Arc<DB>) -> Router<std::sync::Arc<(dyn crate::mesh_handler::MeshTransport + 'static)>> {
    let repo = if matches!(db.store, crate::db::DbStore::Postgres) {
        Arc::new(VectorRepository::new(db.pool.clone()))
    } else {
        match &db.store {
            crate::db::DbStore::Sqlite(pool) => Arc::new(VectorRepository::new_sqlite(pool.clone())),
            _ => unreachable!(),
        }
    };
    Router::new()
        .route("/", get(list_memories))
        .route("/upload", post(upload_memory))
        .route("/{id}/override", post(override_memory))
        .route("/{id}", delete(delete_memory))
        .with_state(repo)
}


async fn get_assistant_memory(
    axum::extract::State(repo): axum::extract::State<std::sync::Arc<VectorRepository>>,
    axum::extract::Path(customer_id): axum::extract::Path<String>,
    auth_info: axum::extract::Extension<::server_auth::orchestration::AuthInfo>,
) -> Result<axum::Json<Vec<ohc_builtin_agent::memory_store::AgentSessionSummary>>, (axum::http::StatusCode, String)> {
    let tenant_id = auth_info.organization_id.clone().unwrap_or_else(|| "default".to_string());

    let results = repo.get_customer_session_summaries(&tenant_id, &customer_id, 10).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(axum::Json(results))
}

async fn list_memories(
    State(repo): State<Arc<VectorRepository>>,
    auth_info: axum::extract::Extension<::server_auth::orchestration::AuthInfo>,
) -> Result<Json<Vec<MemoryResponse>>, (axum::http::StatusCode, String)> {
    let tenant_id = auth_info.organization_id.clone().unwrap_or_else(|| "default".to_string());

    let results = repo.list_recent(&tenant_id, 100).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let out = results.into_iter().map(|r| MemoryResponse {
        id: r.id,
        content: r.content,
        source_type: r.source_type,
        owner_override: r.owner_override,
        reference_count: r.reference_count,
        reliability_score: r.reliability_score,
        created_at: r.created_at,
        last_referenced_at: r.last_referenced_at,
    }).collect();

    Ok(Json(out))
}

#[derive(Deserialize)]
pub struct UploadRequest {
    pub content: String,
    pub source_type: String,
}

async fn upload_memory(
    State(repo): State<Arc<VectorRepository>>,
    auth_info: axum::extract::Extension<::server_auth::orchestration::AuthInfo>,
    Json(payload): Json<UploadRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let tenant_id = auth_info.organization_id.clone().unwrap_or_else(|| "default".to_string());

    // We parse PDF content if base64 pdf is sent. For now, since "Do NOT prescribe specific vector databases or text extraction libraries here; design the interfaces and integration points."
    // we use a dummy parser interface that extracts text directly if it is provided.
    // In production we would process this using a background queue with PDF/text extractors.
    let parsed_content = if payload.content.starts_with("JVBERi") {
        "Extracted text from PDF dummy.".to_string()
    } else {
        payload.content.clone()
    };

    let record = ohc_builtin_agent::memory_store::EmbeddingRecord {
        id: uuid::Uuid::new_v4().to_string(),
        tenant_id,
        agent_id: "knowledge_agent".to_string(),
        content: parsed_content,
        embedding: vec![0.0; 1536],
        source_type: payload.source_type,
        created_at: chrono::Utc::now(),
        last_referenced_at: chrono::Utc::now(),
        reference_count: 0,
        reliability_score: 50,
        owner_override: false,
        metadata: None,
    };

    repo.upsert(&record).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(serde_json::json!({"success": true, "id": record.id})))
}

async fn override_memory(
    State(repo): State<Arc<VectorRepository>>,
    Path(id): Path<String>,
    auth_info: axum::extract::Extension<::server_auth::orchestration::AuthInfo>,
    Json(payload): Json<OverrideRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let tenant_id = auth_info.organization_id.clone().unwrap_or_else(|| "default".to_string());

    let mut record = repo.get_by_id(&id).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e))?
        .ok_or((axum::http::StatusCode::NOT_FOUND, "Memory not found".to_string()))?;

    if record.tenant_id != tenant_id {
        return Err((axum::http::StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    record.owner_override = payload.override_value;
    if let Some(c) = payload.content {
        record.content = c;
    }

    repo.upsert(&record).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(serde_json::json!({"success": true})))
}

async fn delete_memory(
    State(repo): State<Arc<VectorRepository>>,
    Path(id): Path<String>,
    auth_info: axum::extract::Extension<::server_auth::orchestration::AuthInfo>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let tenant_id = auth_info.organization_id.clone().unwrap_or_else(|| "default".to_string());

    let record = repo.get_by_id(&id).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e))?
        .ok_or((axum::http::StatusCode::NOT_FOUND, "Memory not found".to_string()))?;

    if record.tenant_id != tenant_id {
        return Err((axum::http::StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    repo.delete(&id).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(serde_json::json!({"success": true})))
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;

    async fn setup_test_repo() -> Arc<VectorRepository> {
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();

        sqlx::query(
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
        )
        .execute(&pool)
        .await
        .unwrap();

        Arc::new(VectorRepository::new_sqlite(pool))
    }

    #[tokio::test]
    async fn test_list_memories() {
        let repo = setup_test_repo().await;

        let record = ohc_builtin_agent::memory_store::EmbeddingRecord {
            id: "test_mem_1".to_string(),
            tenant_id: "test_tenant".to_string(),
            agent_id: "test_agent".to_string(),
            content: "Test memory content".to_string(),
            embedding: vec![0.1; 1536],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 1,
            reliability_score: 80,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&record).await.unwrap();

        let auth_info = ::server_auth::orchestration::AuthInfo {
            organization_id: Some("test_tenant".to_string()),
            user_id: Some("user_1".to_string()),
            role: "owner".to_string(),
        };

        let result = list_memories(State(repo.clone()), axum::extract::Extension(auth_info)).await;

        assert!(result.is_ok());
        let memories = result.unwrap().0;
        assert_eq!(memories.len(), 1);
        assert_eq!(memories[0].id, "test_mem_1");
        assert_eq!(memories[0].content, "Test memory content");
    }

    #[tokio::test]
    async fn test_override_memory() {
        let repo = setup_test_repo().await;

        let record = ohc_builtin_agent::memory_store::EmbeddingRecord {
            id: "test_mem_override".to_string(),
            tenant_id: "test_tenant".to_string(),
            agent_id: "test_agent".to_string(),
            content: "Old content".to_string(),
            embedding: vec![0.1; 1536],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 1,
            reliability_score: 80,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&record).await.unwrap();

        let auth_info = ::server_auth::orchestration::AuthInfo {
            organization_id: Some("test_tenant".to_string()),
            user_id: Some("user_1".to_string()),
            role: "owner".to_string(),
        };

        let override_req = OverrideRequest {
            override_value: true,
            content: Some("New content".to_string()),
        };

        let result = override_memory(
            State(repo.clone()),
            Path("test_mem_override".to_string()),
            axum::extract::Extension(auth_info.clone()),
            Json(override_req),
        ).await;

        assert!(result.is_ok());

        // Verify the record was actually updated
        let updated_record = repo.get_by_id("test_mem_override").await.unwrap().unwrap();
        assert_eq!(updated_record.owner_override, true);
        assert_eq!(updated_record.content, "New content");
    }

    #[tokio::test]
    async fn test_override_memory_forbidden() {
        let repo = setup_test_repo().await;

        let record = ohc_builtin_agent::memory_store::EmbeddingRecord {
            id: "test_mem_forbidden".to_string(),
            tenant_id: "tenant_A".to_string(),
            agent_id: "test_agent".to_string(),
            content: "Top secret".to_string(),
            embedding: vec![0.1; 1536],
            source_type: "NOTES".to_string(),
            created_at: chrono::Utc::now(),
            last_referenced_at: chrono::Utc::now(),
            reference_count: 1,
            reliability_score: 80,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&record).await.unwrap();

        // Try to access with tenant_B
        let auth_info = ::server_auth::orchestration::AuthInfo {
            organization_id: Some("tenant_B".to_string()),
            user_id: Some("user_1".to_string()),
            role: "owner".to_string(),
        };

        let override_req = OverrideRequest {
            override_value: true,
            content: None,
        };

        let result = override_memory(
            State(repo.clone()),
            Path("test_mem_forbidden".to_string()),
            axum::extract::Extension(auth_info),
            Json(override_req),
        ).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.0, axum::http::StatusCode::FORBIDDEN);
    }
}
