use crate::app;
use std::rc::Rc;
use std::cell::RefCell;

fn create_agents_ui() -> app::Agents {
    crate::ui_tests::init();
    app::Agents::new().unwrap()
}

fn create_department_settings_ui() -> app::DepartmentSettings {
    crate::ui_tests::init();
    app::DepartmentSettings::new().unwrap()
}

// 1. E2E Test: Navigate to Team screen from Dashboard
#[test]
fn test_e2e_navigate_to_team_dashboard() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let dashboard_ui = app::Dashboard::new().unwrap();
    let agents_ui_opened = Rc::new(RefCell::new(false));
    let agents_ui_opened_clone = agents_ui_opened.clone();

    dashboard_ui.on_action_manage_my_ai_team(move || {
        *agents_ui_opened_clone.borrow_mut() = true;
    });

    dashboard_ui.invoke_action_manage_my_ai_team();
    assert!(*agents_ui_opened.borrow(), "Should open Team dashboard when 'Manage My AI Team' is clicked.");
}

// 2. E2E Test: Tap department opens Department Settings
#[test]
fn test_e2e_tap_department_opens_settings() {
    let ui = create_agents_ui();
    let opened_department = Rc::new(RefCell::new(String::new()));
    let opened_department_clone = opened_department.clone();

    ui.on_open_department(move |dept_id| {
        *opened_department_clone.borrow_mut() = dept_id.into();
    });

    ui.invoke_open_department("ambassador".into());
    assert_eq!(*opened_department.borrow(), "ambassador", "Tapping department card should invoke open_department callback with correct ID.");
}

// 3. E2E Test: Toggle department ON/OFF
#[test]
fn test_e2e_toggle_department_status() {
    let ui = create_department_settings_ui();

    // Initial state
    assert_eq!(ui.get_is_active(), false);

    // User toggles ON
    ui.set_is_active(true);
    assert_eq!(ui.get_is_active(), true);

    // User toggles OFF
    ui.set_is_active(false);
    assert_eq!(ui.get_is_active(), false);
}

// 4. E2E Test: Update Mandate and Toggle Draft Mode
#[test]
fn test_e2e_update_mandate_and_draft_mode() {
    let ui = create_department_settings_ui();

    // Initial state
    assert_eq!(ui.get_draft_for_review(), true);

    // User updates mandate
    let new_mandate = "Be extremely enthusiastic and use emojis! 🎉";
    ui.set_mandate(new_mandate.into());
    assert_eq!(ui.get_mandate(), new_mandate);

    // User switches to auto-execute
    ui.set_draft_for_review(false);
    assert_eq!(ui.get_draft_for_review(), false);
}

// 5. E2E Test: Save Settings triggers callback
#[test]
fn test_e2e_save_department_settings() {
    let ui = create_department_settings_ui();
    let save_triggered = Rc::new(RefCell::new(false));
    let save_triggered_clone = save_triggered.clone();

    ui.on_save_settings(move || {
        *save_triggered_clone.borrow_mut() = true;
    });

    ui.invoke_save_settings();
    assert!(*save_triggered.borrow(), "Clicking 'Save Settings' should trigger the save callback.");
}

// 6. E2E Test: Close Settings triggers callback
#[test]
fn test_e2e_close_department_settings() {
    let ui = create_department_settings_ui();
    let close_triggered = Rc::new(RefCell::new(false));
    let close_triggered_clone = close_triggered.clone();

    ui.on_close_settings(move || {
        *close_triggered_clone.borrow_mut() = true;
    });

    ui.invoke_close_settings();
    assert!(*close_triggered.borrow(), "Clicking 'Close' should trigger the close callback.");
}
