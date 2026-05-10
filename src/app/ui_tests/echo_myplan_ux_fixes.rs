use slint::ComponentHandle;
use std::rc::Rc;
use std::cell::RefCell;
use crate::app;

#[test]
fn test_myplan_ux_flow_login_to_dashboard() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login = app::Login::new().unwrap();
    login.set_username("test@example.com".into());
    login.set_password("pass".into());
    let login_clicked = Rc::new(RefCell::new(false));
    let login_clicked_clone = login_clicked.clone();
    login.on_login(move |_, _| { *login_clicked_clone.borrow_mut() = true; });
    login.invoke_login(login.get_username(), login.get_password());
    assert!(*login_clicked.borrow());
}

#[test]
fn test_myplan_ux_flow_dashboard_to_myplan() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let dashboard = app::Dashboard::new().unwrap();
    let myplan_clicked = Rc::new(RefCell::new(false));
    let myplan_clicked_clone = myplan_clicked.clone();
    dashboard.on_open_my_plan(move || { *myplan_clicked_clone.borrow_mut() = true; });
    dashboard.invoke_open_my_plan();
    assert!(*myplan_clicked.borrow());
}

#[test]
fn test_myplan_ux_hint_toggle() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let ui = app::MyPlan::new().unwrap();
    assert_eq!(ui.get_show_hint(), false);
    ui.set_show_hint(true);
    assert_eq!(ui.get_show_hint(), true);
}

#[test]
fn test_myplan_ux_hint_toggle_off() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let ui = app::MyPlan::new().unwrap();
    ui.set_show_hint(true);
    ui.set_show_hint(false);
    assert_eq!(ui.get_show_hint(), false);
}

#[test]
fn test_myplan_ux_flow_full_journey() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login = app::Login::new().unwrap();
    let login_clicked = Rc::new(RefCell::new(false));
    let login_clicked_clone = login_clicked.clone();
    login.on_login(move |_, _| { *login_clicked_clone.borrow_mut() = true; });
    login.invoke_login(login.get_username(), login.get_password());
    assert!(*login_clicked.borrow());

    let dashboard = app::Dashboard::new().unwrap();
    let myplan_clicked = Rc::new(RefCell::new(false));
    let myplan_clicked_clone = myplan_clicked.clone();
    dashboard.on_open_my_plan(move || { *myplan_clicked_clone.borrow_mut() = true; });
    dashboard.invoke_open_my_plan();
    assert!(*myplan_clicked.borrow());

    let ui = app::MyPlan::new().unwrap();
    ui.set_show_hint(true);
    assert_eq!(ui.get_show_hint(), true);
}
