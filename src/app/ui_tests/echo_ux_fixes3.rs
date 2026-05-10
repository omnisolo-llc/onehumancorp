use slint::ComponentHandle;
use crate::app;
use std::rc::Rc;
use std::cell::RefCell;

#[test]
fn test_echo_ux_env_wizard_step_0() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login = app::Login::new().unwrap();
    let setup_wizard_launched = Rc::new(RefCell::new(false));
    let setup_wizard_launched_clone = setup_wizard_launched.clone();

    login.on_start_setup_wizard(move || {
        *setup_wizard_launched_clone.borrow_mut() = true;
    });

    login.invoke_start_setup_wizard();
    assert!(*setup_wizard_launched.borrow(), "Should navigate away from login");

    let env_ui = app::EnvWizard::new().unwrap();
    assert_eq!(env_ui.get_step(), 0);
    env_ui.set_is_advanced(true);
    assert_eq!(env_ui.get_is_advanced(), true);
}

#[test]
fn test_echo_ux_env_wizard_step_1() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login = app::Login::new().unwrap();
    let login_clicked = Rc::new(RefCell::new(false));
    let login_clicked_clone = login_clicked.clone();

    login.on_login(move |_, _| {
        *login_clicked_clone.borrow_mut() = true;
    });

    login.set_username("test@example.com".into());
    login.set_password("pass".into());
    login.invoke_login(login.get_username(), login.get_password());

    assert!(*login_clicked.borrow(), "Should click login");

    let env_ui = app::EnvWizard::new().unwrap();

    let step_advanced = Rc::new(RefCell::new(false));
    let step_advanced_clone = step_advanced.clone();

    env_ui.on_next_step(move || {
        *step_advanced_clone.borrow_mut() = true;
    });

    env_ui.invoke_next_step();
    assert!(*step_advanced.borrow(), "Step should advance");

    env_ui.set_step(1);
    assert_eq!(env_ui.get_step(), 1);

    env_ui.set_multitenant(true);
    assert_eq!(env_ui.get_multitenant(), true);
}

#[test]
fn test_echo_ux_env_wizard_step_2() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login = app::Login::new().unwrap();
    let setup_wizard_launched = Rc::new(RefCell::new(false));
    let setup_wizard_launched_clone = setup_wizard_launched.clone();

    login.on_start_setup_wizard(move || {
        *setup_wizard_launched_clone.borrow_mut() = true;
    });

    login.invoke_start_setup_wizard();
    assert!(*setup_wizard_launched.borrow(), "Should navigate away from login");

    let env_ui = app::EnvWizard::new().unwrap();
    env_ui.set_step(2);
    assert_eq!(env_ui.get_step(), 2);

    env_ui.set_openai_key("sk-test".into());
    assert_eq!(env_ui.get_openai_key(), "sk-test");
    env_ui.set_anthropic_key("sk-test-ant".into());
    assert_eq!(env_ui.get_anthropic_key(), "sk-test-ant");
}

#[test]
fn test_echo_ux_agent_config_step_0() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login = app::Login::new().unwrap();

    let login_clicked = Rc::new(RefCell::new(false));
    let login_clicked_clone = login_clicked.clone();

    login.on_login(move |_, _| {
        *login_clicked_clone.borrow_mut() = true;
    });

    login.invoke_login("test@test.com".into(), "pass".into());
    assert!(*login_clicked.borrow(), "Should click login");

    let agent_config = app::AgentConfig::new().unwrap();
    assert_eq!(agent_config.get_step(), 0);
    agent_config.set_selected_agent("SOFTWARE_ENGINEER".into());
    assert_eq!(agent_config.get_selected_agent(), "SOFTWARE_ENGINEER");
}

#[test]
fn test_echo_ux_agent_config_step_1() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login = app::Login::new().unwrap();
    let login_clicked = Rc::new(RefCell::new(false));
    let login_clicked_clone = login_clicked.clone();

    login.on_login(move |_, _| {
        *login_clicked_clone.borrow_mut() = true;
    });

    login.invoke_login("test@test.com".into(), "pass".into());
    assert!(*login_clicked.borrow(), "Should click login");

    let agent_config = app::AgentConfig::new().unwrap();

    let step_advanced = Rc::new(RefCell::new(false));
    let step_advanced_clone = step_advanced.clone();

    agent_config.on_next_step(move || {
        *step_advanced_clone.borrow_mut() = true;
    });

    agent_config.invoke_next_step();
    assert!(*step_advanced.borrow(), "Should proceed step");

    agent_config.set_step(1);
    assert_eq!(agent_config.get_step(), 1);

    agent_config.set_is_advanced(true);
    agent_config.set_api_scope_override("[\"read\"]".into());
    assert_eq!(agent_config.get_api_scope_override(), "[\"read\"]");
}
