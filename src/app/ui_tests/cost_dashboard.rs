use crate::app;
use slint::Model;
use std::rc::Rc;

fn create() -> app::CostDashboard {
    crate::ui_tests::init();
    app::CostDashboard::new().unwrap()
}

// --- Hacking / Corner Cases ---

#[test]
fn cost_xss_spend() {
    let ui = create();
    let xss = "<img src=x onerror=alert('spend')>";
    ui.set_total_spend(xss.into());
    assert_eq!(ui.get_total_spend(), xss);
}

#[test]
fn cost_injection_tokens() {
    let ui = create();
    let inj = "1000000'); DROP TABLE tokens; --";
    ui.set_total_tokens(inj.into());
    assert_eq!(ui.get_total_tokens(), inj);
}

#[test]
fn cost_massive_list() {
    let ui = create();
    let v: Vec<app::UiAgentCost> = (0..500)
        .map(|i| app::UiAgentCost {
            name: format!("Agent {}", i).into(),
            cost: format!("${}", i).into(),
            roi: "High".into(),
            efficiency: "Good".into(),
            pct: (i % 100) as f32 / 100.0,
        })
        .collect();
    ui.set_agent_costs(Rc::new(slint::VecModel::from(v)).into());
    assert_eq!(ui.get_agent_costs().row_count(), 500);
}

// --- Unique Scenarios with Verification ---

#[test]
fn cost_zero_cost_agent() {
    let ui = create();
    let v: Vec<app::UiAgentCost> = vec![app::UiAgentCost {
        name: "Local Ollama Agent".into(),
        cost: "$0.00".into(),
        roi: "0.00%".into(),
        efficiency: "0.00 tok/$".into(),
        pct: 0.0,
    }];
    ui.set_agent_costs(Rc::new(slint::VecModel::from(v)).into());
    let models = ui.get_agent_costs();
    assert_eq!(models.row_count(), 1);
    let agent = models.row_data(0).unwrap();
    assert_eq!(agent.name, "Local Ollama Agent");
    assert_eq!(agent.cost, "$0.00");
    assert_eq!(agent.roi, "0.00%");
    assert_eq!(agent.efficiency, "0.00 tok/$");
    assert_eq!(agent.pct, 0.0);
}

#[test]
fn cost_zero_cost_multiple_agents() {
    let ui = create();
    let v: Vec<app::UiAgentCost> = vec![
        app::UiAgentCost {
            name: "Cloud GPT-4 Agent".into(),
            cost: "$15.50".into(),
            roi: "150.00%".into(),
            efficiency: "32.50 tok/$".into(),
            pct: 1.0,
        },
        app::UiAgentCost {
            name: "Local Llama 3 Agent".into(),
            cost: "$0.00".into(),
            roi: "0.00%".into(),
            efficiency: "0.00 tok/$".into(),
            pct: 0.0,
        },
    ];
    ui.set_agent_costs(Rc::new(slint::VecModel::from(v)).into());
    let models = ui.get_agent_costs();
    assert_eq!(models.row_count(), 2);
    let zero_agent = models.row_data(1).unwrap();
    assert_eq!(zero_agent.name, "Local Llama 3 Agent");
    assert_eq!(zero_agent.cost, "$0.00");
    assert_eq!(zero_agent.roi, "0.00%");
    assert_eq!(zero_agent.efficiency, "0.00 tok/$");
    assert_eq!(zero_agent.pct, 0.0);
}

#[test]
fn cost_total_spend_zero() {
    let ui = create();
    ui.set_total_spend("$0.00".into());
    assert_eq!(ui.get_total_spend(), "$0.00");
}

#[test]
fn cost_zero_roi_no_division_by_zero_ui_check() {
    let ui = create();
    let v: Vec<app::UiAgentCost> = vec![app::UiAgentCost {
        name: "Zero ROI Agent".into(),
        cost: "$0.00".into(),
        roi: "0.00".into(), // Ensuring raw zero strings map directly
        efficiency: "0.00".into(),
        pct: 0.0,
    }];
    ui.set_agent_costs(Rc::new(slint::VecModel::from(v)).into());
    let models = ui.get_agent_costs();
    let agent = models.row_data(0).unwrap();
    assert_eq!(agent.roi, "0.00");
}

#[test]
fn cost_zero_efficiency_no_division_by_zero_ui_check() {
    let ui = create();
    let v: Vec<app::UiAgentCost> = vec![app::UiAgentCost {
        name: "Zero Efficiency Agent".into(),
        cost: "$0.00".into(),
        roi: "0.00".into(),
        efficiency: "0.00".into(),
        pct: 0.0,
    }];
    ui.set_agent_costs(Rc::new(slint::VecModel::from(v)).into());
    let models = ui.get_agent_costs();
    let agent = models.row_data(0).unwrap();
    assert_eq!(agent.efficiency, "0.00");
}

// --- Consolidated Verified Tests ---
