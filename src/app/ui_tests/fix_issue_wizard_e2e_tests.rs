use crate::app;

#[test]
fn test_e2e_wizard_fix_issue_full_journey() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let login_ui = app::Login::new().unwrap();
    let login_successful = std::rc::Rc::new(std::cell::RefCell::new(false));
    let login_successful_clone = login_successful.clone();

    login_ui.on_login(move |email, password| {
        assert_eq!(email, "test@example.com");
        assert_eq!(password, "password123");
        *login_successful_clone.borrow_mut() = true;
    });

    login_ui.invoke_login("test@example.com".into(), "password123".into());
    assert!(*login_successful.borrow(), "User login should be successful");

    let wizard_ui = app::Wizard::new().unwrap();
    assert_eq!(wizard_ui.get_step(), 0);

    let resolve_issue_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let resolve_issue_called_clone = resolve_issue_called.clone();

    wizard_ui.on_resolve_issue(move || {
        *resolve_issue_called_clone.borrow_mut() = true;
    });

    // Step 0: View Suggested Fix
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 1);

    // Step 1: Refresh & Reconnect
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 2);

    // Step 2: Apply Fix ✓
    wizard_ui.invoke_resolve_issue();
    assert!(*resolve_issue_called.borrow(), "Apply Fix should be clicked and trigger resolve_issue callback");
}
