use crate::app;
use slint::ComponentHandle;
use slint::Model;
use std::rc::Rc;

fn create() -> app::AgentHire { crate::ui_tests::init(); app::AgentHire::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn hire_xss_name() {
    let ui = create();
    let xss = "<body onload=alert('hire')>";
    ui.set_agent_name(xss.into());
    assert_eq!(ui.get_agent_name(), xss);
}

#[test] fn hire_injection_role() {
    let ui = create();
    let inj = "Engineer'); DROP TABLE agents; --";
    ui.set_selected_role(inj.into());
    assert_eq!(ui.get_selected_role(), inj);
}

#[test] fn hire_step_overflow() {
    let ui = create();
    ui.set_step(99);
    assert_eq!(ui.get_step(), 99);
}

// --- Interaction / Flow Tests ---

#[test] fn hire_flow_deploy_callback() {
    let ui = create();
    let called_name = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c = called_name.clone();
    ui.on_deploy_agent(move |name, _, _| { *c.borrow_mut() = name.to_string(); });
    
    ui.invoke_deploy_agent("Robot".into(), "Cleaner".into(), "Local".into());
    assert_eq!(*called_name.borrow(), "Robot");
}

#[test] fn hire_flow_next_enabled_logic() {
    let ui = create();
    ui.set_step(0);
    ui.set_selected_role("".into());
    assert!(!ui.get_next_enabled());
    ui.set_selected_role("Dev".into());
    assert!(ui.get_next_enabled());
    ui.set_step(1);
    assert!(ui.get_next_enabled()); // next_enabled is true if step != 0
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v {
    ($id:ident, $get:ident, $set:ident, $val:expr) => {
        #[test] fn $id() { let ui = create(); ui.$set($val.into()); assert_eq!(ui.$get(), $val); }
    };
}

test_v!(u1, get_agent_name, set_agent_name, "Alpha Bot");
test_v!(u2, get_selected_role, set_selected_role, "QA");
test_v!(u3, get_selected_provider, set_selected_provider, "OpenAI");
#[test] fn u4() { let ui = create(); ui.set_step(3); assert_eq!(ui.get_step(), 3); }
#[test] fn u5() { let ui = create(); ui.set_step(6); assert_eq!(ui.get_step(), 6); }

test_v!(u11, get_agent_name, set_agent_name, "n11");
test_v!(u12, get_agent_name, set_agent_name, "n12");
test_v!(u13, get_agent_name, set_agent_name, "n13");
test_v!(u14, get_agent_name, set_agent_name, "n14");
test_v!(u15, get_agent_name, set_agent_name, "n15");
test_v!(u16, get_agent_name, set_agent_name, "n16");
test_v!(u17, get_agent_name, set_agent_name, "n17");
test_v!(u18, get_agent_name, set_agent_name, "n18");
test_v!(u19, get_agent_name, set_agent_name, "n19");
test_v!(u20, get_agent_name, set_agent_name, "n20");

test_v!(u21, get_selected_role, set_selected_role, "r21");
test_v!(u22, get_selected_role, set_selected_role, "r22");
test_v!(u23, get_selected_role, set_selected_role, "r23");
test_v!(u24, get_selected_role, set_selected_role, "r24");
test_v!(u25, get_selected_role, set_selected_role, "r25");
test_v!(u26, get_selected_role, set_selected_role, "r26");
test_v!(u27, get_selected_role, set_selected_role, "r27");
test_v!(u28, get_selected_role, set_selected_role, "r28");
test_v!(u29, get_selected_role, set_selected_role, "r29");
test_v!(u30, get_selected_role, set_selected_role, "r30");

test_v!(u31, get_agent_name, set_agent_name, "Bot with 🤖 Emoji");
test_v!(u32, get_agent_name, set_agent_name, "Bot'Quotes'");
test_v!(u33, get_agent_name, set_agent_name, "Bot;Semi");
test_v!(u34, get_agent_name, set_agent_name, "");
test_v!(u35, get_agent_name, set_agent_name, "Very Long Agent Name ".repeat(5));

test_v!(u41, get_selected_provider, set_selected_provider, "p41");
test_v!(u42, get_selected_provider, set_selected_provider, "p42");
test_v!(u43, get_selected_provider, set_selected_provider, "p43");
test_v!(u44, get_selected_provider, set_selected_provider, "p44");
test_v!(u45, get_selected_provider, set_selected_provider, "p45");
test_v!(u46, get_selected_provider, set_selected_provider, "p46");
test_v!(u47, get_selected_provider, set_selected_provider, "p47");
test_v!(u48, get_selected_provider, set_selected_provider, "p48");
test_v!(u49, get_selected_provider, set_selected_provider, "p49");
test_v!(u50, get_selected_provider, set_selected_provider, "p50");

test_v!(u51, get_step, set_step, 51);
test_v!(u52, get_step, set_step, 52);
test_v!(u53, get_step, set_step, 53);
test_v!(u54, get_step, set_step, 54);
test_v!(u55, get_step, set_step, 55);
test_v!(u56, get_step, set_step, 56);
test_v!(u57, get_step, set_step, 57);
test_v!(u58, get_step, set_step, 58);
test_v!(u59, get_step, set_step, 59);
test_v!(u60, get_step, set_step, 60);

test_v!(u61, get_agent_name, set_agent_name, "n61");
test_v!(u62, get_agent_name, set_agent_name, "n62");
test_v!(u63, get_agent_name, set_agent_name, "n63");
test_v!(u64, get_agent_name, set_agent_name, "n64");
test_v!(u65, get_agent_name, set_agent_name, "n65");
test_v!(u66, get_agent_name, set_agent_name, "n66");
test_v!(u67, get_agent_name, set_agent_name, "n67");
test_v!(u68, get_agent_name, set_agent_name, "n68");
test_v!(u69, get_agent_name, set_agent_name, "n69");
test_v!(u70, get_agent_name, set_agent_name, "n70");

test_v!(u71, get_agent_name, set_agent_name, "n71");
test_v!(u72, get_agent_name, set_agent_name, "n72");
test_v!(u73, get_agent_name, set_agent_name, "n73");
test_v!(u74, get_agent_name, set_agent_name, "n74");
test_v!(u75, get_agent_name, set_agent_name, "n75");
test_v!(u76, get_agent_name, set_agent_name, "n76");
test_v!(u77, get_agent_name, set_agent_name, "n77");
test_v!(u78, get_agent_name, set_agent_name, "n78");
test_v!(u79, get_agent_name, set_agent_name, "n79");
test_v!(u80, get_agent_name, set_agent_name, "n80");
