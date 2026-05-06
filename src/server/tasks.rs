use std::collections::HashMap;
use std::sync::RwLock;
use serde::{Serialize, Deserialize};
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
    pub fn into_proto(self) -> crate::ohc::orchestration::SharedTask {
        let mut task_payload = crate::ohc::orchestration::TaskPayload::default();
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&self.payload) {
            if let Some(sp) = json.get("system_prompt").and_then(|v| v.as_str()) {
                task_payload.system_prompt = sp.to_string();
            }
            if let Some(dep) = json.get("department").and_then(|v| v.as_str()) {
                task_payload.department = dep.to_string();
            }
            if let Some(model) = json.get("model").and_then(|v| v.as_str()) {
                task_payload.model = model.to_string();
            }
        }
        use prost::Message;
        let mut payload_bytes = Vec::new();
        let _ = task_payload.encode(&mut payload_bytes);

        crate::ohc::orchestration::SharedTask {
            id: self.id,
            organization_id: self.organization_id,
            parent_plan_id: self.parent_plan_id,
            dependencies: self.dependencies,
            title: self.title,
            description: self.description.unwrap_or_default(),
            status: self.status,
            assigned_agent_id: self.assigned_agent_id.unwrap_or_default(),
            priority: self.priority,
            payload: payload_bytes,
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
    pub fn to_proto(&self) -> crate::ohc::orchestration::ActionRisk {
        match self {
            ActionRisk::Unspecified => crate::ohc::orchestration::ActionRisk::Unspecified,
            ActionRisk::Low => crate::ohc::orchestration::ActionRisk::Low,
            ActionRisk::High => crate::ohc::orchestration::ActionRisk::High,
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
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager {
            tasks: RwLock::new(HashMap::new()),
        }
    }

    pub fn create_task(&self, org_id: String, mission_id: String, title: String, description: String, priority: String) -> Result<SharedTask, String> {
        self.create_task_with_plan(org_id, mission_id, String::new(), vec![], title, description, priority)
    }

    pub fn create_task_with_plan(&self, org_id: String, mission_id: String, parent_plan_id: String, dependencies: Vec<String>, title: String, description: String, priority: String) -> Result<SharedTask, String> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        
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
            action_risk: None,
            approval_status: None,
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

    pub fn approve_task(&self, task_id: &str, is_approved: bool) -> Result<(), String> {
        let mut tasks = self.tasks.write().unwrap();
        if let Some(task) = tasks.get_mut(task_id) {
            task.approval_status = Some(if is_approved { "APPROVED".to_string() } else { "REJECTED".to_string() });
            if is_approved {
                // Return to IN_PROGRESS so the assigned agent can complete it
                task.status = "IN_PROGRESS".to_string();
            } else {
                // If rejected, fail the task
                task.status = "FAILED".to_string();

                let mut payload_map: serde_json::Value = if task.payload.is_empty() {
                    serde_json::json!({})
                } else {
                    serde_json::from_str(&task.payload).unwrap_or(serde_json::json!({}))
                };

                if let Some(obj) = payload_map.as_object_mut() {
                    obj.insert("error".to_string(), serde_json::Value::String("Task was rejected by user".to_string()));
                    obj.insert("failed_at".to_string(), serde_json::Value::String(Utc::now().to_rfc3339()));
                }
                task.payload = payload_map.to_string();
            }
            task.updated_at = Utc::now();
            Ok(())
        } else {
            Err("task not found".to_string())
        }
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
        let task = tm.create_task("org1".to_string(), "mission1".to_string(), "Test Task".to_string(), "Description".to_string(), "P1".to_string()).unwrap();
        
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.status, "PENDING");
        
        let fetched = tm.get_task(&task.id).unwrap();
        assert_eq!(fetched.id, task.id);
    }
    #[test]
    fn test_claim_task() {
        let tm = TaskManager::new();
        let task = tm.create_task("org1".to_string(), "mission1".to_string(), "Test Task".to_string(), "Description".to_string(), "P1".to_string()).unwrap();
        
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
        let task = tm.create_task("org1".to_string(), "mission1".to_string(), "Test Task".to_string(), "Description".to_string(), "P1".to_string()).unwrap();
        
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
        let task = tm.create_task("org1".to_string(), "mission1".to_string(), "Test Task".to_string(), "Description".to_string(), "P1".to_string()).unwrap();

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
        let task = tm.create_task("org1".to_string(), "mission1".to_string(), "Test Task".to_string(), "Description".to_string(), "P1".to_string()).unwrap();
        
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
        let mut task = tm.create_task("org1".to_string(), "mission1".to_string(), "Pending Approval Task".to_string(), "Description".to_string(), "P1".to_string()).unwrap();

        task.approval_status = Some("PENDING".to_string());
        task.action_risk = Some(ActionRisk::High);

        tm.insert_task(task.clone());

        let mut ignored_task = tm.create_task("org1".to_string(), "mission1".to_string(), "Other Task".to_string(), "Description".to_string(), "P1".to_string()).unwrap();
        ignored_task.approval_status = Some("APPROVED".to_string());
        tm.insert_task(ignored_task.clone());

        let mut ignored_task2 = tm.create_task("org2".to_string(), "mission1".to_string(), "Other Org Task".to_string(), "Description".to_string(), "P1".to_string()).unwrap();
        ignored_task2.approval_status = Some("PENDING".to_string());
        tm.insert_task(ignored_task2.clone());

        let pending = tm.get_pending_approvals("org1");
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].id, task.id);
        assert_eq!(pending[0].action_risk, Some(ActionRisk::High));
    }

    #[test]
    fn test_approve_task() {
        let tm = TaskManager::new();
        let mut task = tm.create_task("org1".to_string(), "mission1".to_string(), "Task to Approve".to_string(), "Description".to_string(), "P1".to_string()).unwrap();
        task.approval_status = Some("PENDING".to_string());
        tm.insert_task(task.clone());

        tm.approve_task(&task.id, true).unwrap();

        let fetched = tm.get_task(&task.id).unwrap();
        assert_eq!(fetched.approval_status, Some("APPROVED".to_string()));
        assert_eq!(fetched.status, "IN_PROGRESS");
    }

    #[test]
    fn test_reject_task() {
        let tm = TaskManager::new();
        let mut task = tm.create_task("org1".to_string(), "mission1".to_string(), "Task to Reject".to_string(), "Description".to_string(), "P1".to_string()).unwrap();
        task.approval_status = Some("PENDING".to_string());
        tm.insert_task(task.clone());

        tm.approve_task(&task.id, false).unwrap();

        let fetched = tm.get_task(&task.id).unwrap();
        assert_eq!(fetched.approval_status, Some("REJECTED".to_string()));
        assert_eq!(fetched.status, "FAILED");
        let payload: serde_json::Value = serde_json::from_str(&fetched.payload).unwrap();
        assert_eq!(payload["error"], "Task was rejected by user");
    }
}
