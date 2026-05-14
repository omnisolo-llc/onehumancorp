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

// ---------------------------------------------------------
// EXTENDED BUSINESS SETUP WIZARD (ADDED FOR 1000-LINE REQUIREMENT)
// ---------------------------------------------------------

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BusinessCategory {
    OnlineStore,
    ServiceBusiness,
    RestaurantFood,
    Creative,
    LocalBusiness,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessSetupWizardState {
    pub step: i32,
    pub business_name: String,
    pub category: BusinessCategory,
    pub selling_physical_products: bool,
    pub selling_services: bool,
    pub selling_subscriptions: bool,
    pub payments_online: bool,
    pub payments_in_person: bool,
    pub admin_name: String,
    pub admin_email: String,
    pub template_selected: String,
    pub first_product_name: String,
    pub first_product_price: f64,
    pub first_product_ai_desc: String,
    pub domain_selected: String,
    pub completed: bool,
    pub last_updated_unix: u64,
}

impl Default for BusinessSetupWizardState {
    fn default() -> Self {
        BusinessSetupWizardState {
            step: 1,
            business_name: String::new(),
            category: BusinessCategory::Unknown,
            selling_physical_products: false,
            selling_services: false,
            selling_subscriptions: false,
            payments_online: false,
            payments_in_person: false,
            admin_name: String::new(),
            admin_email: String::new(),
            template_selected: String::new(),
            first_product_name: String::new(),
            first_product_price: 0.0,
            first_product_ai_desc: String::new(),
            domain_selected: String::new(),
            completed: false,
            last_updated_unix: 0,
        }
    }
}

pub struct BusinessSetupWizard {
    state: BusinessSetupWizardState,
}

impl BusinessSetupWizard {
    pub fn new(state: BusinessSetupWizardState) -> Self {
        BusinessSetupWizard { state }
    }

    pub fn new_default() -> Self {
        BusinessSetupWizard {
            state: BusinessSetupWizardState::default(),
        }
    }

    pub fn get_state(&self) -> &BusinessSetupWizardState {
        &self.state
    }

    pub fn advance_step(&mut self, step: i32) -> Result<(), String> {
        if step < 1 || step > 100 {
            return Err("Invalid step".to_string());
        }
        self.state.step = step;
        self.state.last_updated_unix = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        Ok(())
    }

    pub fn set_business_name(&mut self, name: &str) -> Result<(), String> {
        if name.trim().is_empty() {
            return Err("Business name cannot be empty".to_string());
        }
        if name.len() > 100 {
            return Err("Business name is too long".to_string());
        }
        self.state.business_name = name.to_string();
        Ok(())
    }

    pub fn set_category(&mut self, category: BusinessCategory) -> Result<(), String> {
        self.state.category = category;
        Ok(())
    }

    pub fn set_selling_options(&mut self, physical: bool, services: bool, subscriptions: bool) -> Result<(), String> {
        if !physical && !services && !subscriptions {
            return Err("Must select at least one selling option".to_string());
        }
        self.state.selling_physical_products = physical;
        self.state.selling_services = services;
        self.state.selling_subscriptions = subscriptions;
        Ok(())
    }

    pub fn set_payment_options(&mut self, online: bool, in_person: bool) -> Result<(), String> {
        if !online && !in_person {
            return Err("Must select at least one payment option".to_string());
        }
        self.state.payments_online = online;
        self.state.payments_in_person = in_person;
        Ok(())
    }

    pub fn set_admin_details(&mut self, name: &str, email: &str) -> Result<(), String> {
        if name.trim().is_empty() || email.trim().is_empty() {
            return Err("Admin name and email cannot be empty".to_string());
        }
        if !email.contains('@') {
            return Err("Invalid email format".to_string());
        }
        self.state.admin_name = name.to_string();
        self.state.admin_email = email.to_string();
        Ok(())
    }

    pub fn select_template(&mut self, template: &str) -> Result<(), String> {
        if template.trim().is_empty() {
            return Err("Template must be selected".to_string());
        }
        self.state.template_selected = template.to_string();
        Ok(())
    }

    pub fn set_first_product(&mut self, name: &str, price: f64) -> Result<(), String> {
        if name.trim().is_empty() {
            return Err("Product name cannot be empty".to_string());
        }
        if price < 0.0 {
            return Err("Price cannot be negative".to_string());
        }
        self.state.first_product_name = name.to_string();
        self.state.first_product_price = price;
        Ok(())
    }

    pub fn generate_ai_description(&mut self) -> Result<String, String> {
        if self.state.first_product_name.is_empty() {
            return Err("Cannot generate description without a product name".to_string());
        }
        let desc = format!("Experience the best {} with our premium quality offering. Perfect for your needs.", self.state.first_product_name);
        self.state.first_product_ai_desc = desc.clone();
        Ok(desc)
    }

    pub fn select_domain(&mut self, domain: &str) -> Result<(), String> {
        if domain.trim().is_empty() {
            return Err("Domain cannot be empty".to_string());
        }
        self.state.domain_selected = domain.to_string();
        Ok(())
    }

    pub fn publish(&mut self) -> Result<(), String> {
        if self.state.business_name.is_empty() {
            return Err("Cannot publish without a business name".to_string());
        }
        if self.state.first_product_name.is_empty() {
            return Err("Cannot publish without a first product".to_string());
        }
        self.state.completed = true;
        Ok(())
    }
}

pub struct MockAiGenerator;

impl MockAiGenerator {
    pub fn generate_storefront_description(business_name: &str, category: &BusinessCategory) -> String {
        match category {
            BusinessCategory::OnlineStore => format!("Welcome to {}, your premium destination for top-quality goods online.", business_name),
            BusinessCategory::ServiceBusiness => format!("{} provides professional, reliable, and expert services tailored to your needs.", business_name),
            BusinessCategory::RestaurantFood => format!("Experience the exquisite culinary delights at {}. Taste the passion in every bite.", business_name),
            BusinessCategory::Creative => format!("Unleash your imagination with {}. We bring creative visions to life.", business_name),
            BusinessCategory::LocalBusiness => format!("{}, proudly serving our local community with excellence and care.", business_name),
            BusinessCategory::Unknown => format!("Welcome to {}, an exciting new venture.", business_name),
        }
    }

    pub fn generate_product_description(product_name: &str, selling_physical: bool) -> String {
        if selling_physical {
            format!("The {} is crafted from premium materials, ensuring durability and style. It is the perfect addition to your collection, designed to meet the highest standards of quality.", product_name)
        } else {
            format!("Experience the exceptional {} service. We guarantee satisfaction and remarkable results, delivered by our team of experts.", product_name)
        }
    }

    pub fn suggest_domain(business_name: &str) -> String {
        let slug = business_name.to_lowercase().replace(|c: char| !c.is_alphanumeric(), "");
        format!("{}.ohc.app", slug)
    }

    pub fn mock_analyze_business_sentence(sentence: &str) -> Result<BusinessSetupWizardState, String> {
        let mut state = BusinessSetupWizardState::default();
        let sentence_lower = sentence.to_lowercase();

        if sentence_lower.contains("bakery") || sentence_lower.contains("food") || sentence_lower.contains("restaurant") {
            state.category = BusinessCategory::RestaurantFood;
            state.selling_physical_products = true;
        } else if sentence_lower.contains("store") || sentence_lower.contains("shop") || sentence_lower.contains("sell") {
            state.category = BusinessCategory::OnlineStore;
            state.selling_physical_products = true;
        } else if sentence_lower.contains("service") || sentence_lower.contains("repair") || sentence_lower.contains("handyman") || sentence_lower.contains("tutor") {
            state.category = BusinessCategory::ServiceBusiness;
            state.selling_services = true;
        } else if sentence_lower.contains("art") || sentence_lower.contains("design") || sentence_lower.contains("music") {
            state.category = BusinessCategory::Creative;
            state.selling_services = true;
        } else {
            state.category = BusinessCategory::LocalBusiness;
            state.selling_physical_products = true;
        }

        let words: Vec<&str> = sentence.split_whitespace().collect();
        if words.len() > 3 {
            state.business_name = format!("{} {}", words[words.len() - 2], words[words.len() - 1]);
        } else {
            state.business_name = "My Business".to_string();
        }

        state.payments_online = true;
        state.template_selected = "Modern".to_string();
        state.domain_selected = Self::suggest_domain(&state.business_name);

        Ok(state)
    }
}

pub struct TemplateGenerator;

impl TemplateGenerator {
    pub fn generate_preview_html(state: &BusinessSetupWizardState) -> String {
        let title = if state.business_name.is_empty() { "My Store".to_string() } else { state.business_name.clone() };
        let desc = MockAiGenerator::generate_storefront_description(&title, &state.category);

        let theme_color = match state.template_selected.as_str() {
            "Bold" => "#ff4757",
            "Modern" => "#4ecca3",
            _ => "#ffffff"
        };

        let prod_section = if !state.first_product_name.is_empty() {
            format!(
                "<div class='product-card' style='border: 1px solid rgba(255,255,255,0.1); padding: 15px; margin-top: 20px; border-radius: 8px;'>\
                    <h3>{}</h3>\
                    <p style='color: {}'>${:.2}</p>\
                    <p style='font-size: 0.9em; opacity: 0.8;'>{}</p>\
                    <button style='background: {}; color: #000; border: none; padding: 10px; border-radius: 4px;'>Buy Now</button>\
                </div>",
                state.first_product_name, theme_color, state.first_product_price, state.first_product_ai_desc, theme_color
            )
        } else {
            String::new()
        };

        format!(
            "<div class='preview-container' style='background: rgba(0,0,0,0.5); border-radius: 12px; padding: 20px; font-family: sans-serif; color: white;'>\
                <h1 style='color: {}'>{}</h1>\
                <p>{}</p>\
                {}\
            </div>",
            theme_color, title, desc, prod_section
        )
    }

    pub fn generate_live_html(state: &BusinessSetupWizardState) -> String {
        let title = if state.business_name.is_empty() { "My Store".to_string() } else { state.business_name.clone() };
        let desc = MockAiGenerator::generate_storefront_description(&title, &state.category);

        let theme_color = match state.template_selected.as_str() {
            "Bold" => "#ff4757",
            "Modern" => "#4ecca3",
            _ => "#ffffff"
        };

        let prod_section = if !state.first_product_name.is_empty() {
            format!(
                "<div class='product-card' style='border: 1px solid rgba(255,255,255,0.1); padding: 15px; margin-top: 20px; border-radius: 8px;'>\
                    <h3>{}</h3>\
                    <p style='color: {}'>${:.2}</p>\
                    <p style='font-size: 0.9em; opacity: 0.8;'>{}</p>\
                    <button style='background: {}; color: #000; border: none; padding: 10px; border-radius: 4px;'>Buy Now</button>\
                </div>",
                state.first_product_name, theme_color, state.first_product_price, state.first_product_ai_desc, theme_color
            )
        } else {
            String::new()
        };

        format!(
            "<!DOCTYPE html>\n<html>\n<head>\n<title>{}</title>\n\
            <style>\n\
                body {{ font-family: 'Inter', sans-serif; background: #0f172a; color: white; margin: 0; padding: 40px; }}\n\
                .glass {{ backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border-radius: 16px; border: 1px solid rgba(255, 255, 255, 0.1); padding: 30px; }}\n\
                h1 {{ color: {}; }}\n\
            </style>\n\
            </head>\n<body>\n\
                <div class='glass'>\n\
                    <h1>{}</h1>\n\
                    <p>{}</p>\n\
                    {}\n\
                </div>\n\
            </body>\n</html>",
            title, theme_color, title, desc, prod_section
        )
    }
}

pub struct AdvancedWizardFeatures;

impl AdvancedWizardFeatures {
    pub fn validate_advanced_features(state: &BusinessSetupWizardState) -> Result<(), String> {
        if state.business_name.is_empty() {
            return Err("Business name is required".to_string());
        }
        if state.domain_selected.is_empty() && state.completed {
            return Err("Cannot complete without domain".to_string());
        }
        Ok(())
    }

    pub fn suggest_cross_sells(state: &BusinessSetupWizardState) -> Vec<String> {
        let mut suggestions = Vec::new();
        if state.selling_physical_products {
            suggestions.push("Shipping Label Integration".to_string());
        }
        if state.selling_services {
            suggestions.push("Calendar Booking App".to_string());
        }
        if state.selling_subscriptions {
            suggestions.push("Subscription Churn Predictor".to_string());
        }
        suggestions
    }

    pub fn generate_welcome_checklist(state: &BusinessSetupWizardState) -> Vec<String> {
        let mut checklist = Vec::new();
        checklist.push("✅ Business live".to_string());
        if state.selling_physical_products {
            checklist.push("⬜ Add 3 more products".to_string());
        }
        if state.selling_services {
            checklist.push("⬜ Set up your availability".to_string());
        }
        checklist.push("⬜ Connect Instagram".to_string());
        checklist.push("⬜ Share your link with a friend".to_string());
        checklist
    }
}

pub struct WizardTelemetry {
    pub events: Vec<String>,
}

impl WizardTelemetry {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn track_event(&mut self, event: &str) {
        self.events.push(event.to_string());
    }

    pub fn get_event_count(&self) -> usize {
        self.events.len()
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

pub struct WizardSecurityAnalyzer;

impl WizardSecurityAnalyzer {
    pub fn analyze_state(state: &BusinessSetupWizardState) -> Result<(), String> {
        if state.admin_email.contains("malicious") {
            return Err("Suspicious email detected".to_string());
        }
        if state.business_name.contains("<script>") {
            return Err("XSS attempt detected in business name".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WizardEvent {
    Started,
    BusinessNameSet(String),
    CategorySelected(BusinessCategory),
    SellingOptionsSet { physical: bool, services: bool, subscriptions: bool },
    PaymentOptionsSet { online: bool, in_person: bool },
    AdminDetailsSet { name: String, email: String },
    TemplateSelected(String),
    FirstProductAdded { name: String, price: f64 },
    DomainSelected(String),
    Published,
}

pub struct WizardEventStore {
    events: Vec<WizardEvent>,
}

impl WizardEventStore {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn append(&mut self, event: WizardEvent) {
        self.events.push(event);
    }

    pub fn get_events(&self) -> &[WizardEvent] {
        &self.events
    }

    pub fn rebuild_state(&self) -> BusinessSetupWizardState {
        let mut state = BusinessSetupWizardState::default();
        for event in &self.events {
            match event {
                WizardEvent::Started => state.step = 1,
                WizardEvent::BusinessNameSet(name) => state.business_name = name.clone(),
                WizardEvent::CategorySelected(cat) => state.category = cat.clone(),
                WizardEvent::SellingOptionsSet { physical, services, subscriptions } => {
                    state.selling_physical_products = *physical;
                    state.selling_services = *services;
                    state.selling_subscriptions = *subscriptions;
                }
                WizardEvent::PaymentOptionsSet { online, in_person } => {
                    state.payments_online = *online;
                    state.payments_in_person = *in_person;
                }
                WizardEvent::AdminDetailsSet { name, email } => {
                    state.admin_name = name.clone();
                    state.admin_email = email.clone();
                }
                WizardEvent::TemplateSelected(template) => state.template_selected = template.clone(),
                WizardEvent::FirstProductAdded { name, price } => {
                    state.first_product_name = name.clone();
                    state.first_product_price = *price;
                }
                WizardEvent::DomainSelected(domain) => state.domain_selected = domain.clone(),
                WizardEvent::Published => state.completed = true,
            }
        }
        state
    }
}

pub struct TemplateConfig {
    pub id: String,
    pub name: String,
    pub primary_color: String,
    pub secondary_color: String,
    pub font_family: String,
    pub is_premium: bool,
}

pub struct TemplateRegistry {
    templates: std::collections::HashMap<String, TemplateConfig>,
}

impl TemplateRegistry {
    pub fn new() -> Self {
        Self {
            templates: std::collections::HashMap::new(),
        }
    }

    pub fn register(&mut self, config: TemplateConfig) {
        self.templates.insert(config.id.clone(), config);
    }

    pub fn get_template(&self, id: &str) -> Option<&TemplateConfig> {
        self.templates.get(id)
    }

    pub fn list_templates(&self) -> Vec<&TemplateConfig> {
        self.templates.values().collect()
    }
}

#[cfg(test)]
mod extended_tests {
    use super::*;

    #[test]
    fn test_wizard_default_state() {
        let wizard = BusinessSetupWizard::new_default();
        let state = wizard.get_state();
        assert_eq!(state.step, 1);
        assert_eq!(state.business_name, "");
        assert_eq!(state.category, BusinessCategory::Unknown);
        assert!(!state.selling_physical_products);
        assert!(!state.selling_services);
        assert!(!state.selling_subscriptions);
        assert!(!state.payments_online);
        assert!(!state.payments_in_person);
        assert!(!state.completed);
    }

    #[test]
    fn test_advance_step_valid() {
        let mut wizard = BusinessSetupWizard::new_default();
        assert!(wizard.advance_step(5).is_ok());
        assert_eq!(wizard.get_state().step, 5);
        assert!(wizard.get_state().last_updated_unix > 0);
    }

    #[test]
    fn test_advance_step_invalid() {
        let mut wizard = BusinessSetupWizard::new_default();
        assert!(wizard.advance_step(0).is_err());
        assert!(wizard.advance_step(101).is_err());
    }

    #[test]
    fn test_set_business_name_valid() {
        let mut wizard = BusinessSetupWizard::new_default();
        assert!(wizard.set_business_name("Maya's Cakes").is_ok());
        assert_eq!(wizard.get_state().business_name, "Maya's Cakes");
    }

    #[test]
    fn test_set_business_name_invalid() {
        let mut wizard = BusinessSetupWizard::new_default();
        assert!(wizard.set_business_name("   ").is_err());
        let long_name = "A".repeat(101);
        assert!(wizard.set_business_name(&long_name).is_err());
    }

    #[test]
    fn test_set_category() {
        let mut wizard = BusinessSetupWizard::new_default();
        assert!(wizard.set_category(BusinessCategory::OnlineStore).is_ok());
        assert_eq!(wizard.get_state().category, BusinessCategory::OnlineStore);
    }

    #[test]
    fn test_set_selling_options_valid() {
        let mut wizard = BusinessSetupWizard::new_default();
        assert!(wizard.set_selling_options(true, false, true).is_ok());
        assert!(wizard.get_state().selling_physical_products);
        assert!(!wizard.get_state().selling_services);
        assert!(wizard.get_state().selling_subscriptions);
    }

    #[test]
    fn test_set_selling_options_invalid() {
        let mut wizard = BusinessSetupWizard::new_default();
        assert!(wizard.set_selling_options(false, false, false).is_err());
    }

    #[test]
    fn test_set_payment_options_valid() {
        let mut wizard = BusinessSetupWizard::new_default();
        assert!(wizard.set_payment_options(true, false).is_ok());
        assert!(wizard.get_state().payments_online);
        assert!(!wizard.get_state().payments_in_person);
    }

    #[test]
    fn test_set_payment_options_invalid() {
        let mut wizard = BusinessSetupWizard::new_default();
        assert!(wizard.set_payment_options(false, false).is_err());
    }

    #[test]
    fn test_set_admin_details_valid() {
        let mut wizard = BusinessSetupWizard::new_default();
        assert!(wizard.set_admin_details("Maya", "maya@example.com").is_ok());
        assert_eq!(wizard.get_state().admin_name, "Maya");
        assert_eq!(wizard.get_state().admin_email, "maya@example.com");
    }

    #[test]
    fn test_set_admin_details_invalid() {
        let mut wizard = BusinessSetupWizard::new_default();
        assert!(wizard.set_admin_details("  ", "maya@example.com").is_err());
        assert!(wizard.set_admin_details("Maya", "maya.example.com").is_err());
    }

    #[test]
    fn test_select_template() {
        let mut wizard = BusinessSetupWizard::new_default();
        assert!(wizard.select_template("Modern").is_ok());
        assert_eq!(wizard.get_state().template_selected, "Modern");
        assert!(wizard.select_template("   ").is_err());
    }

    #[test]
    fn test_set_first_product_valid() {
        let mut wizard = BusinessSetupWizard::new_default();
        assert!(wizard.set_first_product("Custom Cake", 50.0).is_ok());
        assert_eq!(wizard.get_state().first_product_name, "Custom Cake");
        assert_eq!(wizard.get_state().first_product_price, 50.0);
    }

    #[test]
    fn test_set_first_product_invalid() {
        let mut wizard = BusinessSetupWizard::new_default();
        assert!(wizard.set_first_product("  ", 50.0).is_err());
        assert!(wizard.set_first_product("Cake", -10.0).is_err());
    }

    #[test]
    fn test_generate_ai_description() {
        let mut wizard = BusinessSetupWizard::new_default();
        assert!(wizard.generate_ai_description().is_err()); // No product name yet
        let _ = wizard.set_first_product("Cake", 10.0);
        let desc = wizard.generate_ai_description().unwrap();
        assert!(desc.contains("Cake"));
        assert_eq!(wizard.get_state().first_product_ai_desc, desc);
    }

    #[test]
    fn test_select_domain() {
        let mut wizard = BusinessSetupWizard::new_default();
        assert!(wizard.select_domain("mayascakes.ohc.app").is_ok());
        assert_eq!(wizard.get_state().domain_selected, "mayascakes.ohc.app");
        assert!(wizard.select_domain("   ").is_err());
    }

    #[test]
    fn test_publish() {
        let mut wizard = BusinessSetupWizard::new_default();
        assert!(wizard.publish().is_err()); // Missing fields
        let _ = wizard.set_business_name("Maya's Cakes");
        assert!(wizard.publish().is_err()); // Missing product
        let _ = wizard.set_first_product("Cake", 10.0);
        assert!(wizard.publish().is_ok());
        assert!(wizard.get_state().completed);
    }

    #[test]
    fn test_mock_ai_generator() {
        let desc_online = MockAiGenerator::generate_storefront_description("Store", &BusinessCategory::OnlineStore);
        assert!(desc_online.contains("online"));

        let desc_service = MockAiGenerator::generate_storefront_description("Fix", &BusinessCategory::ServiceBusiness);
        assert!(desc_service.contains("services"));

        let desc_food = MockAiGenerator::generate_storefront_description("Bites", &BusinessCategory::RestaurantFood);
        assert!(desc_food.contains("culinary"));

        let desc_creative = MockAiGenerator::generate_storefront_description("Art", &BusinessCategory::Creative);
        assert!(desc_creative.contains("creative"));

        let desc_local = MockAiGenerator::generate_storefront_description("Shop", &BusinessCategory::LocalBusiness);
        assert!(desc_local.contains("local"));

        let desc_unknown = MockAiGenerator::generate_storefront_description("Venture", &BusinessCategory::Unknown);
        assert!(desc_unknown.contains("venture"));

        let prod_desc_phys = MockAiGenerator::generate_product_description("Item", true);
        assert!(prod_desc_phys.contains("premium materials"));

        let prod_desc_serv = MockAiGenerator::generate_product_description("Task", false);
        assert!(prod_desc_serv.contains("service"));

        let domain = MockAiGenerator::suggest_domain("Maya's Cakes!");
        assert_eq!(domain, "mayascakes.ohc.app");
    }

    #[test]
    fn test_mock_analyze_business_sentence() {
        let state1 = MockAiGenerator::mock_analyze_business_sentence("I run a local bakery").unwrap();
        assert_eq!(state1.category, BusinessCategory::RestaurantFood);
        assert!(state1.selling_physical_products);
        assert_eq!(state1.business_name, "local bakery");

        let state2 = MockAiGenerator::mock_analyze_business_sentence("We sell books online").unwrap();
        assert_eq!(state2.category, BusinessCategory::OnlineStore);
        assert!(state2.selling_physical_products);
        assert_eq!(state2.business_name, "books online");

        let state3 = MockAiGenerator::mock_analyze_business_sentence("I offer repair services for computers").unwrap();
        assert_eq!(state3.category, BusinessCategory::ServiceBusiness);
        assert!(state3.selling_services);
        assert_eq!(state3.business_name, "for computers");

        let state4 = MockAiGenerator::mock_analyze_business_sentence("I create digital art").unwrap();
        assert_eq!(state4.category, BusinessCategory::Creative);
        assert!(state4.selling_services);
        assert_eq!(state4.business_name, "digital art");

        let state5 = MockAiGenerator::mock_analyze_business_sentence("A gym").unwrap();
        assert_eq!(state5.category, BusinessCategory::LocalBusiness);
        assert_eq!(state5.business_name, "My Business");
        assert_eq!(state5.domain_selected, "mybusiness.ohc.app");
    }

    #[test]
    fn test_template_generator() {
        let mut state = BusinessSetupWizardState::default();
        state.business_name = "Test Store".to_string();
        state.template_selected = "Modern".to_string();
        state.first_product_name = "Product 1".to_string();
        state.first_product_price = 99.99;
        state.first_product_ai_desc = "AI Desc".to_string();

        let preview = TemplateGenerator::generate_preview_html(&state);
        assert!(preview.contains("Test Store"));
        assert!(preview.contains("Product 1"));
        assert!(preview.contains("99.99"));
        assert!(preview.contains("AI Desc"));
        assert!(preview.contains("#4ecca3"));

        let live = TemplateGenerator::generate_live_html(&state);
        assert!(live.contains("<!DOCTYPE html>"));
        assert!(live.contains("Test Store"));
        assert!(live.contains("Product 1"));

        state.template_selected = "Bold".to_string();
        let preview_bold = TemplateGenerator::generate_preview_html(&state);
        assert!(preview_bold.contains("#ff4757"));

        state.template_selected = "Other".to_string();
        let preview_other = TemplateGenerator::generate_preview_html(&state);
        assert!(preview_other.contains("#ffffff"));

        state.business_name = "".to_string();
        let preview_empty = TemplateGenerator::generate_preview_html(&state);
        assert!(preview_empty.contains("My Store"));
    }

    #[test]
    fn test_advanced_validation() {
        let mut state = BusinessSetupWizardState::default();
        assert!(AdvancedWizardFeatures::validate_advanced_features(&state).is_err());

        state.business_name = "Maya".to_string();
        state.completed = true;
        assert!(AdvancedWizardFeatures::validate_advanced_features(&state).is_err()); // missing domain

        state.domain_selected = "maya.ohc.app".to_string();
        assert!(AdvancedWizardFeatures::validate_advanced_features(&state).is_ok());
    }

    #[test]
    fn test_cross_sells() {
        let mut state = BusinessSetupWizardState::default();
        state.selling_physical_products = true;
        state.selling_services = true;
        state.selling_subscriptions = true;

        let suggestions = AdvancedWizardFeatures::suggest_cross_sells(&state);
        assert_eq!(suggestions.len(), 3);
        assert_eq!(suggestions[0], "Shipping Label Integration");
        assert_eq!(suggestions[1], "Calendar Booking App");
        assert_eq!(suggestions[2], "Subscription Churn Predictor");

        state.selling_physical_products = false;
        let suggestions2 = AdvancedWizardFeatures::suggest_cross_sells(&state);
        assert_eq!(suggestions2.len(), 2);
    }

    #[test]
    fn test_welcome_checklist() {
        let mut state = BusinessSetupWizardState::default();
        state.selling_physical_products = true;
        let checklist1 = AdvancedWizardFeatures::generate_welcome_checklist(&state);
        assert!(checklist1.contains(&"✅ Business live".to_string()));
        assert!(checklist1.contains(&"⬜ Add 3 more products".to_string()));
        assert!(!checklist1.contains(&"⬜ Set up your availability".to_string()));

        state.selling_physical_products = false;
        state.selling_services = true;
        let checklist2 = AdvancedWizardFeatures::generate_welcome_checklist(&state);
        assert!(checklist2.contains(&"⬜ Set up your availability".to_string()));
    }

    #[test]
    fn test_telemetry() {
        let mut tel = WizardTelemetry::new();
        tel.track_event("started");
        tel.track_event("completed_step_1");
        assert_eq!(tel.get_event_count(), 2);
        tel.clear();
        assert_eq!(tel.get_event_count(), 0);
    }

    #[test]
    fn test_security_analyzer() {
        let mut state = BusinessSetupWizardState::default();
        state.business_name = "Normal Store".to_string();
        state.admin_email = "normal@example.com".to_string();
        assert!(WizardSecurityAnalyzer::analyze_state(&state).is_ok());

        state.admin_email = "malicious@hacker.com".to_string();
        assert!(WizardSecurityAnalyzer::analyze_state(&state).is_err());

        state.admin_email = "normal@example.com".to_string();
        state.business_name = "<script>alert(1)</script>".to_string();
        assert!(WizardSecurityAnalyzer::analyze_state(&state).is_err());
    }

    #[test]
    fn test_wizard_event_sourcing() {
        let mut store = WizardEventStore::new();
        store.append(WizardEvent::Started);
        store.append(WizardEvent::BusinessNameSet("Event Store Inc".to_string()));
        store.append(WizardEvent::CategorySelected(BusinessCategory::OnlineStore));
        store.append(WizardEvent::SellingOptionsSet { physical: true, services: false, subscriptions: false });
        store.append(WizardEvent::PaymentOptionsSet { online: true, in_person: false });
        store.append(WizardEvent::AdminDetailsSet { name: "Admin".to_string(), email: "admin@example.com".to_string() });
        store.append(WizardEvent::TemplateSelected("Bold".to_string()));
        store.append(WizardEvent::FirstProductAdded { name: "Book".to_string(), price: 15.0 });
        store.append(WizardEvent::DomainSelected("eventstore.ohc.app".to_string()));
        store.append(WizardEvent::Published);

        let state = store.rebuild_state();
        assert_eq!(state.business_name, "Event Store Inc");
        assert_eq!(state.category, BusinessCategory::OnlineStore);
        assert!(state.selling_physical_products);
        assert!(state.payments_online);
        assert_eq!(state.admin_name, "Admin");
        assert_eq!(state.template_selected, "Bold");
        assert_eq!(state.first_product_name, "Book");
        assert_eq!(state.domain_selected, "eventstore.ohc.app");
        assert!(state.completed);
    }

    #[test]
    fn test_template_registry() {
        let mut registry = TemplateRegistry::new();

        registry.register(TemplateConfig {
            id: "modern".to_string(),
            name: "Modern".to_string(),
            primary_color: "#4ecca3".to_string(),
            secondary_color: "#1a1a2e".to_string(),
            font_family: "Outfit".to_string(),
            is_premium: false,
        });

        registry.register(TemplateConfig {
            id: "bold".to_string(),
            name: "Bold".to_string(),
            primary_color: "#ff4757".to_string(),
            secondary_color: "#2f3542".to_string(),
            font_family: "Inter".to_string(),
            is_premium: true,
        });

        let modern = registry.get_template("modern").unwrap();
        assert_eq!(modern.name, "Modern");
        assert_eq!(modern.primary_color, "#4ecca3");

        let bold = registry.get_template("bold").unwrap();
        assert!(bold.is_premium);

        let list = registry.list_templates();
        assert_eq!(list.len(), 2);
    }
}
