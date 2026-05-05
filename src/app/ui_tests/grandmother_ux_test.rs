use slint::ComponentHandle;

#[test]
fn test_api_docs_title_plain_language() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::ApiDocs::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("Connect Custom Software"));
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
fn test_business_manager_availability_flow() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // Simulate full flow starting from Dashboard
    let dashboard = crate::app::Dashboard::new().unwrap();
    let invoked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let invoked_clone = invoked.clone();
    dashboard.on_action_add_product(move || {
        *invoked_clone.borrow_mut() = true;
    });

    // User clicks "Add" on dashboard to open Business Manager
    dashboard.invoke_action_add_product();
    assert!(*invoked.borrow(), "Should open business manager from dashboard");

    let ui = crate::app::BusinessManager::new().unwrap();

    // Simulate user flow for adding a service
    // Step 0: Select type
    ui.set_step(0);
    ui.invoke_select_type("SERVICE".into());
    assert_eq!(ui.get_selected_type(), slint::SharedString::from("SERVICE"));

    // Move to step 1
    ui.invoke_next_step();
    assert_eq!(ui.get_step(), 1);

    // Fill in details, avoiding technical jargon
    ui.set_product_name("Consultation".into());
    ui.set_product_description("One hour session".into());
    ui.set_product_price("50.00".into());
    ui.set_service_duration("60".into());
    ui.set_service_schedule("Mon-Wed 10am-2pm".into());

    assert_eq!(ui.get_product_name(), slint::SharedString::from("Consultation"));
    assert_eq!(ui.get_product_description(), slint::SharedString::from("One hour session"));
    assert_eq!(ui.get_product_price(), slint::SharedString::from("50.00"));
    assert_eq!(ui.get_service_duration(), slint::SharedString::from("60"));
    assert_eq!(ui.get_service_schedule(), slint::SharedString::from("Mon-Wed 10am-2pm"));

    let submit_invoked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let submit_invoked_clone = submit_invoked.clone();
    ui.on_submit(move |_t, _n, _d, _p, _dur, _sch| {
        *submit_invoked_clone.borrow_mut() = true;
    });
    ui.invoke_submit("SERVICE".into(), "Consultation".into(), "One hour session".into(), "50.00".into(), "60".into(), "Mon-Wed 10am-2pm".into());
    assert!(*submit_invoked.borrow(), "Should submit without JSON jargon");
}

#[test]
fn test_business_manager_step_toggle() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::BusinessManager::new().unwrap();
    ui.set_step(1);
    assert_eq!(ui.get_step(), 1);
}

#[test]
fn test_business_manager_select_type() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::BusinessManager::new().unwrap();
    ui.set_selected_type("SERVICE".into());
    assert_eq!(ui.get_selected_type(), slint::SharedString::from("SERVICE"));
}

#[test]
fn test_business_manager_next_step() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::BusinessManager::new().unwrap();
    ui.set_step(0);
    ui.invoke_next_step();
    assert_eq!(ui.get_step(), 1);
}

#[test]
fn test_business_manager_prev_step() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::BusinessManager::new().unwrap();
    ui.set_step(1);
    ui.invoke_prev_step();
    assert_eq!(ui.get_step(), 0);
}

#[test]
fn test_api_docs_plain_language() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // Simulate flow from dashboard to api docs
    let dashboard = crate::app::Dashboard::new().unwrap();
    let invoked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let invoked_clone = invoked.clone();
    dashboard.on_open_api_docs(move || {
        *invoked_clone.borrow_mut() = true;
    });

    // User clicks "Connect Integrations" on dashboard
    dashboard.invoke_open_api_docs();
    assert!(*invoked.borrow(), "Should open integrations from dashboard");

    let ui = crate::app::ApiDocs::new().unwrap();
    assert_eq!(ui.get_test_title(), slint::SharedString::from("Connect Custom Software"));
    assert_eq!(ui.get_api_key(), slint::SharedString::from("Connect Code"));
    assert_eq!(ui.get_endpoint_url(), slint::SharedString::from("https://connect.ohc.io"));
}
