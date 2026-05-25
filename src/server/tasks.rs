use std::collections::HashMap;
use std::sync::RwLock;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use crate::db::DB;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub fn to_proto(&self) -> ::server_ohc::orchestration::ActionRisk {
        match self {
            ActionRisk::Unspecified => ::server_ohc::orchestration::ActionRisk::Unspecified,
            ActionRisk::Low => ::server_ohc::orchestration::ActionRisk::Low,
            ActionRisk::High => ::server_ohc::orchestration::ActionRisk::High,
        }
    }
}

impl ActionRisk {
    pub fn as_str(&self) -> &'static str {
        match self {
            ActionRisk::Unspecified => "UNSPECIFIED",
            ActionRisk::Low => "LOW",
            ActionRisk::High => "HIGH",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "LOW" => ActionRisk::Low,
            "HIGH" => ActionRisk::High,
            _ => ActionRisk::Unspecified,
        }
    }
}

pub struct TaskManager {
    pub(crate) tasks: RwLock<HashMap<String, SharedTask>>,
    pub(crate) db: RwLock<Option<Arc<DB>>>,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager {
            tasks: RwLock::new(HashMap::new()),
            db: RwLock::new(None),
        }
    }

    pub fn with_db(db: Arc<DB>) -> Self {
        TaskManager {
            tasks: RwLock::new(HashMap::new()),
            db: RwLock::new(Some(db)),
        }
    }

    pub fn create_task(&self, org_id: String, mission_id: String, title: String, description: String, priority: String) -> Result<SharedTask, String> {
        self.create_task_with_plan(org_id, mission_id, String::new(), vec![], title, description, priority)
    }

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

    pub fn insert_task(&self, task: SharedTask) {
        let mut tasks = self.tasks.write().unwrap();
        tasks.insert(task.id.clone(), task);
    }

    pub fn get_task(&self, task_id: &str) -> Result<SharedTask, String> {
        let tasks = self.tasks.read().unwrap();
        tasks.get(task_id).cloned().ok_or_else(|| "task not found".to_string())
    }

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

    pub fn get_pending_approvals(&self, org_id: &str) -> Vec<SharedTask> {
        let tasks = self.tasks.read().unwrap();
        tasks.values()
            .filter(|t| t.organization_id == org_id && t.approval_status.as_deref() == Some("PENDING"))
            .cloned()
            .collect()
    }

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
mod tests {
    use super::*;
    #[test]
    fn test_create_and_get_task() {
        let tm = TaskManager::new();
        let task = tm.create_task("org1".to_string(), "mission1".to_string(), "Test Task".to_string(), "Description".to_string(), "P2".to_string()).unwrap();
        
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.status, "PENDING");
        
        let fetched = tm.get_task(&task.id).unwrap();
        assert_eq!(fetched.id, task.id);
    }
    #[test]
    fn test_claim_task() {
        let tm = TaskManager::new();
        let task = tm.create_task("org1".to_string(), "mission1".to_string(), "Test Task".to_string(), "Description".to_string(), "P2".to_string()).unwrap();
        
        let claimed = tm.claim_task(&task.id, "agent1".to_string()).unwrap();
        assert!(claimed.is_some());
        let claimed = claimed.unwrap();
        assert_eq!(claimed.status, "IN_PROGRESS");
        assert_eq!(claimed.assigned_agent_id, Some("agent1".to_string()));
        
        // Try to claim again
        let claimed_again = tm.claim_task(&task.id, "agent2".to_string()).unwrap();
        assert!(claimed_again.is_none());
    }
    #[test]
    fn test_review_task() {
        let tm = TaskManager::new();
        let task = tm.create_task("org1".to_string(), "mission1".to_string(), "Test Task".to_string(), "Description".to_string(), "P2".to_string()).unwrap();
        
        tm.claim_task(&task.id, "agent1".to_string()).unwrap();
        
        tm.review_task(&task.id, "agent1").unwrap();
        
        let fetched = tm.get_task(&task.id).unwrap();
        assert_eq!(fetched.status, "REVIEW");
        
        // Try to review with wrong agent
        assert!(tm.review_task(&task.id, "agent2").is_err());
    }
    #[test]
    fn test_fail_task() {
        let tm = TaskManager::new();
        let task = tm.create_task("org1".to_string(), "mission1".to_string(), "Test Task".to_string(), "Description".to_string(), "P2".to_string()).unwrap();

        tm.claim_task(&task.id, "agent1".to_string()).unwrap();

        tm.fail_task(&task.id, "agent1", "Error reason").unwrap();

        let fetched = tm.get_task(&task.id).unwrap();
        assert_eq!(fetched.status, "FAILED");

        let payload: serde_json::Value = serde_json::from_str(&fetched.payload).unwrap();
        assert_eq!(payload["error"], "Error reason");
        assert!(payload["failed_at"].is_string());
    }
    #[test]
    fn test_complete_task() {
        let tm = TaskManager::new();
        let task = tm.create_task("org1".to_string(), "mission1".to_string(), "Test Task".to_string(), "Description".to_string(), "P2".to_string()).unwrap();
        
        tm.claim_task(&task.id, "agent1".to_string()).unwrap();
        
        tm.complete_task(&task.id, "agent1", "Success result".to_string()).unwrap();
        
        let fetched = tm.get_task(&task.id).unwrap();
        assert_eq!(fetched.status, "COMPLETED");
        
        let payload: serde_json::Value = serde_json::from_str(&fetched.payload).unwrap();
        assert_eq!(payload["result"], "Success result");
        assert!(payload["completed_at"].is_string());
    }

    #[test]
    fn test_get_pending_approvals() {
        let tm = TaskManager::new();
        let mut task = tm.create_task("org1".to_string(), "mission1".to_string(), "Pending Approval Task".to_string(), "Description".to_string(), "P2".to_string()).unwrap();

        task.approval_status = Some("PENDING".to_string());
        task.action_risk = Some(ActionRisk::High);

        tm.insert_task(task.clone());

        let mut ignored_task = tm.create_task("org1".to_string(), "mission1".to_string(), "Other Task".to_string(), "Description".to_string(), "P2".to_string()).unwrap();
        ignored_task.approval_status = Some("APPROVED".to_string());
        tm.insert_task(ignored_task.clone());

        let mut ignored_task2 = tm.create_task("org2".to_string(), "mission1".to_string(), "Other Org Task".to_string(), "Description".to_string(), "P2".to_string()).unwrap();
        ignored_task2.approval_status = Some("PENDING".to_string());
        tm.insert_task(ignored_task2.clone());

        let pending = tm.get_pending_approvals("org1");
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].id, task.id);
        assert_eq!(pending[0].action_risk, Some(ActionRisk::High));
    }

    #[tokio::test]
    async fn test_approve_task() {
        let tm = TaskManager::new();
        let mut task = tm.create_task("org1".to_string(), "mission1".to_string(), "Task to Approve".to_string(), "Description".to_string(), "P2".to_string()).unwrap();
        task.approval_status = Some("PENDING".to_string());
        tm.insert_task(task.clone());

        tm.approve_task(&task.id, true, "org1").await.unwrap();

        let fetched = tm.get_task(&task.id).unwrap();
        assert_eq!(fetched.approval_status, Some("APPROVED".to_string()));
        assert_eq!(fetched.status, "IN_PROGRESS");
    }

    #[tokio::test]
    async fn test_reject_task() {
        let tm = TaskManager::new();
        let mut task = tm.create_task("org1".to_string(), "mission1".to_string(), "Task to Reject".to_string(), "Description".to_string(), "P2".to_string()).unwrap();
        task.approval_status = Some("PENDING".to_string());
        tm.insert_task(task.clone());

        tm.approve_task(&task.id, false, "org1").await.unwrap();

        let fetched = tm.get_task(&task.id).unwrap();
        assert_eq!(fetched.approval_status, Some("REJECTED".to_string()));
        assert_eq!(fetched.status, "FAILED");
        let payload: serde_json::Value = serde_json::from_str(&fetched.payload).unwrap();
        assert_eq!(payload["error"], "Task was rejected by user");
    }
    #[tokio::test]
    async fn test_approve_task_integration() {
        let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();

        let _ = sqlx::query(
            "CREATE TABLE shared_tasks_decomposition (id TEXT PRIMARY KEY, status TEXT, dependencies TEXT, assigned_agent_id TEXT, updated_at TEXT, payload TEXT, title TEXT, description TEXT, priority TEXT, locked_until TEXT, ultraplan_phase TEXT, deliberation_log TEXT, depth INTEGER, created_at TEXT, action_risk TEXT, approval_status TEXT, proposed_content TEXT, organization_id TEXT, mission_id TEXT, parent_plan_id TEXT)"
        ).execute(&pool).await;

        let db = std::sync::Arc::new(crate::db::DB {
            store: crate::db::DbStore::Sqlite(pool.clone()),
            pool: sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).connect_lazy("postgres://dummy").unwrap(),
        });

        let tm = TaskManager::with_db(db);
        let mut task = tm.create_task("org_int".to_string(), "mission1".to_string(), "Int Task".to_string(), "Desc".to_string(), "P2".to_string()).unwrap();
        task.approval_status = Some("PENDING".to_string());
        tm.insert_task(task.clone());

        // Insert into DB directly for the query test
        let _ = sqlx::query("INSERT INTO shared_tasks_decomposition (id, organization_id, status) VALUES (?, ?, ?)")
            .bind(&task.id).bind("org_int").bind("PENDING")
            .execute(&pool).await.unwrap();

        tm.approve_task(&task.id, true, "org_int").await.unwrap();

        let row: (String,) = sqlx::query_as("SELECT approval_status FROM shared_tasks_decomposition WHERE id = ?")
            .bind(&task.id)
            .fetch_one(&pool).await.unwrap();
        assert_eq!(row.0, "APPROVED");
    }

}
