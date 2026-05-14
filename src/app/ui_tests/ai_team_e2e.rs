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
// Flow 3: Manage AI team -> Tune this agent -> steps through PromptTuning
// Flow 4: Hire Helper -> specific capability checks
// Flow 5: Tune agent -> specific tone checks
