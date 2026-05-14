pub mod wizard;
#[cfg(test)]
pub mod wizard_test;

use axum::Router;
use std::sync::Arc;

pub fn router(_agent: Arc<crate::services::onboarding::onboarding_agent::OnboardingAgent>) -> Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    Router::new()
        //.nest("/wizard", wizard::router(wizard::AppState { db: agent.db.clone(), hub: agent.hub.clone() }))
}
