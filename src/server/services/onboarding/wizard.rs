use std::collections::HashMap;
use crate::services::onboarding::preflight;
use crate::services::onboarding::provisioner;

pub struct InteractiveWizard;

impl InteractiveWizard {
    pub fn new() -> Self {
        InteractiveWizard
    }

    pub fn run_interactive_setup(&self, is_cloud: bool) -> Result<HashMap<String, String>, String> {
        let preflight_res = preflight::run_preflight_check(is_cloud);
        if !preflight_res.passed {
            return Err(format!("preflight check failed: {}", preflight_res.message));
        }

        let mut config = HashMap::new();
        if is_cloud {
            config.insert("mode".to_string(), "cloud".to_string());
            config.insert("db".to_string(), "postgres".to_string());
            config.insert("cache".to_string(), "redis".to_string());
        } else {
            config.insert("mode".to_string(), "standalone".to_string());
            config.insert("db".to_string(), "sqlite".to_string());
            config.insert("cache".to_string(), "memory".to_string());
        }

        Ok(config)
    }

    pub fn generate_wizard_ui(&self, is_cloud: bool) -> String {
        let mode = if is_cloud { "Cloud-native" } else { "Standalone" };

        format!(
            "<div style=\"backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 24px; border-radius: 16px; border: 1px solid rgba(255, 255, 255, 0.1); box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);\">\n\
              <h2 style=\"margin-top: 0; color: #ffffff; font-weight: 600; font-size: 24px;\">OHC Interactive Setup ({})</h2>\n\
              <p style=\"color: rgba(255, 255, 255, 0.7); font-size: 16px; line-height: 1.5; margin-bottom: 0;\">Please review your configuration options.</p>\n\
            </div>",
            mode
        )
    }


    pub fn save_onboarding_state(&self, org_id: &str, user_id: &str, step: i32, state_json: &str) -> Result<(), String> {
        let org_id = org_id.to_string();
        let user_id = user_id.to_string();
        let state_json_parsed = state_json.to_string();

        tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current().block_on(async move {
                let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
                let pool = sqlx::AnyPool::connect(&db_url).await.unwrap();
                let _ = sqlx::query(
                    "INSERT INTO onboarding_state (tenant_id, organization_id, user_id, current_step, state_json) VALUES ($1, $2, $3, $4, $5) ON CONFLICT(organization_id) DO UPDATE SET current_step = EXCLUDED.current_step, state_json = EXCLUDED.state_json"
                )
                .bind(&org_id)
                .bind(&org_id)
                .bind(&user_id)
                .bind(step)
                .bind(&state_json_parsed)
                .execute(&pool)
                .await;
            })
        });
        Ok(())
    }

    pub fn get_onboarding_state(&self, org_id: &str) -> Result<String, String> {
        let org_id = org_id.to_string();
        let res = tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current().block_on(async move {
                let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
                let pool = sqlx::AnyPool::connect(&db_url).await.unwrap();
                use sqlx::Row;
                let row = sqlx::query("SELECT state_json FROM onboarding_state WHERE organization_id = $1")
                    .bind(&org_id)
                    .fetch_one(&pool)
                    .await;

                match row {
                    Ok(r) => {
                        let val: String = r.try_get("state_json").unwrap_or_else(|_| r#"{"step": 0}"#.to_string());
                        Ok(val)
                    },
                    Err(_) => Ok(r#"{"step": 0}"#.to_string()),
                }
            })
        });
        res
    }

    pub fn reset_environment(&self, is_cloud: bool) -> Result<(), String> {
        provisioner::cleanup_environment(is_cloud)?;
        provisioner::provision_environment(is_cloud)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_interactive_wizard_cloud() {
        let w = InteractiveWizard::new();
        let cfg = w.run_interactive_setup(true).unwrap();
        assert_eq!(cfg.get("mode").unwrap(), "cloud");
    }

    #[test]
    fn test_interactive_wizard_standalone() {
        let w = InteractiveWizard::new();
        let cfg = w.run_interactive_setup(false).unwrap();
        assert_eq!(cfg.get("mode").unwrap(), "standalone");
    }

    #[test]
    fn test_reset_environment() {
        let w = InteractiveWizard::new();
        
        // Ensure clean slate
        let _ = fs::remove_dir_all(".ohc-local-data");

        let res = w.reset_environment(false);
        assert!(res.is_ok());

        assert!(provisioner::check_environment(false).is_ok());

        fs::remove_dir_all(".ohc-local-data").unwrap();
    }
}
