use std::sync::Arc;
use crate::db::{DB, DbStore};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Epic {
    pub id: Uuid,
    pub title: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EpicTask {
    pub id: Uuid,
    pub epic_id: Option<Uuid>,
    pub title: String,
    pub status: String,
    pub assigned_agent: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct EpicTaskRepo {
    db: Arc<DB>,
}

impl EpicTaskRepo {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn create_epic(&self, title: &str) -> Result<Epic, String> {
        match &self.db.store {
            DbStore::Postgres => {
                let row = sqlx::query_as::<_, Epic>(
                    r#"
                    INSERT INTO epics (title)
                    VALUES ($1)
                    RETURNING id, title, status, created_at, updated_at
                    "#
                )
                .bind(title)
                .fetch_one(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
                Ok(row)
            }
            DbStore::Sqlite(sqlite_pool) => {
                let id = Uuid::new_v4();
                let now = Utc::now();
                sqlx::query(
                    r#"
                    INSERT INTO epics (id, title, status, created_at, updated_at)
                    VALUES (?, ?, 'PENDING', ?, ?)
                    "#
                )
                .bind(id.to_string())
                .bind(title)
                .bind(now)
                .bind(now)
                .execute(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;

                Ok(Epic {
                    id,
                    title: title.to_string(),
                    status: "PENDING".to_string(),
                    created_at: now,
                    updated_at: now,
                })
            }
        }
    }

    pub async fn create_task(&self, epic_id: Option<Uuid>, title: &str, status: &str) -> Result<EpicTask, String> {
        match &self.db.store {
            DbStore::Postgres => {
                let row = sqlx::query_as::<_, EpicTask>(
                    r#"
                    INSERT INTO tasks (epic_id, title, status)
                    VALUES ($1, $2, $3)
                    RETURNING id, epic_id, title, status, assigned_agent, created_at, updated_at
                    "#
                )
                .bind(epic_id)
                .bind(title)
                .bind(status)
                .fetch_one(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
                Ok(row)
            }
            DbStore::Sqlite(sqlite_pool) => {
                let id = Uuid::new_v4();
                let now = Utc::now();
                sqlx::query(
                    r#"
                    INSERT INTO tasks (id, epic_id, title, status, created_at, updated_at)
                    VALUES (?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(id.to_string())
                .bind(epic_id.map(|e| e.to_string()))
                .bind(title)
                .bind(status)
                .bind(now)
                .bind(now)
                .execute(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;

                Ok(EpicTask {
                    id,
                    epic_id,
                    title: title.to_string(),
                    status: status.to_string(),
                    assigned_agent: None,
                    created_at: now,
                    updated_at: now,
                })
            }
        }
    }

    pub async fn get_tasks_for_epic(&self, epic_id: Uuid) -> Result<Vec<EpicTask>, String> {
        match &self.db.store {
            DbStore::Postgres => {
                let tasks = sqlx::query_as::<_, EpicTask>(
                    r#"
                    SELECT id, epic_id, title, status, assigned_agent, created_at, updated_at
                    FROM tasks
                    WHERE epic_id = $1
                    "#
                )
                .bind(epic_id)
                .fetch_all(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
                Ok(tasks)
            }
            DbStore::Sqlite(sqlite_pool) => {
                let rows = sqlx::query(
                    r#"
                    SELECT id, epic_id, title, status, assigned_agent, created_at, updated_at
                    FROM tasks
                    WHERE epic_id = ?
                    "#
                )
                .bind(epic_id.to_string())
                .fetch_all(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;

                let mut tasks = Vec::new();
                for row in rows {
                    use sqlx::Row;
                    let id_str: String = row.get("id");
                    let epic_id_str: Option<String> = row.get("epic_id");
                    let created_str: String = row.get("created_at");
                    let updated_str: String = row.get("updated_at");

                    let created_at = chrono::NaiveDateTime::parse_from_str(&created_str, "%Y-%m-%d %H:%M:%S%.f")
                        .map(|nd| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(nd, chrono::Utc))
                        .or_else(|_| chrono::DateTime::parse_from_rfc3339(&created_str).map(|d| d.with_timezone(&chrono::Utc)))
                        .unwrap_or_else(|_| Utc::now());

                    let updated_at = chrono::NaiveDateTime::parse_from_str(&updated_str, "%Y-%m-%d %H:%M:%S%.f")
                        .map(|nd| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(nd, chrono::Utc))
                        .or_else(|_| chrono::DateTime::parse_from_rfc3339(&updated_str).map(|d| d.with_timezone(&chrono::Utc)))
                        .unwrap_or_else(|_| Utc::now());


                    tasks.push(EpicTask {
                        id: Uuid::parse_str(&id_str).unwrap(),
                        epic_id: epic_id_str.map(|s| Uuid::parse_str(&s).unwrap()),
                        title: row.get("title"),
                        status: row.get("status"),
                        assigned_agent: row.get("assigned_agent"),
                        created_at,
                        updated_at,
                    });
                }
                Ok(tasks)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup_test_db() -> Arc<DB> {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            r#"
            CREATE TABLE epics (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'PENDING',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
            CREATE TABLE tasks (
                id TEXT PRIMARY KEY,
                epic_id TEXT REFERENCES epics(id),
                title TEXT NOT NULL,
                status TEXT NOT NULL,
                assigned_agent TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        let pg_pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
            .unwrap();

        Arc::new(DB {
            pool: pg_pool,
            store: DbStore::Sqlite(pool),
        })
    }

    #[tokio::test]
    async fn test_epic_task_repo() {
        let db = setup_test_db().await;
        let repo = EpicTaskRepo::new(db);

        let epic = repo.create_epic("Test Epic").await.unwrap();
        assert_eq!(epic.title, "Test Epic");
        assert_eq!(epic.status, "PENDING");

        let task1 = repo.create_task(Some(epic.id), "Test Task 1", "PENDING").await.unwrap();
        assert_eq!(task1.title, "Test Task 1");
        assert_eq!(task1.epic_id, Some(epic.id));

        let task2 = repo.create_task(Some(epic.id), "Test Task 2", "CLAIMED").await.unwrap();
        assert_eq!(task2.title, "Test Task 2");

        let tasks = repo.get_tasks_for_epic(epic.id).await.unwrap();
        assert_eq!(tasks.len(), 2);
    }
}
