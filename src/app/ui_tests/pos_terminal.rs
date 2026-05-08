use crate::app;
use slint::ComponentHandle;

#[test]
fn test_pos_terminal_initial_state() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = app::PosTerminal::new().unwrap();
    assert_eq!(ui.get_amount(), "0.00");
    assert_eq!(ui.get_status(), "idle");
}

#[test]
fn test_pos_terminal_add_digit() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = app::PosTerminal::new().unwrap();

    // We can't directly invoke the inner Rust closure because it's wired in main.rs,
    // but we can test the UI property setters directly to ensure Slint interface is correct.
    // Or we can mock the callback if we want.
    let amount_updated = std::rc::Rc::new(std::cell::RefCell::new(false));
    let amount_updated_clone = amount_updated.clone();

    ui.on_add_digit(move |_d| {
        *amount_updated_clone.borrow_mut() = true;
    });

    ui.invoke_add_digit("5".into());
    assert!(*amount_updated.borrow());
}

#[test]
fn test_pos_terminal_clear_amount() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = app::PosTerminal::new().unwrap();

    let clear_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let clear_called_clone = clear_called.clone();

    ui.on_clear_amount(move || {
        *clear_called_clone.borrow_mut() = true;
    });

    ui.invoke_clear_amount();
    assert!(*clear_called.borrow());
}

#[test]
fn test_pos_terminal_charge_callback() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = app::PosTerminal::new().unwrap();

    let charge_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let charge_called_clone = charge_called.clone();

    ui.on_charge(move || {
        *charge_called_clone.borrow_mut() = true;
    });

    ui.invoke_charge();
    assert!(*charge_called.borrow());
}

#[test]
fn test_pos_terminal_status_update() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = app::PosTerminal::new().unwrap();

    ui.set_status("connecting".into());
    assert_eq!(ui.get_status(), "connecting");

    ui.set_status("processing".into());
    assert_eq!(ui.get_status(), "processing");

    ui.set_status("approved".into());
    assert_eq!(ui.get_status(), "approved");
}
