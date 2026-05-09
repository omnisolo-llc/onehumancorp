use crate::app;

#[test]
fn test_e2e_wizard_agent_config_full_journey() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let login_ui = app::Login::new().unwrap();
    let login_successful = std::rc::Rc::new(std::cell::RefCell::new(false));
    let login_successful_clone = login_successful.clone();

    login_ui.on_login(move |email, password| {
        assert_eq!(email, "test@example.com");
        assert_eq!(password, "password123");
        *login_successful_clone.borrow_mut() = true;
    });

    login_ui.invoke_login("test@example.com".into(), "password123".into());
    assert!(*login_successful.borrow(), "User login should be successful");

    let wizard_ui = app::AgentConfig::new().unwrap();
    assert_eq!(wizard_ui.get_step(), 0);

    let activate_agent_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let activate_agent_called_clone = activate_agent_called.clone();

    wizard_ui.on_activate_agent(move |name, _reply, _social, _desc, _updates, _freq, _api_scope, _cron, _raw| {
        assert_eq!(name, "Customer Support");
        *activate_agent_called_clone.borrow_mut() = true;
    });

    // Step 0: Agent selection
    wizard_ui.set_selected_agent("Customer Support".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 1);

    // Step 1: Capability selection
    wizard_ui.set_can_reply(true);
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 2);

    // Step 2: Frequency selection
    wizard_ui.set_frequency_value(2.0);
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 3);

    // Step 3: Apply Fix ✓
    wizard_ui.invoke_activate_agent("Customer Support".into(), true, false, false, false, "Daily".into(), "".into(), "".into(), "".into());
    assert!(*activate_agent_called.borrow(), "Activate Agent should be clicked and trigger activate_agent callback");
}
