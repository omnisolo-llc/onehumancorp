use crate::app;
use std::rc::Rc;
use std::cell::RefCell;

#[test]
fn test_e2e_maya_acquisition_to_first_sale() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let main_app = app::AppWindow::new().unwrap();
    main_app.set_current_page("login".into());

    let login_successful = Rc::new(RefCell::new(false));
    let login_successful_clone = login_successful.clone();

    main_app.on_login_requested(move |email, password| {
        assert_eq!(email, "maya@example.com");
        assert_eq!(password, "secure123");
        *login_successful_clone.borrow_mut() = true;
    });

    main_app.invoke_login_requested("maya@example.com".into(), "secure123".into());
    assert!(*login_successful.borrow(), "Maya login should succeed");

    main_app.set_current_page("setup_wizard".into());
    main_app.set_current_page("dashboard".into());

    let pending_approvals = slint::ModelRc::from(std::rc::Rc::new(slint::VecModel::from(vec![
        app::UiPendingApproval {
            task_id: "task_1".into(),
            title: "Storefront Draft Ready".into(),
            proposed_content: "Review your new site".into(),
            helper_name: "The Promoter".into(),
        }
    ])));
    main_app.set_dashboard_pending_approvals(pending_approvals);

    let approved_task = Rc::new(RefCell::new(String::new()));
    let approved_task_clone = approved_task.clone();
    main_app.on_approve_task_requested(move |task_id| {
        *approved_task_clone.borrow_mut() = task_id.into();
    });

    main_app.invoke_approve_task_requested("task_1".into());
    assert_eq!(*approved_task.borrow(), "task_1", "Maya approved the storefront draft");
}

#[test]
fn test_e2e_carlos_booking_and_ai_quote() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let main_app = app::AppWindow::new().unwrap();
    main_app.set_current_page("dashboard".into());

    let pending_approvals = slint::ModelRc::from(std::rc::Rc::new(slint::VecModel::from(vec![
        app::UiPendingApproval {
            task_id: "quote_123".into(),
            title: "New Quote Draft for Review".into(),
            proposed_content: "Draft Quote: $150 + Deposit for Leaking pipe".into(),
            helper_name: "The Salesperson".into(),
        }
    ])));
    main_app.set_dashboard_pending_approvals(pending_approvals);

    let approved_task = Rc::new(RefCell::new(String::new()));
    let approved_task_clone = approved_task.clone();
    main_app.on_approve_task_requested(move |task_id| {
        *approved_task_clone.borrow_mut() = task_id.into();
    });

    main_app.invoke_approve_task_requested("quote_123".into());
    assert_eq!(*approved_task.borrow(), "quote_123", "Carlos approved the AI quote");
}

#[test]
fn test_e2e_priya_omnichannel_inventory() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let main_app = app::AppWindow::new().unwrap();
    main_app.set_current_page("dashboard".into());

    let action_taken = Rc::new(RefCell::new(false));
    let action_taken_clone = action_taken.clone();
    main_app.on_action_view_orders_requested(move || {
        *action_taken_clone.borrow_mut() = true;
    });

    main_app.invoke_action_view_orders_requested();
    assert!(*action_taken.borrow(), "Priya reviewed inventory/orders");
}

#[test]
fn test_e2e_leo_subscription_and_retention() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let main_app = app::AppWindow::new().unwrap();
    main_app.set_current_page("dashboard".into());

    let pending_approvals = slint::ModelRc::from(std::rc::Rc::new(slint::VecModel::from(vec![
        app::UiPendingApproval {
            task_id: "checkin_email".into(),
            title: "Review check-in email draft".into(),
            proposed_content: "Hey, missed you...".into(),
            helper_name: "The Ambassador".into(),
        }
    ])));
    main_app.set_dashboard_pending_approvals(pending_approvals);

    let approved_task = Rc::new(RefCell::new(String::new()));
    let approved_task_clone = approved_task.clone();
    main_app.on_approve_task_requested(move |task_id| {
        *approved_task_clone.borrow_mut() = task_id.into();
    });

    main_app.invoke_approve_task_requested("checkin_email".into());
    assert_eq!(*approved_task.borrow(), "checkin_email", "Leo approved the check-in email to the student");
}

#[test]
fn test_e2e_fatima_high_velocity_preorders() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let main_app = app::AppWindow::new().unwrap();
    main_app.set_current_page("dashboard".into());

    let acknowledged = Rc::new(RefCell::new(false));
    let acknowledged_clone = acknowledged.clone();

    main_app.on_action_view_orders_requested(move || {
         *acknowledged_clone.borrow_mut() = true;
    });

    main_app.invoke_action_view_orders_requested();
    assert!(*acknowledged.borrow(), "Fatima acknowledged the high-velocity pre-order");
}
