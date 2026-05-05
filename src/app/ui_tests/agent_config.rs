use crate::app;

fn create() -> app::AgentConfig { crate::ui_tests::init(); app::AgentConfig::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn agentcfg_xss_name() {
    let ui = create();
    let xss = "<script>alert('agentcfg')</script>";
    ui.set_selected_helper(xss.into());
    assert_eq!(ui.get_selected_helper(), xss);
}

#[test] fn agentcfg_step_bounds() {
    let ui = create();
    ui.set_step(10);
    assert_eq!(ui.get_step(), 10);
    ui.set_step(-5);
    assert_eq!(ui.get_step(), -5);
}

#[test] fn agentcfg_freq_bounds() {
    let ui = create();
    ui.set_frequency_value(5.0);
    assert_eq!(ui.get_frequency_value(), 5.0);
}

// --- Interaction / Flow Tests ---

#[test] fn agentcfg_flow_activate_callback() {
    let ui = create();
    let called_agent = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c = called_agent.clone();
    ui.on_activate_helper(move |name, _, _, _, _, _| { *c.borrow_mut() = name.to_string(); });

    ui.set_selected_helper("Robot".into());
    ui.invoke_activate_helper("Robot".into(), true, false, false, false, "Daily".into());
    assert_eq!(*called_agent.borrow(), "Robot");
}

#[test] fn agentcfg_flow_toast() {
    let ui = create();
    ui.set_show_toast(true);
    assert!(ui.get_show_toast());
    ui.set_show_toast(false);
    assert!(!ui.get_show_toast());
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_selected_helper() {
    let ui = create();
    ui.set_selected_helper("Data Scientist".into());
    assert_eq!(ui.get_selected_helper(), "Data Scientist");
    ui.set_selected_helper("a11".into());
    assert_eq!(ui.get_selected_helper(), "a11");
    ui.set_selected_helper("a12".into());
    assert_eq!(ui.get_selected_helper(), "a12");
}

#[test]
fn create_verify_can_reply() {
    let ui = create();
    ui.set_can_reply(true);
    assert_eq!(ui.get_can_reply(), true);
}

#[test]
fn create_verify_can_social() {
    let ui = create();
    ui.set_can_social(true);
    assert_eq!(ui.get_can_social(), true);
}

#[test]
fn create_verify_step() {
    let ui = create();
    ui.set_step(31);
    assert_eq!(ui.get_step(), 31);
    ui.set_step(32);
    assert_eq!(ui.get_step(), 32);
    ui.set_step(33);
    assert_eq!(ui.get_step(), 33);
}

#[test]
fn create_verify_frequency_value() {
    let ui = create();
    ui.set_frequency_value(0.5);
    assert_eq!(ui.get_frequency_value(), 0.5);
    ui.set_frequency_value(1.5);
    assert_eq!(ui.get_frequency_value(), 1.5);
    ui.set_frequency_value(2.5);
    assert_eq!(ui.get_frequency_value(), 2.5);
}

#[test]
fn create_verify_is_advanced() {
    let ui = create();
    ui.set_is_advanced(true);
    assert_eq!(ui.get_is_advanced(), true);
}
