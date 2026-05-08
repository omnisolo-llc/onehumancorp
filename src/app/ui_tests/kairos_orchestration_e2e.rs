use slint::ComponentHandle;
use std::rc::Rc;
use std::cell::RefCell;

#[test]
fn test_kairos_orchestration_cuj_step_1() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // 1. Start from Login
    let login_ui = crate::app::Login::new().unwrap();
    let login_successful = Rc::new(RefCell::new(false));
    let login_successful_clone = login_successful.clone();

    login_ui.on_login(move |email, password| {
        assert_eq!(email, "cuj@example.com");
        assert_eq!(password, "securepassword");
        *login_successful_clone.borrow_mut() = true;
    });

    login_ui.invoke_login("cuj@example.com".into(), "securepassword".into());
    assert!(*login_successful.borrow(), "Login should succeed");

    // 2. Dashboard
    let dashboard_ui = crate::app::Dashboard::new().unwrap();
    let kairos_opened = Rc::new(RefCell::new(false));
    let kairos_opened_clone = kairos_opened.clone();

    dashboard_ui.on_action_open_kairos_orchestration(move || {
        *kairos_opened_clone.borrow_mut() = true;
    });

    dashboard_ui.invoke_action_open_kairos_orchestration();
    assert!(*kairos_opened.borrow(), "Kairos Orchestration walkthrough should open");

    // 3. Open Walkthrough & Assert first step
    let ui = crate::app::KairosOrchestrationWalkthrough::new().unwrap();
    assert_eq!(ui.get_current_step(), 0, "Should start at step 0");
    assert_eq!(ui.get_test_title(), slint::SharedString::from("How Your Helpers Work Together"));
}

#[test]
fn test_kairos_orchestration_cuj_step_2() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login_ui = crate::app::Login::new().unwrap();
    let login_successful = Rc::new(RefCell::new(false));
    let login_successful_clone = login_successful.clone();
    login_ui.on_login(move |_email, _password| {
        *login_successful_clone.borrow_mut() = true;
    });
    login_ui.invoke_login("user".into(), "pass".into());
    assert!(*login_successful.borrow());

    let dashboard_ui = crate::app::Dashboard::new().unwrap();
    let kairos_opened = Rc::new(RefCell::new(false));
    let kairos_opened_clone = kairos_opened.clone();
    dashboard_ui.on_action_open_kairos_orchestration(move || {
        *kairos_opened_clone.borrow_mut() = true;
    });
    dashboard_ui.invoke_action_open_kairos_orchestration();
    assert!(*kairos_opened.borrow());

    let ui = crate::app::KairosOrchestrationWalkthrough::new().unwrap();
    ui.set_current_step(1); // Advance to step 1
    assert_eq!(ui.get_current_step(), 1, "Should be on step 1");
}

#[test]
fn test_kairos_orchestration_cuj_step_3() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login_ui = crate::app::Login::new().unwrap();
    let login_successful = Rc::new(RefCell::new(false));
    let login_successful_clone = login_successful.clone();
    login_ui.on_login(move |_email, _password| {
        *login_successful_clone.borrow_mut() = true;
    });
    login_ui.invoke_login("user".into(), "pass".into());
    assert!(*login_successful.borrow());

    let dashboard_ui = crate::app::Dashboard::new().unwrap();
    let kairos_opened = Rc::new(RefCell::new(false));
    let kairos_opened_clone = kairos_opened.clone();
    dashboard_ui.on_action_open_kairos_orchestration(move || {
        *kairos_opened_clone.borrow_mut() = true;
    });
    dashboard_ui.invoke_action_open_kairos_orchestration();
    assert!(*kairos_opened.borrow());

    let ui = crate::app::KairosOrchestrationWalkthrough::new().unwrap();
    ui.set_current_step(2); // Advance to step 2
    assert_eq!(ui.get_current_step(), 2, "Should be on step 2");
}

#[test]
fn test_kairos_orchestration_cuj_step_4_done() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login_ui = crate::app::Login::new().unwrap();
    let login_successful = Rc::new(RefCell::new(false));
    let login_successful_clone = login_successful.clone();
    login_ui.on_login(move |_email, _password| {
        *login_successful_clone.borrow_mut() = true;
    });
    login_ui.invoke_login("user".into(), "pass".into());
    assert!(*login_successful.borrow());

    let dashboard_ui = crate::app::Dashboard::new().unwrap();
    let kairos_opened = Rc::new(RefCell::new(false));
    let kairos_opened_clone = kairos_opened.clone();
    dashboard_ui.on_action_open_kairos_orchestration(move || {
        *kairos_opened_clone.borrow_mut() = true;
    });
    dashboard_ui.invoke_action_open_kairos_orchestration();
    assert!(*kairos_opened.borrow());

    let ui = crate::app::KairosOrchestrationWalkthrough::new().unwrap();
    ui.set_current_step(3); // Advance to final step 3
    assert_eq!(ui.get_current_step(), 3, "Should be on final step 3");
}

#[test]
fn test_kairos_orchestration_cuj_back_navigation() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login_ui = crate::app::Login::new().unwrap();
    let login_successful = Rc::new(RefCell::new(false));
    let login_successful_clone = login_successful.clone();
    login_ui.on_login(move |_email, _password| {
        *login_successful_clone.borrow_mut() = true;
    });
    login_ui.invoke_login("user".into(), "pass".into());
    assert!(*login_successful.borrow());

    let dashboard_ui = crate::app::Dashboard::new().unwrap();
    let kairos_opened = Rc::new(RefCell::new(false));
    let kairos_opened_clone = kairos_opened.clone();
    dashboard_ui.on_action_open_kairos_orchestration(move || {
        *kairos_opened_clone.borrow_mut() = true;
    });
    dashboard_ui.invoke_action_open_kairos_orchestration();
    assert!(*kairos_opened.borrow());

    let ui = crate::app::KairosOrchestrationWalkthrough::new().unwrap();
    ui.set_current_step(2);
    assert_eq!(ui.get_current_step(), 2);
    // Move backwards
    ui.set_current_step(1);
    assert_eq!(ui.get_current_step(), 1);
}
