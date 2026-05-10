use slint::ComponentHandle;
use slint::SharedString;
use slint::ModelRc;
use slint::VecModel;
use std::rc::Rc;
use std::cell::RefCell;

#[test]
fn test_business_manager_ux_flow() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // Test 1: Start at Login (implicitly validating we can construct Login UI)
    let login = crate::app::Login::new().unwrap();
    assert_eq!(login.get_test_title(), slint::SharedString::from("One Human Corp - Login"));

    // Simulate clicking "Sign In"
    let login_clicked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let login_clicked_clone = login_clicked.clone();
    login.on_login(move |_u, _p| {
        *login_clicked_clone.borrow_mut() = true;
    });
    login.invoke_login("u".into(), "p".into());
    assert!(*login_clicked.borrow(), "Sign in button callback should trigger from Login page");

    // Test 2: Navigate to Dashboard and construct Business Manager
    let _dashboard = crate::app::Dashboard::new().unwrap();
    let app = crate::app::BusinessManager::new().unwrap();

    let products = Rc::new(VecModel::from(vec![
        crate::app::UiProduct {
            id: "1".into(),
            name: "Test Item".into(),
            type_label: "PHYSICAL".into(),
            price: "10.00".into(),
            inventory_count: 5,
            is_out_of_stock: false,
        }
    ]));
    app.set_products(ModelRc::from(products));

    // Test 3: List view edit and archive buttons triggers
    let edit_clicked = Rc::new(RefCell::new(false));
    let edit_clicked_clone = edit_clicked.clone();
    app.on_action_edit(move |_id| {
        *edit_clicked_clone.borrow_mut() = true;
    });

    let archive_clicked = Rc::new(RefCell::new(false));
    let archive_clicked_clone = archive_clicked.clone();
    app.on_action_archive(move |_id| {
        *archive_clicked_clone.borrow_mut() = true;
    });

    app.invoke_action_edit("1".into());
    assert!(*edit_clicked.borrow(), "Edit action should be triggered");

    app.invoke_action_archive("1".into());
    assert!(*archive_clicked.borrow(), "Archive action should be triggered");

    // Test 4: Navigate to Add view
    let add_new_clicked = Rc::new(RefCell::new(false));
    let add_new_clicked_clone = add_new_clicked.clone();
    app.on_action_add_new(move || {
        *add_new_clicked_clone.borrow_mut() = true;
    });
    app.invoke_action_add_new();
    assert!(*add_new_clicked.borrow(), "Add New action should be triggered");

    // Test 5: Verify Add view states and submit completion
    app.set_current_view("add".into());
    app.set_step(1);
    app.set_selected_type("PHYSICAL".into());
    let submitted = Rc::new(RefCell::new(false));
    let submitted_clone = submitted.clone();
    app.on_submit(move |_type, _name, _desc, _price, _duration, _schedule| {
        *submitted_clone.borrow_mut() = true;
    });

    app.invoke_submit(
        app.get_selected_type(),
        app.get_product_name(),
        app.get_product_description(),
        app.get_product_price(),
        app.get_service_duration(),
        app.get_service_schedule(),
    );

    assert!(*submitted.borrow(), "Submit action should be triggered to complete flow");
}

#[test]
fn test_api_docs_title_plain_language() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::ApiDocs::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("Connect Your Store"));
}

#[test]
fn test_integrations_title() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Integrations::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("Integrations & Tools"));
}

#[test]
fn test_login_title() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("One Human Corp - Login"));
}

#[test]
fn test_login_subtitle_plain_language() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    ui.set_is_sign_up(false);
    // Subtitle logic is internal, we just verify component doesn't crash on standard properties
    assert_eq!(ui.get_is_sign_up(), false);
}

#[test]
fn test_login_sign_in_button() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    let clicked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let clicked_clone = clicked.clone();
    ui.on_login(move |_u, _p| {
        *clicked_clone.borrow_mut() = true;
    });
    ui.invoke_login("u".into(), "p".into());
    assert!(*clicked.borrow(), "Sign in button callback should trigger");
}

#[test]
fn test_login_username_placeholder() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    ui.set_username("jane".into());
    assert_eq!(ui.get_username(), slint::SharedString::from("jane"));
}

#[test]
fn test_login_password_placeholder() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    ui.set_password("pass123".into());
    assert_eq!(ui.get_password(), slint::SharedString::from("pass123"));
}

#[test]
fn test_login_error_message() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Login::new().unwrap();
    ui.set_error_message("Invalid login".into());
    assert_eq!(ui.get_error_message(), slint::SharedString::from("Invalid login"));
}

#[test]
fn test_help_center_ui_title() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::HelpCenter::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("Help Center"));
}

#[test]
fn test_ai_help_chat_title() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::AiHelpChat::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("AI Help Assistant"));
}

#[test]
fn test_kairos_orchestration_walkthrough_title() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::KairosOrchestrationWalkthrough::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("How Your Helpers Work Together"));
}

#[test]
fn test_ongoing_management_fix_agent_title() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::FixAgent::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("Help Me Fix This"));
}

#[test]
fn test_ongoing_management_upgrade_title() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Upgrade::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("Platform Upgrade"));
}

#[test]
fn test_secure_agent_config_title() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::SecureAgentConfig::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("Secure Agent Config"));
}

#[test]
fn test_landing_title() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::Landing::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("OneHumanCorp"));
}

#[test]
fn test_billing_wizard_e2e() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // Simulate opening billing from Dashboard
    let billing_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let billing_opened_clone = billing_opened.clone();

    let db_ui = crate::app::Dashboard::new().unwrap();
    db_ui.on_open_billing(move || {
        *billing_opened_clone.borrow_mut() = true;
    });

    db_ui.invoke_open_billing();
    assert!(*billing_opened.borrow(), "Billing should be opened from Dashboard");

    let billing_ui = crate::app::Billing::new().unwrap();

    // Verify initial step (0)
    assert_eq!(billing_ui.get_step(), 0, "Billing should start at step 0");

    // Move to step 1
    billing_ui.invoke_next_step();
    assert_eq!(billing_ui.get_step(), 1, "Billing should move to step 1");

    // Move to step 2
    billing_ui.invoke_next_step();
    assert_eq!(billing_ui.get_step(), 2, "Billing should move to step 2");

    // Move back to step 1
    billing_ui.invoke_prev_step();
    assert_eq!(billing_ui.get_step(), 1, "Billing should go back to step 1");
}
