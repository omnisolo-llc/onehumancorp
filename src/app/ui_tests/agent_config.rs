use crate::app;
use slint::ComponentHandle;

fn create() -> app::AgentConfig { crate::ui_tests::init(); app::AgentConfig::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn agentcfg_xss_name() {
    let ui = create();
    let xss = "<script>alert('agentcfg')</script>";
    ui.set_selected_agent(xss.into());
    assert_eq!(ui.get_selected_agent(), xss);
}

#[test] fn agentcfg_step_bounds() {
    let ui = create();
    ui.set_step(10);
    assert_eq!(ui.get_step(), 10);
    ui.set_step(-5);
    assert_eq!(ui.get_step(), -5);
}

#[test] fn agentcfg_freq_bounds() {
    let ui = create();
    ui.set_frequency_value(5.0);
    assert_eq!(ui.get_frequency_value(), 5.0);
}

// --- Interaction / Flow Tests ---

#[test] fn agentcfg_flow_activate_callback() {
    let ui = create();
    let called_agent = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c = called_agent.clone();
    ui.on_activate_agent(move |name, _, _, _, _, _| { *c.borrow_mut() = name.to_string(); });

    ui.set_selected_agent("Robot".into());
    ui.invoke_activate_agent("Robot".into(), true, false, false, false, "Daily".into());
    assert_eq!(*called_agent.borrow(), "Robot");
}

#[test] fn agentcfg_flow_toast() {
    let ui = create();
    ui.set_show_toast(true);
    assert!(ui.get_show_toast());
    ui.set_show_toast(false);
    assert!(!ui.get_show_toast());
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v {
    ($id:ident, $get:ident, $set:ident, $val:expr) => {
        #[test] fn $id() { let ui = create(); ui.$set($val.into()); assert_eq!(ui.$get(), $val); }
    };
}

test_v!(u1, get_selected_agent, set_selected_agent, "Data Scientist");
test_v!(u2, get_can_reply, set_can_reply, true);
test_v!(u3, get_can_social, set_can_social, true);

test_v!(u11, get_selected_agent, set_selected_agent, "a11");
test_v!(u12, get_selected_agent, set_selected_agent, "a12");
test_v!(u13, get_selected_agent, set_selected_agent, "a13");
test_v!(u14, get_selected_agent, set_selected_agent, "a14");
test_v!(u15, get_selected_agent, set_selected_agent, "a15");
test_v!(u16, get_selected_agent, set_selected_agent, "a16");
test_v!(u17, get_selected_agent, set_selected_agent, "a17");
test_v!(u18, get_selected_agent, set_selected_agent, "a18");
test_v!(u19, get_selected_agent, set_selected_agent, "a19");
test_v!(u20, get_selected_agent, set_selected_agent, "a20");

test_v!(u21, get_selected_agent, set_selected_agent, "Agent with 🤖 Emoji");
test_v!(u22, get_selected_agent, set_selected_agent, "Agent'Quotes'");
test_v!(u23, get_selected_agent, set_selected_agent, "Agent ; Semi");
test_v!(u24, get_selected_agent, set_selected_agent, "");
test_v!(u25, get_selected_agent, set_selected_agent, "Very Long Agent Name ".repeat(5));

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

test_v!(u41, get_frequency_value, set_frequency_value, 0.5);
test_v!(u42, get_frequency_value, set_frequency_value, 1.5);
test_v!(u43, get_frequency_value, set_frequency_value, 2.5);
test_v!(u44, get_is_advanced, set_is_advanced, true);

test_v!(u51, get_selected_agent, set_selected_agent, "a51");
test_v!(u52, get_selected_agent, set_selected_agent, "a52");
test_v!(u53, get_selected_agent, set_selected_agent, "a53");
test_v!(u54, get_selected_agent, set_selected_agent, "a54");
test_v!(u55, get_selected_agent, set_selected_agent, "a55");
test_v!(u56, get_selected_agent, set_selected_agent, "a56");
test_v!(u57, get_selected_agent, set_selected_agent, "a57");
test_v!(u58, get_selected_agent, set_selected_agent, "a58");
test_v!(u59, get_selected_agent, set_selected_agent, "a59");
test_v!(u60, get_selected_agent, set_selected_agent, "a60");

test_v!(u61, get_selected_agent, set_selected_agent, "a61");
test_v!(u62, get_selected_agent, set_selected_agent, "a62");
test_v!(u63, get_selected_agent, set_selected_agent, "a63");
test_v!(u64, get_selected_agent, set_selected_agent, "a64");
test_v!(u65, get_selected_agent, set_selected_agent, "a65");
test_v!(u66, get_selected_agent, set_selected_agent, "a66");
test_v!(u67, get_selected_agent, set_selected_agent, "a67");
test_v!(u68, get_selected_agent, set_selected_agent, "a68");
test_v!(u69, get_selected_agent, set_selected_agent, "a69");
test_v!(u70, get_selected_agent, set_selected_agent, "a70");

test_v!(u71, get_selected_agent, set_selected_agent, "a71");
test_v!(u72, get_selected_agent, set_selected_agent, "a72");
test_v!(u73, get_selected_agent, set_selected_agent, "a73");
test_v!(u74, get_selected_agent, set_selected_agent, "a74");
test_v!(u75, get_selected_agent, set_selected_agent, "a75");
test_v!(u76, get_selected_agent, set_selected_agent, "a76");
test_v!(u77, get_selected_agent, set_selected_agent, "a77");
test_v!(u78, get_selected_agent, set_selected_agent, "a78");
test_v!(u79, get_selected_agent, set_selected_agent, "a79");
test_v!(u80, get_selected_agent, set_selected_agent, "a80");
