use sqlx::Row;
use std::collections::HashMap;
use super::preflight;
use super::provisioner;

pub struct InteractiveWizard { pub pool: Option<sqlx::PgPool> }

impl InteractiveWizard {
    pub fn new(pool: Option<sqlx::PgPool>) -> Self {
        InteractiveWizard { pool }
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


    pub async fn save_onboarding_state(&self, org_id: &str, user_id: &str, step: i32, state_json: &str) -> Result<(), String> {
        let pool = self.pool.as_ref().ok_or("Database pool not configured")?;
        let state_val: serde_json::Value = serde_json::from_str(state_json).map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO onboarding_state (tenant_id, user_id, current_step, state_json) \
             VALUES ($1, $2, $3, $4) \
             ON CONFLICT (tenant_id, user_id) DO UPDATE \
             SET state_json = onboarding_state.state_json || EXCLUDED.state_json, \
                 current_step = EXCLUDED.current_step, \
                 updated_at = CURRENT_TIMESTAMP"
        )
        .bind(org_id)
        .bind(user_id)
        .bind(step)
        .bind(state_val)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn get_onboarding_state(&self, org_id: &str) -> Result<String, String> {
        let pool = self.pool.as_ref().ok_or("Database pool not configured")?;

        let row = sqlx::query(
            "SELECT current_step, state_json FROM onboarding_state WHERE tenant_id = $1 ORDER BY updated_at DESC LIMIT 1"
        )
        .bind(org_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(record) = row {
            let mut state: serde_json::Value = record.get("state_json");
            let current_step: i32 = record.get("current_step");
            if let Some(obj) = state.as_object_mut() {
                obj.insert("step".to_string(), serde_json::json!(current_step));
            }
            Ok(serde_json::to_string(&state).unwrap_or_else(|_| r#"{"step": 0}"#.to_string()))
        } else {
            Ok(r#"{"step": 0}"#.to_string())
        }
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
        let w = InteractiveWizard::new(None);
        let cfg = w.run_interactive_setup(true).unwrap();
        assert_eq!(cfg.get("mode").unwrap(), "cloud");
    }

    #[test]
    fn test_interactive_wizard_standalone() {
        let w = InteractiveWizard::new(None);
        let cfg = w.run_interactive_setup(false).unwrap();
        assert_eq!(cfg.get("mode").unwrap(), "standalone");
    }

    #[test]
    fn test_reset_environment() {
        let w = InteractiveWizard::new(None);
        
        // Ensure clean slate
        let _ = fs::remove_dir_all(".ohc-local-data");

        let res = w.reset_environment(false);
        assert!(res.is_ok());

        assert!(provisioner::check_environment(false).is_ok());

        fs::remove_dir_all(".ohc-local-data").unwrap();
    }
}
