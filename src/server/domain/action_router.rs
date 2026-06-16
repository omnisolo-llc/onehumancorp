
use std::collections::HashMap;
use serde_json::Value;
use sqlx::PgPool;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub type ActionHandler = Box<dyn Fn(String, Value, PgPool) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>> + Send + Sync>;

pub struct ActionRouter {
    pub handlers: HashMap<String, Arc<ActionHandler>>,
}

impl ActionRouter {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register<F, Fut>(&mut self, feature_type: &str, handler: F)
    where
        F: Fn(String, Value, PgPool) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), String>> + Send + 'static,
    {
        self.handlers.insert(
            feature_type.to_string(),
            Arc::new(Box::new(move |tenant_id, payload, pool| Box::pin(handler(tenant_id, payload, pool)))),
        );
    }

    pub async fn dispatch(&self, feature_type: &str, tenant_id: String, payload: Value, pool: PgPool) -> Result<(), String> {
        if let Some(handler) = self.handlers.get(feature_type) {
            handler(tenant_id, payload, pool).await
        } else {
            Err(format!("No handler registered for feature_type: {}", feature_type))
        }
    }
}

pub fn get_action_router() -> &'static ActionRouter {
    static ROUTER: std::sync::OnceLock<ActionRouter> = std::sync::OnceLock::new();
    ROUTER.get_or_init(|| {
        let mut router = ActionRouter::new();
        router.register("incident_resolution", crate::domain::incident::handle_incident_resolution);
        router.register("social_post_draft", crate::domain::social::handle_social_post_draft);
        router.register("quote_draft", crate::domain::sales::handle_quote_draft);
        router.register("ambassador_reply", crate::domain::crm::handle_ambassador_reply);
        router.register("instagram_dm", crate::domain::crm::handle_instagram_dm);
        router
    })
}

#[cfg(test)]
mod tests {
    use super::get_action_router;

    #[tokio::test]
    async fn test_action_router_dispatch_not_found() {
        assert!(!get_action_router().handlers.contains_key("non_existent"));
    }
}
