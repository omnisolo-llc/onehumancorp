use super::schema::OhcEvent;
use super::queue::EventQueue;
use uuid::Uuid;
use chrono::Utc;
use serde_json::Value;
use std::sync::Arc;

pub struct IngressApi {
    queue: Arc<dyn EventQueue>,
}

impl IngressApi {
    pub fn new(queue: Arc<dyn EventQueue>) -> Self {
        Self { queue }
    }

    pub async fn handle_webhook(
        &self,
        tenant_id: Uuid,
        source: &str,
        payload: Value,
    ) -> Result<Uuid, String> {
        // Mock signature validation
        self.validate_signature(&payload)?;

        // Normalize
        let event_type = self.normalize_event_type(source, &payload);

        let event = OhcEvent {
            event_id: Uuid::new_v4(),
            tenant_id,
            source: source.to_string(),
            event_type,
            payload,
            created_at: Utc::now(),
        };

        let event_id = event.event_id;
        self.queue.enqueue(event).await?;

        Ok(event_id)
    }

    fn validate_signature(&self, _payload: &Value) -> Result<(), String> {
        // Dummy Zero Trust signature check
        Ok(())
    }

    fn normalize_event_type(&self, source: &str, payload: &Value) -> String {
        match source {
            "instagram" => "MessageReceived".to_string(),
            "stripe" => "PaymentSucceeded".to_string(),
            _ => "UnknownEvent".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::eventbridge::queue::InMemoryEventQueue;
    use serde_json::json;

    #[tokio::test]
    async fn test_handle_webhook() {
        let queue = Arc::new(InMemoryEventQueue::new());
        let ingress = IngressApi::new(queue.clone());

        let tenant_id = Uuid::new_v4();
        let payload = json!({"data": "test"});
        let id = ingress.handle_webhook(tenant_id, "instagram", payload).await.unwrap();

        let dequeued = queue.dequeue().await.unwrap().unwrap();
        assert_eq!(dequeued.event_id, id);
        assert_eq!(dequeued.event_type, "MessageReceived");
    }
}
