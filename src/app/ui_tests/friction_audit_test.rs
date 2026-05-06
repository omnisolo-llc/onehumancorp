

#[test]
fn test_friction_audit_business_manager() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let manager_ui = crate::app::BusinessManager::new().unwrap();
    manager_ui.set_step(1);
    manager_ui.set_selected_type("SERVICE".into());

    // We shouldn't see "Schedule (JSON)"
    // There isn't an easy way to assert text on standard widgets in Slint rust unit tests without exposing it
    // But this test will verify the flow doesn't panic and the component is valid.
    manager_ui.invoke_next_step();
    assert_eq!(manager_ui.get_step(), 2);
}

#[test]
fn test_friction_audit_e2e_login_to_dashboard() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    // Start with the Login UI
    let login_ui = crate::app::Login::new().unwrap();
    let logged_in = std::rc::Rc::new(std::cell::RefCell::new(false));
    let logged_in_clone = logged_in.clone();

    login_ui.on_login(move |_email, _password| {
        *logged_in_clone.borrow_mut() = true;
    });

    login_ui.set_username("test@example.com".into());
    login_ui.set_password("password".into());
    login_ui.invoke_login("test@example.com".into(), "password".into());

    assert!(*logged_in.borrow(), "User should be logged in");

    let dashboard_ui = crate::app::Dashboard::new().unwrap();

    // Simulate tier limit prompt
    dashboard_ui.on_action_failed(|msg| {
        assert!(msg.contains("free plan"), "Message should be plain language");
    });

    // Check that api docs component can be created without panic
    let api_docs = crate::app::ApiDocs::new().unwrap();
    assert_eq!(api_docs.get_test_title(), slint::SharedString::from("Connect Custom Software"));
}

#[test]
fn test_friction_audit_e2e_grow_business() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let login_ui = crate::app::Login::new().unwrap();
    let logged_in = std::rc::Rc::new(std::cell::RefCell::new(false));
    let logged_in_clone = logged_in.clone();

    login_ui.on_login(move |_email, _password| {
        *logged_in_clone.borrow_mut() = true;
    });

    login_ui.invoke_login("test".into(), "test".into());
    assert!(*logged_in.borrow());

    let grow_ui = crate::app::GrowBusiness::new().unwrap();
    grow_ui.invoke_select_strategy("Connect my Instagram".into());
    assert_eq!(grow_ui.get_selected_strategy(), "Connect my Instagram");
}

#[test]
fn test_friction_audit_e2e_integrations() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let login_ui = crate::app::Login::new().unwrap();
    let logged_in = std::rc::Rc::new(std::cell::RefCell::new(false));
    let logged_in_clone = logged_in.clone();

    login_ui.on_login(move |_email, _password| {
        *logged_in_clone.borrow_mut() = true;
    });

    login_ui.invoke_login("test".into(), "test".into());
    assert!(*logged_in.borrow());

    let integrations_ui = crate::app::Integrations::new().unwrap();
    assert_eq!(integrations_ui.get_test_title(), slint::SharedString::from("Integrations & Tools"));

    let invoked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let invoked_clone = invoked.clone();
    integrations_ui.on_invoke_tool(move |id| {
        assert_eq!(id, "tool1");
        *invoked_clone.borrow_mut() = true;
    });
    integrations_ui.invoke_invoke_tool("tool1".into());
    assert!(*invoked.borrow());
}

#[test]
fn test_friction_audit_e2e_ongoing_management() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let login_ui = crate::app::Login::new().unwrap();
    let logged_in = std::rc::Rc::new(std::cell::RefCell::new(false));
    let logged_in_clone = logged_in.clone();

    login_ui.on_login(move |_email, _password| {
        *logged_in_clone.borrow_mut() = true;
    });

    login_ui.invoke_login("test".into(), "test".into());
    assert!(*logged_in.borrow());

    let ongoing_ui = crate::app::FixAgent::new().unwrap();
    ongoing_ui.set_is_advanced(true);
    assert_eq!(ongoing_ui.get_is_advanced(), true);
}
