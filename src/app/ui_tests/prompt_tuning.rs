use crate::app;
use slint::Model;
use std::rc::Rc;

fn create() -> app::PromptTuning {
    crate::ui_tests::init();
    app::PromptTuning::new().unwrap()
}

// --- Hacking / Corner Cases ---

#[test]
fn prompt_xss_tone() {
    let ui = create();
    let xss = "<script>alert('tone')</script>";
    ui.set_tone(xss.into());
    assert_eq!(ui.get_tone(), xss);
}

#[test]
fn prompt_injection_example() {
    let ui = create();
    let inj = "Question'); DROP TABLE prompts; --";
    let model = slint::VecModel::from(vec![app::UiPromptExample {
        q: inj.into(),
        a: "Answer".into(),
    }]);
    ui.set_examples(Rc::new(model).into());
    assert_eq!(ui.get_examples().row_data(0).unwrap().q, inj);
}

#[test]
fn prompt_step_bounds() {
    let ui = create();
    ui.set_step(10);
    assert_eq!(ui.get_step(), 10);
    ui.set_step(-5);
    assert_eq!(ui.get_step(), -5);
}

// --- Interaction / Flow Tests ---

#[test]
fn prompt_flow_callbacks() {
    let ui = create();
    let c1 = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c2 = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c3 = std::rc::Rc::new(std::cell::RefCell::new(false));

    let w1 = c1.clone();
    ui.on_add_example(move || {
        *w1.borrow_mut() = true;
    });
    let w2 = c2.clone();
    ui.on_save_prompt(move || {
        *w2.borrow_mut() = true;
    });
    let w3 = c3.clone();
    ui.on_save_state(move || {
        *w3.borrow_mut() = true;
    });

    ui.invoke_add_example();
    assert!(*c1.borrow());
    ui.invoke_save_prompt();
    assert!(*c2.borrow());
    ui.invoke_save_state();
    assert!(*c3.borrow());
}

#[test]
fn prompt_flow_step_logic() {
    let ui = create();
    ui.set_step(0);
    ui.invoke_next_step();
    assert_eq!(ui.get_step(), 1);
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_tone() {
    let ui = create();
    ui.set_tone("Aggressive".into());
    assert_eq!(ui.get_tone(), "Aggressive");
    ui.set_tone("t11".into());
    assert_eq!(ui.get_tone(), "t11");
    ui.set_tone("t12".into());
    assert_eq!(ui.get_tone(), "t12");
}

#[test]
fn create_verify_is_advanced() {
    let ui = create();
    ui.set_is_advanced(true);
    assert_eq!(ui.get_is_advanced(), true);
}

#[test]
fn create_verify_focus_only_business() {
    let ui = create();
    ui.set_focus_only_business(true);
    assert_eq!(ui.get_focus_only_business(), true);
}

#[test]
fn create_verify_step() {
    let ui = create();
    ui.set_step(31);
    assert_eq!(ui.get_step(), 31);
    ui.set_step(32);
    assert_eq!(ui.get_step(), 32);
    ui.set_step(33);
    assert_eq!(ui.get_step(), 33);
}

#[test]
fn create_verify_focus_avoid_competitors() {
    let ui = create();
    ui.set_focus_avoid_competitors(true);
    assert_eq!(ui.get_focus_avoid_competitors(), true);
    ui.set_focus_avoid_competitors(false);
    assert_eq!(ui.get_focus_avoid_competitors(), false);
}

#[test]
fn create_verify_focus_reply_spanish() {
    let ui = create();
    ui.set_focus_reply_spanish(true);
    assert_eq!(ui.get_focus_reply_spanish(), true);
    ui.set_focus_reply_spanish(false);
    assert_eq!(ui.get_focus_reply_spanish(), false);
}
