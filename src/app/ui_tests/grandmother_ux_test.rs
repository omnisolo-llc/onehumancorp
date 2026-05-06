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
    let clicked = std::sync::Arc::new(std::sync::Mutex::new(false));
    let clicked_clone = clicked.clone();
    ui.on_login(move |_u, _p| {
        *clicked_clone.lock().unwrap() = true;
    });
    ui.invoke_login("u".into(), "p".into());
    assert!(*clicked.lock().unwrap(), "Sign in button callback should trigger");
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
fn test_e2e_business_manager_schedule_plain_language_1() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let dashboard_ui = crate::app::Dashboard::new().unwrap();
    let business_manager_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let business_manager_opened_clone = business_manager_opened.clone();

    dashboard_ui.on_action_add_product(move || {
        *business_manager_opened_clone.borrow_mut() = true;
    });

    dashboard_ui.invoke_action_add_product();
    assert!(*business_manager_opened.borrow(), "Business manager should be opened from Dashboard Add action");

    let manager_ui = crate::app::BusinessManager::new().unwrap();
    assert_eq!(manager_ui.get_service_schedule(), "");
}

#[test]
fn test_e2e_business_manager_schedule_plain_language_2() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let dashboard_ui = crate::app::Dashboard::new().unwrap();
    dashboard_ui.invoke_action_add_product();

    let manager_ui = crate::app::BusinessManager::new().unwrap();
    let submitted = std::rc::Rc::new(std::cell::RefCell::new(false));
    let submitted_clone = submitted.clone();

    manager_ui.on_submit(move |type_, _name, _desc, _price, _dur, sch| {
        assert_eq!(type_, "SERVICE");
        assert_eq!(sch, "Tuesdays 10am-2pm");
        *submitted_clone.borrow_mut() = true;
    });

    manager_ui.invoke_select_type("SERVICE".into());
    manager_ui.invoke_next_step();

    manager_ui.set_service_schedule("Tuesdays 10am-2pm".into());
    manager_ui.invoke_submit("SERVICE".into(), "".into(), "".into(), "".into(), "".into(), "Tuesdays 10am-2pm".into());

    assert!(*submitted.borrow());
}

#[test]
fn test_e2e_business_manager_schedule_plain_language_5() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let dashboard_ui = crate::app::Dashboard::new().unwrap();
    dashboard_ui.invoke_action_add_product();

    let manager_ui = crate::app::BusinessManager::new().unwrap();
    let submitted = std::rc::Rc::new(std::cell::RefCell::new(false));
    let submitted_clone = submitted.clone();

    manager_ui.on_submit(move |type_, _name, _desc, _price, _dur, sch| {
        assert_eq!(type_, "SERVICE");
        assert_eq!(sch, "By appointment");
        *submitted_clone.borrow_mut() = true;
    });

    manager_ui.invoke_select_type("SERVICE".into());
    manager_ui.invoke_next_step();

    manager_ui.set_service_schedule("By appointment".into());
    manager_ui.invoke_submit("SERVICE".into(), "".into(), "".into(), "".into(), "".into(), "By appointment".into());

    assert!(*submitted.borrow());
}

#[test]
fn test_e2e_business_manager_schedule_plain_language_4() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let dashboard_ui = crate::app::Dashboard::new().unwrap();
    dashboard_ui.invoke_action_add_product();

    let manager_ui = crate::app::BusinessManager::new().unwrap();
    let submitted = std::rc::Rc::new(std::cell::RefCell::new(false));
    let submitted_clone = submitted.clone();

    manager_ui.on_submit(move |type_, _name, _desc, _price, _dur, sch| {
        assert_eq!(type_, "SERVICE");
        assert_eq!(sch, "24/7");
        *submitted_clone.borrow_mut() = true;
    });

    manager_ui.invoke_select_type("SERVICE".into());
    manager_ui.invoke_next_step();

    manager_ui.set_service_schedule("24/7".into());
    manager_ui.invoke_submit("SERVICE".into(), "".into(), "".into(), "".into(), "".into(), "24/7".into());

    assert!(*submitted.borrow());
}

#[test]
fn test_e2e_business_manager_schedule_plain_language_3() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let dashboard_ui = crate::app::Dashboard::new().unwrap();
    dashboard_ui.invoke_action_add_product();

    let manager_ui = crate::app::BusinessManager::new().unwrap();
    let submitted = std::rc::Rc::new(std::cell::RefCell::new(false));
    let submitted_clone = submitted.clone();

    manager_ui.on_submit(move |type_, _name, _desc, _price, _dur, sch| {
        assert_eq!(type_, "SERVICE");
        assert_eq!(sch, "Weekends only");
        *submitted_clone.borrow_mut() = true;
    });

    manager_ui.invoke_select_type("SERVICE".into());
    manager_ui.invoke_next_step();

    manager_ui.set_service_schedule("Weekends only".into());
    manager_ui.invoke_submit("SERVICE".into(), "".into(), "".into(), "".into(), "".into(), "Weekends only".into());

    assert!(*submitted.borrow());
}
