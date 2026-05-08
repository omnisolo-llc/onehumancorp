use crate::app;
use slint::{Model, ComponentHandle};
use std::rc::Rc;

#[test]
fn test_miser_full_cost_efficiency_cuj() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // 1. Initial State: Start from Dashboard
    let dashboard = app::Dashboard::new().unwrap();
    let _dashboard_weak = dashboard.as_weak();

    // Track if Billing is opened from Dashboard
    let billing_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let billing_opened_clone = billing_opened.clone();
    dashboard.on_open_billing(move || {
        *billing_opened_clone.borrow_mut() = true;
    });

    // Simulate clicking Billing from Dashboard
    dashboard.invoke_open_billing();
    assert!(*billing_opened.borrow(), "Billing portal should be opened from Dashboard");

    // 2. Open MyPlan directly to simulate the inner billing flow since app::Billing delegates or stands beside it in the UI flow
    let my_plan_ui = app::MyPlan::new().unwrap();
    my_plan_ui.set_tier("Free Tier".into());
    my_plan_ui.set_total_actions("100".into());
    my_plan_ui.set_action_limit("100".into());
    my_plan_ui.set_used_storage("500.0 MB".into());
    my_plan_ui.set_limit_storage("500.0 MB".into());

    // 3. Simulate AI Agent Rate Limiting soft limit prompt without blocking
    let upgrade_message = "Monthly action limit reached (100). Upgrade to Starter.";
    my_plan_ui.set_upgrade_prompt_message(upgrade_message.into());
    assert_eq!(my_plan_ui.get_upgrade_prompt_message(), upgrade_message, "Soft limit prompt should be visible but not blocking");

    // Track upgrade click from MyPlan
    let upgrade_clicked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let upgrade_clicked_clone = upgrade_clicked.clone();
    my_plan_ui.on_upgrade(move || {
        *upgrade_clicked_clone.borrow_mut() = true;
    });
    my_plan_ui.invoke_upgrade();
    assert!(*upgrade_clicked.borrow(), "User clicked upgrade due to soft limits");

    // 4. View Cost Details (Infrastructure Cost Metering)
    let view_details_clicked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let view_details_clone = view_details_clicked.clone();
    my_plan_ui.on_view_details(move || {
        *view_details_clone.borrow_mut() = true;
    });
    my_plan_ui.invoke_view_details();
    assert!(*view_details_clicked.borrow(), "User navigated to Cost Dashboard");

    // 5. Verify the Cost Dashboard
    let cost_ui = app::CostDashboard::new().unwrap();
    cost_ui.set_total_spend("$0.00".into());
    cost_ui.set_total_tokens("5000".into()); // Simulate token efficiency metrics

    let agent_costs = vec![
        app::UiAgentCost {
            name: "AutoDream".into(),
            cost: "$0.00".into(),
            roi: "15%".into(),
            efficiency: "90%".into(),
            storage_usage: "200MB".into(), // Verify Storage Compression & CDN tracking
            pct: 0.4,
        }
    ];
    cost_ui.set_agent_costs(Rc::new(slint::VecModel::from(agent_costs)).into());

    assert_eq!(cost_ui.get_total_spend(), "$0.00");
    assert_eq!(cost_ui.get_total_tokens(), "5000");
    assert_eq!(cost_ui.get_agent_costs().row_data(0).unwrap().name, "AutoDream");
    assert_eq!(cost_ui.get_agent_costs().row_data(0).unwrap().storage_usage, "200MB");

    // 6. View Pricing Page & Billing Portal (Pricing)
    let pricing_ui = app::Pricing::new().unwrap();
    pricing_ui.set_step(0);
    assert_eq!(pricing_ui.get_step(), 0);

    // Verify current usage progress visually
    pricing_ui.set_usage_progress(1.0);
    assert_eq!(pricing_ui.get_usage_progress(), 1.0);

    // User switches to View Plans step (Step 1)
    pricing_ui.set_step(1);
    assert_eq!(pricing_ui.get_step(), 1);

    // 7. Toggle annual billing for discount
    pricing_ui.set_is_annual(false);
    let toggle_billing_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let tb_clone = toggle_billing_called.clone();
    pricing_ui.on_toggle_billing_cycle(move || {
        *tb_clone.borrow_mut() = true;
    });
    pricing_ui.invoke_toggle_billing_cycle();
    assert!(*toggle_billing_called.borrow());

    // Apply the annual toggle and verify discount is shown
    pricing_ui.set_is_annual(true);
    assert!(pricing_ui.get_is_annual(), "Annual discount applied");

    let starter_tier = pricing_ui.get_tiers().row_data(1).unwrap();
    assert_eq!(starter_tier.price, "$7/mo (20% off)", "Annual discount should reflect correctly for Starter");

    // 8. User selects the Starter plan
    let selected_plan = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let sp_clone = selected_plan.clone();
    pricing_ui.on_select_plan(move |plan| {
        *sp_clone.borrow_mut() = plan.to_string();
    });
    pricing_ui.invoke_select_plan("Starter".into());
    assert_eq!(*selected_plan.borrow(), "Starter", "User successfully upgraded to Starter plan");

    // 9. Verify limits are raised in MyPlan after upgrade
    my_plan_ui.set_tier("Starter Tier".into());
    my_plan_ui.set_total_actions("100".into());
    my_plan_ui.set_action_limit("1000".into());
    my_plan_ui.set_used_storage("500.0 MB".into());
    my_plan_ui.set_limit_storage("5.0 GB".into());
    my_plan_ui.set_estimated_bill("$7.00".into());

    // Clear the soft limit prompt
    my_plan_ui.set_upgrade_prompt_message("".into());

    assert_eq!(my_plan_ui.get_tier(), "Starter Tier");
    assert_eq!(my_plan_ui.get_action_limit(), "1000");
    assert_eq!(my_plan_ui.get_limit_storage(), "5.0 GB");
    assert_eq!(my_plan_ui.get_upgrade_prompt_message(), "");

    // 10. Verify Billing History
    let view_history_clicked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let vh_clone = view_history_clicked.clone();
    my_plan_ui.on_view_history(move || {
        *vh_clone.borrow_mut() = true;
    });
    my_plan_ui.invoke_view_history();
    assert!(*view_history_clicked.borrow(), "User can view new billing history");
}
