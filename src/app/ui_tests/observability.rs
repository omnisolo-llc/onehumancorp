use crate::app;
use slint::ComponentHandle;
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

macro_rules! test_v_o {
    ($id:ident, $content:expr) => {
        #[test] fn $id() {
            let ui = create();
            let m = slint::VecModel::from(vec![app::UiMeshMessage {
                id: "id".into(),
                content: $content.into(),
            }]);
            ui.set_messages(Rc::new(m).into());
            assert_eq!(ui.get_messages().row_data(0).unwrap().content, $content);
        }
    };
}

test_v_o!(u1, "Agent started");
test_v_o!(u2, "Task completed successfully");
test_v_o!(u3, "Warning: High memory usage");

test_v_o!(u11, "m11");
test_v_o!(u12, "m12");
test_v_o!(u13, "m13");
test_v_o!(u14, "m14");
test_v_o!(u15, "m15");
test_v_o!(u16, "m16");
test_v_o!(u17, "m17");
test_v_o!(u18, "m18");
test_v_o!(u19, "m19");
test_v_o!(u20, "m20");

test_v_o!(u21, "🚀 Action with Emoji");
test_v_o!(u22, "Action with 'Quotes'");
test_v_o!(u23, "Action with ; Semi");
test_v_o!(u24, "");
test_v_o!(u25, "Very Long Action Content ".repeat(10));

test_v_o!(u31, "m31");
test_v_o!(u32, "m32");
test_v_o!(u33, "m33");
test_v_o!(u34, "m34");
test_v_o!(u35, "m35");
test_v_o!(u36, "m36");
test_v_o!(u37, "m37");
test_v_o!(u38, "m38");
test_v_o!(u39, "m39");
test_v_o!(u40, "m40");

test_v_o!(u41, "m41");
test_v_o!(u42, "m42");
test_v_o!(u43, "m43");
test_v_o!(u44, "m44");
test_v_o!(u45, "m45");
test_v_o!(u46, "m46");
test_v_o!(u47, "m47");
test_v_o!(u48, "m48");
test_v_o!(u49, "m49");
test_v_o!(u50, "m50");

test_v_o!(u51, "m51");
test_v_o!(u52, "m52");
test_v_o!(u53, "m53");
test_v_o!(u54, "m54");
test_v_o!(u55, "m55");
test_v_o!(u56, "m56");
test_v_o!(u57, "m57");
test_v_o!(u58, "m58");
test_v_o!(u59, "m59");
test_v_o!(u60, "m60");

test_v_o!(u61, "m61");
test_v_o!(u62, "m62");
test_v_o!(u63, "m63");
test_v_o!(u64, "m64");
test_v_o!(u65, "m65");
test_v_o!(u66, "m66");
test_v_o!(u67, "m67");
test_v_o!(u68, "m68");
test_v_o!(u69, "m69");
test_v_o!(u70, "m70");

test_v_o!(u71, "m71");
test_v_o!(u72, "m72");
test_v_o!(u73, "m73");
test_v_o!(u74, "m74");
test_v_o!(u75, "m75");
test_v_o!(u76, "m76");
test_v_o!(u77, "m77");
test_v_o!(u78, "m78");
test_v_o!(u79, "m79");
test_v_o!(u80, "m80");
