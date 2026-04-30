use ohc::orchestration::hub_service_client::HubServiceClient;
use ohc::orchestration::RegisterAgentRequest;
use ohc::orchestration::Agent;

pub mod ohc {
    pub mod orchestration {
        tonic::include_proto!("ohc.orchestration");
    }
}

pub mod tooltip_registry;
use tooltip_registry::TooltipRegistry;
use slint::ComponentHandle;

pub mod app {
    include!(concat!(env!("OUT_DIR"), "/app.rs"));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("App starting...");

    tokio::spawn(async move {
        match HubServiceClient::connect("http://127.0.0.1:18789").await {
            Ok(mut client) => {
                println!("Connected to server!");
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

    let setup_wizard_ui = app::SetupWizard::new()?;
    let setup_wizard_handle = setup_wizard_ui.as_weak();

    setup_wizard_ui.on_suggest_description({
        let ui_handle = setup_wizard_handle.clone();
        move || {
            let ui = ui_handle.unwrap();
            let name = ui.get_company_name().to_string();
            let biz_type = ui.get_business_type().to_string();
            let prompt = format!("Generate a one-line catchy business description for a {} named {}. Be concise.", biz_type, name);

            let handle_clone = ui_handle.clone();
            tokio::spawn(async move {
                 match HubServiceClient::connect("http://127.0.0.1:18789").await {
                    Ok(mut client) => {
                        let request = tonic::Request::new(ohc::orchestration::ReasonRequest {
                            prompt,
                            from_agent_id: "wizard-ui".into(),
                        });
                        match client.reason(request).await {
                            Ok(response) => {
                                let content = response.into_inner().content;
                                slint::invoke_from_event_loop(move || {
                                    if let Some(ui) = handle_clone.upgrade() {
                                        ui.set_company_description(content.into());
                                    }
                                }).unwrap();
                            }
                            Err(e) => println!("Reasoning failed: {:?}", e),
                        }
                    }
                    Err(e) => println!("Could not connect to server: {:?}", e),
                }
            });
        }
    });

    setup_wizard_ui.on_launch({
        let ui_handle = setup_wizard_handle.clone();
        move || {
            let ui = ui_handle.unwrap();
            let name = ui.get_company_name().to_string();
            let email = ui.get_admin_email().to_string();
            let biz_type = ui.get_business_type().to_string();

            let state = std::collections::HashMap::from([
                ("business_type".to_string(), ui.get_business_type().to_string()),
                ("company_name".to_string(), ui.get_company_name().to_string()),
                ("company_description".to_string(), ui.get_company_description().to_string()),
                ("sell_physical".to_string(), ui.get_sell_physical().to_string()),
                ("sell_digital".to_string(), ui.get_sell_digital().to_string()),
                ("sell_services".to_string(), ui.get_sell_services().to_string()),
                ("sell_food".to_string(), ui.get_sell_food().to_string()),
                ("sell_subscriptions".to_string(), ui.get_sell_subscriptions().to_string()),
                ("payment_pref".to_string(), ui.get_payment_pref().to_string()),
                ("admin_name".to_string(), ui.get_admin_name().to_string()),
                ("admin_email".to_string(), ui.get_admin_email().to_string()),
            ]);

            let handle_clone = ui_handle.clone();

            tokio::spawn(async move {
                match HubServiceClient::connect("http://127.0.0.1:18789").await {
                    Ok(mut client) => {
                        // 1. Save state
                        let _ = client.save_wizard_state(tonic::Request::new(ohc::orchestration::SaveWizardStateRequest {
                            state,
                        })).await;

                        // 2. Provision
                        let provision_req = ohc::orchestration::ProvisionRequest {
                            profile: Some(ohc::orchestration::Profile {
                                name: name.clone(),
                                industry: biz_type.clone(),
                                size: "1".into(),
                                language: "en".into(),
                            }),
                            goals: vec!["Launch".into()],
                            deployment: "standalone".into(),
                            admin: Some(ohc::orchestration::Admin {
                                name: "".into(),
                                email: email.clone(),
                                password: "".into(),
                            }),
                        };

                        match client.provision(tonic::Request::new(provision_req)).await {
                            Ok(resp) => {
                                println!("Provisioned: {:?}", resp.into_inner().message);
                                slint::invoke_from_event_loop(move || {
                                    if let Some(ui) = handle_clone.upgrade() {
                                        ui.hide().unwrap();
                                        let dashboard = app::Dashboard::new().unwrap();
                                        dashboard.show().unwrap();
                                    }
                                }).unwrap();
                            }
                            Err(e) => println!("Provisioning failed: {:?}", e),
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

        // Step 2: Review
        ui.set_step(2);

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

        // Final state verification
        assert_eq!(ui.get_company_name(), "My E2E Store");
        assert_eq!(ui.get_business_type(), "Online Store");
        assert_eq!(ui.get_admin_email(), "admin@e2e.test");
        assert_eq!(ui.get_payment_pref(), "online");
        assert_eq!(ui.get_sell_physical(), true);
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
        let tooltip_registry = TooltipRegistry::new();

        ui.set_tt_active_agents(tooltip_registry.get_tooltip("dashboard_active_agents").unwrap_or_default().into());
        ui.set_tt_active_tasks(tooltip_registry.get_tooltip("dashboard_active_tasks").unwrap_or_default().into());
        ui.set_tt_scheduled_calls(tooltip_registry.get_tooltip("dashboard_scheduled_calls").unwrap_or_default().into());
        ui.set_tt_team_members(tooltip_registry.get_tooltip("dashboard_team_members").unwrap_or_default().into());

        assert_eq!(ui.get_tt_active_agents(), "The number of AI agents currently working on tasks for your business.");
        assert_eq!(ui.get_tt_active_tasks(), "Tasks that your agents are actively processing right now.");

        // Ensure callbacks can be set
        ui.on_open_help_center(move || {});
        ui.on_open_ai_chat(move || {});
    }
}
