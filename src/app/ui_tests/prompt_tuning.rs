use crate::app;
use slint::ComponentHandle;
use slint::Model;
use std::rc::Rc;

fn create() -> app::PromptTuning { crate::ui_tests::init(); app::PromptTuning::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn prompt_xss_tone() {
    let ui = create();
    let xss = "<script>alert('tone')</script>";
    ui.set_tone(xss.into());
    assert_eq!(ui.get_tone(), xss);
}

#[test] fn prompt_injection_example() {
    let ui = create();
    let inj = "Question'); DROP TABLE prompts; --";
    let model = slint::VecModel::from(vec![app::UiPromptExample {
        q: inj.into(),
        a: "Answer".into(),
    }]);
    ui.set_examples(Rc::new(model).into());
    assert_eq!(ui.get_examples().row_data(0).unwrap().q, inj);
}

#[test] fn prompt_step_bounds() {
    let ui = create();
    ui.set_step(10);
    assert_eq!(ui.get_step(), 10);
    ui.set_step(-5);
    assert_eq!(ui.get_step(), -5);
}

// --- Interaction / Flow Tests ---

#[test] fn prompt_flow_callbacks() {
    let ui = create();
    let c1 = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c2 = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c3 = std::rc::Rc::new(std::cell::RefCell::new(false));
    
    let w1 = c1.clone(); ui.on_add_example(move || { *w1.borrow_mut() = true; });
    let w2 = c2.clone(); ui.on_save_prompt(move || { *w2.borrow_mut() = true; });
    let w3 = c3.clone(); ui.on_save_state(move || { *w3.borrow_mut() = true; });
    
    ui.invoke_add_example(); assert!(*c1.borrow());
    ui.invoke_save_prompt(); assert!(*c2.borrow());
    ui.invoke_save_state(); assert!(*c3.borrow());
}

#[test] fn prompt_flow_step_logic() {
    let ui = create();
    ui.set_step(0);
    ui.invoke_next_step();
    assert_eq!(ui.get_step(), 1);
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v {
    ($id:ident, $get:ident, $set:ident, $val:expr) => {
        #[test] fn $id() { let ui = create(); ui.$set($val.into()); assert_eq!(ui.$get(), $val); }
    };
}

test_v!(u1, get_tone, set_tone, "Aggressive");
test_v!(u2, get_is_advanced, set_is_advanced, true);
test_v!(u3, get_focus_only_business, set_focus_only_business, true);

test_v!(u11, get_tone, set_tone, "t11");
test_v!(u12, get_tone, set_tone, "t12");
test_v!(u13, get_tone, set_tone, "t13");
test_v!(u14, get_tone, set_tone, "t14");
test_v!(u15, get_tone, set_tone, "t15");
test_v!(u16, get_tone, set_tone, "t16");
test_v!(u17, get_tone, set_tone, "t17");
test_v!(u18, get_tone, set_tone, "t18");
test_v!(u19, get_tone, set_tone, "t19");
test_v!(u20, get_tone, set_tone, "t20");

test_v!(u21, get_tone, set_tone, "Tone with 🎭 Emoji");
test_v!(u22, get_tone, set_tone, "Tone'Quotes'");
test_v!(u23, get_tone, set_tone, "Tone ; Semi");
test_v!(u24, get_tone, set_tone, "");
test_v!(u25, get_tone, set_tone, "Very Long Tone Name ".repeat(5));

test_v!(u31, get_step, set_step, 31);
test_v!(u32, get_step, set_step, 32);
test_v!(u33, get_step, set_step, 33);
test_v!(u34, get_step, set_step, 34);
test_v!(u35, get_step, set_step, 35);
test_v!(u36, get_step, set_step, 36);
test_v!(u37, get_step, set_step, 37);
test_v!(u38, get_step, set_step, 38);
test_v!(u39, get_step, set_step, 39);
test_v!(u40, get_step, set_step, 40);

test_v!(u41, get_focus_avoid_competitors, set_focus_avoid_competitors, true);
test_v!(u42, get_focus_avoid_competitors, set_focus_avoid_competitors, false);
test_v!(u43, get_focus_reply_spanish, set_focus_reply_spanish, true);
test_v!(u44, get_focus_reply_spanish, set_focus_reply_spanish, false);

test_v!(u51, get_tone, set_tone, "t51");
test_v!(u52, get_tone, set_tone, "t52");
test_v!(u53, get_tone, set_tone, "t53");
test_v!(u54, get_tone, set_tone, "t54");
test_v!(u55, get_tone, set_tone, "t55");
test_v!(u56, get_tone, set_tone, "t56");
test_v!(u57, get_tone, set_tone, "t57");
test_v!(u58, get_tone, set_tone, "t58");
test_v!(u59, get_tone, set_tone, "t59");
test_v!(u60, get_tone, set_tone, "t60");

test_v!(u61, get_tone, set_tone, "t61");
test_v!(u62, get_tone, set_tone, "t62");
test_v!(u63, get_tone, set_tone, "t63");
test_v!(u64, get_tone, set_tone, "t64");
test_v!(u65, get_tone, set_tone, "t65");
test_v!(u66, get_tone, set_tone, "t66");
test_v!(u67, get_tone, set_tone, "t67");
test_v!(u68, get_tone, set_tone, "t68");
test_v!(u69, get_tone, set_tone, "t69");
test_v!(u70, get_tone, set_tone, "t70");

test_v!(u71, get_tone, set_tone, "t71");
test_v!(u72, get_tone, set_tone, "t72");
test_v!(u73, get_tone, set_tone, "t73");
test_v!(u74, get_tone, set_tone, "t74");
test_v!(u75, get_tone, set_tone, "t75");
test_v!(u76, get_tone, set_tone, "t76");
test_v!(u77, get_tone, set_tone, "t77");
test_v!(u78, get_tone, set_tone, "t78");
test_v!(u79, get_tone, set_tone, "t79");
test_v!(u80, get_tone, set_tone, "t80");
