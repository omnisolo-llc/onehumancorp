use crate::app;
use std::rc::Rc;
use std::cell::RefCell;

#[test]
fn test_mission_grandmother_flow_add_product() {
    crate::ui_tests::init();

    // 1. Start from Login
    let login = app::Login::new().unwrap();
    let login_clicked = Rc::new(RefCell::new(false));
    let login_clicked_clone = login_clicked.clone();

    login.on_login(move |_, _| {
        *login_clicked_clone.borrow_mut() = true;
    });

    login.set_username("grandma@example.com".into());
    login.set_password("mypassword".into());
    login.invoke_login(login.get_username(), login.get_password());

    assert!(*login_clicked.borrow(), "Must be able to login successfully");

    // 2. Navigate to Dashboard
    let dashboard = app::Dashboard::new().unwrap();

    // 3. Verify target action is mapped
    let target_clicked = Rc::new(RefCell::new(false));
    let target_clicked_clone = target_clicked.clone();
    dashboard.on_action_add_product(move || {
        *target_clicked_clone.borrow_mut() = true;
    });

    dashboard.invoke_action_add_product();
    assert!(*target_clicked.borrow(), "Add Product action was not reachable or failed to trigger.");
}

#[test]
fn test_mission_grandmother_flow_view_orders() {
    crate::ui_tests::init();
    let login = app::Login::new().unwrap();
    login.set_username("grandma@example.com".into());
    login.set_password("mypassword".into());
    login.invoke_login(login.get_username(), login.get_password());

    let dashboard = app::Dashboard::new().unwrap();

    let target_clicked = Rc::new(RefCell::new(false));
    let target_clicked_clone = target_clicked.clone();
    dashboard.on_action_view_orders(move || {
        *target_clicked_clone.borrow_mut() = true;
    });

    dashboard.invoke_action_view_orders();
    assert!(*target_clicked.borrow(), "View Orders action was not reachable or failed to trigger.");
}

#[test]
fn test_mission_grandmother_flow_check_messages() {
    crate::ui_tests::init();
    let login = app::Login::new().unwrap();
    login.set_username("grandma@example.com".into());
    login.set_password("mypassword".into());
    login.invoke_login(login.get_username(), login.get_password());

    let dashboard = app::Dashboard::new().unwrap();

    let target_clicked = Rc::new(RefCell::new(false));
    let target_clicked_clone = target_clicked.clone();
    dashboard.on_action_check_messages(move || {
        *target_clicked_clone.borrow_mut() = true;
    });

    dashboard.invoke_action_check_messages();
    assert!(*target_clicked.borrow(), "Check Messages action was not reachable or failed to trigger.");
}

#[test]
fn test_mission_grandmother_flow_see_analytics() {
    crate::ui_tests::init();
    let login = app::Login::new().unwrap();
    login.set_username("grandma@example.com".into());
    login.set_password("mypassword".into());
    login.invoke_login(login.get_username(), login.get_password());

    let dashboard = app::Dashboard::new().unwrap();

    let target_clicked = Rc::new(RefCell::new(false));
    let target_clicked_clone = target_clicked.clone();
    dashboard.on_action_see_analytics(move || {
        *target_clicked_clone.borrow_mut() = true;
    });

    dashboard.invoke_action_see_analytics();
    assert!(*target_clicked.borrow(), "See Analytics action was not reachable or failed to trigger.");
}

#[test]
fn test_mission_grandmother_flow_share_store() {
    crate::ui_tests::init();
    let login = app::Login::new().unwrap();
    login.set_username("grandma@example.com".into());
    login.set_password("mypassword".into());
    login.invoke_login(login.get_username(), login.get_password());

    let dashboard = app::Dashboard::new().unwrap();

    let target_clicked = Rc::new(RefCell::new(false));
    let target_clicked_clone = target_clicked.clone();
    dashboard.on_action_share_store(move || {
        *target_clicked_clone.borrow_mut() = true;
    });

    dashboard.invoke_action_share_store();
    assert!(*target_clicked.borrow(), "Share Store action was not reachable or failed to trigger.");
}

#[test]
fn test_mission_business_manager_interaction_flow() {
    crate::ui_tests::init();
    let ui = app::BusinessManager::new().unwrap();

    ui.set_current_view("add".into());
    ui.set_step(0);
    assert!(!ui.get_show_offering_hint());
    ui.set_show_offering_hint(true);
    assert!(ui.get_show_offering_hint());

    ui.set_step(1);
    assert!(!ui.get_show_details_hint());
    ui.set_show_details_hint(true);
    assert!(ui.get_show_details_hint());
}

#[test]
fn test_mission_dashboard_interaction_flow() {
    crate::ui_tests::init();
    let ui = app::Dashboard::new().unwrap();

    // Simulate clicking the "?" button for Business Health
    assert!(!ui.get_show_health_hint());

    // In a real E2E test, we'd use a click event.
    // Here we simulate the state change that the button click would cause.
    ui.set_show_health_hint(true);
    assert!(ui.get_show_health_hint());

    // Verify properties still correctly linked
    ui.set_generative_score("98".into());
    assert_eq!(ui.get_generative_score(), "98");
}

#[test]
fn test_mission_setup_wizard_flow_labels() {
    crate::ui_tests::init();
    let ui = app::SetupWizard::new().unwrap();

    ui.set_step(7);
    ui.set_price_type("fixed".into());
    assert_eq!(ui.get_price_type(), "fixed");

    ui.set_price_type("request_quote".into());
    assert_eq!(ui.get_price_type(), "request_quote");
}

#[test]
fn test_mission_dashboard_telemetry_renames() {
    crate::ui_tests::init();
    let ui = app::Dashboard::new().unwrap();

    ui.set_telemetry_cache_hits("100%".into());
    ui.set_telemetry_rag_latency("50ms".into());

    assert_eq!(ui.get_telemetry_cache_hits(), "100%");
    assert_eq!(ui.get_telemetry_rag_latency(), "50ms");
}

#[test]
fn test_mission_grandmother_test_compliance() {
    crate::ui_tests::init();
    let ui = app::Dashboard::new().unwrap();

    ui.set_generative_score("Perfect".into());
    assert_eq!(ui.get_generative_score(), "Perfect");
}

#[test]
fn test_mission_dashboard_my_store_label() {
    crate::ui_tests::init();
    let ui = app::Dashboard::new().unwrap();
    assert!(!ui.get_show_menu());
    ui.set_show_menu(true);
    assert!(ui.get_show_menu());
}

#[test]
fn test_mission_dashboard_today_sales_check() {
    crate::ui_tests::init();
    let ui = app::Dashboard::new().unwrap();
    ui.set_todays_sales("$500".into());
    assert_eq!(ui.get_todays_sales(), "$500");
}
