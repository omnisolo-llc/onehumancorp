use std::collections::HashMap;
use std::sync::RwLock;
use chrono::{DateTime, Utc, Duration};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScheduleType {
    Once,
    Interval,
    Cron,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub r#type: ScheduleType,
    pub at: Option<DateTime<Utc>>,
    pub interval_s: Option<u64>,
    pub expression: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub organization_id: String,
    pub agent_id: String,
    pub name: String,
    pub schedule: Schedule,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub last_run_at: Option<DateTime<Utc>>,
    pub next_run_at: Option<DateTime<Utc>>,
    pub payload: serde_json::Value,
}

pub struct Scheduler {
    tasks: RwLock<HashMap<String, Task>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            tasks: RwLock::new(HashMap::new()),
        }
    }

    pub fn create(&self, task: Task) -> Result<(), String> {
        let mut tasks = self.tasks.write().unwrap();
        if tasks.contains_key(&task.id) {
            return Err("task already exists".to_string());
        }
        tasks.insert(task.id.clone(), task);
        Ok(())
    }

    pub fn cancel(&self, org_id: &str, id: &str) -> Result<(), String> {
        let mut tasks = self.tasks.write().unwrap();
        if let Some(task) = tasks.get_mut(id) {
            if task.organization_id == org_id {
                task.status = TaskStatus::Cancelled;
                return Ok(());
            }
        }
        Err("task not found or does not belong to organization".to_string())
    }

    pub fn list_for_org(&self, org_id: &str) -> Vec<Task> {
        let tasks = self.tasks.read().unwrap();
        tasks.values().filter(|t| t.organization_id == org_id).cloned().collect()
    }

    pub fn poll_due(&self) -> Vec<Task> {
        let tasks = self.tasks.read().unwrap();
        let now = Utc::now();
        tasks.values()
            .filter(|t| t.status == TaskStatus::Pending && t.next_run_at.map_or(false, |at| at < now))
            .cloned()
            .collect()
    }

    pub fn mark_running(&self, org_id: &str, id: &str) -> Result<Task, String> {
        let mut tasks = self.tasks.write().unwrap();
        if let Some(task) = tasks.get_mut(id) {
            if task.organization_id == org_id {
                let now = Utc::now();
                task.status = TaskStatus::Running;
                task.last_run_at = Some(now);
                return Ok(task.clone());
            }
        }
        Err("task not found or does not belong to organization".to_string())
    }

    pub fn mark_done(&self, org_id: &str, id: &str, success: bool) -> Result<(), String> {
        let mut tasks = self.tasks.write().unwrap();
        if let Some(task) = tasks.get_mut(id) {
            if task.organization_id == org_id {
                if success {
                    task.status = TaskStatus::Succeeded;
                    if let ScheduleType::Interval = task.schedule.r#type {
                        if let Some(interval) = task.schedule.interval_s {
                            let next = Utc::now() + Duration::seconds(interval as i64);
                            task.next_run_at = Some(next);
                            task.status = TaskStatus::Pending;
                        }
                    }
                } else {
                    task.status = TaskStatus::Failed;
                }
                return Ok(());
            }
        }
        Err("task not found or does not belong to organization".to_string())
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_and_poll_task() {
        let s = Scheduler::new();
        let now = Utc::now();
        
        let task = Task {
            id: "task1".to_string(),
            organization_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            name: "Test Task".to_string(),
            schedule: Schedule {
                r#type: ScheduleType::Once,
                at: Some(now - Duration::seconds(10)),
                interval_s: None,
                expression: None,
            },
            status: TaskStatus::Pending,
            created_at: now,
            last_run_at: None,
            next_run_at: Some(now - Duration::seconds(10)),
            payload: serde_json::json!({}),
        };
        
        s.create(task.clone()).unwrap();
        
        let due = s.poll_due();
        assert_eq!(due.len(), 1);
        assert_eq!(due[0].id, "task1");
    }

    #[test]
    fn test_mark_running_and_done() {
        let s = Scheduler::new();
        let now = Utc::now();
        
        let task = Task {
            id: "task2".to_string(),
            organization_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            name: "Test Task 2".to_string(),
            schedule: Schedule {
                r#type: ScheduleType::Once,
                at: Some(now),
                interval_s: None,
                expression: None,
            },
            status: TaskStatus::Pending,
            created_at: now,
            last_run_at: None,
            next_run_at: Some(now),
            payload: serde_json::json!({}),
        };
        
        s.create(task.clone()).unwrap();
        
        let running = s.mark_running("org1", "task2").unwrap();
        assert_eq!(running.status, TaskStatus::Running);
        
        s.mark_done("org1", "task2", true).unwrap();
        
        let tasks = s.list_for_org("org1");
        assert_eq!(tasks[0].status, TaskStatus::Succeeded);
    }
}
