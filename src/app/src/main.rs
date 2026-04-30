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

    setup_wizard_ui.on_launch({
        let ui_handle = setup_wizard_handle.clone();
        move || {
            let ui = ui_handle.unwrap();
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
                        let request = tonic::Request::new(ohc::orchestration::SaveWizardStateRequest {
                            state,
                        });
                        if let Err(e) = client.save_wizard_state(request).await {
                            println!("Failed to save wizard state: {:?}", e);
                        } else {
                            println!("Wizard state saved to backend.");
                            slint::invoke_from_event_loop(move || {
                                if let Some(_ui) = handle_clone.upgrade() {
                                    // Done launching!
                                }
                            }).unwrap();
                        }
                    }
                    Err(e) => {
                        println!("Could not connect to server: {:?}", e);
                    }
                }
            });
        }
    });

    let ui = app::BusinessSetup::new()?;
    let ui_handle = ui.as_weak();

    ui.on_launch({
        let ui_handle = ui_handle.clone();
        move || {
            let ui = ui_handle.unwrap();
            let state = std::collections::HashMap::from([
                ("business_type".to_string(), ui.get_business_type().to_string()),
                ("company_name".to_string(), ui.get_company_name().to_string()),
                ("company_description".to_string(), ui.get_company_description().to_string()),
                ("website_template".to_string(), ui.get_website_template().to_string()),
                ("product_name".to_string(), ui.get_product_name().to_string()),
                ("product_price".to_string(), ui.get_product_price().to_string()),
                ("domain_choice".to_string(), ui.get_domain_choice().to_string()),
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
                        let request = tonic::Request::new(ohc::orchestration::SaveWizardStateRequest {
                            state,
                        });
                        if let Err(e) = client.save_wizard_state(request).await {
                            println!("Failed to save wizard state: {:?}", e);
                        } else {
                            println!("Wizard state saved to backend.");
                            slint::invoke_from_event_loop(move || {
                                if let Some(_ui) = handle_clone.upgrade() {
                                    // Done launching!
                                }
                            }).unwrap();
                        }
                    }
                    Err(e) => {
                        println!("Could not connect to server: {:?}", e);
                    }
                }
            });
        }
    });

    ui.run()?;
    
    Ok(())
}

#[cfg(test)]
mod e2e_tests {
    use super::*;

    #[test]
    fn test_e2e_wizard_flow() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
            println!("Skipping E2E test_e2e_wizard_flow because no display server is available.");
            return;
        }

        let ui = app::BusinessSetup::new().unwrap();

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
        ui.set_product_name("My First Product".into());
        ui.set_product_price("10.00".into());
        ui.set_step(8);

        // Step 8: Domain -> Step 9
        ui.set_domain_choice("subdomain".into());
        ui.set_step(9);

        // Final state verification
        assert_eq!(ui.get_company_name(), "My E2E Store");
        assert_eq!(ui.get_business_type(), "Online Store");
        assert_eq!(ui.get_admin_email(), "admin@e2e.test");
        assert_eq!(ui.get_payment_pref(), "online");
        assert_eq!(ui.get_sell_physical(), true);
        assert_eq!(ui.get_website_template(), "Modern");
        assert_eq!(ui.get_product_name(), "My First Product");
        assert_eq!(ui.get_product_price(), "10.00");
        assert_eq!(ui.get_domain_choice(), "subdomain");
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

#[cfg(test)]
mod cost_transparency_e2e_tests {
    use super::*;
    use slint::Model;

    #[test]
    fn test_e2e_cost_transparency_flow() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

        let my_plan_ui = app::MyPlan::new().unwrap();

        assert_eq!(my_plan_ui.get_tier(), "Pro Tier");

        my_plan_ui.set_tier("Starter Tier".into());
        my_plan_ui.set_total_actions("150".into());
        my_plan_ui.set_action_limit("1000".into());
        my_plan_ui.set_used_storage("150.5 MB".into());
        my_plan_ui.set_limit_storage("5.0 GB".into());
        my_plan_ui.set_estimated_bill("$29.00".into());

        assert_eq!(my_plan_ui.get_tier(), "Starter Tier");
        assert_eq!(my_plan_ui.get_total_actions(), "150");
        assert_eq!(my_plan_ui.get_action_limit(), "1000");
        assert_eq!(my_plan_ui.get_used_storage(), "150.5 MB");
        assert_eq!(my_plan_ui.get_limit_storage(), "5.0 GB");
        assert_eq!(my_plan_ui.get_estimated_bill(), "$29.00");

        let cost_ui = app::CostDashboard::new().unwrap();

        cost_ui.set_total_spend("$45.50".into());
        cost_ui.set_total_tokens("1,500,000".into());

        let agent_costs = slint::ModelRc::new(slint::VecModel::from(vec![
            app::UiAgentCost {
                name: "Customer Support Agent".into(),
                cost: "$25.00".into(),
                pct: 0.55,
            },
            app::UiAgentCost {
                name: "Marketing Agent".into(),
                cost: "$20.50".into(),
                pct: 0.45,
            }
        ]));

        cost_ui.set_agent_costs(agent_costs.clone());

        assert_eq!(cost_ui.get_total_spend(), "$45.50");
        assert_eq!(cost_ui.get_total_tokens(), "1,500,000");

        let retrieved_costs = cost_ui.get_agent_costs();
        assert_eq!(retrieved_costs.row_count(), 2);
        let first_agent = retrieved_costs.row_data(0).unwrap();
        assert_eq!(first_agent.name, "Customer Support Agent");
        assert_eq!(first_agent.cost, "$25.00");
    }
}
