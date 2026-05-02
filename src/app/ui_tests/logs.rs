use crate::app;
use slint::ComponentHandle;
use slint::Model;
use std::rc::Rc;

fn create() -> app::Logs { crate::ui_tests::init(); app::Logs::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn logs_xss_text() {
    let ui = create();
    let xss = "<script>alert('log')</script>";
    let model = slint::VecModel::from(vec![app::UiLogLine {
        index: 1,
        text: xss.into(),
        color: slint::Color::from_rgb_u8(255, 255, 255),
    }]);
    ui.set_logs(Rc::new(model).into());
    assert_eq!(ui.get_logs().row_data(0).unwrap().text, xss);
}

#[test] fn logs_injection_text() {
    let ui = create();
    let inj = "Log content'); DROP TABLE logs; --";
    let model = slint::VecModel::from(vec![app::UiLogLine {
        index: 1,
        text: inj.into(),
        color: slint::Color::from_rgb_u8(255, 255, 255),
    }]);
    ui.set_logs(Rc::new(model).into());
    assert_eq!(ui.get_logs().row_data(0).unwrap().text, inj);
}

#[test] fn logs_massive_list() {
    let ui = create();
    let v: Vec<app::UiLogLine> = (0..500).map(|i| app::UiLogLine {
        index: i as i32,
        text: format!("Log line {}", i).into(),
        color: slint::Color::from_rgb_u8(200, 200, 200),
    }).collect();
    ui.set_logs(Rc::new(slint::VecModel::from(v)).into());
    assert_eq!(ui.get_logs().row_count(), 500);
}

// --- Interaction / Flow Tests ---

#[test] fn logs_flow_refresh_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_refresh(move || { *c.borrow_mut() = true; });
    ui.invoke_refresh();
    assert!(*called.borrow());
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v_l {
    ($id:ident, $text:expr) => {
        #[test] fn $id() {
            let ui = create();
            let model = slint::VecModel::from(vec![app::UiLogLine {
                index: 0,
                text: $text.into(),
                color: slint::Color::from_rgb_u8(0, 255, 0),
            }]);
            ui.set_logs(Rc::new(model).into());
            assert_eq!(ui.get_logs().row_data(0).unwrap().text, $text);
        }
    };
}

test_v_l!(u1, "Service started");
test_v_l!(u2, "Connection established");
test_v_l!(u3, "Task processed");

test_v_l!(u11, "l11");
test_v_l!(u12, "l12");
test_v_l!(u13, "l13");
test_v_l!(u14, "l14");
test_v_l!(u15, "l15");
test_v_l!(u16, "l16");
test_v_l!(u17, "l17");
test_v_l!(u18, "l18");
test_v_l!(u19, "l19");
test_v_l!(u20, "l20");

test_v_l!(u21, "Log with 📝 Emoji");
test_v_l!(u22, "Log'Quotes'");
test_v_l!(u23, "Log ; Semi");
test_v_l!(u24, "");
test_v_l!(u25, "Very Long Log Line Content ".repeat(10));

test_v_l!(u31, "l31");
test_v_l!(u32, "l32");
test_v_l!(u33, "l33");
test_v_l!(u34, "l34");
test_v_l!(u35, "l35");
test_v_l!(u36, "l36");
test_v_l!(u37, "l37");
test_v_l!(u38, "l38");
test_v_l!(u39, "l39");
test_v_l!(u40, "l40");

test_v_l!(u41, "l41");
test_v_l!(u42, "l42");
test_v_l!(u43, "l43");
test_v_l!(u44, "l44");
test_v_l!(u45, "l45");
test_v_l!(u46, "l46");
test_v_l!(u47, "l47");
test_v_l!(u48, "l48");
test_v_l!(u49, "l49");
test_v_l!(u50, "l50");

test_v_l!(u51, "l51");
test_v_l!(u52, "l52");
test_v_l!(u53, "l53");
test_v_l!(u54, "l54");
test_v_l!(u55, "l55");
test_v_l!(u56, "l56");
test_v_l!(u57, "l57");
test_v_l!(u58, "l58");
test_v_l!(u59, "l59");
test_v_l!(u60, "l60");

test_v_l!(u61, "l61");
test_v_l!(u62, "l62");
test_v_l!(u63, "l63");
test_v_l!(u64, "l64");
test_v_l!(u65, "l65");
test_v_l!(u66, "l66");
test_v_l!(u67, "l67");
test_v_l!(u68, "l68");
test_v_l!(u69, "l69");
test_v_l!(u70, "l70");

test_v_l!(u71, "l71");
test_v_l!(u72, "l72");
test_v_l!(u73, "l73");
test_v_l!(u74, "l74");
test_v_l!(u75, "l75");
test_v_l!(u76, "l76");
test_v_l!(u77, "l77");
test_v_l!(u78, "l78");
test_v_l!(u79, "l79");
test_v_l!(u80, "l80");
