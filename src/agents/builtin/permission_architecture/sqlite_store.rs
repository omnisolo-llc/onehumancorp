use sqlx::{SqlitePool, Row};
use async_trait::async_trait;

use super::models::{ApprovalRequest, ApprovalStatus};
use super::store::ApprovalStore;

#[derive(Debug)]
pub struct SqliteApprovalStore {
    pool: SqlitePool,
}

impl SqliteApprovalStore {
    pub async fn new(pool: SqlitePool) -> Result<Self, String> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS permission_approval_requests (
                id TEXT PRIMARY KEY,
                session_id TEXT NOT NULL,
                tool_name TEXT NOT NULL,
                arguments TEXT NOT NULL,
                status TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                reasoning TEXT
            )"
        ).execute(&pool).await.map_err(|e| e.to_string())?;

        Ok(Self { pool })
    }
}

#[async_trait]
impl ApprovalStore for SqliteApprovalStore {
    async fn create_request(&self, request: ApprovalRequest) -> Result<(), String> {
        let args_str = serde_json::to_string(&request.arguments).map_err(|e| e.to_string())?;
        let status_str = match request.status {
            ApprovalStatus::Pending => "Pending",
            ApprovalStatus::Approved => "Approved",
            ApprovalStatus::Denied => "Denied",
        };

        sqlx::query(
            "INSERT INTO permission_approval_requests (id, session_id, tool_name, arguments, status, created_at, updated_at, reasoning)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&request.id)
        .bind(&request.session_id)
        .bind(&request.tool_name)
        .bind(&args_str)
        .bind(status_str)
        .bind(request.created_at)
        .bind(request.updated_at)
        .bind(&request.reasoning)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_request(&self, id: &str) -> Result<Option<ApprovalRequest>, String> {
        let row = sqlx::query("SELECT * FROM permission_approval_requests WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(row) = row {
            let status_str: String = row.get("status");
            let status = match status_str.as_str() {
                "Approved" => ApprovalStatus::Approved,
                "Denied" => ApprovalStatus::Denied,
                _ => ApprovalStatus::Pending,
            };

            let args_str: String = row.get("arguments");
            let arguments = serde_json::from_str(&args_str).unwrap_or(serde_json::Value::Null);

            Ok(Some(ApprovalRequest {
                id: row.get("id"),
                session_id: row.get("session_id"),
                tool_name: row.get("tool_name"),
                arguments,
                status,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                reasoning: row.get("reasoning"),
            }))
        } else {
            Ok(None)
        }
    }

    async fn update_status(&self, id: &str, status: ApprovalStatus, reasoning: Option<String>) -> Result<(), String> {
        let status_str = match status {
            ApprovalStatus::Pending => "Pending",
            ApprovalStatus::Approved => "Approved",
            ApprovalStatus::Denied => "Denied",
        };

        let updated_at = chrono::Utc::now().timestamp();

        let result = sqlx::query(
            "UPDATE permission_approval_requests SET status = ?, reasoning = ?, updated_at = ? WHERE id = ?"
        )
        .bind(status_str)
        .bind(reasoning)
        .bind(updated_at)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err(format!("Request with ID {} not found", id))
        } else {
            Ok(())
        }
    }

    async fn list_pending(&self, session_id: &str) -> Result<Vec<ApprovalRequest>, String> {
        let rows = sqlx::query("SELECT * FROM permission_approval_requests WHERE session_id = ? AND status = 'Pending'")
            .bind(session_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut requests = Vec::new();
        for row in rows {
            let args_str: String = row.get("arguments");
            let arguments = serde_json::from_str(&args_str).unwrap_or(serde_json::Value::Null);

            requests.push(ApprovalRequest {
                id: row.get("id"),
                session_id: row.get("session_id"),
                tool_name: row.get("tool_name"),
                arguments,
                status: ApprovalStatus::Pending,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                reasoning: row.get("reasoning"),
            });
        }

        Ok(requests)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;
    use uuid::Uuid;

    async fn get_test_store() -> SqliteApprovalStore {
        let db_id = Uuid::new_v4();
        let uri = format!("sqlite:file:{}?mode=memory&cache=shared", db_id);
        let pool = SqlitePoolOptions::new()
            .connect(&uri)
            .await
            .unwrap();
        SqliteApprovalStore::new(pool).await.unwrap()
    }

    #[tokio::test]
    async fn test_sqlite_store_create_get() {
        let store = get_test_store().await;
        let req = ApprovalRequest {
            id: "req-1".to_string(),
            session_id: "sess-1".to_string(),
            tool_name: "test_tool".to_string(),
            arguments: serde_json::json!({"key": "val"}),
            status: ApprovalStatus::Pending,
            created_at: 1000,
            updated_at: 1000,
            reasoning: None,
        };

        assert!(store.create_request(req.clone()).await.is_ok());

        let retrieved = store.get_request("req-1").await.unwrap().unwrap();
        assert_eq!(retrieved.id, "req-1");
        assert_eq!(retrieved.status, ApprovalStatus::Pending);
        assert_eq!(retrieved.arguments["key"], "val");
    }

    #[tokio::test]
    async fn test_sqlite_store_update_status() {
        let store = get_test_store().await;
        let req = ApprovalRequest {
            id: "req-1".to_string(),
            session_id: "sess-1".to_string(),
            tool_name: "test_tool".to_string(),
            arguments: serde_json::json!({}),
            status: ApprovalStatus::Pending,
            created_at: 1000,
            updated_at: 1000,
            reasoning: None,
        };

        store.create_request(req).await.unwrap();

        assert!(store.update_status("req-1", ApprovalStatus::Approved, Some("User OK".to_string())).await.is_ok());

        let retrieved = store.get_request("req-1").await.unwrap().unwrap();
        assert_eq!(retrieved.status, ApprovalStatus::Approved);
        assert_eq!(retrieved.reasoning, Some("User OK".to_string()));
    }

    #[tokio::test]
    async fn test_sqlite_store_list_pending() {
        let store = get_test_store().await;
        let req1 = ApprovalRequest {
            id: "req-1".to_string(),
            session_id: "sess-1".to_string(),
            tool_name: "test_tool".to_string(),
            arguments: serde_json::json!({}),
            status: ApprovalStatus::Pending,
            created_at: 1000,
            updated_at: 1000,
            reasoning: None,
        };
        let req2 = ApprovalRequest {
            id: "req-2".to_string(),
            session_id: "sess-1".to_string(),
            tool_name: "test_tool2".to_string(),
            arguments: serde_json::json!({}),
            status: ApprovalStatus::Approved,
            created_at: 1000,
            updated_at: 1000,
            reasoning: None,
        };

        store.create_request(req1).await.unwrap();
        store.create_request(req2).await.unwrap();

        let pending = store.list_pending("sess-1").await.unwrap();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].id, "req-1");
    }
}
