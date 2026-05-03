use crate::app;
use slint::ComponentHandle;
use slint::Model;
use std::rc::Rc;

fn create() -> app::CostDashboard { crate::ui_tests::init(); app::CostDashboard::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn cost_xss_spend() {
    let ui = create();
    let xss = "<img src=x onerror=alert('spend')>";
    ui.set_total_spend(xss.into());
    assert_eq!(ui.get_total_spend(), xss);
}

#[test] fn cost_injection_tokens() {
    let ui = create();
    let inj = "1000000'); DROP TABLE tokens; --";
    ui.set_total_tokens(inj.into());
    assert_eq!(ui.get_total_tokens(), inj);
}

#[test] fn cost_massive_list() {
    let ui = create();
    let v: Vec<app::UiAgentCost> = (0..500).map(|i| app::UiAgentCost {
        name: format!("Agent {}", i).into(),
        cost: format!("${}", i).into(),
        roi: "High".into(),
        efficiency: "Good".into(),
        pct: (i % 100) as f32 / 100.0,
    }).collect();
    ui.set_agent_costs(Rc::new(slint::VecModel::from(v)).into());
    assert_eq!(ui.get_agent_costs().row_count(), 500);
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---
