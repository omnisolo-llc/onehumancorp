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
    let called_name = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c = called_name.clone();
    ui.on_deploy_agent(move |name, _, _| { *c.borrow_mut() = name.to_string(); });
    
    ui.invoke_deploy_agent("Robot".into(), "Cleaner".into(), "Local".into());
    assert_eq!(*called_name.borrow(), "Robot");
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

#[test] fn test_wizard_hire_onboarding_full_flow() {
    let ui = create();

    // Step 0: Initial state, button should be disabled because selected_role is empty
    ui.set_step(0);
    ui.set_selected_role("".into());
    assert!(!ui.get_next_enabled());

    // Select role
    ui.set_selected_role("Customer Support".into());
    assert!(ui.get_next_enabled());
    ui.set_agent_name("Support Bot".into());

    // Move to step 1
    ui.set_step(1);
    assert!(ui.get_next_enabled());

    // Select provider
    ui.set_selected_provider("openai".into());

    // Move to step 6 (Confirm Deployment)
    ui.set_step(6);

    // Test that the deploy button callback works properly
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let called_clone = called.clone();
    ui.on_deploy_agent(move |name, role, provider| {
        assert_eq!(name, "Support Bot");
        assert_eq!(role, "Customer Support");
        assert_eq!(provider, "openai");
        *called_clone.borrow_mut() = true;
    });

    // Trigger deploy
    ui.invoke_deploy_agent(
        ui.get_agent_name(),
        ui.get_selected_role(),
        ui.get_selected_provider()
    );

    assert!(*called.borrow());
}

#[test] fn test_wizard_hire_onboarding_navigation_flow() {
    let ui = create();

    // Check initial state
    assert_eq!(ui.get_step(), 0);
    assert!(!ui.get_next_enabled());

    ui.set_selected_role("Tester".into());
    assert!(ui.get_next_enabled());
    ui.set_agent_name("Test Agent".into());

    // Ensure all steps can be visited
    for step in 1..=6 {
        ui.set_step(step);
        assert_eq!(ui.get_step(), step);
        assert!(ui.get_next_enabled());
    }
}
