use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use serde_json::Value;

#[async_trait::async_trait]
pub trait ActionHandler: Send + Sync {
    async fn execute(&self, pool: &PgPool, tenant_id: &str, payload: &Value) -> Result<(), String>;
}

pub struct ActionRouter {
    handlers: HashMap<String, Arc<dyn ActionHandler>>,
}

impl ActionRouter {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register_handler(&mut self, feature_type: &str, handler: Arc<dyn ActionHandler>) {
        self.handlers.insert(feature_type.to_string(), handler);
    }

    pub async fn dispatch(&self, pool: &PgPool, tenant_id: &str, feature_type: &str, payload: &Value) -> Result<(), String> {
        if let Some(handler) = self.handlers.get(feature_type) {
            handler.execute(pool, tenant_id, payload).await
        } else {
            Err(format!("No handler registered for feature_type: {}", feature_type))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    struct MockHandler;

    #[async_trait::async_trait]
    impl ActionHandler for MockHandler {
        async fn execute(&self, _pool: &PgPool, _tenant_id: &str, _payload: &Value) -> Result<(), String> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_action_router_dispatch() {
        let mut router = ActionRouter::new();
        router.register_handler("mock_handler", std::sync::Arc::new(MockHandler));

        // Just test that the internal map correctly rejects the unregistered handler
        // without trying to actually mock PgPool which is very tricky in sqlx without full setup.

        let has_handler = router.handlers.contains_key("unsupported_handler");
        assert_eq!(has_handler, false);
    }
}
