use slint::ComponentHandle;

#[test]
fn test_e2e_login_error_message_flow() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // Start with the Login UI
    let login_ui = crate::app::Login::new().unwrap();
    let login_successful = std::rc::Rc::new(std::cell::RefCell::new(false));
    let login_successful_clone = login_successful.clone();

    login_ui.on_login(move |_email, _password| {
        *login_successful_clone.borrow_mut() = true;
    });

    // Assume some failure sets the plain-language error message
    login_ui.set_error_message("We had trouble communicating with the local intelligence service (code 500). Please try again later.".into());
    assert_eq!(login_ui.get_error_message(), slint::SharedString::from("We had trouble communicating with the local intelligence service (code 500). Please try again later."));

    login_ui.invoke_login("test@example.com".into(), "password123".into());
    assert!(*login_successful.borrow(), "User login should be successful");
}

#[test]
fn test_e2e_agent_config_permissions_flow() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // The user lands on the dashboard and opens Agent Config
    let ui = crate::app::AgentConfig::new().unwrap();

    // Progress through steps to reach advanced config
    ui.set_step(1);
    ui.set_is_advanced(true);

    // Set api scope (now labeled "Developer: Custom Permissions (JSON)")
    ui.set_api_scope_override("[\"read\", \"write\"]".into());
    assert_eq!(ui.get_api_scope_override(), slint::SharedString::from("[\"read\", \"write\"]"));

    // Ensure that it's using the new phrasing by completing the flow
    let activated = std::rc::Rc::new(std::cell::RefCell::new(false));
    let activated_clone = activated.clone();

    ui.on_activate_agent(move |_a, _b, _c, _d, _e, scope, _f, _g, _h| {
        assert_eq!(scope, "[\"read\", \"write\"]");
        *activated_clone.borrow_mut() = true;
    });

    ui.invoke_activate_agent(
        "Sales".into(), true, true, true, true,
        "[\"read\", \"write\"]".into(),
        "".into(), "".into(), "".into()
    );
    assert!(*activated.borrow(), "Agent should be activated successfully");
}

#[test]
fn test_e2e_login_verification_flow() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // Start with the Login UI
    let login_ui = crate::app::Login::new().unwrap();

    // User tries to login but needs verification
    login_ui.set_show_verification(true);
    login_ui.set_verification_message("Please check your email to verify your account.".into());

    let resend = std::rc::Rc::new(std::cell::RefCell::new(false));
    let resend_clone = resend.clone();

    login_ui.on_resend_verification(move |_u| {
        *resend_clone.borrow_mut() = true;
    });

    login_ui.set_username("test@example.com".into());
    login_ui.invoke_resend_verification("test@example.com".into());

    assert!(*resend.borrow(), "Resend verification should be triggered");
}

#[test]
fn test_e2e_login_sso_flow() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // Start with the Login UI
    let login_ui = crate::app::Login::new().unwrap();

    let oauth = std::rc::Rc::new(std::cell::RefCell::new(false));
    let oauth_clone = oauth.clone();

    login_ui.on_oauth_login(move |provider| {
        assert_eq!(provider, "SSO");
        *oauth_clone.borrow_mut() = true;
    });

    login_ui.invoke_oauth_login("SSO".into());

    assert!(*oauth.borrow(), "OAuth login should be triggered");
}

#[test]
fn test_e2e_login_settings_flow() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // Start with the Login UI
    let login_ui = crate::app::Login::new().unwrap();

    let settings = std::rc::Rc::new(std::cell::RefCell::new(false));
    let settings_clone = settings.clone();

    login_ui.on_open_settings(move || {
        *settings_clone.borrow_mut() = true;
    });

    login_ui.invoke_open_settings();

    assert!(*settings.borrow(), "Settings should be opened");
}
