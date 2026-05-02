use std::collections::HashMap;
use std::sync::RwLock;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use ohc_builtin_agent::mesh::transport::MeshTransport;

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
    pub action_risk: Option<String>,
    pub approval_status: Option<String>,
    pub proposed_content: Option<String>,
}

pub enum TaskPool {
    Postgres(sqlx::PgPool),
    Sqlite(sqlx::SqlitePool),
}

pub struct TaskManager {
    pub(crate) tasks: RwLock<HashMap<String, SharedTask>>,
    pub(crate) pool: Option<TaskPool>,
    pub(crate) mesh_transport: Option<Arc<dyn MeshTransport>>,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager {
            tasks: RwLock::new(HashMap::new()),
            pool: None,
            mesh_transport: None,
        }
    }

    pub fn with_pool(mut self, pool: TaskPool) -> Self {
        self.pool = Some(pool);
        self
    }

    pub fn with_mesh(mut self, mesh: Arc<dyn MeshTransport>) -> Self {
        self.mesh_transport = Some(mesh);
        self
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
        
        self.insert_task(task.clone());
        
        Ok(task)
    }

    pub fn insert_task(&self, task: SharedTask) {
        let mut tasks = self.tasks.write().unwrap();
        tasks.insert(task.id.clone(), task.clone());
        drop(tasks);

        if let Some(pool) = &self.pool {
            let t = task.clone();
            let pool = match pool {
                TaskPool::Postgres(p) => TaskPool::Postgres(p.clone()),
                TaskPool::Sqlite(p) => TaskPool::Sqlite(p.clone()),
            };
            tokio::spawn(async move {
                let payload_json = serde_json::to_value(&t.payload).unwrap_or(serde_json::Value::Null);
                let deps_json = serde_json::to_value(&t.dependencies).unwrap_or(serde_json::json!([]));
                match pool {
                    TaskPool::Postgres(p) => {
                        let res = sqlx::query("INSERT INTO shared_tasks (id, organization_id, mission_id, parent_plan_id, dependencies, title, description, status, agent_id, priority, payload, locked_until, ultraplan_phase, deliberation_log, depth, action_risk, approval_status, proposed_content) \
                            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18) \
                            ON CONFLICT(id) DO UPDATE SET \
                            status=EXCLUDED.status, agent_id=EXCLUDED.agent_id, payload=EXCLUDED.payload, \
                            locked_until=EXCLUDED.locked_until, ultraplan_phase=EXCLUDED.ultraplan_phase, \
                            deliberation_log=EXCLUDED.deliberation_log, depth=EXCLUDED.depth, \
                            action_risk=EXCLUDED.action_risk, approval_status=EXCLUDED.approval_status, \
                            proposed_content=EXCLUDED.proposed_content, updated_at=CURRENT_TIMESTAMP")
                            .bind(&t.id).bind(&t.organization_id).bind(&t.mission_id).bind(&t.parent_plan_id).bind(&deps_json)
                            .bind(&t.title).bind(&t.description).bind(&t.status).bind(&t.assigned_agent_id).bind(&t.priority)
                            .bind(&payload_json).bind(t.locked_until).bind(&t.ultraplan_phase).bind(&t.deliberation_log)
                            .bind(t.depth).bind(&t.action_risk).bind(&t.approval_status).bind(&t.proposed_content)
                            .execute(&p).await;
                        if let Err(e) = res {
                            eprintln!("Failed to persist task to Postgres: {}", e);
                        }
                    },
                    TaskPool::Sqlite(p) => {
                        let res = sqlx::query("INSERT INTO shared_tasks (id, organization_id, mission_id, parent_plan_id, dependencies, title, description, status, agent_id, priority, payload, locked_until, ultraplan_phase, deliberation_log, depth, action_risk, approval_status, proposed_content) \
                            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) \
                            ON CONFLICT(id) DO UPDATE SET \
                            status=excluded.status, agent_id=excluded.agent_id, payload=excluded.payload, \
                            locked_until=excluded.locked_until, ultraplan_phase=excluded.ultraplan_phase, \
                            deliberation_log=excluded.deliberation_log, depth=excluded.depth, \
                            action_risk=excluded.action_risk, approval_status=excluded.approval_status, \
                            proposed_content=excluded.proposed_content, updated_at=CURRENT_TIMESTAMP")
                            .bind(&t.id).bind(&t.organization_id).bind(&t.mission_id).bind(&t.parent_plan_id).bind(deps_json.to_string())
                            .bind(&t.title).bind(&t.description).bind(&t.status).bind(&t.assigned_agent_id).bind(&t.priority)
                            .bind(payload_json.to_string()).bind(t.locked_until).bind(&t.ultraplan_phase).bind(&t.deliberation_log)
                            .bind(t.depth).bind(&t.action_risk).bind(&t.approval_status).bind(&t.proposed_content)
                            .execute(&p).await;
                        if let Err(e) = res {
                            eprintln!("Failed to persist task to SQLite: {}", e);
                        }
                    }
                }
            });
        }

        if let Some(mesh) = &self.mesh_transport {
            let mesh = mesh.clone();
            let t = task.clone();
            tokio::spawn(async move {
                let payload = serde_json::to_vec(&t).unwrap_or_default();
                if let Err(e) = mesh.publish("agent_jobs", ohc_builtin_agent::mesh::transport::Message {
                    topic: "agent_jobs".to_string(),
                    payload,
                }).await {
                    eprintln!("Failed to publish agent job to mesh: {}", e);
                }
            });
        }
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
            let task_clone = task.clone();
            drop(tasks);
            self.insert_task(task_clone);
            Ok(())
        } else {
            Err("task not found".to_string())
        }
    }

    pub fn claim_task(&self, task_id: &str, agent_id: String) -> Result<Option<SharedTask>, String> {
        if let Some(mesh) = &self.mesh_transport {
            // Use mesh lock for cross-node exclusion
            let resource = format!("task:{}", task_id);
            let mesh_clone = mesh.clone();
            let agent_id_clone = agent_id.clone();

            // Note: Since this is a synchronous method in the existing API, we use a block_on or just try to acquire
            // For now, we'll implement it as a "try lock"
            // In a real production system, we might want to make this async.
            let acquired = futures::executor::block_on(mesh_clone.acquire_lock(&resource, &agent_id_clone, 30))?;
            if !acquired {
                return Ok(None);
            }
        }

        let mut tasks = self.tasks.write().unwrap();
        let result = if let Some(task) = tasks.get_mut(task_id) {
            if task.status == "PENDING" {
                task.status = "IN_PROGRESS".to_string();
                task.assigned_agent_id = Some(agent_id.clone());
                task.updated_at = Utc::now();
                Some(task.clone())
            } else {
                None
            }
        } else {
            None
        };
        drop(tasks);

        if let Some(task_clone) = result {
            self.insert_task(task_clone.clone());
            if let Some(mesh) = &self.mesh_transport {
                let _ = futures::executor::block_on(mesh.release_lock(&format!("task:{}", task_id), &agent_id));
            }
            return Ok(Some(task_clone));
        }

        if let Some(mesh) = &self.mesh_transport {
            let _ = futures::executor::block_on(mesh.release_lock(&format!("task:{}", task_id), &agent_id));
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
                
                payload_map["result"] = serde_json::Value::String(result);
                payload_map["completed_at"] = serde_json::Value::String(Utc::now().to_rfc3339());
                
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
                task.status = "APPROVED".to_string();
            } else {
                task.status = "REJECTED".to_string();
            }
            task.updated_at = Utc::now();
            Ok(())
        } else {
            Err("task not found".to_string())
        }
    }

    pub fn poll_tasks(&self, agent_id: &str, limit: usize) -> Vec<SharedTask> {
        let mut tasks = self.tasks.write().unwrap();
        let mut claimed_tasks = Vec::new();
        
        for task in tasks.values_mut() {
            if task.status == "PENDING" {
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
}
