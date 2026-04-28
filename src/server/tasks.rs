use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};

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

#[derive(Clone)]
pub struct TaskManager {
    pool: Arc<PgPool>,
    pub(crate) tasks: Arc<RwLock<HashMap<String, SharedTask>>>,
    event_tx: tokio::sync::mpsc::Sender<serde_json::Value>,
}

impl TaskManager {
    pub fn new(pool: Arc<PgPool>, event_tx: tokio::sync::mpsc::Sender<serde_json::Value>) -> Self {
        TaskManager {
            pool,
            tasks: Arc::new(RwLock::new(HashMap::new())),
            event_tx,
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
        tasks.insert(id.clone(), task.clone());

        let pool = self.pool.clone();
        let id_clone = id.clone();
        let deps_json = serde_json::to_string(&dependencies).unwrap_or_else(|_| "[]".to_string());
        
        tokio::spawn(async move {
            let res = sqlx::query(
                "INSERT INTO shared_tasks_v4 (id, organization_id, title, description, priority, parent_plan_id, dependencies)
                 VALUES ($1, $2, $3, $4, $5, $6, $7)"
            )
            .bind(&id_clone)
            .bind(&org_id)
            .bind(&title)
            .bind(&description)
            .bind(&priority)
            .bind(&parent_plan_id)
            .bind(&deps_json)
            .execute(&*pool)
            .await;
        });

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
            task.status = new_status.clone();
            task.updated_at = Utc::now();

            let pool = self.pool.clone();
            let id = task_id.to_string();
            tokio::spawn(async move {
                let _ = sqlx::query("UPDATE shared_tasks_v4 SET status = $1, updated_at = NOW() WHERE id = $2")
                    .bind(new_status)
                    .bind(id)
                    .execute(&*pool)
                    .await;
            });
            Ok(())
        } else {
            Err("task not found".to_string())
        }
    }

    pub fn claim_task(&self, task_id: &str, agent_id: String) -> Result<Option<SharedTask>, String> {
        let mut tasks = self.tasks.write().unwrap();
        if let Some(task) = tasks.get_mut(task_id) {
            if task.status == "PENDING" {
                task.status = "IN_PROGRESS".to_string();
                task.assigned_agent_id = Some(agent_id.clone());
                task.updated_at = Utc::now();

                let pool = self.pool.clone();
                let id = task_id.to_string();
                tokio::spawn(async move {
                    let _ = sqlx::query("UPDATE shared_tasks_v4 SET status = 'IN_PROGRESS', agent_id = $1, updated_at = NOW() WHERE id = $2")
                        .bind(agent_id)
                        .bind(id)
                        .execute(&*pool)
                        .await;
                });
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

                let pool = self.pool.clone();
                let id = task_id.to_string();
                tokio::spawn(async move {
                    let _ = sqlx::query("UPDATE shared_tasks_v4 SET status = 'REVIEW', updated_at = NOW() WHERE id = $1")
                        .bind(id)
                        .execute(&*pool)
                        .await;
                });
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

                let pool = self.pool.clone();
                let id = task_id.to_string();
                let p = task.payload.clone();
                let agent = agent_id.to_string();

                tokio::spawn(async move {
                    let _ = sqlx::query("UPDATE shared_tasks_v4 SET status = 'COMPLETED', payload = $1, updated_at = NOW() WHERE id = $2")
                        .bind(p)
                        .bind(id.clone())
                        .execute(&*pool)
                        .await;

                    let _ = sqlx::query("INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id) VALUES ($1, $2, 'shared_task', 'IN_PROGRESS', 'COMPLETED', $3)")
                        .bind(uuid::Uuid::new_v4().to_string())
                        .bind(id)
                        .bind(agent)
                        .execute(&*pool)
                        .await;
                });
                return Ok(());
            } else {
                return Err("task not assigned to this agent".to_string());
            }
        }
        Err("task not found".to_string())
    }

    pub fn poll_tasks(&self, agent_id: &str, limit: usize) -> Vec<SharedTask> {
        let pool = self.pool.clone();
        let agent_id_str = agent_id.to_string();

        let tasks_lock = self.tasks.clone();
        let mut claimed_tasks = Vec::new();
        
        let pool_for_spawn = pool.clone();
        tokio::spawn(async move {
            let res = sqlx::query("UPDATE shared_tasks_v4 SET status = 'IN_PROGRESS', agent_id = $1, updated_at = NOW() WHERE id IN (SELECT id FROM shared_tasks_v4 WHERE status = 'PENDING' ORDER BY created_at ASC FOR UPDATE SKIP LOCKED LIMIT $2) RETURNING id")
                .bind(&agent_id_str)
                .bind(limit as i64)
                .fetch_all(&*pool_for_spawn)
                .await;

            if let Ok(rows) = res {
                let mut ids_to_claim = Vec::new();
                for row in rows {
                    let id: String = row.get("id");
                    ids_to_claim.push(id);
                }

                let mut t_lock = tasks_lock.write().unwrap();
                for id in ids_to_claim {
                    if let Some(task) = t_lock.get_mut(&id) {
                        task.status = "IN_PROGRESS".to_string();
                        task.assigned_agent_id = Some(agent_id_str.clone());
                        task.updated_at = Utc::now();
                    }
                }
            }
        });

        let mut tasks = self.tasks.write().unwrap();


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
        let (tx, _) = tokio::sync::mpsc::channel(100);
        let tm = TaskManager::new(tx);
        let task = tm.create_task("org1".to_string(), "mission1".to_string(), "Test Task".to_string(), "Description".to_string(), "P1".to_string()).unwrap();
        
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.status, "PENDING");
        
        let fetched = tm.get_task(&task.id).unwrap();
        assert_eq!(fetched.id, task.id);
    }

    #[test]
    fn test_claim_task() {
        let (tx, _) = tokio::sync::mpsc::channel(100);
        let tm = TaskManager::new(tx);
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
        let (tx, _) = tokio::sync::mpsc::channel(100);
        let tm = TaskManager::new(tx);
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
        let (tx, _) = tokio::sync::mpsc::channel(100);
        let tm = TaskManager::new(tx);
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
