use crate::app;

fn create() -> app::AgentHire { crate::ui_tests::init(); app::AgentHire::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn hire_xss_name() {
    let ui = create();
    let xss = "<body onload=alert('hire')>";
    ui.set_agent_name(xss.into());
    assert_eq!(ui.get_agent_name(), xss);
}

#[test] fn hire_injection_role() {
    let ui = create();
    let inj = "Engineer'); DROP TABLE agents; --";
    ui.set_selected_role(inj.into());
    assert_eq!(ui.get_selected_role(), inj);
}

#[test] fn hire_step_overflow() {
    let ui = create();
    ui.set_step(99);
    assert_eq!(ui.get_step(), 99);
}

// --- Interaction / Flow Tests ---

#[test] fn hire_flow_deploy_callback() {
    let ui = create();
    let called_name = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
    let c = called_name.clone();
    ui.on_deploy_agent(move |name, _, _| { *c.lock().unwrap() = name.to_string(); });
    
    ui.invoke_deploy_agent("Robot".into(), "Cleaner".into(), "Local".into());
    assert_eq!(*called_name.lock().unwrap(), "Robot");
}

#[test] fn hire_flow_next_enabled_logic() {
    let ui = create();
    ui.set_step(0);
    ui.set_selected_role("".into());
    assert!(!ui.get_next_enabled());
    ui.set_selected_role("Dev".into());
    assert!(ui.get_next_enabled());
    ui.set_step(1);
    assert!(ui.get_next_enabled()); // next_enabled is true if step != 0
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_agent_name() {
    let ui = create();
    ui.set_agent_name("Alpha Bot".into());
    assert_eq!(ui.get_agent_name(), "Alpha Bot");
    ui.set_agent_name("n11".into());
    assert_eq!(ui.get_agent_name(), "n11");
    ui.set_agent_name("n12".into());
    assert_eq!(ui.get_agent_name(), "n12");
}

#[test]
fn create_verify_selected_role() {
    let ui = create();
    ui.set_selected_role("QA".into());
    assert_eq!(ui.get_selected_role(), "QA");
    ui.set_selected_role("r21".into());
    assert_eq!(ui.get_selected_role(), "r21");
    ui.set_selected_role("r22".into());
    assert_eq!(ui.get_selected_role(), "r22");
}

#[test]
fn create_verify_selected_provider() {
    let ui = create();
    ui.set_selected_provider("OpenAI".into());
    assert_eq!(ui.get_selected_provider(), "OpenAI");
    ui.set_selected_provider("p41".into());
    assert_eq!(ui.get_selected_provider(), "p41");
    ui.set_selected_provider("p42".into());
    assert_eq!(ui.get_selected_provider(), "p42");
}

#[test]
fn create_verify_step() {
    let ui = create();
    ui.set_step(51);
    assert_eq!(ui.get_step(), 51);
    ui.set_step(52);
    assert_eq!(ui.get_step(), 52);
    ui.set_step(53);
    assert_eq!(ui.get_step(), 53);
}
