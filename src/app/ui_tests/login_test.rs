use crate::app;
use slint::ComponentHandle;

fn create() -> app::Login { crate::ui_tests::init(); app::Login::new().unwrap() }

#[test]
fn test_login_open_settings_callback() {
    let ui = create();
    let counter = std::rc::Rc::new(std::cell::RefCell::new(0));
    let c = counter.clone();
    ui.on_open_settings(move || { *c.borrow_mut() += 1; });
    ui.invoke_open_settings();
    assert_eq!(*counter.borrow(), 1);
}

#[test]
fn test_login_start_setup_wizard_callback() {
    let ui = create();
    let counter = std::rc::Rc::new(std::cell::RefCell::new(0));
    let c = counter.clone();
    ui.on_start_setup_wizard(move || { *c.borrow_mut() += 1; });
    ui.invoke_start_setup_wizard();
    assert_eq!(*counter.borrow(), 1);
}

#[test]
fn test_login_resend_verification_callback() {
    let ui = create();
    let counter = std::rc::Rc::new(std::cell::RefCell::new(0));
    let c = counter.clone();
    ui.on_resend_verification(move |_| { *c.borrow_mut() += 1; });
    ui.invoke_resend_verification("".into());
    assert_eq!(*counter.borrow(), 1);
}

#[test]
fn test_login_oauth_login_callback() {
    let ui = create();
    let counter = std::rc::Rc::new(std::cell::RefCell::new(0));
    let c = counter.clone();
    ui.on_oauth_login(move |_| { *c.borrow_mut() += 1; });
    ui.invoke_oauth_login("SSO".into());
    assert_eq!(*counter.borrow(), 1);
}

#[test]
fn test_login_set_loading() {
    let ui = create();
    ui.set_loading(true);
    assert_eq!(ui.get_loading(), true);
    ui.set_loading(false);
    assert_eq!(ui.get_loading(), false);
}
