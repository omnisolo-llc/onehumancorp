use super::schema::{AgentFeedItem, AgentFeedItemStatus, OhcEvent};
use super::queue::EventQueue;
use uuid::Uuid;
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Dispatcher {
    queue: Arc<dyn EventQueue>,
    feed_items: Arc<Mutex<Vec<AgentFeedItem>>>,
}

impl Dispatcher {
    pub fn new(queue: Arc<dyn EventQueue>, feed_items: Arc<Mutex<Vec<AgentFeedItem>>>) -> Self {
        Self { queue, feed_items }
    }

    pub async fn process_next(&self) -> Result<bool, String> {
        if let Some(event) = self.queue.dequeue().await? {
            let item = self.route_event(event);
            let mut items = self.feed_items.lock().await;
            items.push(item);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn route_event(&self, event: OhcEvent) -> AgentFeedItem {
        let (agent_type, action_payload) = match event.event_type.as_str() {
            "MessageReceived" => (
                "The Ambassador".to_string(),
                serde_json::json!({"action": "draft_reply", "data": event.payload}),
            ),
            "ProductCreated" => (
                "The Promoter".to_string(),
                serde_json::json!({"action": "schedule_post", "data": event.payload}),
            ),
            _ => (
                "General Ops".to_string(),
                serde_json::json!({"action": "log", "data": event.payload}),
            ),
        };

        AgentFeedItem {
            item_id: Uuid::new_v4(),
            tenant_id: event.tenant_id,
            agent_type,
            status: AgentFeedItemStatus::PendingApproval,
            action_payload,
            created_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::eventbridge::queue::InMemoryEventQueue;
    use serde_json::json;

    #[tokio::test]
    async fn test_dispatch() {
        let queue = Arc::new(InMemoryEventQueue::new());
        let feed = Arc::new(Mutex::new(Vec::new()));
        let dispatcher = Dispatcher::new(queue.clone(), feed.clone());

        let event = OhcEvent {
            event_id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            source: "instagram".to_string(),
            event_type: "MessageReceived".to_string(),
            payload: json!({"msg": "hello"}),
            created_at: Utc::now(),
        };

        queue.enqueue(event).await.unwrap();

        let processed = dispatcher.process_next().await.unwrap();
        assert!(processed);

        let items = feed.lock().await;
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].agent_type, "The Ambassador");
        assert_eq!(items[0].status, AgentFeedItemStatus::PendingApproval);
    }
}
