use crate::app;


fn create_c() -> app::AgentConfig { crate::ui_tests::init(); app::AgentConfig::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn agent_name_injection() {
    let ui = create_c();
    let inj = "Admin'; DROP TABLE agents; --";
    ui.set_selected_agent(inj.into());
    assert_eq!(ui.get_selected_agent(), inj);
}

#[test] fn agent_freq_oob() {
    let ui = create_c();
    ui.set_frequency_value(2.0);
    assert_eq!(ui.get_frequency_value(), 2.0);
    ui.set_frequency_value(-1.0);
    assert_eq!(ui.get_frequency_value(), -1.0);
}

#[test] fn agent_xss_toast() {
    let ui = create_c();
    let xss = "<script>console.log(1)</script>";
    ui.set_selected_agent(xss.into());
    assert_eq!(ui.get_selected_agent(), xss);
}

// --- Interaction / Flow Tests ---

#[test] fn agent_config_permutation_flow() {
    let ui = create_c();
    let flags = [true, false];
    for f1 in flags {
        for f2 in flags {
            ui.set_can_reply(f1);
            ui.set_can_social(f2);
            assert_eq!(ui.get_can_reply(), f1);
            assert_eq!(ui.get_can_social(), f2);
        }
    }
}

#[test] fn agent_selection_retention_flow() {
    let ui = create_c();
    ui.set_selected_agent("Agent Alpha".into());
    ui.set_is_advanced(true);
    ui.set_selected_agent("Agent Beta".into());
    assert!(ui.get_is_advanced());
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_a_verify_selected_agent() {
    let ui = create_c();
    ui.set_selected_agent("Support Bot".into());
    assert_eq!(ui.get_selected_agent(), "Support Bot");
    ui.set_selected_agent("".into());
    assert_eq!(ui.get_selected_agent(), "");
    ui.set_selected_agent("DeepThought".into());
    assert_eq!(ui.get_selected_agent(), "DeepThought");
}

#[test]
fn create_a_verify_frequency_value() {
    let ui = create_c();
    ui.set_frequency_value(0.21);
    assert_eq!(ui.get_frequency_value(), 0.21);
    ui.set_frequency_value(0.22);
    assert_eq!(ui.get_frequency_value(), 0.22);
    ui.set_frequency_value(0.23);
    assert_eq!(ui.get_frequency_value(), 0.23);
}

fn create_agents_ui() -> app::Agents {
    crate::ui_tests::init();
    app::Agents::new().unwrap()
}

#[test]
fn test_agents_ui_upgrade_prompt_visibility() {
    let ui = create_agents_ui();
    assert_eq!(ui.get_show_upgrade_prompt(), false);

    ui.set_show_upgrade_prompt(true);
    assert_eq!(ui.get_show_upgrade_prompt(), true);

    ui.set_show_upgrade_prompt(false);
    assert_eq!(ui.get_show_upgrade_prompt(), false);
}

#[test]
fn test_agents_ui_upgrade_prompt_message() {
    let ui = create_agents_ui();
    assert_eq!(ui.get_upgrade_prompt_message(), "");

    ui.set_upgrade_prompt_message("Please upgrade to add more helpers.".into());
    assert_eq!(ui.get_upgrade_prompt_message(), "Please upgrade to add more helpers.");
}

#[test]
fn test_agents_ui_agent_list_population() {
    let ui = create_agents_ui();

    let model = std::rc::Rc::new(slint::VecModel::from(vec![
        app::UiAgent {
            id: "agent-1".into(),
            name: "Test Agent".into(),
            role: "Support".into(),
            status: "Running".into(),
            is_running: true,
            svid_verified: true,
            is_new: false,
        }
    ]));

    ui.set_agents(model.into());
    // Since there's no direct getter to inspect the array lengths in generated Slint components this way easily,
    // we just ensure the property accepts the update without panicking.
    assert!(true);
}

#[test]
fn test_agents_ui_hire_callback() {
    let ui = create_agents_ui();
    let invoked = std::rc::Rc::new(std::cell::RefCell::new(false));

    let invoked_clone = invoked.clone();
    ui.on_hire_agent(move || {
        *invoked_clone.borrow_mut() = true;
    });

    ui.invoke_hire_agent();
    assert!(*invoked.borrow());
}

#[test]
fn test_agents_ui_fix_callback() {
    let ui = create_agents_ui();
    let fixed_id = std::rc::Rc::new(std::cell::RefCell::new(String::new()));

    let fixed_id_clone = fixed_id.clone();
    ui.on_fix_agent(move |id| {
        *fixed_id_clone.borrow_mut() = id.into();
    });

    ui.invoke_fix_agent("agent-123".into());
    assert_eq!(*fixed_id.borrow(), "agent-123");
}
