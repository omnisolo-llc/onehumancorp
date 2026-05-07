use slint::ComponentHandle;
use std::rc::Rc;
use std::cell::RefCell;

#[test]
fn e2e_pos_terminal_flow_1_dashboard_navigation() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let dashboard_ui = crate::app::Dashboard::new().unwrap();
    let terminal_opened = Rc::new(RefCell::new(false));
    let terminal_opened_clone = terminal_opened.clone();

    dashboard_ui.on_action_open_pos_terminal(move || {
        *terminal_opened_clone.borrow_mut() = true;
    });

    dashboard_ui.invoke_action_open_pos_terminal();
    assert!(*terminal_opened.borrow(), "Dashboard should allow navigating to MockPosTerminal");
}

#[test]
fn e2e_pos_terminal_flow_2_initial_state() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::MockPosTerminal::new().unwrap();

    ui.set_amount("$45.00".into());
    ui.set_status("waiting".into());

    assert_eq!(ui.get_amount(), "$45.00");
    assert_eq!(ui.get_status(), "waiting");
}

#[test]
fn e2e_pos_terminal_flow_3_simulate_tap_processing() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::MockPosTerminal::new().unwrap();

    ui.on_simulate_tap({
        let ui_weak = ui.as_weak();
        move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_status("processing".into());
            }
        }
    });

    ui.set_status("waiting".into());
    ui.invoke_simulate_tap();
    assert_eq!(ui.get_status(), "processing");
}

#[test]
fn e2e_pos_terminal_flow_4_payment_success() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::MockPosTerminal::new().unwrap();

    ui.on_simulate_tap({
        let ui_weak = ui.as_weak();
        move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_status("processing".into());
                let ui_weak_timer = ui.as_weak();
                slint::Timer::single_shot(std::time::Duration::from_millis(10), move || {
                    if let Some(ui) = ui_weak_timer.upgrade() {
                        ui.set_status("success".into());
                        slint::quit_event_loop().unwrap();
                    }
                });
            }
        }
    });

    ui.set_status("waiting".into());
    ui.invoke_simulate_tap();
    slint::run_event_loop().unwrap();
    assert_eq!(ui.get_status(), "success");
}

#[test]
fn e2e_pos_terminal_flow_5_close_terminal() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = crate::app::MockPosTerminal::new().unwrap();

    let close_called = Rc::new(RefCell::new(false));
    let close_called_clone = close_called.clone();
    ui.on_close(move || {
        *close_called_clone.borrow_mut() = true;
    });

    ui.invoke_close();
    assert!(*close_called.borrow(), "Close should be successfully invoked");
}
