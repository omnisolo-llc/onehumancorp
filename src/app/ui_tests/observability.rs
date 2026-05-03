use crate::app;

use slint::Model;
use std::rc::Rc;

fn create() -> app::SwarmObservabilityWindow { crate::ui_tests::init(); app::SwarmObservabilityWindow::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn obs_xss_content() {
    let ui = create();
    let xss = "<script>alert('obs')</script>";
    let msgs = slint::VecModel::from(vec![
        app::UiMeshMessage {
            id: "1".into(),
            content: xss.into(),
        }
    ]);
    ui.set_messages(Rc::new(msgs).into());
    assert_eq!(ui.get_messages().row_data(0).unwrap().content, xss);
}

#[test] fn obs_injection_id() {
    let ui = create();
    let inj = "msg'); DROP TABLE mesh; --";
    let msgs = slint::VecModel::from(vec![
        app::UiMeshMessage {
            id: inj.into(),
            content: "Ping".into(),
        }
    ]);
    ui.set_messages(Rc::new(msgs).into());
    assert_eq!(ui.get_messages().row_data(0).unwrap().id, inj);
}

#[test] fn obs_massive_list() {
    let ui = create();
    let v: Vec<app::UiMeshMessage> = (0..500).map(|i| app::UiMeshMessage {
        id: format!("m-{}", i).into(),
        content: format!("Message content for {}", i).into(),
    }).collect();
    ui.set_messages(Rc::new(slint::VecModel::from(v)).into());
    assert_eq!(ui.get_messages().row_count(), 500);
}

// --- Interaction / Flow Tests ---

#[test] fn obs_flow_empty_check() {
    let ui = create();
    ui.set_messages(Rc::new(slint::VecModel::default()).into());
    assert_eq!(ui.get_messages().row_count(), 0);
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---
