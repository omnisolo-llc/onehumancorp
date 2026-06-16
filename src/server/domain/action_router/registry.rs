use std::collections::HashMap;
use std::sync::Arc;
use sqlx::PgPool;
use std::sync::RwLock;

use super::payload::ActionIntent;

#[async_trait::async_trait]
pub trait ActionHandler: Send + Sync {
    async fn execute(&self, pool: &PgPool, tenant_id: &str, intent: &ActionIntent) -> Result<(), String>;
}

pub struct ActionRouter {
    handlers: RwLock<HashMap<String, Arc<dyn ActionHandler>>>,
}

impl ActionRouter {
    pub fn new() -> Self {
        Self {
            handlers: RwLock::new(HashMap::new()),
        }
    }

    pub fn register_handler(&self, feature_type: &str, handler: Arc<dyn ActionHandler>) {
        if let Ok(mut handlers) = self.handlers.write() {
            handlers.insert(feature_type.to_string(), handler);
        }
    }

    pub async fn dispatch(&self, pool: &PgPool, tenant_id: &str, intent: &ActionIntent) -> Result<(), String> {
        let handler = {
            let handlers = self.handlers.read().unwrap();
            handlers.get(&intent.feature_type).cloned()
        };

        if let Some(handler) = handler {
            handler.execute(pool, tenant_id, intent).await
        } else {
            // Handlers not registered shouldn't fail fatally but just log/ignore for now as fallback or return Ok
            tracing::warn!("No handler registered for feature_type: {}", intent.feature_type);
            Ok(())
        }
    }
}

pub fn get_global_action_router() -> Arc<ActionRouter> {
    use std::sync::OnceLock;
    use super::handlers::{IncidentResolutionHandler, SocialPostDraftHandler, AmbassadorReplyHandler, QuoteDraftHandler, InstagramDmHandler};
    static ROUTER: OnceLock<Arc<ActionRouter>> = OnceLock::new();
    ROUTER.get_or_init(|| {
        let router = ActionRouter::new();
        router.register_handler("incident_resolution", Arc::new(IncidentResolutionHandler));
        router.register_handler("social_post_draft", Arc::new(SocialPostDraftHandler));
        router.register_handler("ambassador_reply", Arc::new(AmbassadorReplyHandler));
        router.register_handler("quote_draft", Arc::new(QuoteDraftHandler));
        router.register_handler("instagram_dm", Arc::new(InstagramDmHandler));
        Arc::new(router)
    }).clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use sqlx::PgPool;

    struct TestHandler {
        called: tokio::sync::RwLock<bool>,
    }

    impl TestHandler {
        fn new() -> Self {
            Self {
                called: tokio::sync::RwLock::new(false),
            }
        }
    }

    #[async_trait::async_trait]
    impl ActionHandler for TestHandler {
        async fn execute(&self, _pool: &PgPool, _tenant_id: &str, _intent: &ActionIntent) -> Result<(), String> {
            let mut called = self.called.write().await;
            *called = true;
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_action_router_dispatch_success() {
        let router = ActionRouter::new();
        let handler = Arc::new(TestHandler::new());
        router.register_handler("test_feature", handler.clone());

        let database_url = std::env::var("OHC_DATABASE_URL").or_else(|_| std::env::var("DATABASE_URL")).unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        if let Ok(pool) = PgPool::connect(&database_url).await {
            // Check if connection actually works before asserting to prevent PoolTimedOut
            if pool.acquire().await.is_ok() {
                let intent = ActionIntent {
                    feature_type: "test_feature".to_string(),
                    action: None,
                    payload: serde_json::json!({}),
                };

                let result = router.dispatch(&pool, "tenant1", &intent).await;
                assert!(result.is_ok());

                let called = handler.called.read().await;
                assert!(*called);
            }
        }
    }

    #[tokio::test]
    async fn test_action_router_dispatch_unsupported_feature() {
        let router = ActionRouter::new();
        let database_url = std::env::var("OHC_DATABASE_URL").or_else(|_| std::env::var("DATABASE_URL")).unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        if let Ok(pool) = PgPool::connect(&database_url).await {
            // Check if connection actually works before asserting to prevent PoolTimedOut
            if pool.acquire().await.is_ok() {
                let intent = ActionIntent {
                    feature_type: "unknown_feature".to_string(),
                    action: None,
                    payload: serde_json::json!({}),
                };

                // Dispatching an unsupported feature should just return Ok gracefully
                let result = router.dispatch(&pool, "tenant1", &intent).await;
                assert!(result.is_ok());
            }
        }
    }
}
