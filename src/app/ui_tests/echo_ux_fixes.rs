use slint::ComponentHandle;
use slint::SharedString;
use std::rc::Rc;
use std::cell::RefCell;
use crate::app;

fn simulate_full_flow() -> (app::Login, app::Dashboard, app::BusinessManager) {
    let login = app::Login::new().unwrap();
    let login_clicked = Rc::new(RefCell::new(false));
    let login_clicked_clone = login_clicked.clone();

    login.on_login(move |_u, _p| {
        *login_clicked_clone.borrow_mut() = true;
    });

    login.set_error_message("We couldn't sign you in. Please check your email and password and try again.".into());
    login.set_username("test@example.com".into());
    login.set_password("pass".into());
    login.invoke_login(login.get_username(), login.get_password());

    assert!(*login_clicked.borrow(), "Login button should be clickable");

    let dashboard = app::Dashboard::new().unwrap();
    let add_product_clicked = Rc::new(RefCell::new(false));
    let add_product_clicked_clone = add_product_clicked.clone();

    dashboard.on_action_add_product(move || {
        *add_product_clicked_clone.borrow_mut() = true;
    });

    dashboard.invoke_action_add_product();
    assert!(*add_product_clicked.borrow(), "Add Product action should be triggered");

    let biz_manager = app::BusinessManager::new().unwrap();
    (login, dashboard, biz_manager)
}

#[test]
fn e2e_flow_ux_fixes() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // Verify error message formatting logic
    let login = app::Login::new().unwrap();
    login.set_error_message("We couldn't sign you in. Please check your email and password and try again.".into());
    assert_eq!(login.get_error_message(), slint::SharedString::from("We couldn't sign you in. Please check your email and password and try again."));

    let dashboard = app::Dashboard::new().unwrap();
    dashboard.set_telemetry_cache_hits("95%".into());
    dashboard.set_telemetry_rag_latency("100ms".into());
    assert_eq!(dashboard.get_telemetry_cache_hits(), slint::SharedString::from("95%"));

    let (_login, _dashboard, biz_manager) = simulate_full_flow();

    let submit_clicked = Rc::new(RefCell::new(false));
    let submit_clicked_clone = submit_clicked.clone();

    biz_manager.on_submit(move |_t, _n, _d, _p, _dur, _sch| {
        *submit_clicked_clone.borrow_mut() = true;
    });

    biz_manager.invoke_action_add_new();
    assert_eq!(biz_manager.get_current_view(), "add");
    assert_eq!(biz_manager.get_step(), 0);

    assert_eq!(biz_manager.get_show_offering_hint(), false);
    biz_manager.set_show_offering_hint(true);
    assert_eq!(biz_manager.get_show_offering_hint(), true);

    biz_manager.invoke_select_type("PHYSICAL".into());
    biz_manager.invoke_next_step();

    assert_eq!(biz_manager.get_step(), 1);

    biz_manager.set_product_name("Custom Cake".into());
    biz_manager.set_product_price("20.00".into());

    biz_manager.invoke_submit(
        biz_manager.get_selected_type(),
        biz_manager.get_product_name(),
        biz_manager.get_product_description(),
        biz_manager.get_product_price(),
        biz_manager.get_service_duration(),
        biz_manager.get_service_schedule(),
    );

    assert!(*submit_clicked.borrow(), "Submit should be called from the completed UX flow");
}

#[test]
fn e2e_flow_ux_fixes_digital_product() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let (_login, _dashboard, biz_manager) = simulate_full_flow();

    let submit_clicked = Rc::new(RefCell::new(false));
    let submit_clicked_clone = submit_clicked.clone();

    biz_manager.on_submit(move |_t, _n, _d, _p, _dur, _sch| {
        *submit_clicked_clone.borrow_mut() = true;
    });

    biz_manager.invoke_action_add_new();
    biz_manager.invoke_select_type("DIGITAL".into());
    biz_manager.invoke_next_step();

    assert_eq!(biz_manager.get_step(), 1);

    biz_manager.set_product_name("Digital Book".into());
    biz_manager.set_product_price("15.00".into());

    biz_manager.invoke_submit(
        biz_manager.get_selected_type(),
        biz_manager.get_product_name(),
        biz_manager.get_product_description(),
        biz_manager.get_product_price(),
        biz_manager.get_service_duration(),
        biz_manager.get_service_schedule(),
    );

    assert!(*submit_clicked.borrow(), "Submit should be called from the completed UX flow for digital product");
}

#[test]
fn e2e_flow_ux_fixes_service_product() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let (_login, _dashboard, biz_manager) = simulate_full_flow();

    let submit_clicked = Rc::new(RefCell::new(false));
    let submit_clicked_clone = submit_clicked.clone();

    biz_manager.on_submit(move |_t, _n, _d, _p, _dur, _sch| {
        *submit_clicked_clone.borrow_mut() = true;
    });

    biz_manager.invoke_action_add_new();
    biz_manager.invoke_select_type("SERVICE".into());
    biz_manager.invoke_next_step();

    assert_eq!(biz_manager.get_step(), 1);

    biz_manager.set_product_name("Consultation".into());
    biz_manager.set_product_price("100.00".into());
    biz_manager.set_service_duration("30".into());

    biz_manager.invoke_submit(
        biz_manager.get_selected_type(),
        biz_manager.get_product_name(),
        biz_manager.get_product_description(),
        biz_manager.get_product_price(),
        biz_manager.get_service_duration(),
        biz_manager.get_service_schedule(),
    );

    assert!(*submit_clicked.borrow(), "Submit should be called from the completed UX flow for service product");
}

#[test]
fn e2e_flow_ux_fixes_back_navigation() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let (_login, _dashboard, biz_manager) = simulate_full_flow();

    biz_manager.invoke_action_add_new();
    biz_manager.invoke_select_type("SERVICE".into());
    biz_manager.invoke_next_step();
    assert_eq!(biz_manager.get_step(), 1);

    biz_manager.invoke_prev_step();
    assert_eq!(biz_manager.get_step(), 0);
    assert_eq!(biz_manager.get_selected_type(), slint::SharedString::from("SERVICE"));
}

#[test]
fn e2e_flow_ux_fixes_close_navigation() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let (_login, _dashboard, biz_manager) = simulate_full_flow();

    let close_clicked = Rc::new(RefCell::new(false));
    let close_clicked_clone = close_clicked.clone();

    biz_manager.on_close(move || {
        *close_clicked_clone.borrow_mut() = true;
    });

    biz_manager.invoke_action_add_new();
    biz_manager.invoke_select_type("SERVICE".into());
    biz_manager.invoke_next_step();
    assert_eq!(biz_manager.get_step(), 1);

    biz_manager.invoke_close();
    assert!(*close_clicked.borrow(), "Close should be called");
}
