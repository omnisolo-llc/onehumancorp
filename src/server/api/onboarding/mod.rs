
use axum::{
    extract::{State, Json},
    routing::{post, get},
    Router,
};
use std::sync::Arc;
use crate::services::onboarding::onboarding_agent::OnboardingAgent;
use ::server_ohc::orchestration::{StartOnboardingRequest, StartOnboardingResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteWizardPayload {
    pub current_step: Option<i32>,
    pub business_type: Option<String>,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub selling_cats: Option<Vec<String>>,
    pub payment: Option<String>,
    pub admin_name: Option<String>,
    pub admin_email: Option<String>,
    pub admin_pass: Option<String>,
    pub template: Option<String>,
    pub domain: Option<String>,
    pub agents: Option<Vec<String>>,
    pub colors: Option<Vec<String>>,
    pub agent_tone: Option<String>,
    pub agent_schedule: Option<i32>,
    pub agent_focus: Option<Vec<String>>,
}

pub fn router(agent: Arc<OnboardingAgent>) -> Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    let r = Router::new()
        .route("/start", post(start_onboarding))
        .route("/state", get(get_state))
        .route("/state", post(save_state))
        .route("/launch", post(launch_business))
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
    State(agent): State<Arc<OnboardingAgent>>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let email = params.get("email").map(|s| s.as_str()).unwrap_or("guest@local");

    let pool = &agent.db.pool;
    let res = sqlx::query("SELECT state_json FROM onboarding_state WHERE organization_id = $1")
        .bind(email)
        .fetch_optional(pool)
        .await;

    match res {
        Ok(Some(row)) => {
            use sqlx::Row;
            let val: serde_json::Value = row.try_get("state_json").unwrap_or_else(|_| serde_json::json!({ "step": 0 }));
            Ok(Json(val))
        },
        _ => Ok(Json(serde_json::json!({ "step": 0 }))),
    }
}


async fn save_state(
    State(agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<CompleteWizardPayload>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    // Empty name is fine initially
    let json_val = serde_json::to_value(&payload).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let email = payload.admin_email.unwrap_or_else(|| "guest@local".to_string());

    let pool = &agent.db.pool;
    let res = sqlx::query("INSERT INTO onboarding_state (organization_id, state_json) VALUES ($1, $2) ON CONFLICT(organization_id) DO UPDATE SET state_json = EXCLUDED.state_json")
        .bind(&email)
        .bind(json_val)
        .execute(pool).await;

    if res.is_err() {
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(axum::http::StatusCode::OK)
}


async fn launch_business(
    State(agent): State<Arc<OnboardingAgent>>,
    Json(payload): Json<CompleteWizardPayload>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    if payload.name.is_none() || payload.name.as_ref().unwrap().is_empty() {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }

    let new_org_id = uuid::Uuid::new_v4().to_string();
    let pool = &agent.db.pool;

    // Create Organization
    let res = sqlx::query("INSERT INTO organizations (id, name, template_id) VALUES ($1, $2, $3)")
        .bind(&new_org_id)
        .bind(payload.name.as_deref().unwrap_or("New Business"))
        .bind(payload.template.as_deref().unwrap_or("modern"))
        .execute(pool).await;

    if res.is_err() { return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR); }



    // Create Admin User securely
    if let (Some(email), Some(name), Some(pass)) = (&payload.admin_email, &payload.admin_name, &payload.admin_pass) {
        // Use a secure hashing mechanism (Argon2 or similar). For this environment, we'll use a mocked strong hash


        let pass_clone = pass.clone();
        let hashed_pass = match tokio::task::spawn_blocking(move || bcrypt::hash(pass_clone, bcrypt::DEFAULT_COST)).await {
            Ok(Ok(h)) => h,
            _ => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
        };


        let res = sqlx::query("INSERT INTO users (id, organization_id, email, username, password_hash, roles) VALUES ($1, $2, $3, $4, $5, $6)")

            .bind(uuid::Uuid::new_v4().to_string())
            .bind(&new_org_id)
            .bind(email)
            .bind(name)
            .bind(hashed_pass)
            .bind(serde_json::json!(["admin"]))
            .execute(pool).await;

        if res.is_err() { return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR); }
    } else {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }


    // Create Agents
    if let Some(agents) = &payload.agents {
        for role in agents {
            let _ = sqlx::query("INSERT INTO agents (id, organization_id, role, status) VALUES ($1, $2, $3, $4)")
                .bind(uuid::Uuid::new_v4().to_string())
                .bind(&new_org_id)
                .bind(role)
                .bind("IDLE")
                .execute(pool).await;
        }
    }
    Ok(axum::http::StatusCode::OK)
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_wizard_deserialization() {
        let json = r#"{"name": "Test Bakery", "agents": ["Customer Support"], "sellingCats": ["Physical products"], "businessType": "Online Store", "currentStep": 3}"#;
        let payload: CompleteWizardPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.name.unwrap(), "Test Bakery");
        assert_eq!(payload.agents.unwrap().len(), 1);
        assert_eq!(payload.selling_cats.unwrap()[0], "Physical products");
        assert_eq!(payload.business_type.unwrap(), "Online Store");
        assert_eq!(payload.current_step.unwrap(), 3);
    }
}

#[cfg(test)]
mod onboarding_handler_tests {
    use super::*;
    use axum::http::StatusCode;
    use crate::db::DB;
    use crate::hub::Hub;
    use crate::services::onboarding::onboarding_agent::OnboardingAgent;
    use std::sync::Arc;

    async fn setup_test_agent() -> Option<Arc<OnboardingAgent>> {
        let _ = std::env::var("DATABASE_URL").ok()?;
        unsafe { std::env::set_var("OHC_SQLITE_KEY", "test-fallback-key"); }
        let db = Arc::new(DB::new().await.ok()?);
        let (tx, _) = tokio::sync::mpsc::channel(10);
        let hub = Arc::new(Hub::new(tx, db.pool.clone()));
        Some(Arc::new(OnboardingAgent::new(db, hub)))
    }


    #[tokio::test]
    async fn test_save_state_missing_name() {
        let agent = match setup_test_agent().await { Some(a) => a, None => return };
        let payload = CompleteWizardPayload {
            current_step: Some(1),
            business_type: None,
            name: Some("".to_string()), // Empty name is valid for early steps
            desc: None,
            selling_cats: None,
            payment: None,
            admin_name: None,
            admin_email: Some("test@example.com".to_string()),
            admin_pass: None,
            template: None,
            domain: None,
            agents: None,
            colors: None,
            agent_tone: None,
            agent_schedule: None,
            agent_focus: None,
        };

        let res = save_state(State(agent.clone()), Json(payload)).await;
        // The handler now accepts empty names to support early wizard steps
        assert!(res.is_ok());
    }


    #[tokio::test]
    async fn test_save_and_get_state_success() {
        let agent = match setup_test_agent().await { Some(a) => a, None => return };

        let test_email = format!("test-{}@example.com", uuid::Uuid::new_v4());
        let payload = CompleteWizardPayload {
            current_step: Some(5),
            business_type: Some("Online Store".to_string()),
            name: Some("My Shop".to_string()),
            desc: None,
            selling_cats: None,
            payment: None,
            admin_name: None,
            admin_email: Some(test_email.clone()),
            admin_pass: None,
            template: None,
            domain: None,
            agents: None,
            colors: None,
            agent_tone: None,
            agent_schedule: None,
            agent_focus: None,
        };

        // 1. Save state
        let res = save_state(State(agent.clone()), Json(payload)).await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), StatusCode::OK);

        // 2. Get state
        let mut hm = std::collections::HashMap::new();
        hm.insert("email".to_string(), test_email.clone());
        let res_get = get_state(State(agent.clone()), axum::extract::Query(hm)).await;
        assert!(res_get.is_ok());
        let val = res_get.unwrap().0;

        // Assert deserialized payload properties match
        assert_eq!(val["currentStep"].as_i64(), Some(5));
        assert_eq!(val["businessType"].as_str(), Some("Online Store"));
        assert_eq!(val["name"].as_str(), Some("My Shop"));
        assert_eq!(val["adminEmail"].as_str(), Some(test_email.as_str()));
    }

    #[tokio::test]
    async fn test_launch_business_success() {
        let agent = match setup_test_agent().await { Some(a) => a, None => return };

        let test_email = format!("launch-{}@example.com", uuid::Uuid::new_v4());
        let payload = CompleteWizardPayload {
            current_step: Some(10),
            business_type: Some("Service".to_string()),
            name: Some("Awesome Launch Service".to_string()),
            desc: None,
            selling_cats: None,
            payment: None,
            admin_name: Some("Admin User".to_string()),
            admin_email: Some(test_email.clone()),
            admin_pass: Some("super-secret-password".to_string()),
            template: Some("modern".to_string()),
            domain: None,
            agents: Some(vec!["Sales".to_string(), "Support".to_string()]),
            colors: None,
            agent_tone: None,
            agent_schedule: None,
            agent_focus: None,
        };

        let res = launch_business(State(agent.clone()), Json(payload)).await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), StatusCode::OK);

        let pool = &agent.db.pool;

        // Verify User Creation
        use sqlx::Row;
        let user_row = sqlx::query("SELECT username, roles, organization_id FROM users WHERE email = $1")
            .bind(&test_email)
            .fetch_one(pool)
            .await
            .unwrap();

        assert_eq!(user_row.get::<String, _>("username"), "Admin User");
        assert!(user_row.get::<String, _>("roles").contains("admin"));

        let org_id: String = user_row.get("organization_id");

        // Verify Organization Creation
        let org_row = sqlx::query("SELECT name, template_id FROM organizations WHERE id = $1")
            .bind(&org_id)
            .fetch_one(pool)
            .await
            .unwrap();

        assert_eq!(org_row.get::<String, _>("name"), "Awesome Launch Service");
        assert_eq!(org_row.get::<String, _>("template_id"), "modern");

        // Verify Agents Creation
        let agent_rows = sqlx::query("SELECT role FROM agents WHERE organization_id = $1")
            .bind(&org_id)
            .fetch_all(pool)
            .await
            .unwrap();

        assert_eq!(agent_rows.len(), 2);
        let roles: Vec<String> = agent_rows.into_iter().map(|r| r.get("role")).collect();
        assert!(roles.contains(&"Sales".to_string()));
        assert!(roles.contains(&"Support".to_string()));
    }
}
