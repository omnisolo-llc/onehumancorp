use crate::app;

fn create() -> app::UpgradePrompt {
    crate::ui_tests::init();
    app::UpgradePrompt::new().unwrap()
}

#[test]
fn test_upgrade_prompt_message() {
    let ui = create();
    assert_eq!(ui.get_message(), "You've reached a limit on your current plan.");
    ui.set_message("Please upgrade to add more products.".into());
    assert_eq!(ui.get_message(), "Please upgrade to add more products.");
}

#[test]
fn test_upgrade_prompt_action_upgrade() {
    let ui = create();
    let invoked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let invoked_clone = invoked.clone();

    ui.on_action_upgrade(move || {
        *invoked_clone.borrow_mut() = true;
    });

    ui.invoke_action_upgrade();
    assert!(*invoked.borrow());
}

#[test]
fn test_upgrade_prompt_action_dismiss() {
    let ui = create();
    let invoked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let invoked_clone = invoked.clone();

    ui.on_action_dismiss(move || {
        *invoked_clone.borrow_mut() = true;
    });

    ui.invoke_action_dismiss();
    assert!(*invoked.borrow());
}