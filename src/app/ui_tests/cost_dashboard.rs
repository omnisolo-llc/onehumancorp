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

macro_rules! test_v_c {
    ($id:ident, $name:expr) => {
        #[test] fn $id() {
            let ui = create();
            let model = slint::VecModel::from(vec![app::UiAgentCost {
                name: $name.into(),
                cost: "$1".into(),
                roi: "1".into(),
                efficiency: "1".into(),
                pct: 0.5,
            }]);
            ui.set_agent_costs(Rc::new(model).into());
            assert_eq!(ui.get_agent_costs().row_data(0).unwrap().name, $name);
        }
    };
}

test_v_c!(u1, "Marketing Bot");
test_v_c!(u2, "Dev Bot");
test_v_c!(u3, "Support Bot");

test_v_c!(u11, "a11");
test_v_c!(u12, "a12");
test_v_c!(u13, "a13");
test_v_c!(u14, "a14");
test_v_c!(u15, "a15");
test_v_c!(u16, "a16");
test_v_c!(u17, "a17");
test_v_c!(u18, "a18");
test_v_c!(u19, "a19");
test_v_c!(u20, "a20");

test_v_c!(u21, "Agent with 💰 Emoji");
test_v_c!(u22, "Agent'Quotes'");
test_v_c!(u23, "Agent ; Semi");
test_v_c!(u24, "");
test_v_c!(u25, "Very Long Agent Name ".repeat(5));

test_v_c!(u31, "a31");
test_v_c!(u32, "a32");
test_v_c!(u33, "a33");
test_v_c!(u34, "a34");
test_v_c!(u35, "a35");
test_v_c!(u36, "a36");
test_v_c!(u37, "a37");
test_v_c!(u38, "a38");
test_v_c!(u39, "a39");
test_v_c!(u40, "a40");

test_v_c!(u41, "a41");
test_v_c!(u42, "a42");
test_v_c!(u43, "a43");
test_v_c!(u44, "a44");
test_v_c!(u45, "a45");
test_v_c!(u46, "a46");
test_v_c!(u47, "a47");
test_v_c!(u48, "a48");
test_v_c!(u49, "a49");
test_v_c!(u50, "a50");

test_v_c!(u51, "a51");
test_v_c!(u52, "a52");
test_v_c!(u53, "a53");
test_v_c!(u54, "a54");
test_v_c!(u55, "a55");
test_v_c!(u56, "a56");
test_v_c!(u57, "a57");
test_v_c!(u58, "a58");
test_v_c!(u59, "a59");
test_v_c!(u60, "a60");

test_v_c!(u61, "a61");
test_v_c!(u62, "a62");
test_v_c!(u63, "a63");
test_v_c!(u64, "a64");
test_v_c!(u65, "a65");
test_v_c!(u66, "a66");
test_v_c!(u67, "a67");
test_v_c!(u68, "a68");
test_v_c!(u69, "a69");
test_v_c!(u70, "a70");

test_v_c!(u71, "a71");
test_v_c!(u72, "a72");
test_v_c!(u73, "a73");
test_v_c!(u74, "a74");
test_v_c!(u75, "a75");
test_v_c!(u76, "a76");
test_v_c!(u77, "a77");
test_v_c!(u78, "a78");
test_v_c!(u79, "a79");
test_v_c!(u80, "a80");
