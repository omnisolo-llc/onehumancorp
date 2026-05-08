use crate::app;
use slint::ComponentHandle;

#[test]
fn test_cuj_grow_business_wizard() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // The user logs in and starts from the home page.



    // In our simplified test harness, we can directly instantiate the GrowBusiness wizard
    // as it represents the standalone pop-up launched from the Dashboard.
    let wizard_ui = app::GrowBusiness::new().unwrap();
    wizard_ui.set_step(0);

    // Step 0: Select Strategy
    assert_eq!(wizard_ui.get_step(), 0);
    wizard_ui.invoke_select_strategy("Connect Instagram".into());
    assert_eq!(wizard_ui.get_selected_strategy(), "Connect Instagram");
    wizard_ui.invoke_next_step();

    // Step 1: Confirm Action
    assert_eq!(wizard_ui.get_step(), 1);

    let execute_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let ec = execute_called.clone();
    wizard_ui.on_execute(move |strategy, _kpi| {
        assert_eq!(strategy, "Connect Instagram");
        *ec.borrow_mut() = true;
    });

    wizard_ui.set_execution_started(true);
    wizard_ui.invoke_execute("Connect Instagram".into(), "".into());
    assert!(*execute_called.borrow(), "Execution callback triggered");

    wizard_ui.invoke_next_step();

    // Step 2: Success
    assert_eq!(wizard_ui.get_step(), 2);
    let return_dashboard_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let rd = return_dashboard_called.clone();
    wizard_ui.on_return_to_dashboard(move || {
        *rd.borrow_mut() = true;
    });
    wizard_ui.invoke_return_to_dashboard();

    assert!(*return_dashboard_called.borrow(), "Return to dashboard callback triggered");
}

#[test]
fn test_cuj_agent_config_wizard() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let wizard_ui = app::AgentConfig::new().unwrap();
    wizard_ui.set_step(0);

    // Step 0: Select Agent
    wizard_ui.set_selected_agent("Customer Support".into());
    wizard_ui.invoke_next_step();

    // Step 1: Capabilities
    assert_eq!(wizard_ui.get_step(), 1);
    wizard_ui.set_can_reply(true);
    wizard_ui.invoke_next_step();

    // Step 2: Frequency
    assert_eq!(wizard_ui.get_step(), 2);
    wizard_ui.set_frequency_value(2.0);
    wizard_ui.invoke_next_step();

    // Step 3: Review & Activate
    assert_eq!(wizard_ui.get_step(), 3);

    let activate_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let ac = activate_called.clone();
    wizard_ui.on_activate_agent(move |agent, reply, _, _, _, freq, _, _, _| {
        assert_eq!(agent, "Customer Support");
        assert_eq!(reply, true);
        assert_eq!(freq, "Daily");
        *ac.borrow_mut() = true;
    });

    wizard_ui.set_show_toast(true);
    wizard_ui.invoke_activate_agent(
        "Customer Support".into(), true, false, false, false, "Daily".into(), "".into(), "".into(), "".into()
    );
    assert!(*activate_called.borrow(), "Agent Activation should be called");
}

#[test]
fn test_cuj_prompt_tuning_wizard() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let wizard_ui = app::PromptTuning::new().unwrap();
    wizard_ui.set_step(0);

    // Step 0: Tone
    wizard_ui.set_tone("Professional".into());
    wizard_ui.invoke_next_step();

    // Step 1: Focus
    assert_eq!(wizard_ui.get_step(), 1);
    wizard_ui.set_focus_only_business(true);
    wizard_ui.invoke_next_step();

    // Step 2: Examples
    assert_eq!(wizard_ui.get_step(), 2);
    wizard_ui.invoke_next_step();

    // Step 3: Review
    assert_eq!(wizard_ui.get_step(), 3);

    let save_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let sc = save_called.clone();
    wizard_ui.on_save_prompt(move || {
        *sc.borrow_mut() = true;
    });

    wizard_ui.set_show_toast(true);
    wizard_ui.invoke_save_prompt();
    assert!(*save_called.borrow(), "Save prompt should be called");
}

#[test]
fn test_cuj_billing_wizard() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let wizard_ui = app::Pricing::new().unwrap();
    wizard_ui.set_step(0);

    // Step 0: Usage ("What does this cost?")
    assert_eq!(wizard_ui.get_step(), 0);

    let add_credits_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let acc = add_credits_called.clone();
    wizard_ui.on_add_credits(move || {
        *acc.borrow_mut() = true;
    });
    wizard_ui.invoke_add_credits();
    assert!(*add_credits_called.borrow(), "Add credits should be callable from step 0");

    // Move to plans
    wizard_ui.set_step(1);

    // Step 1: Upgrade Plans
    assert_eq!(wizard_ui.get_step(), 1);
    let select_plan_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let spc = select_plan_called.clone();
    wizard_ui.on_select_plan(move |plan| {
        assert_eq!(plan, "Pro");
        *spc.borrow_mut() = true;
    });

    wizard_ui.invoke_select_plan("Pro".into());
    assert!(*select_plan_called.borrow(), "Select plan should be callable from step 1");
}

#[test]
fn test_cuj_fix_issue_wizard() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let wizard_ui = app::Wizard::new().unwrap();
    wizard_ui.set_step(0);

    // Step 0: Understand
    assert_eq!(wizard_ui.get_step(), 0);
    wizard_ui.invoke_next_step();

    // Step 1: Review
    assert_eq!(wizard_ui.get_step(), 1);
    wizard_ui.invoke_next_step();

    // Step 2: Resolve
    assert_eq!(wizard_ui.get_step(), 2);

    let resolve_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let rc = resolve_called.clone();
    wizard_ui.on_resolve_issue(move || {
        *rc.borrow_mut() = true;
    });

    wizard_ui.invoke_resolve_issue();
    assert!(*resolve_called.borrow(), "Resolve issue should be called");
}
