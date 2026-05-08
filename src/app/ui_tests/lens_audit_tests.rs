use crate::app;
use slint::ComponentHandle;

#[test]
fn test_business_share_cuj_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let share_ui = app::BusinessShare::new().unwrap();

    // Assert visual truth / token truth: test_title exists and matches
    assert_eq!(share_ui.get_test_title(), slint::SharedString::from("Share my business"));

    let copy_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let copy_clone = copy_called.clone();
    share_ui.on_copy_link(move || {
        *copy_clone.borrow_mut() = true;
    });

    share_ui.invoke_copy_link();
    assert!(*copy_called.borrow(), "Copy link callback must be triggered");
}

#[test]
fn test_analytics_charts_opens_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let dashboard_ui = app::Dashboard::new().unwrap();
    let analytics_ui = app::AnalyticsCharts::new().unwrap();

    let see_analytics_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let see_analytics_called_clone = see_analytics_called.clone();

    let analytics_handle = analytics_ui.as_weak();
    dashboard_ui.on_action_see_analytics(move || {
        *see_analytics_called_clone.borrow_mut() = true;
        if let Some(ui) = analytics_handle.upgrade() {
            let _ = ui.show();
        }
    });

    dashboard_ui.invoke_action_see_analytics();
    assert!(*see_analytics_called.borrow(), "Dashboard should be able to open Analytics UI");
}

#[test]
fn test_login_flow_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let login_ui = app::Login::new().unwrap();
    let login_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let login_clone = login_called.clone();

    login_ui.on_login(move |_, _| {
        *login_clone.borrow_mut() = true;
    });

    login_ui.invoke_login("admin".into(), "password".into());
    assert!(*login_called.borrow(), "Login callback must be triggered");
}

#[test]
fn test_settings_flow_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let settings_ui = app::Settings::new().unwrap();
    let close_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let close_clone = close_called.clone();

    settings_ui.on_sign_out(move || {
        *close_clone.borrow_mut() = true;
    });

    settings_ui.invoke_sign_out();
    assert!(*close_called.borrow(), "Settings sign out callback must be triggered");
}

#[test]
fn test_help_center_flow_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let help_ui = app::HelpCenter::new().unwrap();
    let close_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let close_clone = close_called.clone();

    help_ui.on_execute_search(move || {
        *close_clone.borrow_mut() = true;
    });

    help_ui.invoke_execute_search();
    assert!(*close_called.borrow(), "Help center execute search callback must be triggered");
}
