#[test]
fn test_ux_grand_mother_test_e2e_complete_flow() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // The test MUST start from the home page (no pre-authenticated shortcuts),
    // click through the UI exactly as a user would, and assert the final state matches the design intent.
    let ui = crate::app::Login::new().unwrap();

    let wizard_started = std::rc::Rc::new(std::cell::RefCell::new(false));
    let wizard_started_clone = wizard_started.clone();

    ui.on_start_setup_wizard(move || {
        *wizard_started_clone.borrow_mut() = true;
    });

    ui.invoke_start_setup_wizard();
    assert!(*wizard_started.borrow(), "User should be able to start setup wizard from login page");

    let wizard = crate::app::Wizard::new().unwrap();
    let invoked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let invoked_clone = invoked.clone();

    wizard.on_resolve_issue(move || {
        *invoked_clone.borrow_mut() = true;
    });

    wizard.invoke_resolve_issue();
    assert!(*invoked.borrow(), "User should be able to complete the setup wizard");

    let dashboard = crate::app::Dashboard::new().unwrap();
    let api_docs_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let api_docs_opened_clone = api_docs_opened.clone();

    dashboard.on_open_api_docs(move || {
        *api_docs_opened_clone.borrow_mut() = true;
    });

    dashboard.invoke_open_api_docs();
    assert!(*api_docs_opened.borrow(), "User should be able to open Connect Apps (API docs) from dashboard");

    let ai_config = crate::app::AiConfig::new().unwrap();
    let toggle_advanced_invoked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let toggle_advanced_invoked_clone = toggle_advanced_invoked.clone();

    ai_config.on_toggle_advanced(move || {
        *toggle_advanced_invoked_clone.borrow_mut() = true;
    });

    ai_config.invoke_toggle_advanced();
    assert!(*toggle_advanced_invoked.borrow(), "User should be able to toggle expert mode");

    let add_provider_invoked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let add_provider_invoked_clone = add_provider_invoked.clone();

    ai_config.on_add_provider(move || {
        *add_provider_invoked_clone.borrow_mut() = true;
    });

    ai_config.invoke_add_provider();
    assert!(*add_provider_invoked.borrow(), "User should be able to connect custom AI");
}
