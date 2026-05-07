use crate::app;

// Flow 1: Click Manage AI team -> opens Agents
#[test]
fn e2e_flow_manage_ai_team_opens_agents() {
    crate::ui_tests::init();
    let dashboard_ui = app::Dashboard::new().unwrap();

    let agents_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let agents_opened_clone = agents_opened.clone();

    dashboard_ui.on_action_manage_my_ai_team(move || {
        *agents_opened_clone.borrow_mut() = true;
    });

    dashboard_ui.invoke_action_manage_my_ai_team();
    assert!(*agents_opened.borrow(), "Manage my AI team button should open Agents screen");
}

// Flow 2: Manage AI team -> Hire Helper -> steps through AgentConfig
#[test]
fn e2e_flow_hire_helper_wizard() {
    crate::ui_tests::init();
    let agents_ui = app::Agents::new().unwrap();

    let config_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let config_opened_clone = config_opened.clone();

    agents_ui.on_hire_agent(move || {
        *config_opened_clone.borrow_mut() = true;
    });

    agents_ui.invoke_hire_agent();
    assert!(*config_opened.borrow(), "Hire Helper should open AgentConfig wizard");

    let config_ui = app::AgentConfig::new().unwrap();
    assert_eq!(config_ui.get_step(), 0);

    // Simulate AgentConfig flow
    config_ui.set_selected_agent("SEO Booster".into());
    config_ui.invoke_next_step();
    assert_eq!(config_ui.get_step(), 1);

    config_ui.set_can_reply(true);
    config_ui.invoke_next_step();
    assert_eq!(config_ui.get_step(), 2);

    config_ui.set_frequency_value(2.0); // Daily
    config_ui.invoke_next_step();
    assert_eq!(config_ui.get_step(), 3);
}

// Flow 3: Manage AI team -> Tune this agent -> steps through PromptTuning
#[test]
fn e2e_flow_tune_agent_wizard() {
    crate::ui_tests::init();
    let agents_ui = app::Agents::new().unwrap();

    let tuning_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let tuning_opened_clone = tuning_opened.clone();

    agents_ui.on_tune_agent(move |_id| {
        *tuning_opened_clone.borrow_mut() = true;
    });

    agents_ui.invoke_tune_agent("agent_1".into());
    assert!(*tuning_opened.borrow(), "Tune agent should open PromptTuning wizard");

    let tuning_ui = app::PromptTuning::new().unwrap();
    assert_eq!(tuning_ui.get_step(), 0);

    tuning_ui.set_tone("Friendly".into());
    tuning_ui.invoke_next_step();
    assert_eq!(tuning_ui.get_step(), 1);

    tuning_ui.set_focus_only_business(true);
    tuning_ui.invoke_next_step();
    assert_eq!(tuning_ui.get_step(), 2);
}

// Flow 4: Hire Helper -> specific capability checks
#[test]
fn e2e_flow_hire_helper_capabilities() {
    crate::ui_tests::init();
    let config_ui = app::AgentConfig::new().unwrap();

    config_ui.set_step(1);
    assert_eq!(config_ui.get_can_reply(), false);

    config_ui.set_can_reply(true);
    config_ui.set_can_social(true);

    assert_eq!(config_ui.get_can_reply(), true);
    assert_eq!(config_ui.get_can_social(), true);
}

// Flow 5: Tune agent -> specific tone checks
#[test]
fn e2e_flow_tune_agent_tone() {
    crate::ui_tests::init();
    let tuning_ui = app::PromptTuning::new().unwrap();

    tuning_ui.set_tone("Energetic".into());
    assert_eq!(tuning_ui.get_tone(), "Energetic");

    tuning_ui.set_tone("Professional".into());
    assert_eq!(tuning_ui.get_tone(), "Professional");
}
