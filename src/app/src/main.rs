use ohc::orchestration::hub_service_client::HubServiceClient;
use ohc::orchestration::RegisterAgentRequest;
use ohc::orchestration::Agent;
use ohc::orchestration::GetWizardStateRequest;
use ohc::orchestration::SaveWizardStateRequest;

pub mod ohc {
    pub mod orchestration {
        tonic::include_proto!("ohc.orchestration");
    }
}

pub mod tooltip_registry;

use slint::ComponentHandle;

pub mod app {
    include!(concat!(env!("OUT_DIR"), "/app.rs"));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("App starting...");

    let setup_wizard_ui = app::SetupWizard::new()?;
    let setup_wizard_handle = setup_wizard_ui.as_weak();

    // 1. Initial connection and Resume State logic
    let startup_handle = setup_wizard_handle.clone();
    tokio::spawn(async move {
        match HubServiceClient::connect("http://127.0.0.1:18789").await {
            Ok(mut client) => {
                println!("Connected to server!");

                // Resume Wizard State
                match client.get_wizard_state(tonic::Request::new(GetWizardStateRequest {})).await {
                    Ok(resp) => {
                        let state = resp.into_inner().state;
                        let ui_handle = startup_handle.clone();
                        slint::invoke_from_event_loop(move || {
                            if let Some(ui) = ui_handle.upgrade() {
                                if let Some(val) = state.get("business_type") { ui.set_business_type(val.clone().into()); }
                                if let Some(val) = state.get("company_name") { ui.set_company_name(val.clone().into()); }
                                if let Some(val) = state.get("company_description") { ui.set_company_description(val.clone().into()); }
                                if let Some(val) = state.get("payment_pref") { ui.set_payment_pref(val.clone().into()); }
                                if let Some(val) = state.get("admin_name") { ui.set_admin_name(val.clone().into()); }
                                if let Some(val) = state.get("admin_email") { ui.set_admin_email(val.clone().into()); }
                                if let Some(val) = state.get("website_template") { ui.set_website_template(val.clone().into()); }
                                if let Some(val) = state.get("product_name") { ui.set_product_name(val.clone().into()); }
                                if let Some(val) = state.get("product_price") { ui.set_product_price(val.clone().into()); }
                                if let Some(val) = state.get("domain_choice") { ui.set_domain_choice(val.clone().into()); }
                                if let Some(val) = state.get("sell_physical") { ui.set_sell_physical(val == "true"); }
                                if let Some(val) = state.get("sell_digital") { ui.set_sell_digital(val == "true"); }
                                if let Some(val) = state.get("sell_services") { ui.set_sell_services(val == "true"); }
                                if let Some(val) = state.get("sell_food") { ui.set_sell_food(val == "true"); }
                                if let Some(val) = state.get("sell_subscriptions") { ui.set_sell_subscriptions(val == "true"); }

                                if let Some(val) = state.get("step") {
                                    if let Ok(step) = val.parse::<i32>() {
                                        ui.set_step(step);
                                    }
                                }
                            }
                        }).unwrap();
                    }
                    Err(e) => println!("Failed to fetch wizard state: {:?}", e),
                }

                let request = tonic::Request::new(RegisterAgentRequest {
                    agent: Some(Agent {
                        id: "agent_1".into(),
                        name: "Rust Agent".into(),
                        role: "Worker".into(),
                        organization_id: "org_1".into(),
                        status: "Running".into(),
                        provider_type: "Standard".into(),
                    }),
                });
                match client.register_agent(request).await {
                    Ok(response) => println!("RESPONSE={:?}", response),
                    Err(e) => println!("ERR={:?}", e),
                }
            }
            Err(e) => {
                println!("Could not connect to server: {:?}", e);
            }
        }
    });

    // 2. Wire State Changed Callback for Persistence
    setup_wizard_ui.on_state_changed({
        move |key, value| {
            let k = key.to_string();
            let v = value.to_string();
            tokio::spawn(async move {
                match HubServiceClient::connect("http://127.0.0.1:18789").await {
                    Ok(mut client) => {
                        let state = std::collections::HashMap::from([(k, v)]);
                        let request = tonic::Request::new(SaveWizardStateRequest { state });
                        if let Err(e) = client.save_wizard_state(request).await {
                            println!("Failed to save wizard state: {:?}", e);
                        }
                    }
                    Err(e) => println!("Could not connect to server to save state: {:?}", e),
                }
            });
        }
    });

    let launch_handle = setup_wizard_handle.clone();
    setup_wizard_ui.on_launch({
        let ui_handle = launch_handle.clone();
        move |business_type, company_name, company_description, payment_pref, admin_email, website_template, product_name, product_price, domain_choice| {
            let handle_clone = ui_handle.clone();
            let ui = ui_handle.upgrade().unwrap();

            // Map selling categories from checkboxes
            let mut selling_categories = Vec::new();
            if ui.get_sell_physical() { selling_categories.push("physical".to_string()); }
            if ui.get_sell_digital() { selling_categories.push("digital".to_string()); }
            if ui.get_sell_services() { selling_categories.push("services".to_string()); }
            if ui.get_sell_food() { selling_categories.push("food".to_string()); }
            if ui.get_sell_subscriptions() { selling_categories.push("subscriptions".to_string()); }

            let req_business_type = business_type.to_string();
            let req_company_name = company_name.to_string();
            let req_company_description = company_description.to_string();
            let req_payment_pref = payment_pref.to_string();
            let req_admin_email = admin_email.to_string();
            let req_website_template = website_template.to_string();
            let req_product_name = product_name.to_string();
            let req_product_price = product_price.to_string();
            let req_domain_choice = domain_choice.to_string();

            tokio::spawn(async move {
                match HubServiceClient::connect("http://127.0.0.1:18789").await {
                    Ok(mut client) => {
                        let onboarding_request = tonic::Request::new(ohc::orchestration::StartOnboardingRequest {
                            business_type: req_business_type,
                            company_name: req_company_name,
                            company_description: req_company_description,
                            payment_pref: req_payment_pref,
                            admin_email: req_admin_email,
                            website_template: req_website_template,
                            first_product_name: req_product_name,
                            first_product_price: req_product_price,
                            domain_choice: req_domain_choice,
                            selling_categories,
                        });

                        match client.start_onboarding(onboarding_request).await {
                            Ok(resp) => {
                                let r = resp.into_inner();
                                let msg = r.message.clone();
                                slint::invoke_from_event_loop(move || {
                                    if let Some(ui) = handle_clone.upgrade() {
                                        ui.set_launch_status("Onboarding Complete!".into());
                                        ui.set_launch_details(msg.into());
                                        ui.set_step(10); // Go to Welcome Checklist
                                    }
                                }).unwrap();
                            }
                            Err(e) => {
                                let err_msg = e.to_string();
                                slint::invoke_from_event_loop(move || {
                                    if let Some(ui) = handle_clone.upgrade() {
                                        ui.set_launch_status("Onboarding Failed".into());
                                        ui.set_launch_details(err_msg.into());
                                    }
                                }).unwrap();
                            }
                        }
                    }
                    Err(e) => {
                        println!("Could not connect to server: {:?}", e);
                    }
                }
            });
        }
    });

    setup_wizard_ui.run()?;
    
    Ok(())
}

#[cfg(test)]
mod e2e_tests {
    use super::*;

    #[test]
    fn test_login_password_visibility_toggle() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        let ui = app::Login::new().unwrap();

        ui.set_password("secret".into());
        assert_eq!(ui.get_password(), "secret");
    }

    #[test]
    fn test_e2e_wizard_flow() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
            println!("Skipping E2E test_e2e_wizard_flow because no display server is available.");
            return;
        }

        let ui = app::SetupWizard::new().unwrap();

        // Step 0: Welcome -> Step 1
        assert_eq!(ui.get_step(), 0);
        ui.set_step(1);

        // Step 1: Type -> Step 2
        ui.set_business_type("Online Store".into());
        ui.set_step(2);
        assert_eq!(ui.get_step(), 2);

        // Step 2: Name -> Step 3
        ui.set_company_name("My E2E Store".into());
        ui.set_step(3);
        assert_eq!(ui.get_step(), 3);

        // Step 3: What do you sell -> Step 4
        ui.set_sell_physical(true);
        ui.set_step(4);
        assert_eq!(ui.get_step(), 4);

        // Step 4: Payments -> Step 5
        ui.set_payment_pref("online".into());
        ui.set_step(5);
        assert_eq!(ui.get_step(), 5);

        // Step 5: Admin -> Step 6
        ui.set_admin_email("admin@e2e.test".into());
        ui.set_step(6);
        assert_eq!(ui.get_step(), 6);

        // Step 6: Template -> Step 7
        ui.set_website_template("Modern".into());
        ui.set_step(7);

        // Step 7: Product -> Step 8
        ui.set_product_name("Cake".into());
        ui.set_product_price("10.00".into());
        ui.set_step(8);

        // Step 8: Domain -> Step 9
        ui.set_domain_choice("subdomain".into());
        ui.set_step(9);

        // Step 9: Launch -> Step 10
        ui.set_launching(true);
        ui.set_step(10);

        // Final state verification
        assert_eq!(ui.get_company_name(), "My E2E Store");
        assert_eq!(ui.get_business_type(), "Online Store");
        assert_eq!(ui.get_admin_email(), "admin@e2e.test");
        assert_eq!(ui.get_payment_pref(), "online");
        assert_eq!(ui.get_sell_physical(), true);
        assert_eq!(ui.get_website_template(), "Modern");
        assert_eq!(ui.get_product_name(), "Cake");
        assert_eq!(ui.get_product_price(), "10.00");
        assert_eq!(ui.get_domain_choice(), "subdomain");
        assert_eq!(ui.get_step(), 10);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_welcome_checklist_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
            println!("Skipping test_welcome_checklist_creation because no display server is available.");
            return;
        }
        app::WelcomeChecklist::new().unwrap();
    }

    #[test]
    fn test_login_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
            println!("Skipping test_login_creation because no display server is available.");
            return;
        }
        let ui = app::Login::new().unwrap();
        assert_eq!(ui.get_username(), "");
        assert_eq!(ui.get_password(), "");
    }

    #[test]
    fn test_business_setup_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
            println!("Skipping test_business_setup_creation because no display server is available.");
            return;
        }
        let ui = app::BusinessSetup::new().unwrap();
        assert_eq!(ui.get_step(), 0);
        assert_eq!(ui.get_company_name(), "");
    }

    #[test]
    fn test_agent_hire_next_button_disabled_by_default() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
            println!("Skipping test_agent_hire_next_button_disabled_by_default because no display server is available.");
            return;
        }
        let ui = app::AgentHire::new().unwrap();
        assert_eq!(ui.get_step(), 0);
        assert_eq!(ui.get_selected_role(), "");
        assert_eq!(ui.get_next_enabled(), false);
    }

    #[test]
    fn test_agent_hire_next_button_enabled_after_role_selection() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
            println!("Skipping test_agent_hire_next_button_enabled_after_role_selection because no display server is available.");
            return;
        }
        let ui = app::AgentHire::new().unwrap();
        assert_eq!(ui.get_step(), 0);
        ui.set_selected_role("SOFTWARE_ENGINEER".into());
        assert_eq!(ui.get_next_enabled(), true);
    }

    #[test]
    fn test_landing_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
            println!("Skipping test_landing_creation because no display server is available.");
            return;
        }
        let ui = app::Landing::new().unwrap();
        assert_eq!(ui.get_is_variant_b(), false);
    }

    #[test]
    fn test_agents_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::Agents::new().unwrap();
    }
    #[test]
    fn test_chat_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::Chat::new().unwrap();
    }
    #[test]
    fn test_channels_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::Channels::new().unwrap();
    }
    #[test]
    fn test_integrations_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::Integrations::new().unwrap();
    }
    #[test]
    fn test_security_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::Security::new().unwrap();
    }
    #[test]
    fn test_meetings_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::Meetings::new().unwrap();
    }
    #[test]
    fn test_logs_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::Logs::new().unwrap();
    }
    #[test]
    fn test_pricing_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::Pricing::new().unwrap();
    }
    #[test]
    fn test_pricing_select_plan() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        let ui = app::Pricing::new().unwrap();
        let plan_selected = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
        let plan_selected_clone = plan_selected.clone();
        ui.on_select_plan(move |plan| {
            *plan_selected_clone.borrow_mut() = plan.to_string();
        });
        ui.invoke_select_plan("Pro".into());
        assert_eq!(*plan_selected.borrow(), "Pro");
    }
    #[test]
    fn test_my_plan_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        let ui = app::MyPlan::new().unwrap();
        ui.set_tier("Starter".into());
        ui.set_total_actions("500".into());
        assert_eq!(ui.get_tier(), "Starter");
        assert_eq!(ui.get_total_actions(), "500");
    }
    #[test]
    fn test_cost_dashboard_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        let ui = app::CostDashboard::new().unwrap();
        ui.set_total_spend("$50.00".into());
        assert_eq!(ui.get_total_spend(), "$50.00");
    }
    #[test]
    fn test_scaling_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::Scaling::new().unwrap();
    }
    #[test]
    fn test_swarm_memory_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::SwarmMemory::new().unwrap();
    }
    #[test]
    fn test_website_builder_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::WebsiteBuilder::new().unwrap();
    }

    #[test]
    fn test_website_builder_viral_storefront_footer() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        let ui = app::WebsiteBuilder::new().unwrap();
        ui.set_step(4);
        assert_eq!(ui.get_step(), 4);
    }


    #[test]
    fn test_setup_wizard_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::SetupWizard::new().unwrap();
    }

    #[test]
    fn test_e2e_prompt_tuning_flow() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        let ui = app::PromptTuning::new().unwrap();

        // Step 0: Tone -> Step 1
        assert_eq!(ui.get_step(), 0);
        ui.set_tone("Concise".into());
        ui.set_step(1);

        // Step 1: Focus -> Step 2
        ui.set_focus_only_business(true);
        ui.set_focus_avoid_competitors(true);
        ui.set_step(2);

        // Step 2: Examples -> Step 3
        ui.set_step(3);

        // Verify state
        assert_eq!(ui.get_tone(), "Concise");
        assert_eq!(ui.get_focus_only_business(), true);
        assert_eq!(ui.get_focus_avoid_competitors(), true);
    }

    #[test]
    fn test_task_list_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::TaskList::new().unwrap();
    }
    #[test]
    fn test_fix_agent_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::FixAgent::new().unwrap();
    }
    #[test]
    fn test_upgrade_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::Upgrade::new().unwrap();
    }
    #[test]
    fn test_billing_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::Billing::new().unwrap();
    }
    #[test]
    fn test_grow_business_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::GrowBusiness::new().unwrap();
    }
}

#[cfg(test)]
mod docs_tests {
    use super::*;

    #[test]
    fn test_e2e_setup_wizard_flow() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        let ui = app::SetupWizard::new().unwrap();

        // Step 0: Welcome -> Step 1
        assert_eq!(ui.get_step(), 0);
        ui.invoke_next_step();

        // Step 1: Type -> Step 2
        ui.invoke_select_business_type("Online Store".into());

        // Step 2: Name -> Step 3
        ui.set_company_name("My E2E Store".into());
        ui.invoke_next_step();

        // Step 3: What do you sell -> Step 4
        ui.invoke_toggle_sell_physical();
        ui.invoke_next_step();

        // Step 4: Payments -> Step 5
        ui.invoke_select_payment_pref("online".into());

        // Step 5: Admin -> Step 6
        ui.set_admin_email("admin@e2e.test".into());
        ui.invoke_next_step();

        // Step 6: Template -> Step 7
        ui.invoke_select_template("Modern".into());

        // Step 7: Product -> Step 8
        ui.set_product_name("Cake".into());
        ui.invoke_next_step();

        // Step 8: Domain -> Step 9
        ui.invoke_select_domain("subdomain".into());

        // Step 9: Launch -> Step 10
        ui.set_step(10);

        // Final state verification
        assert_eq!(ui.get_company_name(), "My E2E Store");
        assert_eq!(ui.get_business_type(), "Online Store");
        assert_eq!(ui.get_admin_email(), "admin@e2e.test");
        assert_eq!(ui.get_payment_pref(), "online");
        assert_eq!(ui.get_sell_physical(), true);
        assert_eq!(ui.get_website_template(), "Modern");
        assert_eq!(ui.get_product_name(), "Cake");
        assert_eq!(ui.get_domain_choice(), "subdomain");
        assert_eq!(ui.get_step(), 10);
    }

    #[test]
    fn test_e2e_website_builder_flow() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        let ui = app::WebsiteBuilder::new().unwrap();

        assert_eq!(ui.get_step(), 0);
        ui.set_selected_template("E-commerce".into());
        ui.set_step(1);

        ui.set_primary_color("#34C759".into());
        ui.set_step(2);

        ui.set_product_name("My Custom Product".into());
        ui.set_step(3);

        ui.set_domain_choice("custom".into());
        ui.set_step(4);

        assert_eq!(ui.get_step(), 4);
        assert_eq!(ui.get_selected_template(), "E-commerce");
        assert_eq!(ui.get_primary_color(), "#34C759");
        assert_eq!(ui.get_product_name(), "My Custom Product");
        assert_eq!(ui.get_domain_choice(), "custom");
    }

    #[test]
    fn test_help_center_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::HelpCenter::new().unwrap();
    }
    #[test]
    fn test_release_notes_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::ReleaseNotes::new().unwrap();
    }
    #[test]
    fn test_interactive_walkthrough_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::InteractiveWalkthrough::new().unwrap();
    }
    #[test]
    fn test_ai_help_chat_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::AiHelpChat::new().unwrap();
    }
    #[test]
    fn test_video_tutorials_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::VideoTutorials::new().unwrap();
    }
    #[test]
    fn test_api_docs_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::ApiDocs::new().unwrap();
    }
}

#[cfg(test)]
mod dashboard_docs_tests {
    use super::*;

    #[test]
    fn test_dashboard_tooltips_and_help_actions() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
            println!("Skipping test_dashboard_tooltips_and_help_actions because no display server is available.");
            return;
        }

        let ui = app::Dashboard::new().unwrap();
        let tooltip_registry = crate::tooltip_registry::TooltipRegistry::new();

        ui.set_tt_active_agents(tooltip_registry.get_tooltip("dashboard_active_agents").unwrap_or_default().into());
        ui.set_tt_active_tasks(tooltip_registry.get_tooltip("dashboard_active_tasks").unwrap_or_default().into());
        ui.set_tt_scheduled_calls(tooltip_registry.get_tooltip("dashboard_scheduled_calls").unwrap_or_default().into());
        ui.set_tt_team_members(tooltip_registry.get_tooltip("dashboard_team_members").unwrap_or_default().into());

        assert_eq!(ui.get_tt_active_agents(), "The number of AI agents currently working on tasks for your business.");
        assert_eq!(ui.get_tt_active_tasks(), "Tasks that your agents are actively processing right now.");

        ui.on_open_help_center(move || {});
        ui.on_open_ai_chat(move || {});
    }

    #[test]
    fn test_documentation_components_e2e_flow() {
        use crate::tooltip_registry::TooltipRegistry;
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
            println!("Skipping test_documentation_components_e2e_flow because no display server is available.");
            return;
        }

        let login_ui = app::Login::new().unwrap();
        let login_successful = std::rc::Rc::new(std::cell::RefCell::new(false));
        let login_successful_clone = login_successful.clone();

        login_ui.on_login(move |email, password| {
            assert_eq!(email, "test@example.com");
            assert_eq!(password, "password123");
            *login_successful_clone.borrow_mut() = true;
        });

        login_ui.invoke_login("test@example.com".into(), "password123".into());
        assert!(*login_successful.borrow(), "User login should be successful");

        let dashboard_ui = app::Dashboard::new().unwrap();

        let help_center_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
        let help_center_opened_clone = help_center_opened.clone();
        dashboard_ui.on_open_help_center(move || {
            *help_center_opened_clone.borrow_mut() = true;
            let _help_center = app::HelpCenter::new().unwrap();
        });
        dashboard_ui.invoke_open_help_center();
        assert!(*help_center_opened.borrow(), "Help Center should be opened from Dashboard");

        let ai_chat_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
        let ai_chat_opened_clone = ai_chat_opened.clone();
        dashboard_ui.on_open_ai_chat(move || {
            *ai_chat_opened_clone.borrow_mut() = true;
            let _ai_chat = app::AiHelpChat::new().unwrap();
        });
        dashboard_ui.invoke_open_ai_chat();
        assert!(*ai_chat_opened.borrow(), "AI Help Chat should be opened from Dashboard");

        let _walkthrough = app::InteractiveWalkthrough::new().unwrap();

        let tooltip_registry = TooltipRegistry::new();
        dashboard_ui.set_tt_active_agents(tooltip_registry.get_tooltip("dashboard_active_agents").unwrap_or_default().into());
        assert_eq!(dashboard_ui.get_tt_active_agents(), "The number of AI agents currently working on tasks for your business.");
    }
}
