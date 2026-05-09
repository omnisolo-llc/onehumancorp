use crate::app;
use slint::Model;
use std::rc::Rc;

fn create() -> app::Logs {
    crate::ui_tests::init();
    app::Logs::new().unwrap()
}

// --- Hacking / Corner Cases ---

#[test]
fn logs_xss_text() {
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

#[test]
fn logs_injection_text() {
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

#[test]
fn logs_massive_list() {
    let ui = create();
    let v: Vec<app::UiLogLine> = (0..500)
        .map(|i| app::UiLogLine {
            index: i as i32,
            text: format!("Log line {}", i).into(),
            color: slint::Color::from_rgb_u8(200, 200, 200),
        })
        .collect();
    ui.set_logs(Rc::new(slint::VecModel::from(v)).into());
    assert_eq!(ui.get_logs().row_count(), 500);
}

// --- Interaction / Flow Tests ---

#[test]
fn logs_flow_refresh_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_refresh(move || {
        *c.borrow_mut() = true;
    });
    ui.invoke_refresh();
    assert!(*called.borrow());
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---
