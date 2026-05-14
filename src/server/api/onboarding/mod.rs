
use axum::{
    extract::{State, Json, Extension},
    routing::{post, get},
    Router,
};
use std::sync::Arc;
use crate::services::onboarding::onboarding_agent::OnboardingAgent;
use crate::services::onboarding::wizard::InteractiveWizard;
use ::server_ohc::orchestration::{StartOnboardingRequest, StartOnboardingResponse};
use ::server_auth::orchestration::AuthInfo;

pub fn router(agent: Arc<OnboardingAgent>) -> Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    let r = Router::new()
        .route("/start", post(start_onboarding))
        .route("/state", get(get_state).post(save_state))
        .with_state(agent);

    Router::new().merge(r)
}

async fn start_onboarding(
    State(agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<StartOnboardingRequest>,
) -> Result<Json<StartOnboardingResponse>, axum::http::StatusCode> {
    match agent.start_onboarding(payload).await {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_state(
    auth: Option<Extension<AuthInfo>>,
    headers: axum::http::HeaderMap,
    State(_agent): State<Arc<OnboardingAgent>>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let tid = headers.get("X-Transient-Id").and_then(|v| v.to_str().ok()).unwrap_or("anonymous_tenant");
    let org_id = auth.map(|a| a.org_id.clone()).unwrap_or_else(|| tid.to_string());
    let wizard = InteractiveWizard::new();
    match wizard.get_onboarding_state(&org_id) {
        Ok(state) => {
            if let Ok(json) = serde_json::from_str(&state) {
                Ok(Json(json))
            } else {
                Ok(Json(serde_json::json!({})))
            }
        },
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn save_state(
    auth: Option<Extension<AuthInfo>>,
    headers: axum::http::HeaderMap,
    State(_agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<serde_json::Value>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let tid = headers.get("X-Transient-Id").and_then(|v| v.to_str().ok()).unwrap_or("anonymous_tenant");
    let org_id = auth.as_ref().map(|a| a.org_id.clone()).unwrap_or_else(|| tid.to_string());
    let user_id = auth.map(|a| a.spiffe_id.clone()).unwrap_or_else(|| tid.to_string());

    let wizard = InteractiveWizard::new();
    let state_str = payload.to_string();
    let step = payload.get("step").and_then(|v| v.as_i64()).unwrap_or(1) as i32;
    match wizard.save_onboarding_state(&org_id, &user_id, step, &state_str) {
        Ok(_) => Ok(axum::http::StatusCode::NO_CONTENT),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use serde_json::json;
    use crate::db::{DB, DbStore};
    use sqlx::sqlite::SqlitePoolOptions;

    // We mock the DB just to satisfy OnboardingAgent creation requirements
    async fn setup_agent() -> Arc<OnboardingAgent> {
        let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();
        let pg_pool = sqlx::PgPool::connect_lazy("postgres://localhost/dummy").unwrap();
        let db = Arc::new(DB { pool: pg_pool, store: DbStore::Sqlite(pool) });
        let (tx, _rx) = tokio::sync::mpsc::channel(10);
        let hub = Arc::new(crate::hub::Hub::new(tx, db.pool.clone()));
        Arc::new(OnboardingAgent::new(db, hub))
    }

    #[tokio::test]
    async fn test_get_state_no_auth() {
        let agent = setup_agent().await;
        let res = get_state(None, axum::http::HeaderMap::new(), State(agent)).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_save_state_no_auth() {
        let agent = setup_agent().await;
        let payload = Json(json!({"step": 2, "businessName": "Test"}));
        let res = save_state(None, axum::http::HeaderMap::new(), State(agent), payload).await;
        assert_eq!(res.unwrap(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_save_state_with_auth() {
        let agent = setup_agent().await;
        let auth = Extension(AuthInfo {
            spiffe_id: "test-user".to_string(),
            org_id: "test-org".to_string(),
            agent_id: "".to_string(),
        });
        let payload = Json(json!({"step": 3, "businessName": "Test Org"}));
        let res = save_state(Some(auth), axum::http::HeaderMap::new(), State(agent), payload).await;
        assert_eq!(res.unwrap(), StatusCode::NO_CONTENT);
    }
}
