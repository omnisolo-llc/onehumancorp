use crate::app;

fn create() -> app::GrowBusiness { crate::ui_tests::init(); app::GrowBusiness::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn grow_xss_strategy() {
    let ui = create();
    let xss = "<script>alert('grow')</script>";
    ui.set_selected_strategy(xss.into());
    assert_eq!(ui.get_selected_strategy(), xss);
}

#[test] fn grow_step_overflow() {
    let ui = create();
    ui.set_step(999);
    assert_eq!(ui.get_step(), 999);
}

#[test] fn grow_step_underflow() {
    let ui = create();
    ui.set_step(-999);
    assert_eq!(ui.get_step(), -999);
}

// --- Interaction / Flow Tests ---

#[test] fn grow_flow_retention_switch() {
    let ui = create();
    ui.set_selected_strategy("A".into());
    ui.set_is_advanced(true);
    ui.set_selected_strategy("B".into());
    assert!(ui.get_is_advanced());
}

#[test] fn grow_flow_step_loop() {
    let ui = create();
    for i in 0..10 {
        ui.set_step(i);
        assert_eq!(ui.get_step(), i);
    }
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_selected_strategy() {
    let ui = create();
    ui.set_selected_strategy("Inbound Marketing".into());
    assert_eq!(ui.get_selected_strategy(), "Inbound Marketing");
    ui.set_selected_strategy("Outbound Sales".into());
    assert_eq!(ui.get_selected_strategy(), "Outbound Sales");
    ui.set_selected_strategy("Content Creation".into());
    assert_eq!(ui.get_selected_strategy(), "Content Creation");
}

#[test]
fn create_verify_step() {
    let ui = create();
    ui.set_step(21);
    assert_eq!(ui.get_step(), 21);
    ui.set_step(22);
    assert_eq!(ui.get_step(), 22);
    ui.set_step(23);
    assert_eq!(ui.get_step(), 23);
}

// --- Added tests for 100% coverage and 5 tests rule ---

#[test]
fn grow_business_test_set_step() {
    let ui = create();
    ui.set_step(1);
    assert_eq!(ui.get_step(), 1);
    ui.set_step(2);
    assert_eq!(ui.get_step(), 2);
}

#[test]
fn grow_business_test_set_selected_strategy() {
    let ui = create();
    ui.set_selected_strategy("Add 5 more products".into());
    assert_eq!(ui.get_selected_strategy(), "Add 5 more products");
    ui.set_selected_strategy("Connect Instagram".into());
    assert_eq!(ui.get_selected_strategy(), "Connect Instagram");
}

#[test]
fn grow_business_test_set_is_advanced() {
    let ui = create();
    ui.set_is_advanced(true);
    assert_eq!(ui.get_is_advanced(), true);
    ui.set_is_advanced(false);
    assert_eq!(ui.get_is_advanced(), false);
}

#[test]
fn grow_business_test_set_execution_started() {
    let ui = create();
    ui.set_execution_started(true);
    assert_eq!(ui.get_execution_started(), true);
    ui.set_execution_started(false);
    assert_eq!(ui.get_execution_started(), false);
}

#[test]
fn grow_business_test_set_kpi_target() {
    let ui = create();
    ui.set_kpi_target("20%".into());
    assert_eq!(ui.get_kpi_target(), "20%");
    ui.set_kpi_target("50%".into());
    assert_eq!(ui.get_kpi_target(), "50%");
}

#[test]
fn grow_business_test_callbacks() {
    let ui = create();

    // Test toggle advanced
    let toggle_advanced_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let t1 = toggle_advanced_called.clone();
    ui.on_toggle_advanced(move || {
        *t1.borrow_mut() = true;
    });
    ui.invoke_toggle_advanced();
    assert!(*toggle_advanced_called.borrow());

    // Test select strategy
    let select_strategy_called = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let s1 = select_strategy_called.clone();
    ui.on_select_strategy(move |s| {
        *s1.borrow_mut() = s.to_string();
    });
    ui.invoke_select_strategy("Connect Instagram".into());
    assert_eq!(*select_strategy_called.borrow(), "Connect Instagram");

    // Test next_step
    let next_step_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let n1 = next_step_called.clone();
    ui.on_next_step(move || {
        *n1.borrow_mut() = true;
    });
    ui.invoke_next_step();
    assert!(*next_step_called.borrow());

    // Test prev_step
    let prev_step_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let p1 = prev_step_called.clone();
    ui.on_prev_step(move || {
        *p1.borrow_mut() = true;
    });
    ui.invoke_prev_step();
    assert!(*prev_step_called.borrow());

    // Test return_to_dashboard
    let return_to_dashboard_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let r1 = return_to_dashboard_called.clone();
    ui.on_return_to_dashboard(move || {
        *r1.borrow_mut() = true;
    });
    ui.invoke_return_to_dashboard();
    assert!(*return_to_dashboard_called.borrow());
}

#[test]
fn grow_business_test_social_media_connect_instagram() {
    let ui = create();
    ui.set_selected_strategy("Connect Instagram".into());
    assert_eq!(ui.get_selected_strategy(), "Connect Instagram");
}

#[test]
fn grow_business_test_social_media_run_email_campaign() {
    let ui = create();
    ui.set_selected_strategy("Run your first email campaign".into());
    assert_eq!(ui.get_selected_strategy(), "Run your first email campaign");
}

#[test]
fn grow_business_test_social_media_add_products() {
    let ui = create();
    ui.set_selected_strategy("Add 5 more products".into());
    assert_eq!(ui.get_selected_strategy(), "Add 5 more products");
}

#[test]
fn test_grow_business_e2e_flow_products() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::GrowBusiness::new().unwrap();

    assert_eq!(ui.get_step(), 0);
    ui.invoke_select_strategy("Add 5 more products".into());
    assert_eq!(ui.get_selected_strategy(), "Add 5 more products");

    ui.invoke_next_step();
    assert_eq!(ui.get_step(), 1);

    let executed = std::rc::Rc::new(std::cell::RefCell::new(false));
    let executed_clone = executed.clone();
    ui.on_execute(move |strategy, _kpi| {
        assert_eq!(strategy, "Add 5 more products");
        *executed_clone.borrow_mut() = true;
    });

    ui.invoke_execute("Add 5 more products".into(), "".into());
    assert!(*executed.borrow(), "Execute should be triggered");

    ui.invoke_next_step();
    assert_eq!(ui.get_step(), 2);

    let returned = std::rc::Rc::new(std::cell::RefCell::new(false));
    let returned_clone = returned.clone();
    ui.on_return_to_dashboard(move || {
        *returned_clone.borrow_mut() = true;
    });
    ui.invoke_return_to_dashboard();
    assert!(*returned.borrow(), "Return to dashboard should be triggered");
}

#[test]
fn test_grow_business_advanced_toggle() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::GrowBusiness::new().unwrap();

    assert_eq!(ui.get_is_advanced(), false);
    ui.invoke_toggle_advanced();
    assert_eq!(ui.get_is_advanced(), true);

    ui.set_kpi_target("15".into());
    assert_eq!(ui.get_kpi_target(), "15");
}

#[test]
fn test_grow_business_e2e_flow_instagram() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::GrowBusiness::new().unwrap();

    assert_eq!(ui.get_step(), 0);
    ui.invoke_select_strategy("Connect Instagram".into());
    assert_eq!(ui.get_selected_strategy(), "Connect Instagram");

    ui.invoke_next_step();
    assert_eq!(ui.get_step(), 1);
}

#[test]
fn test_grow_business_e2e_flow_email() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::GrowBusiness::new().unwrap();

    assert_eq!(ui.get_step(), 0);
    ui.invoke_select_strategy("Run your first email campaign".into());
    assert_eq!(ui.get_selected_strategy(), "Run your first email campaign");

    ui.invoke_next_step();
    assert_eq!(ui.get_step(), 1);

    ui.invoke_prev_step();
    assert_eq!(ui.get_step(), 0);
}

#[test]
fn test_grow_business_step_bounds() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::GrowBusiness::new().unwrap();

    ui.set_step(10);
    assert_eq!(ui.get_step(), 10);
    ui.set_step(-1);
    assert_eq!(ui.get_step(), -1);
}
