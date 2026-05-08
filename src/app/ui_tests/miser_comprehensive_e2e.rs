use crate::app;
use slint::Model;
use std::rc::Rc;

#[test]
fn test_miser_comprehensive_e2e_cost_resilience() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // --- Phase 1: Soft Limits and Usage Constraints ---
    let my_plan = app::MyPlan::new().unwrap();

    // Initial Free Tier Setup
    my_plan.set_tier("Free".into());
    my_plan.set_action_limit("100".into());
    my_plan.set_limit_storage("500MB".into());

    // Simulate approaching soft limit (soft warning)
    my_plan.set_total_actions("85".into());
    my_plan.set_used_storage("450MB".into());
    let soft_warning = "You are approaching your monthly action limit (85/100). Consider upgrading.";
    my_plan.set_upgrade_prompt_message(soft_warning.into());
    assert_eq!(my_plan.get_upgrade_prompt_message(), soft_warning);

    // Simulate hitting soft limit (upgrade prompt, NO hard block)
    my_plan.set_total_actions("100".into());
    let soft_limit_hit = "Monthly action limit reached (100). Upgrade to Starter to continue seamlessly.";
    my_plan.set_upgrade_prompt_message(soft_limit_hit.into());
    assert_eq!(my_plan.get_upgrade_prompt_message(), soft_limit_hit);

    // Track user clicking Upgrade from the prompt
    let upgrade_clicked = Rc::new(std::cell::RefCell::new(false));
    let uc_clone = upgrade_clicked.clone();
    my_plan.on_upgrade(move || {
        *uc_clone.borrow_mut() = true;
    });
    my_plan.invoke_upgrade();
    assert!(*upgrade_clicked.borrow(), "User clicked upgrade prompt");

    // --- Phase 2: Cost Dashboard Transparency ---
    let cost_dash = app::CostDashboard::new().unwrap();

    // Verify initial state
    cost_dash.set_total_spend("$0.00".into());
    cost_dash.set_total_tokens("50000".into());

    let agents = vec![
        app::UiAgentCost {
            name: "AutoDream".into(),
            cost: "$0.00".into(),
            roi: "N/A".into(),
            efficiency: "N/A".into(),
            storage_usage: "450MB".into(),
            pct: 0.9,
        }
    ];
    cost_dash.set_agent_costs(Rc::new(slint::VecModel::from(agents)).into());

    assert_eq!(cost_dash.get_total_spend(), "$0.00");
    assert_eq!(cost_dash.get_agent_costs().row_data(0).unwrap().storage_usage, "450MB");

    // Simulate tracking new usage (LLM Token Efficiency)
    cost_dash.set_total_spend("$2.50".into());
    cost_dash.set_total_tokens("125000".into());
    let agents_updated = vec![
        app::UiAgentCost {
            name: "AutoDream".into(),
            cost: "$1.50".into(),
            roi: "300%".into(),
            efficiency: "500 tok/$".into(),
            storage_usage: "450MB".into(),
            pct: 0.9,
        },
        app::UiAgentCost {
            name: "DataProcessor".into(),
            cost: "$1.00".into(),
            roi: "150%".into(),
            efficiency: "750 tok/$".into(),
            storage_usage: "0MB".into(),
            pct: 0.1,
        }
    ];
    cost_dash.set_agent_costs(Rc::new(slint::VecModel::from(agents_updated)).into());
    assert_eq!(cost_dash.get_total_spend(), "$2.50");
    assert_eq!(cost_dash.get_agent_costs().row_count(), 2);

    // Test refresh data callback
    let refresh_clicked = Rc::new(std::cell::RefCell::new(false));
    let rc_clone = refresh_clicked.clone();
    cost_dash.on_refresh_data(move || {
        *rc_clone.borrow_mut() = true;
    });
    cost_dash.invoke_refresh_data();
    assert!(*refresh_clicked.borrow(), "User refreshed cost data");

    // --- Phase 3: Pricing Page & Billing Portal ---
    let pricing = app::Pricing::new().unwrap();

    // Verify default layout
    assert_eq!(pricing.get_step(), 0);

    // Navigate to Plans view
    pricing.set_step(1);
    assert_eq!(pricing.get_step(), 1);

    // Test Billing Cycle Toggle (Annual vs Monthly)
    pricing.set_is_annual(false);
    let toggle_cycle = Rc::new(std::cell::RefCell::new(false));
    let tc_clone = toggle_cycle.clone();
    pricing.on_toggle_billing_cycle(move || {
        *tc_clone.borrow_mut() = true;
    });
    pricing.invoke_toggle_billing_cycle();
    assert!(*toggle_cycle.borrow(), "Billing cycle toggled");

    // Simulate backend updating the state after toggle
    pricing.set_is_annual(true);
    assert!(pricing.get_is_annual());

    // Verify Starter Tier pricing updates with discount
    let starter_tier = pricing.get_tiers().row_data(1).unwrap();
    assert_eq!(starter_tier.price, "$7/mo (20% off)");
    assert_eq!(starter_tier.storage_limit, "5GB storage limit");

    // User selects Starter Plan
    let selected_plan = Rc::new(std::cell::RefCell::new(String::new()));
    let sp_clone = selected_plan.clone();
    pricing.on_select_plan(move |plan| {
        *sp_clone.borrow_mut() = plan.to_string();
    });
    pricing.invoke_select_plan("Starter".into());
    assert_eq!(*selected_plan.borrow(), "Starter");

    // --- Phase 4: Verification of Upgraded State ---
    // Simulate backend sync back to MyPlan after successful payment
    my_plan.set_tier("Starter".into());
    my_plan.set_action_limit("1000".into());
    my_plan.set_limit_storage("5GB".into());
    my_plan.set_upgrade_prompt_message("".into()); // Soft limit cleared

    assert_eq!(my_plan.get_tier(), "Starter");
    assert_eq!(my_plan.get_action_limit(), "1000");
    assert_eq!(my_plan.get_limit_storage(), "5GB");
    assert_eq!(my_plan.get_upgrade_prompt_message(), ""); // No blocking prompts

    // Verify other Billing Portal actions work post-upgrade
    let view_history = Rc::new(std::cell::RefCell::new(false));
    let vh_clone = view_history.clone();
    my_plan.on_view_history(move || { *vh_clone.borrow_mut() = true; });
    my_plan.invoke_view_history();
    assert!(*view_history.borrow());

    let download_invoice = Rc::new(std::cell::RefCell::new(false));
    let di_clone = download_invoice.clone();
    my_plan.on_download_invoice(move || { *di_clone.borrow_mut() = true; });
    my_plan.invoke_download_invoice();
    assert!(*download_invoice.borrow());
}
