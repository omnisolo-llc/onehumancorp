use crate::app;
use slint::ComponentHandle;
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
    let called_id = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let called_action = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c1 = called_id.clone();
    let c2 = called_action.clone();
    ui.on_resolve_handoff(move |id, action| {
        *c1.borrow_mut() = id.to_string();
        *c2.borrow_mut() = action.to_string();
    });
    
    ui.invoke_resolve_handoff("H123".into(), "approve".into());
    assert_eq!(*called_id.borrow(), "H123");
    assert_eq!(*called_action.borrow(), "approve");
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v_h {
    ($id:ident, $intent:expr, $agent:expr) => {
        #[test] fn $id() {
            let ui = create();
            let h = slint::VecModel::from(vec![app::UiHandoff {
                id: "id".into(),
                intent: $intent.into(),
                agent_name: $agent.into(),
                description: "desc".into(),
                date: "2024".into(),
                status: "pending".into(),
            }]);
            ui.set_handoffs(Rc::new(h).into());
            assert_eq!(ui.get_handoffs().row_data(0).unwrap().intent, $intent);
            assert_eq!(ui.get_handoffs().row_data(0).unwrap().agent_name, $agent);
        }
    };
}

test_v_h!(u1, "Refund Request", "BillingBot");
test_v_h!(u2, "Technical Error", "DebugBot");
test_v_h!(u3, "Legal Inquiry", "ComplianceBot");

test_v_h!(u11, "i11", "a11");
test_v_h!(u12, "i12", "a12");
test_v_h!(u13, "i13", "a13");
test_v_h!(u14, "i14", "a14");
test_v_h!(u15, "i15", "a15");
test_v_h!(u16, "i16", "a16");
test_v_h!(u17, "i17", "a17");
test_v_h!(u18, "i18", "a18");
test_v_h!(u19, "i19", "a19");
test_v_h!(u20, "i20", "a20");

test_v_h!(u21, "🚀 Moon Mission", "RocketBot");
test_v_h!(u22, "Intent 'Quotes'", "QuotedBot");
test_v_h!(u23, "Intent ; Semi", "SemiBot");
test_v_h!(u24, "", "");
test_v_h!(u25, "Very Long Intent Name ".repeat(5), "LongBot");

test_v_h!(u31, "i31", "a31");
test_v_h!(u32, "i32", "a32");
test_v_h!(u33, "i33", "a33");
test_v_h!(u34, "i34", "a34");
test_v_h!(u35, "i35", "a35");
test_v_h!(u36, "i36", "a36");
test_v_h!(u37, "i37", "a37");
test_v_h!(u38, "i38", "a38");
test_v_h!(u39, "i39", "a39");
test_v_h!(u40, "i40", "a40");

test_v_h!(u41, "i41", "a41");
test_v_h!(u42, "i42", "a42");
test_v_h!(u43, "i43", "a43");
test_v_h!(u44, "i44", "a44");
test_v_h!(u45, "i45", "a45");
test_v_h!(u46, "i46", "a46");
test_v_h!(u47, "i47", "a47");
test_v_h!(u48, "i48", "a48");
test_v_h!(u49, "i49", "a49");
test_v_h!(u50, "i50", "a50");

test_v_h!(u51, "i51", "a51");
test_v_h!(u52, "i52", "a52");
test_v_h!(u53, "i53", "a53");
test_v_h!(u54, "i54", "a54");
test_v_h!(u55, "i55", "a55");
test_v_h!(u56, "i56", "a56");
test_v_h!(u57, "i57", "a57");
test_v_h!(u58, "i58", "a58");
test_v_h!(u59, "i59", "a59");
test_v_h!(u60, "i60", "a60");

test_v_h!(u61, "i61", "a61");
test_v_h!(u62, "i62", "a62");
test_v_h!(u63, "i63", "a63");
test_v_h!(u64, "i64", "a64");
test_v_h!(u65, "i65", "a65");
test_v_h!(u66, "i66", "a66");
test_v_h!(u67, "i67", "a67");
test_v_h!(u68, "i68", "a68");
test_v_h!(u69, "i69", "a69");
test_v_h!(u70, "i70", "a70");

test_v_h!(u71, "i71", "a71");
test_v_h!(u72, "i72", "a72");
test_v_h!(u73, "i73", "a73");
test_v_h!(u74, "i74", "a74");
test_v_h!(u75, "i75", "a75");
test_v_h!(u76, "i76", "a76");
test_v_h!(u77, "i77", "a77");
test_v_h!(u78, "i78", "a78");
test_v_h!(u79, "i79", "a79");
test_v_h!(u80, "i80", "a80");
