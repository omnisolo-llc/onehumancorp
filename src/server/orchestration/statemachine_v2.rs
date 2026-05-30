use std::collections::HashMap;
use std::sync::Arc;
use super::locks::DistributedLock;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum State {
    Pending,
    Ready,
    InProgress,
    Blocked,
    Completed,
    Failed,
}

impl State {
    pub fn as_str(&self) -> &'static str {
        match self {
            State::Pending => "PENDING",
            State::Ready => "READY",
            State::InProgress => "IN_PROGRESS",
            State::Blocked => "BLOCKED",
            State::Completed => "COMPLETED",
            State::Failed => "FAILED",
        }
    }
}

pub trait Repository: Send + Sync {
    fn get_task_state(&self, task_id: &str) -> Result<State, String>;
    fn update_task_state(&self, task_id: &str, from_state: State, to_state: State, agent_id: &str) -> Result<(), String>;
}

pub struct DbRepository {
    db: Arc<crate::db::DB>,
}

impl DbRepository {
    pub fn new(db: Arc<crate::db::DB>) -> Self {
        Self { db }
    }
}

impl Repository for DbRepository {
    fn get_task_state(&self, task_id: &str) -> Result<State, String> {
        let task_id_clone = task_id.to_string();
        let db_clone = self.db.clone();

        let handle = std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                match &db_clone.store {
                    crate::db::DbStore::Postgres => {
                        let row: Result<(String,), _> = sqlx::query_as("SELECT status FROM shared_tasks_decomposition WHERE id = $1")
                            .bind(&task_id_clone)
                            .fetch_one(&db_clone.pool)
                            .await;
                        match row {
                            Ok((status,)) => Ok(status),
                            Err(e) => Err(e.to_string()),
                        }
                    }
                    crate::db::DbStore::Sqlite(pool) => {
                        let row: Result<(String,), _> = sqlx::query_as("SELECT status FROM shared_tasks_decomposition WHERE id = ?")
                            .bind(&task_id_clone)
                            .fetch_one(pool)
                            .await;
                        match row {
                            Ok((status,)) => Ok(status),
                            Err(e) => Err(e.to_string()),
                        }
                    }
                }
            })
        });

        let status = handle.join().unwrap()?;
        match status.as_str() {
            "PENDING" => Ok(State::Pending),
            "READY" => Ok(State::Ready),
            "IN_PROGRESS" => Ok(State::InProgress),
            "BLOCKED" => Ok(State::Blocked),
            "COMPLETED" => Ok(State::Completed),
            "FAILED" => Ok(State::Failed),
            _ => Err(format!("Unknown state: {}", status)),
        }
    }

    fn update_task_state(&self, task_id: &str, from_state: State, to_state: State, agent_id: &str) -> Result<(), String> {
        let task_id_clone = task_id.to_string();
        let from_state_str = from_state.as_str().to_string();
        let to_state_str = to_state.as_str().to_string();
        let agent_id_clone = agent_id.to_string();
        let db_clone = self.db.clone();

        let handle = std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                let trans_id = uuid::Uuid::new_v4().to_string();
                let now = chrono::Utc::now();
                match &db_clone.store {
                    crate::db::DbStore::Postgres => {
                        let mut tx = db_clone.pool.begin().await.map_err(|e| e.to_string())?;

                        sqlx::query("UPDATE shared_tasks_decomposition SET status = $1, assigned_agent_id = $2, updated_at = $3 WHERE id = $4")
                            .bind(&to_state_str)
                            .bind(&agent_id_clone)
                            .bind(now)
                            .bind(&task_id_clone)
                            .execute(&mut *tx)
                            .await
                            .map_err(|e| e.to_string())?;

                        sqlx::query(
                            "INSERT INTO state_machine_transitions (id, task_id, from_state, to_state, agent_id, transitioned_at) VALUES ($1, $2, $3, $4, $5, $6)"
                        )
                        .bind(&trans_id)
                        .bind(&task_id_clone)
                        .bind(&from_state_str)
                        .bind(&to_state_str)
                        .bind(&agent_id_clone)
                        .bind(now)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                        tx.commit().await.map_err(|e| e.to_string())?;
                        Ok(())
                    }
                    crate::db::DbStore::Sqlite(pool) => {
                        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

                        sqlx::query("UPDATE shared_tasks_decomposition SET status = ?, assigned_agent_id = ?, updated_at = ? WHERE id = ?")
                            .bind(&to_state_str)
                            .bind(&agent_id_clone)
                            .bind(now.to_rfc3339())
                            .bind(&task_id_clone)
                            .execute(&mut *tx)
                            .await
                            .map_err(|e| e.to_string())?;

                        sqlx::query(
                            "INSERT INTO state_machine_transitions (id, task_id, from_state, to_state, agent_id, transitioned_at) VALUES (?, ?, ?, ?, ?, ?)"
                        )
                        .bind(&trans_id)
                        .bind(&task_id_clone)
                        .bind(&from_state_str)
                        .bind(&to_state_str)
                        .bind(&agent_id_clone)
                        .bind(now.to_rfc3339())
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                        tx.commit().await.map_err(|e| e.to_string())?;
                        Ok(())
                    }
                }
            })
        });

        handle.join().unwrap()
    }
}

use crate::orchestration::mesh::TeammateMesh;
use ::server_ohc::orchestration::TeammateMeshEvent;

pub struct StateMachine {
    repo: Arc<dyn Repository>,
    lock: Arc<dyn DistributedLock>,
    mesh: Arc<dyn TeammateMesh>,
    allowed_transitions: HashMap<State, Vec<State>>,
}

impl StateMachine {
    pub fn new(repo: Arc<dyn Repository>, lock: Arc<dyn DistributedLock>, mesh: Arc<dyn TeammateMesh>) -> Self {
        let mut allowed_transitions = HashMap::new();
        allowed_transitions.insert(State::Pending, vec![State::Ready]);
        allowed_transitions.insert(State::Ready, vec![State::InProgress]);
        allowed_transitions.insert(State::InProgress, vec![State::Completed, State::Blocked, State::Failed]);
        allowed_transitions.insert(State::Blocked, vec![State::InProgress, State::Failed]);

        Self {
            repo,
            lock,
            mesh,
            allowed_transitions,
        }
    }

    pub async fn broadcast_transition(&self, task_id: &str, from_state: &State, to_state: &State, agent_id: &str) -> Result<(), String> {
        let payload = serde_json::json!({
            "task_id": task_id,
            "from_state": from_state.as_str(),
            "to_state": to_state.as_str(),
            "agent_id": agent_id,
        });

        let payload_bytes = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let _ = self.mesh.publish("mesh:tasks", payload_bytes).await;
        Ok(())
    }

    pub async fn transition(&self, task_id: &str, new_state: State, agent_id: &str) -> Result<(), String> {
        let _guard = self.lock.acquire(task_id).await?;

        let current_state = self.repo.get_task_state(task_id)?;

        let valid_transitions = self.allowed_transitions.get(&current_state)
            .ok_or_else(|| format!("no valid transitions from state {:?}", current_state))?;

        if !valid_transitions.contains(&new_state) {
            return Err(format!("invalid transition from {:?} to {:?}", current_state, new_state));
        }

        self.repo.update_task_state(task_id, current_state.clone(), new_state.clone(), agent_id)?;

        // Publish to Teammate Mesh here
        let _ = self.broadcast_transition(task_id, &current_state, &new_state, agent_id).await;

        Ok(())
    }

    pub async fn transition_to_ready(&self, task_id: &str) -> Result<(), String> {
        self.transition(task_id, State::Ready, "").await
    }

    pub async fn transition_to_in_progress(&self, task_id: &str, agent_id: &str) -> Result<(), String> {
        self.transition(task_id, State::InProgress, agent_id).await
    }

    pub async fn transition_to_completed(&self, task_id: &str) -> Result<(), String> {
        self.transition(task_id, State::Completed, "").await
    }

    pub async fn transition_to_blocked(&self, task_id: &str) -> Result<(), String> {
        self.transition(task_id, State::Blocked, "").await
    }

    pub async fn transition_to_failed(&self, task_id: &str) -> Result<(), String> {
        self.transition(task_id, State::Failed, "").await
    }
}
