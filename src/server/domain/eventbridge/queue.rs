use uuid::Uuid;
use super::schema::OhcEvent;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::VecDeque;

#[async_trait::async_trait]
pub trait EventQueue: Send + Sync {
    async fn enqueue(&self, event: OhcEvent) -> Result<(), String>;
    async fn dequeue(&self) -> Result<Option<OhcEvent>, String>;
}

pub struct InMemoryEventQueue {
    queue: Arc<Mutex<VecDeque<OhcEvent>>>,
}

impl InMemoryEventQueue {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
}

#[async_trait::async_trait]
impl EventQueue for InMemoryEventQueue {
    async fn enqueue(&self, event: OhcEvent) -> Result<(), String> {
        let mut q = self.queue.lock().await;
        q.push_back(event);
        Ok(())
    }

    async fn dequeue(&self) -> Result<Option<OhcEvent>, String> {
        let mut q = self.queue.lock().await;
        Ok(q.pop_front())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use serde_json::json;

    #[tokio::test]
    async fn test_enqueue_dequeue() {
        let queue = InMemoryEventQueue::new();
        let event = OhcEvent {
            event_id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            source: "test".to_string(),
            event_type: "MessageReceived".to_string(),
            payload: json!({"message": "hello"}),
            created_at: Utc::now(),
        };

        queue.enqueue(event.clone()).await.unwrap();
        let dequeued = queue.dequeue().await.unwrap().unwrap();
        assert_eq!(dequeued.event_id, event.event_id);
    }
}
