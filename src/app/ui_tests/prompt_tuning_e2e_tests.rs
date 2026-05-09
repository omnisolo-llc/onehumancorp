use crate::app;

#[test]
fn test_e2e_wizard_prompt_tuning_full_journey() {
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

    let wizard_ui = app::PromptTuning::new().unwrap();
    assert_eq!(wizard_ui.get_step(), 0);

    let save_prompt_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let save_prompt_called_clone = save_prompt_called.clone();

    wizard_ui.on_save_prompt(move || {
        *save_prompt_called_clone.borrow_mut() = true;
    });

    // Step 0: Tone selection
    wizard_ui.set_tone("Professional".into());
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 1);

    // Step 1: Focus selection
    wizard_ui.set_focus_only_business(true);
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 2);

    // Step 2: Examples selection
    wizard_ui.invoke_next_step();
    assert_eq!(wizard_ui.get_step(), 3);

    // Step 3: Save prompt
    wizard_ui.invoke_save_prompt();
    assert!(*save_prompt_called.borrow(), "Save prompt should be clicked and trigger save_prompt callback");
}
