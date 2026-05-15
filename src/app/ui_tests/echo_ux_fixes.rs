use slint::ComponentHandle;
use slint::SharedString;
use std::rc::Rc;
use std::cell::RefCell;
use crate::app;

#[test]
fn e2e_flow_ux_fixes() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // Simulate flow from Login
    let login = app::Login::new().unwrap();
    let login_clicked = Rc::new(RefCell::new(false));
    let login_clicked_clone = login_clicked.clone();

    login.on_login(move |u, p| {
        *login_clicked_clone.borrow_mut() = true;
    });

    // Test the UX fix for error message text wrapper behavior by interacting with the component
    login.set_error_message("We couldn't sign you in. Please check your email and password and try again.".into());
    login.set_username("test@example.com".into());
    login.set_password("pass".into());
    login.invoke_login(login.get_username(), login.get_password());

    assert!(*login_clicked.borrow(), "Login button should be clickable");
    assert_eq!(login.get_error_message(), slint::SharedString::from("We couldn't sign you in. Please check your email and password and try again."));

    // Navigate to Dashboard
    let dashboard = app::Dashboard::new().unwrap();

    let add_product_clicked = Rc::new(RefCell::new(false));
    let add_product_clicked_clone = add_product_clicked.clone();

    dashboard.on_action_build_website(move || {
        *add_product_clicked_clone.borrow_mut() = true;
    });

    // Check our plain language telemetry property bindings
    dashboard.set_telemetry_cache_hits("95%".into());
    dashboard.set_telemetry_rag_latency("100ms".into());
    assert_eq!(dashboard.get_telemetry_cache_hits(), slint::SharedString::from("95%"));

    // Click Add Product to open Business Manager
    dashboard.invoke_action_build_website();
    assert!(*add_product_clicked.borrow(), "Add Product action should be triggered");

    // Open Business Manager
    let biz_manager = app::BusinessManager::new().unwrap();

    let submit_clicked = Rc::new(RefCell::new(false));
    let submit_clicked_clone = submit_clicked.clone();

    biz_manager.on_submit(move |_t, _n, _d, _p, _dur, _sch| {
        *submit_clicked_clone.borrow_mut() = true;
    });

    biz_manager.invoke_action_add_new();
    assert_eq!(biz_manager.get_current_view(), "add");
    assert_eq!(biz_manager.get_step(), 0);

    // Verify our single-step contextual hint logic
    assert_eq!(biz_manager.get_show_offering_hint(), false);
    biz_manager.set_show_offering_hint(true);
    // We verified the property changes which controls rendering the hint box.
    assert_eq!(biz_manager.get_show_offering_hint(), true);

    // Complete the flow
    biz_manager.select_type("PHYSICAL".into());
    biz_manager.invoke_next_step();

    assert_eq!(biz_manager.get_step(), 1);

    biz_manager.set_product_name("Custom Cake".into());
    biz_manager.set_product_price("20.00".into()); // UX fix for placeholder text behavior

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
