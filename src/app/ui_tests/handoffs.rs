use crate::app;
use slint::Model;
use std::rc::Rc;

fn create() -> app::Handoffs { crate::ui_tests::init(); app::Handoffs::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn handoffs_xss_intent() {
    let ui = create();
    let xss = "<script>alert('handoff')</script>";
    let handoffs = slint::VecModel::from(vec![
        app::UiHandoff {
            id: "1".into(),
            intent: xss.into(),
            agent_name: "Agent".into(),
            description: "desc".into(),
            date: "now".into(),
            status: "pending".into(),
        }
    ]);
    ui.set_handoffs(Rc::new(handoffs).into());
    assert_eq!(ui.get_handoffs().row_data(0).unwrap().intent, xss);
}

#[test] fn handoffs_injection_agent() {
    let ui = create();
    let inj = "Agent'); DROP TABLE handoffs; --";
    let handoffs = slint::VecModel::from(vec![
        app::UiHandoff {
            id: "2".into(),
            intent: "Escalation".into(),
            agent_name: inj.into(),
            description: "desc".into(),
            date: "today".into(),
            status: "new".into(),
        }
    ]);
    ui.set_handoffs(Rc::new(handoffs).into());
    assert_eq!(ui.get_handoffs().row_data(0).unwrap().agent_name, inj);
}

#[test] fn handoffs_massive_list() {
    let ui = create();
    let v: Vec<app::UiHandoff> = (0..300).map(|i| app::UiHandoff {
        id: format!("h-{}", i).into(),
        intent: "Transfer".into(),
        agent_name: "Bot".into(),
        description: "desc".into(),
        date: "2024".into(),
        status: "open".into(),
    }).collect();
    ui.set_handoffs(Rc::new(slint::VecModel::from(v)).into());
    assert_eq!(ui.get_handoffs().row_count(), 300);
}

// --- Interaction / Flow Tests ---

#[test] fn handoffs_flow_resolve_callback() {
    let ui = create();
    let called_id = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
    let called_action = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
    let c1 = called_id.clone();
    let c2 = called_action.clone();
    ui.on_resolve_handoff(move |id, action| {
        *c1.lock().unwrap() = id.to_string();
        *c2.lock().unwrap() = action.to_string();
    });
    
    ui.invoke_resolve_handoff("H123".into(), "approve".into());
    assert_eq!(*called_id.lock().unwrap(), "H123");
    assert_eq!(*called_action.lock().unwrap(), "approve");
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---
