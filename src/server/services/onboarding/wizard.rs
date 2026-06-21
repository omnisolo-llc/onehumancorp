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
            "<style>\n\
            .wizard-glassmorphism {{\n\
                backdrop-filter: blur(30px) saturate(210%);\n\
                -webkit-backdrop-filter: blur(30px) saturate(210%);\n\
                background: rgba(255, 255, 255, 0.65);\n\
                border: 1px solid rgba(255, 255, 255, 0.4);\n\
                border-radius: 16px;\n\
                padding: 24px;\n\
                box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);\n\
                font-family: 'Outfit', 'Inter', sans-serif;\n\
                color: #1D1D1F;\n\
            }}\n\
            .wizard-glassmorphism p {{\n\
                color: #555555;\n\
            }}\n\
            @media (prefers-color-scheme: dark) {{\n\
                .wizard-glassmorphism {{\n\
                    background: rgba(22, 22, 26, 0.7);\n\
                    border: 1px solid rgba(255, 255, 255, 0.1);\n\
                    color: #F5F5F7;\n\
                }}\n\
                .wizard-glassmorphism p {{\n\
                    color: #A1A1A6;\n\
                }}\n\
            }}\n\
            </style>\n\
            <div class=\"wizard-glassmorphism\">\n\
              <h2 style=\"margin-top: 0; font-weight: 600; font-size: 24px;\">OHC Interactive Setup ({})</h2>\n\
              <p style=\"font-size: 16px; line-height: 1.5; margin-bottom: 0;\">Please review your configuration options.</p>\n\
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

    #[test]
    fn test_generate_wizard_ui_styling() {
        let w = InteractiveWizard::new();
        let ui = w.generate_wizard_ui(false);

        // Verify light mode specifics
        assert!(ui.contains("background: rgba(255, 255, 255, 0.65)"));
        assert!(ui.contains("border: 1px solid rgba(255, 255, 255, 0.4)"));

        // Verify dark mode specifics
        assert!(ui.contains("@media (prefers-color-scheme: dark)"));
        assert!(ui.contains("background: rgba(22, 22, 26, 0.7)"));
        assert!(ui.contains("border: 1px solid rgba(255, 255, 255, 0.1)"));

        // Verify other requirements
        assert!(ui.contains("border-radius: 16px"));
        assert!(ui.contains("backdrop-filter: blur(30px) saturate(210%)"));
    }
}
