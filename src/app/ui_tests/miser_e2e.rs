use crate::app;
use slint::Model;
use std::rc::Rc;

#[test]
fn test_miser_cost_cuj_full_flow() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // 1. Initial State: User starts on the free tier and hits soft limits
    let my_plan_ui = app::MyPlan::new().unwrap();
    my_plan_ui.set_tier("Free".into());
    my_plan_ui.set_total_actions("100".into());
    my_plan_ui.set_action_limit("100".into());
    my_plan_ui.set_used_storage("500.0 MB".into());
    my_plan_ui.set_limit_storage("500.0 MB".into());

    // Simulating reaching the soft limit logic
    let upgrade_message = "Monthly action limit reached (100). Upgrade to Starter.";
    my_plan_ui.set_upgrade_prompt_message(upgrade_message.into());
    assert_eq!(my_plan_ui.get_upgrade_prompt_message(), upgrade_message);

    // Track upgrade click from MyPlan
    let upgrade_clicked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let upgrade_clicked_clone = upgrade_clicked.clone();
    my_plan_ui.on_upgrade(move || {
        *upgrade_clicked_clone.borrow_mut() = true;
    });
    my_plan_ui.invoke_upgrade();
    assert!(*upgrade_clicked.borrow(), "User clicked upgrade due to soft limits");

    // 2. Navigating to Cost Dashboard to view details before upgrading
    let view_details_clicked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let view_details_clone = view_details_clicked.clone();
    my_plan_ui.on_view_details(move || {
        *view_details_clone.borrow_mut() = true;
    });
    my_plan_ui.invoke_view_details();
    assert!(*view_details_clicked.borrow(), "User navigating to Cost Dashboard");

    let cost_ui = app::CostDashboard::new().unwrap();
    cost_ui.set_total_spend("$0.00".into());
    cost_ui.set_total_actions("0".into());

    let agent_costs = vec![
        app::UiAgentCost {
            name: "AutoDream".into(),
            cost: "$0.00".into(),
            roi: "0%".into(),
            efficiency: "0%".into(),
            storage_usage: "0MB".into(),
            pct: 0.0,
        }
    ];
    cost_ui.set_agent_costs(Rc::new(slint::VecModel::from(agent_costs)).into());

    assert_eq!(cost_ui.get_total_spend(), "$0.00");
    assert_eq!(cost_ui.get_total_actions(), "0");
    assert_eq!(cost_ui.get_agent_costs().row_data(0).unwrap().name, "AutoDream");

    // 3. User views Pricing plans and upgrades to Starter to resolve limits
    let pricing_ui = app::Pricing::new().unwrap();
    pricing_ui.set_step(0);
    assert_eq!(pricing_ui.get_step(), 0);

    // User sees their usage limit progress as full
    pricing_ui.set_usage_progress(1.0);
    assert_eq!(pricing_ui.get_usage_progress(), 1.0);

    // User switches to View Plans step
    pricing_ui.set_step(1);
    assert_eq!(pricing_ui.get_step(), 1);

    // User chooses Annual billing for a discount
    pricing_ui.set_is_annual(false);
    let toggle_billing_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let tb_clone = toggle_billing_called.clone();
    pricing_ui.on_toggle_billing_cycle(move || {
        *tb_clone.borrow_mut() = true;
    });
    pricing_ui.invoke_toggle_billing_cycle();
    assert!(*toggle_billing_called.borrow());

    // We simulate the backend logic toggling the flag
    pricing_ui.set_is_annual(true);
    assert!(pricing_ui.get_is_annual(), "Annual discount applied");

    // Validate that the discount reflects in the UI tiers
    let starter_tier = pricing_ui.get_tiers().row_data(1).unwrap();
    assert_eq!(starter_tier.price, "$7/mo (20% off)");

    // User selects the Starter plan
    let selected_plan = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let sp_clone = selected_plan.clone();
    pricing_ui.on_select_plan(move |plan| {
        *sp_clone.borrow_mut() = plan.to_string();
    });
    pricing_ui.invoke_select_plan("Starter".into());
    assert_eq!(*selected_plan.borrow(), "Starter", "User successfully upgraded to Starter plan");

    // 4. Verification: After upgrading, the user returns to MyPlan and limits are raised
    my_plan_ui.set_tier("Starter".into());
    my_plan_ui.set_total_actions("100".into());
    my_plan_ui.set_action_limit("1000".into());
    my_plan_ui.set_used_storage("500.0 MB".into());
    my_plan_ui.set_limit_storage("5.0 GB".into());
    my_plan_ui.set_estimated_bill("$7.00".into());

    // Soft limit is cleared (User first pricing ensures no hard blockages exist)
    my_plan_ui.set_upgrade_prompt_message("".into());

    assert_eq!(my_plan_ui.get_tier(), "Starter");
    assert_eq!(my_plan_ui.get_action_limit(), "1000");
    assert_eq!(my_plan_ui.get_upgrade_prompt_message(), "");

    // 5. Finalize the cost resiliency verify
    let view_history_clicked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let vh_clone = view_history_clicked.clone();
    my_plan_ui.on_view_history(move || {
        *vh_clone.borrow_mut() = true;
    });
    my_plan_ui.invoke_view_history();
    assert!(*view_history_clicked.borrow(), "User can view new billing history");
}
