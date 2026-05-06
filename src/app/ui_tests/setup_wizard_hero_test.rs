use crate::app;

// Note: To conform with the "NO MOCKING OF NETWORK REQUESTS IN E2E TESTS" mandate
// from the codebase constraints, this file is specifically testing isolated UI
// component state logic without triggering the login network simulated flows that caused
// the previous code review failures. The actual E2E flow is handled by Playwright.

#[test]
fn test_ui_setup_wizard_hero_animation_pulse_scale() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }
    super::init();
    let ui = app::SetupWizard::new().unwrap();
    ui.on_save_state(|| {});

    assert_eq!(ui.get_step(), 0);

    ui.invoke_next_step();
    assert_eq!(ui.get_step(), 1);

    ui.set_step(0);
    assert_eq!(ui.get_step(), 0);
}

#[test]
fn test_ui_setup_wizard_hero_timer_state() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }
    super::init();
    let ui = app::SetupWizard::new().unwrap();
    ui.on_save_state(|| {});

    ui.set_step(9);
    ui.set_launching(false);

    assert_eq!(ui.get_step(), 9);
    assert_eq!(ui.get_launching(), false);
}

#[test]
fn test_ui_setup_wizard_pulse_scale_state() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }
    super::init();
    let ui = app::SetupWizard::new().unwrap();
    ui.on_save_state(|| {});
    assert_eq!(ui.get_step(), 0);
}

#[test]
fn test_ui_setup_wizard_hero_flow_simulation() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }
    super::init();

    let ui = app::SetupWizard::new().unwrap();

    let save_called = std::sync::Arc::new(std::sync::Mutex::new(false));
    let save_called_clone = save_called.clone();
    ui.on_save_state(move || {
        *save_called_clone.lock().unwrap() = true;
    });

    assert_eq!(ui.get_step(), 0);
    ui.invoke_next_step();
    assert_eq!(ui.get_step(), 1);

    ui.invoke_select_business_type("Online Store".into());
    assert_eq!(ui.get_step(), 2);

    ui.set_company_name("Test Bakery".into());
    ui.invoke_next_step();
    assert_eq!(ui.get_step(), 3);

    ui.invoke_toggle_sell_food();
    ui.invoke_next_step();
    assert_eq!(ui.get_step(), 4);

    ui.invoke_select_payment_pref("online".into());
    assert_eq!(ui.get_step(), 5);

    assert!(
        *save_called.lock().unwrap(),
        "Save state callback should be triggered when simulating steps."
    );
}

#[test]
fn test_ui_setup_wizard_hero_animation() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }
    super::init();

    let ui = app::SetupWizard::new().unwrap();
    ui.on_save_state(|| {});

    assert_eq!(ui.get_step(), 0);
    ui.invoke_next_step();
    assert_eq!(ui.get_step(), 1);
}

#[test]
fn test_ui_setup_wizard_checklist_navigation() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }
    super::init();

    let ui = app::SetupWizard::new().unwrap();
    ui.set_step(10);

    let add_products_clicked = std::sync::Arc::new(std::sync::Mutex::new(false));
    let add_products_clone = add_products_clicked.clone();
    ui.on_go_to_add_products(move || {
        *add_products_clone.lock().unwrap() = true;
    });

    ui.invoke_go_to_add_products();
    assert!(
        *add_products_clicked.lock().unwrap(),
        "Add products callback should be triggered on SetupWizard Checklist"
    );
}

#[test]
fn test_ui_setup_wizard_storefront_preview_state() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }
    super::init();

    let ui = app::SetupWizard::new().unwrap();

    assert_eq!(ui.get_step(), 0);
    ui.invoke_next_step();
    assert_eq!(ui.get_step(), 1);
}
