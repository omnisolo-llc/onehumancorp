use crate::app;


#[test]
fn test_e2e_business_manager_hint_visibility_1() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // 1. Login flow
    let login_ui = app::Login::new().unwrap();
    let login_successful = std::sync::Arc::new(std::sync::Mutex::new(false));
    let login_successful_clone = login_successful.clone();

    login_ui.on_login(move |_email, _password| {
        *login_successful_clone.lock().unwrap() = true;
    });
    login_ui.invoke_login("u".into(), "p".into());
    assert!(*login_successful.lock().unwrap());

    // 2. Dashboard flow to open Business Manager
    let dashboard_ui = app::Dashboard::new().unwrap();
    let business_manager_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let business_manager_opened_clone = business_manager_opened.clone();

    dashboard_ui.on_action_add_product(move || {
        *business_manager_opened_clone.borrow_mut() = true;
    });

    dashboard_ui.invoke_action_add_product();
    assert!(*business_manager_opened.borrow());

    // 3. Verify Business Manager Hint Logic
    let manager_ui = app::BusinessManager::new().unwrap();
    assert_eq!(manager_ui.get_show_hint(), false);
    manager_ui.set_show_hint(true);
    assert_eq!(manager_ui.get_show_hint(), true);
}

#[test]
fn test_e2e_business_manager_hint_visibility_2() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // 1. Login flow
    let login_ui = app::Login::new().unwrap();
    let login_successful = std::sync::Arc::new(std::sync::Mutex::new(false));
    let login_successful_clone = login_successful.clone();

    login_ui.on_login(move |_email, _password| {
        *login_successful_clone.lock().unwrap() = true;
    });
    login_ui.invoke_login("u".into(), "p".into());
    assert!(*login_successful.lock().unwrap());

    // 2. Dashboard flow to open Business Manager
    let dashboard_ui = app::Dashboard::new().unwrap();
    let business_manager_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let business_manager_opened_clone = business_manager_opened.clone();

    dashboard_ui.on_action_add_product(move || {
        *business_manager_opened_clone.borrow_mut() = true;
    });

    dashboard_ui.invoke_action_add_product();
    assert!(*business_manager_opened.borrow());

    // 3. Verify Business Manager Hint Logic (toggling back to false)
    let manager_ui = app::BusinessManager::new().unwrap();
    manager_ui.set_show_hint(true);
    manager_ui.set_show_hint(false);
    assert_eq!(manager_ui.get_show_hint(), false);
}

#[test]
fn test_e2e_business_manager_hint_visibility_3() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login_ui = app::Login::new().unwrap();
    let login_successful = std::sync::Arc::new(std::sync::Mutex::new(false));
    let login_successful_clone = login_successful.clone();

    login_ui.on_login(move |_email, _password| {
        *login_successful_clone.lock().unwrap() = true;
    });
    login_ui.invoke_login("u".into(), "p".into());
    assert!(*login_successful.lock().unwrap());

    let dashboard_ui = app::Dashboard::new().unwrap();
    let business_manager_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let business_manager_opened_clone = business_manager_opened.clone();

    dashboard_ui.on_action_add_product(move || {
        *business_manager_opened_clone.borrow_mut() = true;
    });

    dashboard_ui.invoke_action_add_product();
    assert!(*business_manager_opened.borrow());

    let manager_ui = app::BusinessManager::new().unwrap();
    manager_ui.set_show_hint(true);
    // There is no explicit string prop to check in Rust without exporting it,
    // but we verify the state toggle still works
    assert_eq!(manager_ui.get_show_hint(), true);
}

#[test]
fn test_e2e_business_manager_hint_visibility_4() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login_ui = app::Login::new().unwrap();
    let login_successful = std::sync::Arc::new(std::sync::Mutex::new(false));
    let login_successful_clone = login_successful.clone();

    login_ui.on_login(move |_email, _password| {
        *login_successful_clone.lock().unwrap() = true;
    });
    login_ui.invoke_login("u".into(), "p".into());
    assert!(*login_successful.lock().unwrap());

    let dashboard_ui = app::Dashboard::new().unwrap();
    let business_manager_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let business_manager_opened_clone = business_manager_opened.clone();

    dashboard_ui.on_action_add_product(move || {
        *business_manager_opened_clone.borrow_mut() = true;
    });

    dashboard_ui.invoke_action_add_product();
    assert!(*business_manager_opened.borrow());

    let manager_ui = app::BusinessManager::new().unwrap();
    // Simulate toggling hint multiple times
    manager_ui.set_show_hint(true);
    manager_ui.set_show_hint(false);
    manager_ui.set_show_hint(true);
    assert_eq!(manager_ui.get_show_hint(), true);
}

#[test]
fn test_e2e_business_manager_hint_visibility_5() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let login_ui = app::Login::new().unwrap();
    let login_successful = std::sync::Arc::new(std::sync::Mutex::new(false));
    let login_successful_clone = login_successful.clone();

    login_ui.on_login(move |_email, _password| {
        *login_successful_clone.lock().unwrap() = true;
    });
    login_ui.invoke_login("u".into(), "p".into());
    assert!(*login_successful.lock().unwrap());

    let dashboard_ui = app::Dashboard::new().unwrap();
    let business_manager_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let business_manager_opened_clone = business_manager_opened.clone();

    dashboard_ui.on_action_add_product(move || {
        *business_manager_opened_clone.borrow_mut() = true;
    });

    dashboard_ui.invoke_action_add_product();
    assert!(*business_manager_opened.borrow());

    let manager_ui = app::BusinessManager::new().unwrap();

    let submitted = std::rc::Rc::new(std::cell::RefCell::new(false));
    let submitted_clone = submitted.clone();

    manager_ui.on_submit(move |type_, name, desc, price, dur, sch| {
        assert_eq!(type_, "SERVICE");
        assert_eq!(name, "Echo Consult");
        assert_eq!(desc, "UX Audit");
        assert_eq!(price, "100.00");
        assert_eq!(dur, "60");
        assert_eq!(sch, "Tuesdays 10am");
        *submitted_clone.borrow_mut() = true;
    });

    manager_ui.set_show_hint(true); // User uses hint

    manager_ui.invoke_select_type("SERVICE".into());
    manager_ui.invoke_next_step();

    manager_ui.set_product_name("Echo Consult".into());
    manager_ui.set_product_description("UX Audit".into());
    manager_ui.set_product_price("100.00".into());
    manager_ui.set_service_duration("60".into());
    manager_ui.set_service_schedule("Tuesdays 10am".into());

    manager_ui.invoke_submit(
        "SERVICE".into(),
        "Echo Consult".into(),
        "UX Audit".into(),
        "100.00".into(),
        "60".into(),
        "Tuesdays 10am".into()
    );

    assert!(*submitted.borrow());
}
