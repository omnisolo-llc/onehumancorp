use std::collections::HashMap;
use sqlx::PgPool;
use crate::domain::action::{ActionHandler, ActionIntent};

pub struct ActionRouter {
    handlers: HashMap<String, Box<dyn ActionHandler>>,
}

impl Default for ActionRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionRouter {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register<S: Into<String>>(mut self, feature_type: S, handler: Box<dyn ActionHandler>) -> Self {
        self.handlers.insert(feature_type.into(), handler);
        self
    }

    pub async fn dispatch(&self, pool: &PgPool, tenant_id: &str, intent: &ActionIntent) -> Result<(), String> {
        if let Some(handler) = self.handlers.get(&intent.feature_type) {
            handler.execute(pool, tenant_id, intent).await
        } else {
            tracing::warn!("No handler registered for feature_type: {}", intent.feature_type);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockHandler;

    #[async_trait]
    impl ActionHandler for MockHandler {
        async fn execute(&self, _pool: &PgPool, _tenant_id: &str, _intent: &ActionIntent) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn test_router_registration() {
        let router = ActionRouter::new().register("test_feature", Box::new(MockHandler));
        assert!(router.handlers.contains_key("test_feature"));
        assert!(!router.handlers.contains_key("unknown_feature"));
    }
}
