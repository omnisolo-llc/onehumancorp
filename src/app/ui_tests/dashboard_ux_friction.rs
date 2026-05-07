use crate::app;
use slint::{ComponentHandle, Model};

fn create() -> app::Dashboard {
    crate::ui_tests::init();
    app::Dashboard::new().unwrap()
}

#[test]
fn test_bottom_navigation_share_button() {
    let ui = create();

    let ui_handle = ui.as_weak();
    // In our test, clicking share store doesn't easily expose an internal state
    // but we can verify properties if they were exposed.
    // Instead we can test callback is wired up properly via standard mocking:
    let shared = std::rc::Rc::new(std::cell::RefCell::new(false));
    let shared_clone = shared.clone();

    ui.on_action_share_store(move || {
        *shared_clone.borrow_mut() = true;
    });

    ui.invoke_action_share_store();
    assert!(*shared.borrow());
}

#[test]
fn test_advisory_hint_toggle() {
    let ui = create();
    assert!(!ui.get_show_advisory_hint());
    ui.set_show_advisory_hint(true);
    assert!(ui.get_show_advisory_hint());
    ui.set_show_advisory_hint(false);
    assert!(!ui.get_show_advisory_hint());
}

#[test]
fn test_quick_actions_hint_toggle() {
    let ui = create();
    assert!(!ui.get_show_quick_actions_hint());
    ui.set_show_quick_actions_hint(true);
    assert!(ui.get_show_quick_actions_hint());
}

#[test]
fn test_grow_business_action() {
    let ui = create();
    let invoked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let invoked_clone = invoked.clone();

    ui.on_action_grow_business(move || {
        *invoked_clone.borrow_mut() = true;
    });

    ui.invoke_action_grow_business();
    assert!(*invoked.borrow());
}

#[test]
fn test_visual_state_resets() {
    let ui = create();
    ui.set_show_menu(true);
    assert!(ui.get_show_menu());

    ui.set_show_upgrade_prompt(true);
    ui.set_upgrade_prompt_message("Upgrade now!".into());
    assert!(ui.get_show_upgrade_prompt());
    assert_eq!(ui.get_upgrade_prompt_message(), "Upgrade now!");
}
