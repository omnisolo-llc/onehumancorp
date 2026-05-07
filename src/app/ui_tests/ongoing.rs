use crate::app;

fn create_f() -> app::FixAgent { crate::ui_tests::init(); app::FixAgent::new().unwrap() }
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
    ui.set_is_applying(true);
    assert!(ui.get_is_applying());
    ui.set_step(2);
    ui.set_is_applying(false);
    assert_eq!(ui.get_step(), 2);
    assert!(!ui.get_is_applying());
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

// --- Advanced mode Tests ---

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
    ui.on_apply_fix(move || { *apply_called_clone.borrow_mut() = true; });
    ui.invoke_apply_fix();
    assert!(*apply_called.borrow());

    let return_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let return_called_clone = return_called.clone();
    ui.on_return_to_agents(move || { *return_called_clone.borrow_mut() = true; });
    ui.invoke_return_to_agents();
    assert!(*return_called.borrow());
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
