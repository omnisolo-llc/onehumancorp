use crate::app;
//use slint::ComponentHandle;

#[test]
fn test_business_manager_cuj_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let business_manager_ui = app::BusinessManager::new().unwrap();

    // Assert visual truth / token truth: test_title exists and matches
    assert_eq!(business_manager_ui.get_test_title(), slint::SharedString::from("Business Manager"));

    let submitted = std::rc::Rc::new(std::cell::RefCell::new(false));
    let submitted_clone = submitted.clone();

    business_manager_ui.on_submit(move |t, name, desc, price, dur, sched| {
        assert_eq!(t, "SERVICE");
        assert_eq!(name, "Lens Audit Fix");
        assert_eq!(desc, "Lens Audit Description");
        assert_eq!(price, "100");
        assert_eq!(dur, "60");
        assert_eq!(sched, "{}");
        *submitted_clone.borrow_mut() = true;
    });

    let closed = std::rc::Rc::new(std::cell::RefCell::new(false));
    let closed_clone = closed.clone();
    business_manager_ui.on_close(move || {
        *closed_clone.borrow_mut() = true;
    });

    // CUJ Walkthrough:
    // 1. Initial State -> "list" view
    assert_eq!(business_manager_ui.get_current_view(), slint::SharedString::from("list"));

    // 2. Click "Add New Offering"
    business_manager_ui.invoke_action_add_new();
    assert_eq!(business_manager_ui.get_current_view(), slint::SharedString::from("add"));
    assert_eq!(business_manager_ui.get_step(), 0);

    // 3. Select type "SERVICE"
    business_manager_ui.invoke_select_type(slint::SharedString::from("SERVICE"));
    assert_eq!(business_manager_ui.get_selected_type(), slint::SharedString::from("SERVICE"));

    // 4. Click "Next"
    business_manager_ui.invoke_next_step();
    assert_eq!(business_manager_ui.get_step(), 1);

    // 5. Fill Details
    business_manager_ui.set_product_name(slint::SharedString::from("Lens Audit Fix"));
    business_manager_ui.set_product_description(slint::SharedString::from("Lens Audit Description"));
    business_manager_ui.set_product_price(slint::SharedString::from("100"));
    business_manager_ui.set_service_duration(slint::SharedString::from("60"));
    business_manager_ui.set_service_schedule(slint::SharedString::from("{}"));

    // 6. Navigate back and forth to ensure state persists and navigation works
    business_manager_ui.invoke_prev_step();
    assert_eq!(business_manager_ui.get_step(), 0);
    business_manager_ui.invoke_next_step();
    assert_eq!(business_manager_ui.get_step(), 1);

    // 7. Click "Create" (Submit)
    business_manager_ui.invoke_submit(
        business_manager_ui.get_selected_type(),
        business_manager_ui.get_product_name(),
        business_manager_ui.get_product_description(),
        business_manager_ui.get_product_price(),
        business_manager_ui.get_service_duration(),
        business_manager_ui.get_service_schedule()
    );

    business_manager_ui.invoke_close();

    // Verification 1: Callback triggered indicating UI correctly piped data to Rust backend hook
    assert!(*submitted.borrow(), "Submit callback must be invoked with correct data");
    assert!(*closed.borrow(), "Close callback must be invoked");
}

#[test]
fn test_business_manager_cuj_lens_audit_2() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let business_manager_ui = app::BusinessManager::new().unwrap();
    business_manager_ui.invoke_action_add_new();
    business_manager_ui.invoke_select_type(slint::SharedString::from("PHYSICAL"));
    assert_eq!(business_manager_ui.get_selected_type(), slint::SharedString::from("PHYSICAL"));
}

#[test]
fn test_business_manager_cuj_lens_audit_3() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let business_manager_ui = app::BusinessManager::new().unwrap();
    business_manager_ui.invoke_action_add_new();
    business_manager_ui.invoke_select_type(slint::SharedString::from("DIGITAL"));
    assert_eq!(business_manager_ui.get_selected_type(), slint::SharedString::from("DIGITAL"));
}

#[test]
fn test_business_manager_cuj_lens_audit_4() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let business_manager_ui = app::BusinessManager::new().unwrap();

    let edit_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let edit_clone = edit_called.clone();
    business_manager_ui.on_action_edit(move |id| {
        assert_eq!(id, "prod_test");
        *edit_clone.borrow_mut() = true;
    });

    business_manager_ui.invoke_action_edit("prod_test".into());
    assert!(*edit_called.borrow(), "Edit callback must be invoked");
}

#[test]
fn test_business_manager_cuj_lens_audit_5() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let business_manager_ui = app::BusinessManager::new().unwrap();

    let archive_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let archive_clone = archive_called.clone();
    business_manager_ui.on_action_archive(move |id| {
        assert_eq!(id, "prod_test");
        *archive_clone.borrow_mut() = true;
    });

    business_manager_ui.invoke_action_archive("prod_test".into());
    assert!(*archive_called.borrow(), "Archive callback must be invoked");
}
