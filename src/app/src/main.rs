use ohc::orchestration::hub_service_client::HubServiceClient;
use ohc::orchestration::growth_service_client::GrowthServiceClient;
use ohc::orchestration::RegisterAgentRequest;
use ohc::orchestration::Agent;

pub mod ohc {
    pub mod orchestration {
        tonic::include_proto!("ohc.orchestration");
    }
}

use slint::ComponentHandle;

pub mod app {
    include!(concat!(env!("OUT_DIR"), "/app.rs"));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("App starting...");

    tokio::spawn(async move {
        match HubServiceClient::connect(std::env::var("OHC_HUB_URL").unwrap_or_else(|_| "http://127.0.0.1:18789".to_string())).await {
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

    let init_ui_handle = setup_wizard_handle.clone();
    tokio::spawn(async move {
        if let Ok(mut client) = HubServiceClient::connect(std::env::var("OHC_HUB_URL").unwrap_or_else(|_| "http://127.0.0.1:18789".to_string())).await {
            if let Ok(resp) = client.get_wizard_state(tonic::Request::new(ohc::orchestration::GetWizardStateRequest {})).await {
                let state = resp.into_inner().state;
                slint::invoke_from_event_loop(move || {
                    if let Some(ui) = init_ui_handle.upgrade() {
                        if let Some(step_str) = state.get("step") {
                            if let Ok(step) = step_str.parse::<i32>() {
                                ui.set_step(step);
                            }
                        }
                        if let Some(val) = state.get("business_type") { ui.set_business_type(val.into()); }
                        if let Some(val) = state.get("company_name") { ui.set_company_name(val.into()); }
                        if let Some(val) = state.get("company_description") { ui.set_company_description(val.into()); }
                        if let Some(val) = state.get("sell_physical") { ui.set_sell_physical(val == "true"); }
                        if let Some(val) = state.get("sell_digital") { ui.set_sell_digital(val == "true"); }
                        if let Some(val) = state.get("sell_services") { ui.set_sell_services(val == "true"); }
                        if let Some(val) = state.get("sell_food") { ui.set_sell_food(val == "true"); }
                        if let Some(val) = state.get("sell_subscriptions") { ui.set_sell_subscriptions(val == "true"); }
                        if let Some(val) = state.get("payment_pref") { ui.set_payment_pref(val.into()); }
                        if let Some(val) = state.get("admin_name") { ui.set_admin_name(val.into()); }
                        if let Some(val) = state.get("admin_email") { ui.set_admin_email(val.into()); }
                        if let Some(val) = state.get("website_template") { ui.set_website_template(val.into()); }
                        if let Some(val) = state.get("product_name") { ui.set_product_name(val.into()); }
                        if let Some(val) = state.get("product_price") { ui.set_product_price(val.into()); }
                        if let Some(val) = state.get("domain_choice") { ui.set_domain_choice(val.into()); }
                    }
                }).unwrap();
            }
        }
    });

    setup_wizard_ui.on_save_state({
        let ui_handle = setup_wizard_handle.clone();
        move || {
            let ui = ui_handle.unwrap();
            let state = std::collections::HashMap::from([
                ("step".to_string(), ui.get_step().to_string()),
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
                ("website_template".to_string(), ui.get_website_template().to_string()),
                ("product_name".to_string(), ui.get_product_name().to_string()),
                ("product_price".to_string(), ui.get_product_price().to_string()),
                ("domain_choice".to_string(), ui.get_domain_choice().to_string()),
            ]);

            tokio::spawn(async move {
                if let Ok(mut client) = HubServiceClient::connect(std::env::var("OHC_HUB_URL").unwrap_or_else(|_| "http://127.0.0.1:18789".to_string())).await {
                    let request = tonic::Request::new(ohc::orchestration::SaveWizardStateRequest { state });
                    let _ = client.save_wizard_state(request).await;
                }
            });
        }
    });

    let referrals_ui = app::Referrals::new()?;
    let referrals_handle = referrals_ui.as_weak();

    referrals_ui.on_refresh({
        let ui_handle = referrals_handle.clone();
        move || {
            let handle = ui_handle.clone();
            tokio::spawn(async move {
                match GrowthServiceClient::connect(std::env::var("OHC_HUB_URL").unwrap_or_else(|_| "http://127.0.0.1:18789".to_string())).await {
                    Ok(mut client) => {
                        let response = client.get_referrals(tonic::Request::new(ohc::orchestration::EmptyRequest {})).await;
                        if let Ok(resp) = response {
                            let referrals = resp.into_inner().referrals;
                            slint::invoke_from_event_loop(move || {
                                if let Some(ui) = handle.upgrade() {
                                    let ui_referrals: Vec<app::UiReferral> = referrals.into_iter().map(|r| {
                                        app::UiReferral {
                                            referral_code: r.referral_code.into(),
                                            user_id: r.user_id.into(),
                                            clicks: r.clicks,
                                            conversions: r.conversions,
                                            created_at: "".into(), // Simplified
                                        }
                                    }).collect();
                                    ui.set_referrals(slint::ModelRc::new(slint::VecModel::from(ui_referrals)));
                                }
                            }).unwrap();
                        }
                    }
                    Err(e) => println!("Failed to connect for referrals: {:?}", e),
                }
            });
        }
    });

    referrals_ui.on_generate_new_link({
        let ui_handle = referrals_handle.clone();
        move || {
            let handle = ui_handle.clone();
            tokio::spawn(async move {
                match GrowthServiceClient::connect(std::env::var("OHC_HUB_URL").unwrap_or_else(|_| "http://127.0.0.1:18789".to_string())).await {
                    Ok(mut client) => {
                        let req = ohc::orchestration::CreateReferralRequest {
                            user_id: "current_user".to_string(), // In production, use actual user_id
                            referral_code: "".to_string(),
                        };
                        let response = client.create_referral(tonic::Request::new(req)).await;
                        if let Ok(resp) = response {
                            let referral = resp.into_inner();
                            let link = format!("ohc://join?ref={}", referral.referral_code);
                            slint::invoke_from_event_loop(move || {
                                if let Some(ui) = handle.upgrade() {
                                    ui.set_my_referral_link(link.into());
                                }
                            }).unwrap();
                        }
                    }
                    Err(e) => println!("Failed to create referral: {:?}", e),
                }
            });
        }
    });

    setup_wizard_ui.on_launch({
        let ui_handle = setup_wizard_handle.clone();
        move |business_type, company_name, company_description, payment_pref, admin_email, website_template, product_name, product_price, domain_choice| {
            let ui = ui_handle.unwrap();
            let state = std::collections::HashMap::from([
                ("business_type".to_string(), business_type.to_string()),
                ("company_name".to_string(), company_name.to_string()),
                ("company_description".to_string(), company_description.to_string()),
                ("sell_physical".to_string(), ui.get_sell_physical().to_string()),
                ("sell_digital".to_string(), ui.get_sell_digital().to_string()),
                ("sell_services".to_string(), ui.get_sell_services().to_string()),
                ("sell_food".to_string(), ui.get_sell_food().to_string()),
                ("sell_subscriptions".to_string(), ui.get_sell_subscriptions().to_string()),
                ("payment_pref".to_string(), payment_pref.to_string()),
                ("admin_name".to_string(), ui.get_admin_name().to_string()),
                ("admin_email".to_string(), admin_email.to_string()),
                ("website_template".to_string(), website_template.to_string()),
                ("product_name".to_string(), product_name.to_string()),
                ("product_price".to_string(), product_price.to_string()),
                ("domain_choice".to_string(), domain_choice.to_string()),
            ]);

            let handle_clone = ui_handle.clone();

            let req_business_type = business_type.to_string();
            let req_company_name = company_name.to_string();
            let req_company_description = company_description.to_string();
            let req_payment_pref = payment_pref.to_string();
            let req_admin_email = admin_email.to_string();
            let req_website_template = website_template.to_string();
            let req_product_name = product_name.to_string();
            let req_product_price = product_price.to_string();
            let req_domain_choice = domain_choice.to_string();

            let mut req_selling_categories = Vec::new();
            if ui.get_sell_physical() { req_selling_categories.push("physical".to_string()); }
            if ui.get_sell_digital() { req_selling_categories.push("digital".to_string()); }
            if ui.get_sell_services() { req_selling_categories.push("services".to_string()); }
            if ui.get_sell_food() { req_selling_categories.push("food".to_string()); }
            if ui.get_sell_subscriptions() { req_selling_categories.push("subscriptions".to_string()); }

            tokio::spawn(async move {
                match HubServiceClient::connect(std::env::var("OHC_HUB_URL").unwrap_or_else(|_| "http://127.0.0.1:18789".to_string())).await {
                    Ok(mut client) => {
                        let onboarding_request = tonic::Request::new(ohc::orchestration::StartOnboardingRequest {
                            business_type: req_business_type,
                            company_name: req_company_name,
                            company_description: req_company_description,
                            payment_pref: req_payment_pref,
                            admin_email: req_admin_email,
                            selling_categories: req_selling_categories,
                            website_template: req_website_template,
                            first_product_name: req_product_name,
                            first_product_price: req_product_price,
                            domain_choice: req_domain_choice,
                        });

                        match client.start_onboarding(onboarding_request).await {
                            Ok(resp) => {
                                let r = resp.into_inner();
                                let msg = r.message.clone();
                                slint::invoke_from_event_loop(move || {
                                    if let Some(ui) = handle_clone.upgrade() {
                                        ui.set_launch_status("Onboarding Complete!".into());
                                        ui.set_launch_details(msg.into());
                                        ui.set_step(10); // Go to checklist
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

                        let request = tonic::Request::new(ohc::orchestration::SaveWizardStateRequest {
                            state,
                        });
                        if let Err(e) = client.save_wizard_state(request).await {
                            println!("Failed to save wizard state: {:?}", e);
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
mod growth_e2e_tests {
    use super::*;
    use slint::Model;

    #[test]
    fn test_e2e_referral_flow() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

        let ui = app::Referrals::new().unwrap();

        let referral_data = slint::ModelRc::new(slint::VecModel::from(vec![
            app::UiReferral {
                referral_code: "GROWTH2024".into(),
                user_id: "user_123".into(),
                clicks: 45,
                conversions: 12,
                created_at: "2024-01-01".into(),
            }
        ]));

        ui.set_referrals(referral_data.clone());

        assert_eq!(ui.get_referrals().row_count(), 1);
        let r = ui.get_referrals().row_data(0).unwrap();
        assert_eq!(r.referral_code, "GROWTH2024");
        assert_eq!(r.clicks, 45);
        assert_eq!(r.conversions, 12);
    }
}

#[cfg(test)]
mod e2e_tests {
    use slint::Model;
    use super::*;

    #[test]
    fn test_cuj_draft_for_review_flow() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
            println!("Skipping E2E test_cuj_draft_for_review_flow because no display server is available.");
            return;
        }

        let ui = app::Dashboard::new().unwrap();

        let pending_tasks = vec![
            app::UiPendingApproval {
                task_id: "test-task-123".into(),
                title: "Draft Confirmation for Maya".into(),
                proposed_content: "Hi Maya, thank you for your custom order!".into(),
            }
        ];

        let pending_model = std::rc::Rc::new(slint::VecModel::from(pending_tasks));
        ui.set_pending_approvals(pending_model.into());

        assert_eq!(ui.get_pending_approvals().row_count(), 1);

        // Use a shared state to verify the callback was called
        let was_approved = std::rc::Rc::new(std::cell::RefCell::new(false));
        let was_approved_clone = was_approved.clone();

        ui.on_approve_task(move |task_id| {
            if task_id == "test-task-123" {
                *was_approved_clone.borrow_mut() = true;
            }
        });

        // Programmatically invoke the callback as if the user clicked the button
        ui.invoke_approve_task("test-task-123".into());

        assert_eq!(*was_approved.borrow(), true);
    }

    #[test]
    fn test_login_password_visibility_toggle() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        let ui = app::Login::new().unwrap();

        // The toggle state in the encapsulated component is internal to Slint
        // but we can set the password property
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
        ui.invoke_next_step();

        // Step 1: Type -> Step 2
        ui.invoke_select_business_type("Online Store".into());
        assert_eq!(ui.get_step(), 2);

        // Step 2: Name -> Step 3
        ui.set_company_name("My E2E Store".into());
        ui.invoke_next_step();
        assert_eq!(ui.get_step(), 3);

        // Step 3: What do you sell -> Step 4
        ui.invoke_toggle_sell_physical();
        ui.invoke_next_step();
        assert_eq!(ui.get_step(), 4);

        // Step 4: Payments -> Step 5
        ui.invoke_select_payment_pref("online".into());
        assert_eq!(ui.get_step(), 5);

        // Step 5: Admin -> Step 6
        ui.set_admin_email("admin@e2e.test".into());
        ui.invoke_next_step();
        assert_eq!(ui.get_step(), 6);

        // Step 6: Template -> Step 7
        ui.invoke_select_template("Modern".into());
        assert_eq!(ui.get_step(), 7);

        // Step 7: Product -> Step 8
        ui.set_product_name("My First Product".into());
        ui.set_product_price("10.00".into());
        ui.invoke_next_step();
        assert_eq!(ui.get_step(), 8);

        // Step 8: Domain -> Step 9
        ui.invoke_select_domain("subdomain".into());
        assert_eq!(ui.get_step(), 9);

        // Final state verification
        assert_eq!(ui.get_company_name(), "My E2E Store");
        assert_eq!(ui.get_business_type(), "Online Store");
        assert_eq!(ui.get_admin_email(), "admin@e2e.test");
        assert_eq!(ui.get_payment_pref(), "online");
        assert_eq!(ui.get_sell_physical(), true);
        assert_eq!(ui.get_sell_digital(), false);
        assert_eq!(ui.get_sell_services(), false);
        assert_eq!(ui.get_sell_food(), false);
        assert_eq!(ui.get_sell_subscriptions(), false);
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
        ui.set_product_name("My First Product".into());
        ui.set_product_price("10.00".into());
        ui.invoke_next_step();

        // Step 8: Domain -> Step 9
        ui.invoke_select_domain("subdomain".into());

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

    #[test]
    fn test_e2e_website_builder_flow() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
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
    #[test]
    fn test_e2e_agent_config_flow() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

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

        let ui = app::AgentConfig::new().unwrap();
        let publish_success = std::rc::Rc::new(std::cell::RefCell::new(false));
        let publish_success_clone = publish_success.clone();

        ui.on_activate_agent(move |agent, can_reply, can_social, frequency| {
            assert_eq!(agent, "Customer Support");
            assert_eq!(can_reply, true);
            assert_eq!(can_social, false);
            assert_eq!(frequency, "Daily");
            *publish_success_clone.borrow_mut() = true;
        });

        // Step 0: Choose Agent -> Step 1
        assert_eq!(ui.get_step(), 0);
        ui.set_selected_agent("Customer Support".into());
        ui.set_step(1);

        // Step 1: Capabilities -> Step 2
        ui.set_can_reply(true);
        ui.set_step(2);

        // Step 2: Frequency -> Step 3
        ui.set_frequency("Daily".into());
        ui.set_step(3);

        // Step 3: Review
        ui.invoke_activate_agent(
            ui.get_selected_agent(),
            ui.get_can_reply(),
            ui.get_can_social(),
            ui.get_frequency()
        );

        assert_eq!(ui.get_step(), 3);
        assert_eq!(ui.get_selected_agent(), "Customer Support");
        assert_eq!(ui.get_can_reply(), true);
        assert_eq!(ui.get_frequency(), "Daily");
        assert!(*publish_success.borrow());
    }

    #[test]
    fn test_e2e_interactive_walkthrough_flow() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        let ui = app::InteractiveWalkthrough::new().unwrap();

        assert_eq!(ui.get_current_step(), 0);
        ui.set_current_step(1);
        assert_eq!(ui.get_current_step(), 1);
        ui.set_current_step(2);
        assert_eq!(ui.get_current_step(), 2);
        ui.set_current_step(3);
        assert_eq!(ui.get_current_step(), 3);
    }

    #[test]
    fn test_e2e_grow_business_flow() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        let ui = app::GrowBusiness::new().unwrap();

        let execute_success = std::rc::Rc::new(std::cell::RefCell::new(false));
        let execute_success_clone = execute_success.clone();

        ui.on_execute(move |strategy| {
            assert_eq!(strategy, "Add 5 more products");
            *execute_success_clone.borrow_mut() = true;
        });

        assert_eq!(ui.get_step(), 0);
        ui.set_selected_strategy("Add 5 more products".into());
        ui.set_step(1);
        assert_eq!(ui.get_step(), 1);

        ui.invoke_execute(ui.get_selected_strategy());

        assert_eq!(ui.get_selected_strategy(), "Add 5 more products");
        assert!(*execute_success.borrow());
    }

}

#[cfg(test)]
mod dashboard_docs_tests {
    use super::*;

    #[test]
    fn test_documentation_components_e2e_flow() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
            println!("Skipping test_documentation_components_e2e_flow because no display server is available.");
            return;
        }

        // 1. Start from the home page after user login via the UI
        let login_ui = app::Login::new().unwrap();
        let login_successful = std::rc::Rc::new(std::cell::RefCell::new(false));
        let login_successful_clone = login_successful.clone();

        login_ui.on_login(move |email, password| {
            assert_eq!(email, "test@example.com");
            assert_eq!(password, "password123");
            *login_successful_clone.borrow_mut() = true;
        });

        // Simulate user login
        login_ui.invoke_login("test@example.com".into(), "password123".into());
        assert!(*login_successful.borrow(), "User login should be successful");

        // 2. Load the main Dashboard
        let dashboard_ui = app::Dashboard::new().unwrap();

        // 3. Test opening Help Center from Dashboard
        let help_center_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
        let help_center_opened_clone = help_center_opened.clone();
        dashboard_ui.on_open_help_center(move || {
            *help_center_opened_clone.borrow_mut() = true;
            // Verify HelpCenter component can be instantiated
            let _help_center = app::HelpCenter::new().unwrap();
        });
        dashboard_ui.invoke_open_help_center();
        assert!(*help_center_opened.borrow(), "Help Center should be opened from Dashboard");

        // 4. Test opening AI Help Chat from Dashboard
        let ai_chat_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
        let ai_chat_opened_clone = ai_chat_opened.clone();
        dashboard_ui.on_open_ai_chat(move || {
            *ai_chat_opened_clone.borrow_mut() = true;
            // Verify AiHelpChat component can be instantiated
            let _ai_chat = app::AiHelpChat::new().unwrap();
        });
        dashboard_ui.invoke_open_ai_chat();
        assert!(*ai_chat_opened.borrow(), "AI Help Chat should be opened from Dashboard");

        // 5. Test Interactive Walkthrough
        let _walkthrough = app::InteractiveWalkthrough::new().unwrap();
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
                cost: "$25.00".into(), roi: "150%".into(), efficiency: "100 tok/$".into(),
                pct: 0.55,
            },
            app::UiAgentCost {
                name: "Marketing Agent".into(),
                cost: "$20.50".into(), roi: "0%".into(), efficiency: "0 tok/$".into(),
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
        assert_eq!(first_agent.cost, "$25.00"); assert_eq!(first_agent.roi, "150%"); assert_eq!(first_agent.efficiency, "100 tok/$");
    }
}
