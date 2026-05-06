use crate::sandbox::session::ShellSession;
use sqlx::SqlitePool;

pub struct LocalStatefulExecutionProxy {
    db: SqlitePool,
    session: ShellSession,
}

impl LocalStatefulExecutionProxy {
    pub async fn new(db: SqlitePool, session: ShellSession) -> Result<Self, String> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS local_execution_results (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                command TEXT NOT NULL,
                output TEXT,
                error TEXT,
                exit_code INTEGER,
                executed_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )"
        ).execute(&db).await.map_err(|e| e.to_string())?;

        Ok(Self { db, session })
    }

    pub async fn execute_command(&self, command: &str) -> Result<String, String> {
        match self.session.run_stateful_command(command).await {
            Ok(output) => {
                if let Err(db_err) = sqlx::query("INSERT INTO local_execution_results (command, output, exit_code) VALUES (?, ?, ?)")
                    .bind(command)
                    .bind(&output)
                    .bind(0)
                    .execute(&self.db).await
                {
                    tracing::error!("Failed to sync local execution result to database: {}", db_err);
                }
                Ok(output)
            }
            Err(e) => {
                if let Err(db_err) = sqlx::query("INSERT INTO local_execution_results (command, error, exit_code) VALUES (?, ?, ?)")
                    .bind(command)
                    .bind(&e)
                    .bind(1)
                    .execute(&self.db).await
                {
                    tracing::error!("Failed to sync local execution error to database: {}", db_err);
                }
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_local_proxy_execution_success() {
        let db_id = Uuid::new_v4();
        let uri = format!("sqlite:file:{}?mode=memory&cache=shared", db_id);
        let pool = SqlitePoolOptions::new()
            .connect(&uri)
            .await
            .unwrap();

        let dir = format!("{}/test_session_{}", std::env::temp_dir().to_string_lossy(), db_id);
        let _ = tokio::fs::remove_dir_all(&dir).await;
        let session = ShellSession::new("sess-proxy-1", &dir).await.unwrap();

        let proxy = LocalStatefulExecutionProxy::new(pool.clone(), session).await.unwrap();

        let res = proxy.execute_command("echo 'proxytest'").await.unwrap();
        assert!(res.contains("proxytest"));

        // Verify the database state
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM local_execution_results WHERE exit_code = 0 AND command = 'echo ''proxytest'''")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(row.0, 1);

        let _ = tokio::fs::remove_dir_all(&dir).await;
    }

    #[tokio::test]
    async fn test_local_proxy_execution_error() {
        let db_id = Uuid::new_v4();
        let uri = format!("sqlite:file:{}?mode=memory&cache=shared", db_id);
        let pool = SqlitePoolOptions::new()
            .connect(&uri)
            .await
            .unwrap();

        let dir = format!("{}/test_session_{}", std::env::temp_dir().to_string_lossy(), db_id);
        let _ = tokio::fs::remove_dir_all(&dir).await;
        let session = ShellSession::new("sess-proxy-2", &dir).await.unwrap();

        let proxy = LocalStatefulExecutionProxy::new(pool.clone(), session).await.unwrap();

        let res = proxy.execute_command("nonexistent_command_12345").await;
        assert!(res.is_err());

        // Verify the database state
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM local_execution_results WHERE exit_code = 1 AND command = 'nonexistent_command_12345'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(row.0, 1);

        let _ = tokio::fs::remove_dir_all(&dir).await;
    }
}
