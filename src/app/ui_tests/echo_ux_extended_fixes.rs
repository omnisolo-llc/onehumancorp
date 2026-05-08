use slint::ComponentHandle;
use std::rc::Rc;
use std::cell::RefCell;
use crate::app;

#[test]
fn e2e_login_to_business_manager_flow_1() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // 1. Start from Login
    let login = app::Login::new().unwrap();
    let login_clicked = Rc::new(RefCell::new(false));
    let login_clicked_clone = login_clicked.clone();

    login.on_login(move |u, p| {
        assert_eq!(u, "ceo@store.com");
        assert_eq!(p, "pass123");
        *login_clicked_clone.borrow_mut() = true;
    });

    login.set_username("ceo@store.com".into());
    login.set_password("pass123".into());
    login.invoke_login(login.get_username(), login.get_password());
    assert!(*login_clicked.borrow(), "Login should trigger");

    // 2. Navigate to Dashboard
    let dashboard = app::Dashboard::new().unwrap();
    let add_product_clicked = Rc::new(RefCell::new(false));
    let add_product_clicked_clone = add_product_clicked.clone();

    dashboard.on_action_add_product(move || {
        *add_product_clicked_clone.borrow_mut() = true;
    });

    dashboard.invoke_action_add_product();
    assert!(*add_product_clicked.borrow(), "Add product clicked");

    // 3. Open Business Manager and check hint functionality
    let biz_manager = app::BusinessManager::new().unwrap();
    biz_manager.invoke_action_add_new();

    assert_eq!(biz_manager.get_show_offering_hint(), false);
    biz_manager.set_show_offering_hint(true);
    assert_eq!(biz_manager.get_show_offering_hint(), true);
}

#[test]
fn e2e_login_to_business_manager_flow_2_physical() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login = app::Login::new().unwrap();
    login.invoke_login("user@test.com".into(), "testpass".into());

    let dashboard = app::Dashboard::new().unwrap();
    dashboard.invoke_action_add_product();

    let biz_manager = app::BusinessManager::new().unwrap();
    biz_manager.invoke_action_add_new();
    biz_manager.invoke_select_type("PHYSICAL".into());
    biz_manager.invoke_next_step();
    assert_eq!(biz_manager.get_step(), 1);
}

#[test]
fn e2e_login_to_business_manager_flow_3_digital() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login = app::Login::new().unwrap();
    login.invoke_login("user@test.com".into(), "testpass".into());

    let dashboard = app::Dashboard::new().unwrap();
    dashboard.invoke_action_add_product();

    let biz_manager = app::BusinessManager::new().unwrap();
    biz_manager.invoke_action_add_new();
    biz_manager.invoke_select_type("DIGITAL".into());
    biz_manager.invoke_next_step();
    assert_eq!(biz_manager.get_step(), 1);
}

#[test]
fn e2e_login_to_business_manager_flow_4_service() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login = app::Login::new().unwrap();
    login.invoke_login("user@test.com".into(), "testpass".into());

    let dashboard = app::Dashboard::new().unwrap();
    dashboard.invoke_action_add_product();

    let biz_manager = app::BusinessManager::new().unwrap();
    biz_manager.invoke_action_add_new();
    biz_manager.invoke_select_type("SERVICE".into());
    biz_manager.invoke_next_step();
    assert_eq!(biz_manager.get_step(), 1);
}

#[test]
fn e2e_login_to_business_manager_flow_5_complete() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login = app::Login::new().unwrap();
    login.invoke_login("ceo@store.com".into(), "pass".into());

    let dashboard = app::Dashboard::new().unwrap();
    dashboard.invoke_action_add_product();

    let biz_manager = app::BusinessManager::new().unwrap();
    let submit_clicked = Rc::new(RefCell::new(false));
    let submit_clicked_clone = submit_clicked.clone();

    biz_manager.on_submit(move |t, n, _d, p, _dur, _sch| {
        assert_eq!(t, "PHYSICAL");
        assert_eq!(n, "Cake");
        assert_eq!(p, "20.00");
        *submit_clicked_clone.borrow_mut() = true;
    });

    biz_manager.invoke_action_add_new();
    biz_manager.invoke_select_type("PHYSICAL".into());
    biz_manager.invoke_next_step();

    biz_manager.set_product_name("Cake".into());
    biz_manager.set_product_price("20.00".into());

    biz_manager.invoke_submit(
        biz_manager.get_selected_type(),
        biz_manager.get_product_name(),
        biz_manager.get_product_description(),
        biz_manager.get_product_price(),
        biz_manager.get_service_duration(),
        biz_manager.get_service_schedule(),
    );

    assert!(*submit_clicked.borrow(), "Submit completed");
}
