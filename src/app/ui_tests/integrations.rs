use crate::app;
use slint::Model;
use std::rc::Rc;

fn create() -> app::Integrations { crate::ui_tests::init(); app::Integrations::new().unwrap() }

#[test] fn integr_flow_configure_jitsi() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c = called.clone();
    ui.on_configure_integration(move |name| { *c.borrow_mut() = name.to_string(); });
    ui.invoke_configure_integration("Jitsi".into());
    assert_eq!(*called.borrow(), "Jitsi");
}

#[test] fn integr_flow_configure_easypost() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c = called.clone();
    ui.on_configure_integration(move |name| { *c.borrow_mut() = name.to_string(); });
    ui.invoke_configure_integration("EasyPost".into());
    assert_eq!(*called.borrow(), "EasyPost");
}

#[test] fn integr_flow_configure_listmonk() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c = called.clone();
    ui.on_configure_integration(move |name| { *c.borrow_mut() = name.to_string(); });
    ui.invoke_configure_integration("Listmonk".into());
    assert_eq!(*called.borrow(), "Listmonk");
}

#[test] fn integr_flow_configure_ayrshare() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c = called.clone();
    ui.on_configure_integration(move |name| { *c.borrow_mut() = name.to_string(); });
    ui.invoke_configure_integration("Ayrshare".into());
    assert_eq!(*called.borrow(), "Ayrshare");
}
