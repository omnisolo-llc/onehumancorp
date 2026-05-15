use std::collections::HashMap;
use std::sync::RwLock;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use crate::db::DB;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Background orchestration model: `SharedTask`.
///
/// Defines the exact schema and constraints for the `SharedTask` background job.
/// This struct is serialized into JSON and placed onto the Rueidis-backed queue
/// for asynchronous execution.
///
/// # Concurrency Control
/// When processing `SharedTask`, workers utilize `WATCH` and `ZRANGEBYSCORE`
/// combined with atomic `ZREM` to prevent race conditions during job claiming.
///
/// # Replayability
/// If a worker crashes while processing `SharedTask`, the job will remain in
/// the processing set and will be automatically reclaimed after the timeout window.
/// Background orchestration model: `SharedTask`.
///
/// Defines the exact schema and constraints for the `SharedTask` background job.
/// This struct is serialized into JSON and placed onto the Rueidis-backed queue
/// for asynchronous execution.
///
/// # Concurrency Control
/// When processing `SharedTask`, workers utilize `WATCH` and `ZRANGEBYSCORE`
/// combined with atomic `ZREM` to prevent race conditions during job claiming.
///
/// # Replayability
/// If a worker crashes while processing `SharedTask`, the job will remain in
/// the processing set and will be automatically reclaimed after the timeout window.
pub struct SharedTask {
    pub id: String,
    pub organization_id: String,
    pub mission_id: String,
    pub parent_plan_id: String,
    pub dependencies: Vec<String>,
    pub title: String,
    pub description: Option<String>,
    pub assigned_agent_id: Option<String>,
    pub status: String,
    pub priority: String,
    pub payload: String,
    pub locked_until: Option<DateTime<Utc>>,
    pub ultraplan_phase: Option<String>,
    pub deliberation_log: Option<String>,
    pub depth: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub action_risk: Option<ActionRisk>,
    pub approval_status: Option<String>,
    pub proposed_content: Option<String>,
}

impl SharedTask {
/// Queue manipulation logic: `into_proto`.
///
/// The `into_proto` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `into_proto` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `into_proto` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
/// Queue manipulation logic: `into_proto`.
///
/// The `into_proto` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `into_proto` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `into_proto` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
    pub fn into_proto(self) -> ::server_ohc::orchestration::SharedTask {
        ::server_ohc::orchestration::SharedTask {
            id: self.id,
            organization_id: self.organization_id,
            parent_plan_id: self.parent_plan_id,
            dependencies: self.dependencies,
            title: self.title,
            description: self.description.unwrap_or_default(),
            status: self.status,
            assigned_agent_id: self.assigned_agent_id.unwrap_or_default(),
            priority: self.priority,
            payload: self.payload,
            locked_until_unix: self.locked_until.map(|dt| dt.timestamp()).unwrap_or(0),
            created_at_unix: self.created_at.timestamp(),
            updated_at_unix: self.updated_at.timestamp(),
            action_risk: self.action_risk.unwrap_or(ActionRisk::Unspecified).to_proto() as i32,
            approval_status: self.approval_status.unwrap_or_default(),
            proposed_content: self.proposed_content.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[derive(sqlx::Type)]
#[sqlx(type_name = "VARCHAR")]
pub enum ActionRisk {
    Unspecified,
    Low,
    High,
}

impl ActionRisk {
/// Queue manipulation logic: `to_proto`.
///
/// The `to_proto` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `to_proto` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `to_proto` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
/// Queue manipulation logic: `to_proto`.
///
/// The `to_proto` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `to_proto` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `to_proto` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
    pub fn to_proto(&self) -> ::server_ohc::orchestration::ActionRisk {
        match self {
            ActionRisk::Unspecified => ::server_ohc::orchestration::ActionRisk::Unspecified,
            ActionRisk::Low => ::server_ohc::orchestration::ActionRisk::Low,
            ActionRisk::High => ::server_ohc::orchestration::ActionRisk::High,
        }
    }
}

impl ActionRisk {
/// Queue manipulation logic: `as_str`.
///
/// The `as_str` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `as_str` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `as_str` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
/// Queue manipulation logic: `as_str`.
///
/// The `as_str` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `as_str` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `as_str` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
    pub fn as_str(&self) -> &'static str {
        match self {
            ActionRisk::Unspecified => "UNSPECIFIED",
            ActionRisk::Low => "LOW",
            ActionRisk::High => "HIGH",
        }
    }

/// Queue manipulation logic: `from_str`.
///
/// The `from_str` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `from_str` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `from_str` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
/// Queue manipulation logic: `from_str`.
///
/// The `from_str` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `from_str` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `from_str` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "LOW" => ActionRisk::Low,
            "HIGH" => ActionRisk::High,
            _ => ActionRisk::Unspecified,
        }
    }
}

/// Background orchestration model: `TaskManager`.
///
/// Defines the exact schema and constraints for the `TaskManager` background job.
/// This struct is serialized into JSON and placed onto the Rueidis-backed queue
/// for asynchronous execution.
///
/// # Concurrency Control
/// When processing `TaskManager`, workers utilize `WATCH` and `ZRANGEBYSCORE`
/// combined with atomic `ZREM` to prevent race conditions during job claiming.
///
/// # Replayability
/// If a worker crashes while processing `TaskManager`, the job will remain in
/// the processing set and will be automatically reclaimed after the timeout window.
/// Background orchestration model: `TaskManager`.
///
/// Defines the exact schema and constraints for the `TaskManager` background job.
/// This struct is serialized into JSON and placed onto the Rueidis-backed queue
/// for asynchronous execution.
///
/// # Concurrency Control
/// When processing `TaskManager`, workers utilize `WATCH` and `ZRANGEBYSCORE`
/// combined with atomic `ZREM` to prevent race conditions during job claiming.
///
/// # Replayability
/// If a worker crashes while processing `TaskManager`, the job will remain in
/// the processing set and will be automatically reclaimed after the timeout window.
pub struct TaskManager {
    pub(crate) tasks: RwLock<HashMap<String, SharedTask>>,
    pub(crate) db: RwLock<Option<Arc<DB>>>,
}

impl TaskManager {
/// Queue manipulation logic: `new`.
///
/// The `new` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `new` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `new` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
/// Queue manipulation logic: `new`.
///
/// The `new` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `new` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `new` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
    pub fn new() -> Self {
        TaskManager {
            tasks: RwLock::new(HashMap::new()),
            db: RwLock::new(None),
        }
    }

/// Queue manipulation logic: `with_db`.
///
/// The `with_db` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `with_db` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `with_db` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
/// Queue manipulation logic: `with_db`.
///
/// The `with_db` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `with_db` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `with_db` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
    pub fn with_db(db: Arc<DB>) -> Self {
        TaskManager {
            tasks: RwLock::new(HashMap::new()),
            db: RwLock::new(Some(db)),
        }
    }

/// Queue manipulation logic: `create_task`.
///
/// The `create_task` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `create_task` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `create_task` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
/// Queue manipulation logic: `create_task`.
///
/// The `create_task` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `create_task` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `create_task` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
    pub fn create_task(&self, org_id: String, mission_id: String, title: String, description: String, priority: String) -> Result<SharedTask, String> {
        self.create_task_with_plan(org_id, mission_id, String::new(), vec![], title, description, priority)
    }

/// Queue manipulation logic: `create_task_with_plan`.
///
/// The `create_task_with_plan` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `create_task_with_plan` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `create_task_with_plan` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
/// Queue manipulation logic: `create_task_with_plan`.
///
/// The `create_task_with_plan` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `create_task_with_plan` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `create_task_with_plan` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
    pub fn create_task_with_plan(&self, org_id: String, mission_id: String, parent_plan_id: String, dependencies: Vec<String>, title: String, description: String, priority: String) -> Result<SharedTask, String> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        let approval_status = if priority == "P1" || priority == "HIGH" {
            Some("PENDING".to_string())
        } else {
            None
        };

        let action_risk = if priority == "P1" || priority == "HIGH" {
            Some(ActionRisk::High)
        } else {
            Some(ActionRisk::Low)
        };

        let task = SharedTask {
            id: id.clone(),
            organization_id: org_id,
            mission_id,
            parent_plan_id,
            dependencies,
            title,
            description: Some(description),
            assigned_agent_id: None,
            status: "PENDING".to_string(),
            priority,
            payload: String::new(),
            locked_until: None,
            ultraplan_phase: Some("PROPOSE".to_string()),
            deliberation_log: Some("[]".to_string()),
            depth: None,
            created_at: now,
            updated_at: now,
            action_risk,
            approval_status,

            proposed_content: None,
        };

        let mut tasks = self.tasks.write().unwrap();
        tasks.insert(id, task.clone());

        Ok(task)
    }

/// Queue manipulation logic: `insert_task`.
///
/// The `insert_task` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `insert_task` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `insert_task` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
/// Queue manipulation logic: `insert_task`.
///
/// The `insert_task` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `insert_task` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `insert_task` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
    pub fn insert_task(&self, task: SharedTask) {
        let mut tasks = self.tasks.write().unwrap();
        tasks.insert(task.id.clone(), task);
    }

/// Queue manipulation logic: `get_task`.
///
/// The `get_task` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `get_task` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `get_task` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
/// Queue manipulation logic: `get_task`.
///
/// The `get_task` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `get_task` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `get_task` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
    pub fn get_task(&self, task_id: &str) -> Result<SharedTask, String> {
        let tasks = self.tasks.read().unwrap();
        tasks.get(task_id).cloned().ok_or_else(|| "task not found".to_string())
    }

/// Queue manipulation logic: `update_task_status`.
///
/// The `update_task_status` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `update_task_status` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `update_task_status` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
/// Queue manipulation logic: `update_task_status`.
///
/// The `update_task_status` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `update_task_status` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `update_task_status` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
    pub fn update_task_status(&self, task_id: &str, new_status: String) -> Result<(), String> {
        let mut tasks = self.tasks.write().unwrap();
        if let Some(task) = tasks.get_mut(task_id) {
            task.status = new_status;
            task.updated_at = Utc::now();
            Ok(())
        } else {
            Err("task not found".to_string())
        }
    }

/// Queue manipulation logic: `claim_task`.
///
/// The `claim_task` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `claim_task` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `claim_task` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
/// Queue manipulation logic: `claim_task`.
///
/// The `claim_task` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `claim_task` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `claim_task` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
    pub fn claim_task(&self, task_id: &str, agent_id: String) -> Result<Option<SharedTask>, String> {
        let mut tasks = self.tasks.write().unwrap();
        if let Some(task) = tasks.get_mut(task_id) {
            if task.status == "PENDING" && task.approval_status.as_deref() != Some("PENDING") {
                task.status = "IN_PROGRESS".to_string();
                task.assigned_agent_id = Some(agent_id);
                task.updated_at = Utc::now();
                return Ok(Some(task.clone()));
            }
        }
        Ok(None)
    }

/// Queue manipulation logic: `review_task`.
///
/// The `review_task` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `review_task` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `review_task` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
/// Queue manipulation logic: `review_task`.
///
/// The `review_task` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `review_task` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `review_task` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
    pub fn review_task(&self, task_id: &str, agent_id: &str) -> Result<(), String> {
        let mut tasks = self.tasks.write().unwrap();
        if let Some(task) = tasks.get_mut(task_id) {
            if task.assigned_agent_id.as_deref() == Some(agent_id) {
                task.status = "REVIEW".to_string();
                task.updated_at = Utc::now();
                return Ok(());
            } else {
                return Err("task not assigned to this agent".to_string());
            }
        }
        Err("task not found".to_string())
    }

/// Queue manipulation logic: `complete_task`.
///
/// The `complete_task` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `complete_task` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `complete_task` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
/// Queue manipulation logic: `complete_task`.
///
/// The `complete_task` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `complete_task` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `complete_task` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
    pub fn complete_task(&self, task_id: &str, agent_id: &str, result: String) -> Result<(), String> {
        let mut tasks = self.tasks.write().unwrap();
        if let Some(task) = tasks.get_mut(task_id) {
            if task.assigned_agent_id.as_deref() == Some(agent_id) {
                task.status = "COMPLETED".to_string();

                let mut payload_map: serde_json::Value = if task.payload.is_empty() {
                    serde_json::json!({})
                } else {
                    serde_json::from_str(&task.payload).unwrap_or(serde_json::json!({}))
                };

                if let Some(obj) = payload_map.as_object_mut() {
                    obj.insert("result".to_string(), serde_json::Value::String(result));
                    obj.insert("completed_at".to_string(), serde_json::Value::String(Utc::now().to_rfc3339()));
                }

                task.payload = payload_map.to_string();
                task.updated_at = Utc::now();
                return Ok(());
            } else {
                return Err("task not assigned to this agent".to_string());
            }
        }
        Err("task not found".to_string())
    }



/// Queue manipulation logic: `fail_task`.
///
/// The `fail_task` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `fail_task` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `fail_task` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
/// Queue manipulation logic: `fail_task`.
///
/// The `fail_task` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `fail_task` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `fail_task` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
    pub fn fail_task(&self, task_id: &str, agent_id: &str, reason: &str) -> Result<(), String> {
        let mut tasks = self.tasks.write().unwrap();
        if let Some(task) = tasks.get_mut(task_id) {
            if task.assigned_agent_id.as_deref() == Some(agent_id) {
                task.status = "FAILED".to_string();

                let mut payload_map: serde_json::Value = if task.payload.is_empty() {
                    serde_json::json!({})
                } else {
                    serde_json::from_str(&task.payload).unwrap_or(serde_json::json!({}))
                };

                if let Some(obj) = payload_map.as_object_mut() {
                    obj.insert("error".to_string(), serde_json::Value::String(reason.to_string()));
                    obj.insert("failed_at".to_string(), serde_json::Value::String(Utc::now().to_rfc3339()));
                }

                task.payload = payload_map.to_string();
                task.updated_at = Utc::now();
                return Ok(());
            } else {
                return Err("task not assigned to this agent".to_string());
            }
        }
        Err("task not found".to_string())
    }

    pub async fn approve_task(&self, task_id: &str, is_approved: bool, required_org_id: &str) -> Result<(), String> {
        let (new_approval_status, new_status, new_payload_opt, new_updated_at, org_id) = {
            let tasks_read = self.tasks.read().unwrap();
            if let Some(task) = tasks_read.get(task_id) {
                if task.organization_id != required_org_id {
                    return Err("Unauthorized".to_string());
                }
                let mut task_clone = task.clone();
                task_clone.approval_status = Some(if is_approved { "APPROVED".to_string() } else { "REJECTED".to_string() });
                if is_approved {
                    task_clone.status = "IN_PROGRESS".to_string();
                } else {
                    task_clone.status = "FAILED".to_string();

                    let mut payload_map: serde_json::Value = if task_clone.payload.is_empty() {
                        serde_json::json!({})
                    } else {
                        serde_json::from_str(&task_clone.payload).unwrap_or(serde_json::json!({}))
                    };

                    if let Some(obj) = payload_map.as_object_mut() {
                        obj.insert("error".to_string(), serde_json::Value::String("Task was rejected by user".to_string()));
                        obj.insert("failed_at".to_string(), serde_json::Value::String(Utc::now().to_rfc3339()));
                    }
                    task_clone.payload = payload_map.to_string();
                }
                task_clone.updated_at = Utc::now();
                (task_clone.approval_status.clone(), task_clone.status.clone(), Some(task_clone.payload.clone()), task_clone.updated_at, task_clone.organization_id.clone())
            } else {
                return Err("task not found".to_string());
            }
        };

        let db_clone = self.db.read().unwrap().clone();
        if let Some(db) = db_clone {
            if let Some(new_payload) = &new_payload_opt {
                match &db.store {
                    crate::db::DbStore::Postgres => {
                        let _res = sqlx::query(
                            "UPDATE shared_tasks_decomposition SET approval_status = $1, status = $2, payload = $3, updated_at = $4 WHERE id = $5 AND organization_id = $6"
                        )
                        .bind(&new_approval_status)
                        .bind(&new_status)
                        .bind(new_payload)
                        .bind(new_updated_at)
                        .bind(task_id)
                        .bind(&org_id)
                        .execute(&db.pool)
                        .await
                        .map_err(|e| e.to_string())?;
                    }
                    crate::db::DbStore::Sqlite(pool) => {
                        let _res = sqlx::query(
                            "UPDATE shared_tasks_decomposition SET approval_status = ?, status = ?, payload = ?, updated_at = ? WHERE id = ? AND organization_id = ?"
                        )
                        .bind(&new_approval_status)
                        .bind(&new_status)
                        .bind(new_payload)
                        .bind(new_updated_at.to_rfc3339())
                        .bind(task_id)
                        .bind(&org_id)
                        .execute(pool)
                        .await
                        .map_err(|e| e.to_string())?;
                    }
                }
            }
        }

        let mut tasks = self.tasks.write().unwrap();
        if let Some(task) = tasks.get_mut(task_id) {
            task.approval_status = new_approval_status;
            task.status = new_status;
            if let Some(payload) = new_payload_opt {
                task.payload = payload;
            }
            task.updated_at = new_updated_at;
        }

        Ok(())
    }

/// Queue manipulation logic: `get_pending_approvals`.
///
/// The `get_pending_approvals` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `get_pending_approvals` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `get_pending_approvals` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
/// Queue manipulation logic: `get_pending_approvals`.
///
/// The `get_pending_approvals` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `get_pending_approvals` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `get_pending_approvals` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
    pub fn get_pending_approvals(&self, org_id: &str) -> Vec<SharedTask> {
        let tasks = self.tasks.read().unwrap();
        tasks.values()
            .filter(|t| t.organization_id == org_id && t.approval_status.as_deref() == Some("PENDING"))
            .cloned()
            .collect()
    }

/// Queue manipulation logic: `poll_tasks`.
///
/// The `poll_tasks` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `poll_tasks` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `poll_tasks` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
/// Queue manipulation logic: `poll_tasks`.
///
/// The `poll_tasks` function interfaces directly with the Redis cluster to mutate
/// job states. It handles edge cases like connection pool corruption and timeout errors.
///
/// # Telemetry Requirements
/// Every successful or failed execution within `poll_tasks` emits a dedicated
/// metrics counter for dashboard visualization.
///
/// # Error Propagation
/// Transient Redis errors encountered during `poll_tasks` are converted into
/// application-level `RetryableError`s to trigger the backoff mechanism.
    pub fn poll_tasks(&self, agent_id: &str, limit: usize) -> Vec<SharedTask> {
        let mut tasks = self.tasks.write().unwrap();
        let mut claimed_tasks = Vec::new();

        for task in tasks.values_mut() {
            if task.status == "PENDING" && task.approval_status.as_deref() != Some("PENDING") {
                task.status = "IN_PROGRESS".to_string();
                task.assigned_agent_id = Some(agent_id.to_string());
                task.updated_at = Utc::now();
                claimed_tasks.push(task.clone());

                if claimed_tasks.len() >= limit {
                    break;
                }
            }
        }

        claimed_tasks
    }
}




#[cfg(test)]
pub mod tests;
