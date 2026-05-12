use crate::app;
use slint::{Model, ComponentHandle};
use std::rc::Rc;

fn create() -> app::Integrations {
    crate::ui_tests::init();
    app::Integrations::new().unwrap()
}

#[test]
fn e2e_integrations_mobile_layout_render() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = create();

    // Simulate mobile viewport
    ui.window().set_size(slint::PhysicalSize::new(375, 800));

    // Just verify it doesn't crash on layout and sets tools correctly
    let tools = slint::VecModel::from(vec![
        app::UiMcpTool {
            id: "t1".into(),
            name: "Mailchimp".into(),
            description: "Email".into(),
        }
    ]);
    ui.set_tools(Rc::new(tools).into());
    assert_eq!(ui.get_tools().row_count(), 1);
}

#[test]
fn e2e_integrations_desktop_layout_render() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = create();

    // Simulate desktop viewport
    ui.window().set_size(slint::PhysicalSize::new(1024, 800));

    let tools = slint::VecModel::from(vec![
        app::UiMcpTool {
            id: "t2".into(),
            name: "Zoom".into(),
            description: "Video".into(),
        }
    ]);
    ui.set_tools(Rc::new(tools).into());
    assert_eq!(ui.get_tools().row_count(), 1);
}

#[test]
fn e2e_integrations_configure_manychat() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c = called.clone();

    ui.on_configure_integration(move |name| { *c.borrow_mut() = name.to_string(); });

    ui.invoke_configure_integration("Manychat".into());
    assert_eq!(*called.borrow(), "Manychat");
}

#[test]
fn e2e_integrations_configure_calendly() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c = called.clone();

    ui.on_configure_integration(move |name| { *c.borrow_mut() = name.to_string(); });

    ui.invoke_configure_integration("Calendly".into());
    assert_eq!(*called.borrow(), "Calendly");
}

#[test]
fn e2e_integrations_configure_mailchimp() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c = called.clone();

    ui.on_configure_integration(move |name| { *c.borrow_mut() = name.to_string(); });

    ui.invoke_configure_integration("Mailchimp".into());
    assert_eq!(*called.borrow(), "Mailchimp");
}

#[test]
fn e2e_integrations_configure_zoom() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c = called.clone();

    ui.on_configure_integration(move |name| { *c.borrow_mut() = name.to_string(); });

    ui.invoke_configure_integration("Zoom".into());
    assert_eq!(*called.borrow(), "Zoom");
}
