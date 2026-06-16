pub mod action_router_router;
pub mod action_router_handlers;

pub mod action_router {
    pub use crate::domain::action_router_router::{ActionRouter, ActionHandler};
    pub use crate::domain::action_router_handlers::{IncidentResolutionHandler, SocialPostDraftHandler, AmbassadorReplyHandler, QuoteDraftHandler, InstagramDmHandler};

    use std::sync::Arc;

    pub fn create_default_router() -> Arc<ActionRouter> {
        static ROUTER: std::sync::OnceLock<Arc<ActionRouter>> = std::sync::OnceLock::new();
        ROUTER.get_or_init(|| {
            let mut router = ActionRouter::new();
            router.register_handler("incident_resolution", Arc::new(IncidentResolutionHandler));
            router.register_handler("social_post_draft", Arc::new(SocialPostDraftHandler));
            router.register_handler("ambassador_reply", Arc::new(AmbassadorReplyHandler));
            router.register_handler("quote_draft", Arc::new(QuoteDraftHandler));
            router.register_handler("instagram_dm", Arc::new(InstagramDmHandler));
            Arc::new(router)
        }).clone()
    }
}

pub mod repository;
pub mod organization;
pub mod model;
pub mod blueprint;
pub mod federation;
pub mod b2b;
pub mod compute;
pub mod sre;

#[cfg(test)]
pub mod unified_tenant_test;
pub mod subscription;
