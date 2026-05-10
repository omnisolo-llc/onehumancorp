use crate::app;

use std::rc::Rc;
use std::cell::RefCell;

#[test]
fn test_dashboard_today_sales_label() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // Simulate flow from Login to Dashboard
    let login = app::Login::new().unwrap();
    let login_clicked = Rc::new(RefCell::new(false));
    let login_clicked_clone = login_clicked.clone();

    login.on_login(move |_, _| {
        *login_clicked_clone.borrow_mut() = true;
    });

    login.set_username("test@example.com".into());
    login.set_password("pass".into());
    login.invoke_login(login.get_username(), login.get_password());

    assert!(*login_clicked.borrow(), "Login button should be clickable");

    // Load Dashboard
    let dashboard = app::Dashboard::new().unwrap();
    dashboard.set_todays_sales("$500.00".into());

    assert_eq!(dashboard.get_todays_sales(), slint::SharedString::from("$500.00"));
}

#[test]
fn test_dashboard_orders() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login = app::Login::new().unwrap();
    login.set_username("a@b.com".into());
    login.set_password("p".into());
    login.invoke_login(login.get_username(), login.get_password());

    let dashboard = app::Dashboard::new().unwrap();
    dashboard.set_new_orders_count(10);
    assert_eq!(dashboard.get_new_orders_count(), 10);
}

#[test]
fn test_dashboard_helpers() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login = app::Login::new().unwrap();
    login.set_username("a@b.com".into());
    login.set_password("p".into());
    login.invoke_login(login.get_username(), login.get_password());

    let dashboard = app::Dashboard::new().unwrap();
    dashboard.set_active_helpers_count(5);
    assert_eq!(dashboard.get_active_helpers_count(), 5);
}

#[test]
fn test_dashboard_tasks() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login = app::Login::new().unwrap();
    login.set_username("a@b.com".into());
    login.set_password("p".into());
    login.invoke_login(login.get_username(), login.get_password());

    let dashboard = app::Dashboard::new().unwrap();
    dashboard.set_tasks_in_progress_count(2);
    assert_eq!(dashboard.get_tasks_in_progress_count(), 2);
}

#[test]
fn test_dashboard_generative_score() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login = app::Login::new().unwrap();
    login.set_username("a@b.com".into());
    login.set_password("p".into());
    login.invoke_login(login.get_username(), login.get_password());

    let dashboard = app::Dashboard::new().unwrap();
    dashboard.set_generative_score("90".into());
    assert_eq!(dashboard.get_generative_score(), slint::SharedString::from("90"));
}
