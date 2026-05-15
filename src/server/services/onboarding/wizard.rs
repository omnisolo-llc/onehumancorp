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
        let welcome_screen = r#"
            <div id="setup-screen" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 24px; border-radius: 16px; border: 1px solid rgba(255, 255, 255, 0.1); box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);">
                <div class="step" id="step-1">
                    <h1 style="color: #ffffff; font-weight: 600; font-size: 28px;">Welcome to OHC</h1>
                    <p style="color: rgba(255, 255, 255, 0.7); font-size: 18px;">Your business, live in minutes.</p>
                    <button onclick="nextStep(2)" style="background: #ffffff; color: #000000; padding: 12px 24px; border-radius: 8px; border: none; font-weight: bold; cursor: pointer;">Start Setup →</button>
                </div>
                <div class="step" id="step-2" style="display: none;">
                    <h2 style="color: #ffffff;">What is your business type?</h2>
                    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 16px; margin-top: 16px;">
                        <button class="type-card" onclick="selectType('Online Store')">🛍️ Online Store</button>
                        <button class="type-card" onclick="selectType('Service Business')">📅 Service Business</button>
                        <button class="type-card" onclick="selectType('Restaurant / Food')">🍽️ Restaurant / Food</button>
                        <button class="type-card" onclick="selectType('Creative / Portfolio')">🎨 Creative / Portfolio</button>
                        <button class="type-card" onclick="selectType('Local Business')">🏪 Local Business</button>
                        <button class="type-card" onclick="selectType('Other')">✨ Other</button>
                    </div>
                </div>
                <div class="step" id="step-3" style="display: none;">
                    <h2 style="color: #ffffff;">Business Name & Description</h2>
                    <input type="text" id="biz-name" placeholder="Your Business Name" style="width: 100%; padding: 16px; border-radius: 8px; background: rgba(255,255,255,0.1); border: 1px solid rgba(255,255,255,0.2); color: white; margin-bottom: 16px; font-size: 18px;" onchange="autoSuggestDescription(this.value)">
                    <textarea id="biz-desc" placeholder="AI auto-suggests a short description..." rows="3" style="width: 100%; padding: 16px; border-radius: 8px; background: rgba(255,255,255,0.1); border: 1px solid rgba(255,255,255,0.2); color: white; font-size: 16px;"></textarea>
                    <button onclick="nextStep(4)" style="margin-top: 16px; background: #ffffff; color: #000000; padding: 12px 24px; border-radius: 8px; border: none; font-weight: bold; cursor: pointer;">Next →</button>
                </div>
                <div class="step" id="step-4" style="display: none;">
                    <h2 style="color: #ffffff;">What do you sell?</h2>
                    <div style="display: flex; flex-direction: column; gap: 12px;">
                        <label style="color: white; display: flex; align-items: center; padding: 12px; background: rgba(255,255,255,0.05); border-radius: 8px;"><input type="checkbox" style="margin-right: 12px;"> Physical products</label>
                        <label style="color: white; display: flex; align-items: center; padding: 12px; background: rgba(255,255,255,0.05); border-radius: 8px;"><input type="checkbox" style="margin-right: 12px;"> Digital downloads</label>
                        <label style="color: white; display: flex; align-items: center; padding: 12px; background: rgba(255,255,255,0.05); border-radius: 8px;"><input type="checkbox" style="margin-right: 12px;"> Services / appointments</label>
                        <label style="color: white; display: flex; align-items: center; padding: 12px; background: rgba(255,255,255,0.05); border-radius: 8px;"><input type="checkbox" style="margin-right: 12px;"> Food & beverages</label>
                        <label style="color: white; display: flex; align-items: center; padding: 12px; background: rgba(255,255,255,0.05); border-radius: 8px;"><input type="checkbox" style="margin-right: 12px;"> Subscriptions</label>
                    </div>
                    <button onclick="nextStep(5)" style="margin-top: 16px; background: #ffffff; color: #000000; padding: 12px 24px; border-radius: 8px; border: none; font-weight: bold; cursor: pointer;">Next →</button>
                </div>
                <div class="step" id="step-5" style="display: none;">
                    <h2 style="color: #ffffff;">How do you want to receive payments?</h2>
                    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 16px;">
                        <div style="padding: 16px; background: rgba(255,255,255,0.05); border-radius: 8px; color: white;">
                            <h3>Online only</h3>
                            <p style="font-size: 14px; opacity: 0.7;">Est. time to first payment: 5 mins</p>
                            <button onclick="nextStep(6)" style="width: 100%; margin-top: 8px;">Select</button>
                        </div>
                        <div style="padding: 16px; background: rgba(255,255,255,0.05); border-radius: 8px; color: white;">
                            <h3>In-person (POS)</h3>
                            <p style="font-size: 14px; opacity: 0.7;">Est. time to first payment: 2 days (hardware delivery)</p>
                            <button onclick="nextStep(6)" style="width: 100%; margin-top: 8px;">Select</button>
                        </div>
                        <div style="padding: 16px; background: rgba(255,255,255,0.05); border-radius: 8px; color: white;">
                            <h3>Both</h3>
                            <p style="font-size: 14px; opacity: 0.7;">Est. time to first payment: 5 mins online</p>
                            <button onclick="nextStep(6)" style="width: 100%; margin-top: 8px;">Select</button>
                        </div>
                        <div style="padding: 16px; background: rgba(255,255,255,0.05); border-radius: 8px; color: white; display: flex; align-items: center; justify-content: center;">
                            <button onclick="nextStep(6)" style="background: transparent; border: 1px solid white; color: white;">Skip for now</button>
                        </div>
                    </div>
                </div>
                <div class="step" id="step-6" style="display: none;">
                    <h2 style="color: #ffffff;">Administrator account</h2>
                    <input type="text" placeholder="Full Name" style="width: 100%; padding: 16px; border-radius: 8px; background: rgba(255,255,255,0.1); border: 1px solid rgba(255,255,255,0.2); color: white; margin-bottom: 12px;">
                    <input type="email" placeholder="Email Address" style="width: 100%; padding: 16px; border-radius: 8px; background: rgba(255,255,255,0.1); border: 1px solid rgba(255,255,255,0.2); color: white; margin-bottom: 12px;">
                    <input type="password" placeholder="Password" style="width: 100%; padding: 16px; border-radius: 8px; background: rgba(255,255,255,0.1); border: 1px solid rgba(255,255,255,0.2); color: white; margin-bottom: 8px;">
                    <div style="height: 4px; background: rgba(255,255,255,0.1); border-radius: 2px; margin-bottom: 16px;"><div style="width: 0%; height: 100%; background: #4CAF50; border-radius: 2px; transition: width 0.3s;" id="pwd-strength"></div></div>
                    <div style="display: flex; gap: 12px;">
                        <button style="flex: 1; padding: 12px; background: white; color: black; border-radius: 8px; border: none; font-weight: bold;">Continue with Google</button>
                        <button style="flex: 1; padding: 12px; background: black; color: white; border-radius: 8px; border: 1px solid rgba(255,255,255,0.2); font-weight: bold;">Continue with Apple</button>
                    </div>
                    <button onclick="nextStep(7)" style="margin-top: 24px; width: 100%; background: #ffffff; color: #000000; padding: 16px; border-radius: 8px; border: none; font-weight: bold; cursor: pointer; font-size: 18px;">Review & Launch</button>
                </div>
                <div class="step" id="step-7" style="display: none; text-align: center;">
                    <h2 style="color: #ffffff;">Review & Launch</h2>
                    <p style="color: rgba(255,255,255,0.7);">Your {mode} setup is almost complete!</p>
                    <button class="pulse-launch" onclick="launchBusiness()" style="background: #4CAF50; color: white; padding: 20px 40px; border-radius: 40px; border: none; font-weight: bold; font-size: 24px; cursor: pointer; margin-top: 24px; box-shadow: 0 0 20px rgba(76, 175, 80, 0.5);">Launch My Business →</button>
                </div>

                <div id="progress-overlay" style="display: none; position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.8); backdrop-filter: blur(10px); z-index: 1000; flex-direction: column; align-items: center; justify-content: center; color: white;">
                    <div class="spinner" style="width: 50px; height: 50px; border: 4px solid rgba(255,255,255,0.1); border-top-color: white; border-radius: 50%; animation: spin 1s linear infinite;"></div>
                    <h2 style="margin-top: 24px;">Your business is setting up...</h2>
                    <p>Provisioning tenant...</p>
                    <p>Selecting starter template...</p>
                    <p>Pre-seeding AI agents...</p>
                </div>

                <style>
                    .type-card { background: rgba(255,255,255,0.1); border: 1px solid rgba(255,255,255,0.2); border-radius: 12px; padding: 20px; color: white; font-size: 16px; cursor: pointer; transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1); }
                    .type-card:hover { background: rgba(255,255,255,0.2); transform: translateY(-2px); }
                    @keyframes spin { to { transform: rotate(360deg); } }
                    @keyframes pulse { 0% { transform: scale(1); } 50% { transform: scale(1.05); } 100% { transform: scale(1); } }
                    .pulse-launch { animation: pulse 2s infinite; }
                    .step { animation: fadeIn 0.3s cubic-bezier(0.4, 0, 0.2, 1); }
                    @keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
                </style>
                <script>
                    function nextStep(step) {
                        document.querySelectorAll('.step').forEach(el => el.style.display = 'none');
                        document.getElementById('step-' + step).style.display = 'block';
                        fetch('/api/onboarding/state', { method: 'POST', body: JSON.stringify({step: step}) });
                    }
                    function selectType(type) {
                        fetch('/api/onboarding/state', { method: 'POST', body: JSON.stringify({business_type: type}) });
                        nextStep(3);
                    }
                    function autoSuggestDescription(name) {
                        document.getElementById('biz-desc').value = "Loading suggestion for " + name + "...";
                        setTimeout(() => {
                            document.getElementById('biz-desc').value = "A premium business offering top-tier products and services under the name " + name + ".";
                        }, 800);
                    }
                    function launchBusiness() {
                        document.getElementById('progress-overlay').style.display = 'flex';
                        setTimeout(() => { window.location.href = '/dashboard'; }, 3000);
                    }
                    document.querySelector('input[type="password"]').addEventListener('input', function(e) {
                        let val = e.target.value;
                        let strength = val.length > 8 ? (val.match(/[0-9]/) ? (val.match(/[^A-Za-z0-9]/) ? 100 : 66) : 33) : (val.length > 0 ? 10 : 0);
                        document.getElementById('pwd-strength').style.width = strength + '%';
                        document.getElementById('pwd-strength').style.background = strength > 66 ? '#4CAF50' : (strength > 33 ? '#FFC107' : '#f44336');
                    });
                </script>
            </div>"#;
        welcome_screen.replace("{mode}", mode)
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
}
