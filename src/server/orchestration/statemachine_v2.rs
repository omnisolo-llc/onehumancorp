use std::collections::HashMap;
use std::sync::Arc;
use crate::db::DbStore;
use crate::orchestration::locks::DistributedLock;
use crate::orchestration::mesh::TeammateMesh;
use chrono::Utc;
use sqlx::Row;
use tokio::sync::Mutex;
use uuid::Uuid;

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

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "PENDING" => Some(State::Pending),
            "READY" => Some(State::Ready),
            "IN_PROGRESS" => Some(State::InProgress),
            "BLOCKED" => Some(State::Blocked),
            "COMPLETED" => Some(State::Completed),
            "FAILED" => Some(State::Failed),
            _ => None,
        }
    }
}

pub struct StateMachine {
    db: DbStore,
    lock: Arc<dyn DistributedLock>,
    mesh: Arc<dyn TeammateMesh>,
    sqlite_mutex: Arc<Mutex<()>>,
    allowed_transitions: HashMap<State, Vec<State>>,
}

impl StateMachine {
    pub fn new(db: DbStore, lock: Arc<dyn DistributedLock>, mesh: Arc<dyn TeammateMesh>) -> Self {
        let mut allowed_transitions = HashMap::new();
        allowed_transitions.insert(State::Pending, vec![State::Ready]);
        allowed_transitions.insert(State::Ready, vec![State::InProgress]);
        allowed_transitions.insert(State::InProgress, vec![State::Completed, State::Blocked, State::Failed]);
        allowed_transitions.insert(State::Blocked, vec![State::InProgress, State::Failed]);

        Self {
            db,
            lock,
            mesh,
            sqlite_mutex: Arc::new(Mutex::new(())),
            allowed_transitions,
        }
    }

    pub async fn broadcast_transition(&self, task_id: &str, from_state: &State, to_state: &State, agent_id: &str) {
        let payload = serde_json::json!({
            "task_id": task_id,
            "from_state": from_state.as_str(),
            "to_state": to_state.as_str(),
            "agent_id": agent_id,
        });

        if let Ok(payload_bytes) = serde_json::to_vec(&payload) {
            let _ = self.mesh.publish("mesh:tasks", payload_bytes).await;
        }
    }

    pub async fn transition_entity(&self, entity_id: &str, entity_type: &str, new_state: State, agent_id: &str) -> Result<(), String> {
        let _guard = self.lock.acquire(entity_id).await?;

        let (table_name, id_col, status_col, org_col) = match entity_type {
            "TASK" => ("shared_tasks_decomposition", "id", "status", "organization_id"),
            "MISSION" => ("swarm_tasks", "id", "status", "tenant_id"), // use tenant_id when mapping to state_machine_transitions
            "SUB_JOB" => ("department_tasks", "id", "status", "tenant_id"),
            _ => return Err(format!("Unsupported entity type: {}", entity_type)),
        };

        let entity_type_db = match entity_type {
            "TASK" => "shared_task",
            "MISSION" => "swarm_task",
            "SUB_JOB" => "department_task",
            _ => "unknown",
        };

        match &self.db {
            DbStore::Postgres => {
                let pool = crate::db::get_pool();
                let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
                ::server_common::auth_utils::set_org_context(&mut *tx, "system").await.map_err(|e| e.to_string())?;

                let query = match entity_type {
                    "MISSION" => format!("SELECT {}, {} FROM {} WHERE {} = $1::uuid FOR UPDATE", status_col, org_col, table_name, id_col),
                    _ => format!("SELECT {}, {} FROM {} WHERE {} = $1 FOR UPDATE", status_col, org_col, table_name, id_col),
                };

                let row = sqlx::query(&query)
                    .bind(entity_id)
                    .fetch_optional(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                let row = match row {
                    Some(r) => r,
                    None => return Err(format!("Entity {} not found", entity_id)),
                };

                let current_state_str: String = row.get(status_col);
                let org_id: String = row.try_get(org_col).unwrap_or_else(|_| "system".to_string());

                let current_state = State::from_str(&current_state_str)
                    .ok_or_else(|| format!("Unknown state: {}", current_state_str))?;

                let valid_transitions = self.allowed_transitions.get(&current_state)
                    .ok_or_else(|| format!("no valid transitions from state {:?}", current_state))?;

                if !valid_transitions.contains(&new_state) {
                    return Err(format!("invalid transition from {:?} to {:?}", current_state, new_state));
                }

                let now = Utc::now();

                let update_query = match entity_type {
                    "TASK" => format!("UPDATE {} SET {} = $1, assigned_agent_id = $2, updated_at = $3 WHERE {} = $4", table_name, status_col, id_col),
                    "MISSION" => format!("UPDATE {} SET {} = $1, assigned_agent_id = $2, updated_at = $3 WHERE {} = $4::uuid", table_name, status_col, id_col),
                    "SUB_JOB" => format!("UPDATE {} SET {} = $1, updated_at = $2 WHERE {} = $3", table_name, status_col, id_col),
                    _ => return Err(format!("Unsupported entity type: {}", entity_type)),
                };

                if entity_type == "SUB_JOB" {
                    sqlx::query(&update_query)
                        .bind(new_state.as_str())
                        .bind(now)
                        .bind(entity_id)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;
                } else {
                    sqlx::query(&update_query)
                        .bind(new_state.as_str())
                        .bind(if agent_id.is_empty() { None } else { Some(agent_id) })
                        .bind(now)
                        .bind(entity_id)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;
                }

                let trans_id = Uuid::new_v4().to_string();
                sqlx::query(
                    r#"
                    INSERT INTO state_machine_transitions
                    (id, tenant_id, entity_id, entity_type, from_state, to_state, agent_id, occurred_at)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                    "#
                )
                .bind(trans_id)
                .bind(&org_id)
                .bind(entity_id)
                .bind(entity_type_db)
                .bind(current_state.as_str())
                .bind(new_state.as_str())
                .bind(if agent_id.is_empty() { None } else { Some(agent_id) })
                .bind(now)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;
                self.broadcast_transition(entity_id, &current_state, &new_state, agent_id).await;
                Ok(())
            }
            DbStore::Sqlite(sqlite_pool) => {
                let _lock = self.sqlite_mutex.lock().await;
                let mut tx = sqlite_pool.begin().await.map_err(|e| e.to_string())?;

                let query = format!("SELECT {}, {} FROM {} WHERE {} = ?", status_col, org_col, table_name, id_col);
                let row = sqlx::query(&query)
                    .bind(entity_id)
                    .fetch_optional(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;

                let row = match row {
                    Some(r) => r,
                    None => return Err(format!("Entity {} not found", entity_id)),
                };

                let current_state_str: String = row.get(status_col);
                let org_id: String = row.try_get(org_col).unwrap_or_else(|_| "system".to_string());

                let current_state = State::from_str(&current_state_str)
                    .ok_or_else(|| format!("Unknown state: {}", current_state_str))?;

                let valid_transitions = self.allowed_transitions.get(&current_state)
                    .ok_or_else(|| format!("no valid transitions from state {:?}", current_state))?;

                if !valid_transitions.contains(&new_state) {
                    return Err(format!("invalid transition from {:?} to {:?}", current_state, new_state));
                }

                let now = Utc::now().to_rfc3339();

                let update_query = match entity_type {
                    "TASK" => format!("UPDATE {} SET {} = ?, assigned_agent_id = ?, updated_at = ? WHERE {} = ?", table_name, status_col, id_col),
                    "MISSION" => format!("UPDATE {} SET {} = ?, assigned_agent_id = ?, updated_at = ? WHERE {} = ?", table_name, status_col, id_col),
                    "SUB_JOB" => format!("UPDATE {} SET {} = ?, updated_at = ? WHERE {} = ?", table_name, status_col, id_col),
                    _ => return Err(format!("Unsupported entity type: {}", entity_type)),
                };

                if entity_type == "SUB_JOB" {
                    sqlx::query(&update_query)
                        .bind(new_state.as_str())
                        .bind(&now)
                        .bind(entity_id)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;
                } else {
                    sqlx::query(&update_query)
                        .bind(new_state.as_str())
                        .bind(if agent_id.is_empty() { None } else { Some(agent_id) })
                        .bind(&now)
                        .bind(entity_id)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;
                }

                let trans_id = Uuid::new_v4().to_string();
                sqlx::query(
                    r#"
                    INSERT INTO state_machine_transitions
                    (id, tenant_id, entity_id, entity_type, from_state, to_state, agent_id, occurred_at)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(trans_id)
                .bind(&org_id)
                .bind(entity_id)
                .bind(entity_type_db)
                .bind(current_state.as_str())
                .bind(new_state.as_str())
                .bind(if agent_id.is_empty() { None } else { Some(agent_id) })
                .bind(&now)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;
                self.broadcast_transition(entity_id, &current_state, &new_state, agent_id).await;
                Ok(())
            }
        }
    }

    pub async fn transition(&self, task_id: &str, new_state: State, agent_id: &str) -> Result<(), String> {
        self.transition_entity(task_id, "TASK", new_state, agent_id).await
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
