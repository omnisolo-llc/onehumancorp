use crate::app;
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
    ui.set_total_ai_usage(inj.into());
    assert_eq!(ui.get_total_ai_usage(), inj);
}

#[test] fn cost_massive_list() {
    let ui = create();
    let v: Vec<app::UiHelperCost> = (0..500).map(|i| app::UiHelperCost {
        name: format!("Helper {}", i).into(),
        cost: format!("${}", i).into(),
        roi: "High".into(),
        efficiency: "Good".into(),
        pct: (i % 100) as f32 / 100.0,
    }).collect();
    ui.set_helper_costs(Rc::new(slint::VecModel::from(v)).into());
    assert_eq!(ui.get_helper_costs().row_count(), 500);
}

// --- Unique Scenarios with Verification ---

#[test] fn cost_zero_cost_helper() {
    let ui = create();
    let v: Vec<app::UiHelperCost> = vec![app::UiHelperCost {
        name: "Local Ollama Helper".into(),
        cost: "$0.00".into(),
        roi: "0.00%".into(),
        efficiency: "0.00 AI/$".into(),
        pct: 0.0,
    }];
    ui.set_helper_costs(Rc::new(slint::VecModel::from(v)).into());
    let models = ui.get_helper_costs();
    assert_eq!(models.row_count(), 1);
    let helper = models.row_data(0).unwrap();
    assert_eq!(helper.name, "Local Ollama Helper");
    assert_eq!(helper.cost, "$0.00");
    assert_eq!(helper.roi, "0.00%");
    assert_eq!(helper.efficiency, "0.00 AI/$");
    assert_eq!(helper.pct, 0.0);
}

#[test] fn cost_zero_cost_multiple_helpers() {
    let ui = create();
    let v: Vec<app::UiHelperCost> = vec![
        app::UiHelperCost {
            name: "Cloud GPT-4 Helper".into(),
            cost: "$15.50".into(),
            roi: "150.00%".into(),
            efficiency: "32.50 AI/$".into(),
            pct: 1.0,
        },
        app::UiHelperCost {
            name: "Local Llama 3 Helper".into(),
            cost: "$0.00".into(),
            roi: "0.00%".into(),
            efficiency: "0.00 AI/$".into(),
            pct: 0.0,
        }
    ];
    ui.set_helper_costs(Rc::new(slint::VecModel::from(v)).into());
    let models = ui.get_helper_costs();
    assert_eq!(models.row_count(), 2);
    let zero_helper = models.row_data(1).unwrap();
    assert_eq!(zero_helper.name, "Local Llama 3 Helper");
    assert_eq!(zero_helper.cost, "$0.00");
    assert_eq!(zero_helper.roi, "0.00%");
    assert_eq!(zero_helper.efficiency, "0.00 AI/$");
    assert_eq!(zero_helper.pct, 0.0);
}

#[test] fn cost_total_spend_zero() {
    let ui = create();
    ui.set_total_spend("$0.00".into());
    assert_eq!(ui.get_total_spend(), "$0.00");
}

#[test] fn cost_zero_roi_no_division_by_zero_ui_check() {
    let ui = create();
    let v: Vec<app::UiHelperCost> = vec![app::UiHelperCost {
        name: "Zero ROI Helper".into(),
        cost: "$0.00".into(),
        roi: "0.00".into(), // Ensuring raw zero strings map directly
        efficiency: "0.00".into(),
        pct: 0.0,
    }];
    ui.set_helper_costs(Rc::new(slint::VecModel::from(v)).into());
    let models = ui.get_helper_costs();
    let helper = models.row_data(0).unwrap();
    assert_eq!(helper.roi, "0.00");
}

#[test] fn cost_zero_efficiency_no_division_by_zero_ui_check() {
    let ui = create();
    let v: Vec<app::UiHelperCost> = vec![app::UiHelperCost {
        name: "Zero Efficiency Helper".into(),
        cost: "$0.00".into(),
        roi: "0.00".into(),
        efficiency: "0.00".into(),
        pct: 0.0,
    }];
    ui.set_helper_costs(Rc::new(slint::VecModel::from(v)).into());
    let models = ui.get_helper_costs();
    let helper = models.row_data(0).unwrap();
    assert_eq!(helper.efficiency, "0.00");
}

// --- Consolidated Verified Tests ---
