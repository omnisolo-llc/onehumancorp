use std::time::Duration;
use std::collections::HashMap;
use async_trait::async_trait;

#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

#[async_trait]
pub trait TaskQueue: Send + Sync {
    async fn enqueue_task(&self, queue_name: &str, payload: Vec<u8>, delay: Duration) -> Result<String, String>;
    async fn get_task_status(&self, task_id: &str) -> Result<TaskStatus, String>;
}

pub struct TaskSchedulerManager {
    queue: Box<dyn TaskQueue>,
    pub is_cloud: bool,
}

impl TaskSchedulerManager {
    pub fn new(queue: Box<dyn TaskQueue>, is_cloud: bool) -> Self {
        Self {
            queue,
            is_cloud,
        }
    }

    pub fn from_env(queue: Box<dyn TaskQueue>) -> Self {
        let is_cloud = std::env::var("OHC_MULTITENANT").unwrap_or_default() == "true";
        Self::new(queue, is_cloud)
    }

    fn format_queue_name(&self, tenant_id: &str, queue_name: &str) -> String {
        if self.is_cloud {
            format!("{}:{}", tenant_id, queue_name)
        } else {
            queue_name.to_string()
        }
    }

    pub async fn enqueue_task(&self, tenant_id: &str, queue_name: &str, payload: Vec<u8>, delay: Duration) -> Result<String, String> {
        let actual_queue_name = self.format_queue_name(tenant_id, queue_name);
        self.queue.enqueue_task(&actual_queue_name, payload, delay).await
    }

    pub async fn get_task_status(&self, task_id: &str) -> Result<TaskStatus, String> {
        self.queue.get_task_status(task_id).await
    }
}

// In-Memory queue for tests and standalone mode abstraction
pub struct MemoryTaskQueue {
    tasks: std::sync::RwLock<HashMap<String, TaskStatus>>,
}

impl MemoryTaskQueue {
    pub fn new() -> Self {
        Self {
            tasks: std::sync::RwLock::new(HashMap::new()),
        }
    }

    pub fn set_task_status(&self, task_id: &str, status: TaskStatus) {
        let mut map = self.tasks.write().unwrap();
        map.insert(task_id.to_string(), status);
    }
}

#[async_trait]
impl TaskQueue for MemoryTaskQueue {
    async fn enqueue_task(&self, queue_name: &str, _payload: Vec<u8>, _delay: Duration) -> Result<String, String> {
        let task_id = format!("{}-{}", queue_name, uuid::Uuid::new_v4());
        let mut map = self.tasks.write().unwrap();
        map.insert(task_id.clone(), TaskStatus::Pending);
        Ok(task_id)
    }

    async fn get_task_status(&self, task_id: &str) -> Result<TaskStatus, String> {
        let map = self.tasks.read().unwrap();
        if let Some(status) = map.get(task_id) {
            Ok(status.clone())
        } else {
            Err("Task not found".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_task_scheduler_manager_cloud() {
        let queue = MemoryTaskQueue::new();
        let manager = TaskSchedulerManager::new(Box::new(queue), true);

        let task_id = manager.enqueue_task("tenant_123", "emails", b"send welcome".to_vec(), Duration::from_secs(1)).await.unwrap();

        // Ensure the queue name was prefixed
        assert!(task_id.starts_with("tenant_123:emails-"));

        let status = manager.get_task_status(&task_id).await.unwrap();
        assert_eq!(status, TaskStatus::Pending);
    }

    #[tokio::test]
    async fn test_task_scheduler_manager_standalone() {
        let queue = MemoryTaskQueue::new();
        let manager = TaskSchedulerManager::new(Box::new(queue), false);

        let task_id = manager.enqueue_task("tenant_123", "emails", b"send welcome".to_vec(), Duration::from_secs(1)).await.unwrap();

        // Ensure the queue name was NOT prefixed
        assert!(task_id.starts_with("emails-"));

        let status = manager.get_task_status(&task_id).await.unwrap();
        assert_eq!(status, TaskStatus::Pending);
    }

    #[tokio::test]
    async fn test_task_not_found() {
        let queue = MemoryTaskQueue::new();
        let manager = TaskSchedulerManager::new(Box::new(queue), false);

        let err = manager.get_task_status("missing-task").await.unwrap_err();
        assert_eq!(err, "Task not found");
    }

    #[tokio::test]
    async fn test_from_env() {
        temp_env::with_var("OHC_MULTITENANT", Some("true"), || {
            let queue = Box::new(MemoryTaskQueue::new());
            let manager = TaskSchedulerManager::from_env(queue);
            assert!(manager.is_cloud);
        });

        temp_env::with_var("OHC_MULTITENANT", None::<&str>, || {
            let queue = Box::new(MemoryTaskQueue::new());
            let manager = TaskSchedulerManager::from_env(queue);
            assert!(!manager.is_cloud);
        });
    }
}
