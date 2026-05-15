use std::collections::HashMap;
use crate::services::onboarding::preflight;
use crate::services::onboarding::provisioner;

pub struct InteractiveWizard;

pub struct WizardStep {
    pub id: String,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
}

pub struct WizardState {
    pub current_step: i32,
    pub steps: Vec<WizardStep>,
    pub completed_percentage: f32,
}

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


    pub fn save_onboarding_state(&self, _org_id: &str, _user_id: &str, _step: i32, _state_json: &str) -> Result<(), String> {
        // Here we would use sqlx to persist to the onboarding_state table
        Ok(())
    }

    pub fn get_onboarding_state(&self, _org_id: &str) -> Result<String, String> {
        // Return dummy json for now
        Ok(r#"{"step": 0}"#.to_string())
    }


    pub fn get_wizard_progress(&self, state: &WizardState) -> f32 {
        if state.steps.is_empty() {
            return 0.0;
        }
        let completed = state.steps.iter().filter(|s| s.is_completed).count() as f32;
        (completed / state.steps.len() as f32) * 100.0
    }

    pub fn generate_welcome_checklist(&self) -> Vec<WizardStep> {
        vec![
            WizardStep {
                id: "step_1".to_string(),
                title: "✅ Business live".to_string(),
                description: "Your business is now live on the internet!".to_string(),
                is_completed: true,
            },
            WizardStep {
                id: "step_2".to_string(),
                title: "⬜ Add 3 more products".to_string(),
                description: "Increase your sales by adding more products.".to_string(),
                is_completed: false,
            },
            WizardStep {
                id: "step_3".to_string(),
                title: "⬜ Connect Instagram".to_string(),
                description: "Reach more customers by connecting your social media.".to_string(),
                is_completed: false,
            },
            WizardStep {
                id: "step_4".to_string(),
                title: "⬜ Share your link with a friend".to_string(),
                description: "Word of mouth is the best marketing.".to_string(),
                is_completed: false,
            },
        ]
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

#[cfg(test)]
mod additional_tests {
    use super::*;

    #[test]
    fn test_wizard_progress() {
        let w = InteractiveWizard::new();
        let state = WizardState {
            current_step: 0,
            steps: w.generate_welcome_checklist(),
            completed_percentage: 0.0,
        };
        assert_eq!(w.get_wizard_progress(&state), 25.0);
    }
}
