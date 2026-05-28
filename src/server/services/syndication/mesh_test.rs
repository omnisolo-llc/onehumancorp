#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use async_trait::async_trait;
    use serde_json::Value;
    use crate::msgbus::{Bus, Message};
    use crate::services::syndication::adapter::SalesChannelAdapter;
    use crate::services::syndication::mesh::SyndicationMeshService;
    use std::time::Duration;

    struct MockBus {
        handlers: Arc<Mutex<Vec<Box<dyn Fn(Message) + Send + Sync>>>>,
    }

    #[async_trait]
    impl Bus for MockBus {
        async fn publish(&self, _msg: Message) -> Result<(), String> {
            Ok(())
        }
        async fn subscribe(&self, _topic: String, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
            let mut handlers = self.handlers.lock().await;
            handlers.push(handler);
            Ok(Box::new(|| {}))
        }
    }

    struct MockAdapter {
        pushes: Arc<Mutex<Vec<Value>>>,
    }

    #[async_trait]
    impl SalesChannelAdapter for MockAdapter {
        fn platform_name(&self) -> &str {
            "mock"
        }
        async fn push_product_update(&self, product: &Value) -> Result<(), String> {
            let mut pushes = self.pushes.lock().await;
            pushes.push(product.clone());
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_syndication_mesh_service() {
        let handlers = Arc::new(Mutex::new(Vec::new()));
        let mock_bus = Arc::new(MockBus { handlers: handlers.clone() });
        let service = SyndicationMeshService::new(mock_bus);

        let pushes = Arc::new(Mutex::new(Vec::new()));
        let adapter = Box::new(MockAdapter { pushes: pushes.clone() });
        service.add_adapter(adapter).await;

        service.start().await.unwrap();

        let handlers_lock = handlers.lock().await;
        let handler = &handlers_lock[0];

        let msg = Message {
            topic: "system:catalog_events".to_string(),
            payload: serde_json::json!({"action": "ProductCreated", "product_id": "123"}).to_string().into_bytes(),
        };

        handler(msg);

        tokio::time::sleep(Duration::from_millis(50)).await;

        let pushes_lock = pushes.lock().await;
        assert_eq!(pushes_lock.len(), 1);
        assert_eq!(pushes_lock[0]["product_id"], "123");
    }
}
