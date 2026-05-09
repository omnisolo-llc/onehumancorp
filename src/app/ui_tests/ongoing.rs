use crate::app;

fn create_f() -> app::Wizard { crate::ui_tests::init(); app::Wizard::new().unwrap() }
fn create_u() -> app::Upgrade { crate::ui_tests::init(); app::Upgrade::new().unwrap() }
fn create_b() -> app::Billing { crate::ui_tests::init(); app::Billing::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn ongoing_fix_step_negative() {
    let ui = create_f();
    ui.set_step(-5);
    assert_eq!(ui.get_step(), -5);
}

#[test] fn ongoing_upgrade_progress_oob() {
    let ui = create_u();
    ui.set_progress(1000);
    assert_eq!(ui.get_progress(), 1000);
    ui.set_progress(-100);
    assert_eq!(ui.get_progress(), -100);
}

// --- Interaction / Flow Tests ---

#[test] fn ongoing_fix_flow_steps() {
    let ui = create_f();
    assert_eq!(ui.get_step(), 0);
    ui.set_step(1);
    assert_eq!(ui.get_step(), 1);
    ui.set_step(2);

    ui.set_step(2);

    assert_eq!(ui.get_step(), 2);

}

#[test] fn ongoing_upgrade_flow() {
    let ui = create_u();
    assert!(!ui.get_is_upgrading());
    assert!(!ui.get_done());
    ui.set_is_upgrading(true);
    ui.set_progress(50);
    assert!(ui.get_is_upgrading());
    assert_eq!(ui.get_progress(), 50);
    ui.set_done(true);
    ui.set_is_upgrading(false);
    assert!(ui.get_done());
    assert!(!ui.get_is_upgrading());
}

// --- Advanced Mode Tests ---

#[test] fn ongoing_fix_advanced_toggle() {
    let ui = create_f();
    assert!(!ui.get_is_advanced());

    let save_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let save_called_clone = save_called.clone();
    ui.on_save_state(move || { *save_called_clone.borrow_mut() = true; });

    // In UI, we toggle. Programmatically simulating setting it
    ui.set_is_advanced(true);
    assert!(ui.get_is_advanced());
    ui.invoke_save_state();
    assert!(*save_called.borrow());
}

#[test] fn ongoing_upgrade_advanced_toggle() {
    let ui = create_u();
    assert!(!ui.get_is_advanced());

    let save_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let save_called_clone = save_called.clone();
    ui.on_save_state(move || { *save_called_clone.borrow_mut() = true; });

    ui.set_is_advanced(true);
    assert!(ui.get_is_advanced());
    ui.invoke_save_state();
    assert!(*save_called.borrow());
}

#[test] fn ongoing_billing_advanced_toggle() {
    let ui = create_b();
    assert!(!ui.get_is_advanced());

    let save_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let save_called_clone = save_called.clone();
    ui.on_save_state(move || { *save_called_clone.borrow_mut() = true; });

    ui.set_is_advanced(true);
    assert!(ui.get_is_advanced());
    ui.invoke_save_state();
    assert!(*save_called.borrow());
}

// --- Unique Scenarios with Verification ---

#[test] fn fix_agent_callbacks() {
    let ui = create_f();

    let apply_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let apply_called_clone = apply_called.clone();
    ui.on_resolve_issue(move || { *apply_called_clone.borrow_mut() = true; });
    ui.invoke_resolve_issue();
    assert!(*apply_called.borrow());

    }

// --- Consolidated Verified Tests ---

#[test]
fn create_f_verify_step() {
    let ui = create_f();
    ui.set_step(10);
    assert_eq!(ui.get_step(), 10);
    ui.set_step(20);
    assert_eq!(ui.get_step(), 20);
    ui.set_step(30);
    assert_eq!(ui.get_step(), 30);
}

#[test]
fn create_u_verify_progress() {
    let ui = create_u();
    ui.set_progress(1);
    assert_eq!(ui.get_progress(), 1);
    ui.set_progress(99);
    assert_eq!(ui.get_progress(), 99);
    ui.set_progress(21);
    assert_eq!(ui.get_progress(), 21);
}

#[test]
fn test_e2e_fix_agent_full_journey() {
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

    let agents_ui = app::Agents::new().unwrap();
    let fix_agent_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let fix_agent_opened_clone = fix_agent_opened.clone();

    agents_ui.on_fix_agent(move |id| {
        assert_eq!(id, "agent_1");
        *fix_agent_opened_clone.borrow_mut() = true;
    });

    agents_ui.invoke_fix_agent("agent_1".into());
    assert!(*fix_agent_opened.borrow(), "Fix Agent should be opened from Agents screen");

    let fix_agent_ui = app::Wizard::new().unwrap();
    assert_eq!(fix_agent_ui.get_step(), 0);

    // Advance to step 1
    fix_agent_ui.set_step(1);

    let apply_fix_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let apply_fix_called_clone = apply_fix_called.clone();
    fix_agent_ui.on_resolve_issue(move || {
        *apply_fix_called_clone.borrow_mut() = true;
    });

    fix_agent_ui.invoke_resolve_issue();
    assert!(*apply_fix_called.borrow(), "Apply fix should be called");

    // Advance to step 2
    fix_agent_ui.set_step(2);

    }

#[test]
fn test_e2e_grow_business_full_journey() {
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

    let dashboard_ui = app::Dashboard::new().unwrap();
    let grow_business_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let grow_business_opened_clone = grow_business_opened.clone();

    dashboard_ui.on_action_grow_business(move || {
        *grow_business_opened_clone.borrow_mut() = true;
    });

    dashboard_ui.invoke_action_grow_business();
    assert!(*grow_business_opened.borrow(), "Grow Business should be opened from Dashboard");

    let ui = app::GrowBusiness::new().unwrap();

    let execute_success = std::rc::Rc::new(std::cell::RefCell::new(false));
    let execute_success_clone = execute_success.clone();

    ui.on_save_state(|| {});

    ui.on_execute(move |strategy, _kpi| {
        assert_eq!(strategy, "Add 5 more products");
        *execute_success_clone.borrow_mut() = true;
    });

    assert_eq!(ui.get_step(), 0);
    assert_eq!(ui.get_is_advanced(), false);

    ui.invoke_toggle_advanced();
    assert_eq!(ui.get_is_advanced(), true);

    ui.invoke_select_strategy("Add 5 more products".into());
    ui.invoke_next_step();

    assert_eq!(ui.get_step(), 1);

    ui.set_kpi_target("20%".into());
    ui.invoke_execute(ui.get_selected_strategy(), ui.get_kpi_target());
    ui.invoke_next_step();

    assert_eq!(ui.get_step(), 2);
    assert_eq!(ui.get_selected_strategy(), "Add 5 more products");

    ui.invoke_return_to_dashboard();
    assert_eq!(ui.get_step(), 0);
    assert_eq!(ui.get_selected_strategy(), "");
}
