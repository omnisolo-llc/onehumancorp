use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use sqlx::PgPool;

pub type HandlerResult = Result<(), String>;
pub type ActionHandler = Arc<dyn Fn(PgPool, String, serde_json::Value) -> Pin<Box<dyn Future<Output = HandlerResult> + Send>> + Send + Sync>;

pub struct ActionRouter {
    handlers: HashMap<String, ActionHandler>,
}

impl ActionRouter {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register<F, Fut>(&mut self, feature_type: &str, handler: F)
    where
        F: Fn(PgPool, String, serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = HandlerResult> + Send + 'static,
    {
        self.handlers.insert(
            feature_type.to_string(),
            Arc::new(move |pool, tenant_id, payload| Box::pin(handler(pool, tenant_id, payload))),
        );
    }

    pub async fn dispatch(&self, pool: PgPool, feature_type: &str, tenant_id: String, payload: serde_json::Value) -> HandlerResult {
        if let Some(handler) = self.handlers.get(feature_type) {
            handler(pool, tenant_id, payload).await
        } else {
            // Graceful handling of unsupported features
            tracing::warn!("No handler registered for feature_type: {}", feature_type);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_action_router_dispatch() {
        let mut router = ActionRouter::new();

        router.register("test_feature", |_pool, tenant_id, payload| async move {
            assert_eq!(tenant_id, "test_tenant");
            assert_eq!(payload.get("key").unwrap().as_str().unwrap(), "value");
            Ok(())
        });
    }
}
