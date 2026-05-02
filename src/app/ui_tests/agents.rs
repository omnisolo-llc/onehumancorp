use crate::app;
use slint::ComponentHandle;

fn create_a() -> app::Agents { crate::ui_tests::init(); app::Agents::new().unwrap() }
fn create_c() -> app::AgentConfig { crate::ui_tests::init(); app::AgentConfig::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn agent_name_injection() {
    let ui = create_c();
    let inj = "Admin'; DROP TABLE agents; --";
    ui.set_selected_agent(inj.into());
    assert_eq!(ui.get_selected_agent(), inj);
}

#[test] fn agent_freq_oob() {
    let ui = create_c();
    ui.set_frequency_value(2.0);
    assert_eq!(ui.get_frequency_value(), 2.0);
    ui.set_frequency_value(-1.0);
    assert_eq!(ui.get_frequency_value(), -1.0);
}

#[test] fn agent_xss_toast() {
    let ui = create_c();
    let xss = "<script>console.log(1)</script>";
    ui.set_selected_agent(xss.into());
    assert_eq!(ui.get_selected_agent(), xss);
}

// --- Interaction / Flow Tests ---

#[test] fn agent_config_permutation_flow() {
    let ui = create_c();
    let flags = [true, false];
    for f1 in flags {
        for f2 in flags {
            ui.set_can_reply(f1);
            ui.set_can_social(f2);
            assert_eq!(ui.get_can_reply(), f1);
            assert_eq!(ui.get_can_social(), f2);
        }
    }
}

#[test] fn agent_selection_retention_flow() {
    let ui = create_c();
    ui.set_selected_agent("Agent Alpha".into());
    ui.set_is_advanced(true);
    ui.set_selected_agent("Agent Beta".into());
    assert!(ui.get_is_advanced());
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v {
    ($id:ident, $get:ident, $set:ident, $val:expr) => {
        #[test] fn $id() { let ui = create_c(); ui.$set($val.into()); assert_eq!(ui.$get(), $val); }
    };
}

test_v!(u1, get_selected_agent, set_selected_agent, "Support Bot");
#[test] fn u2() { let ui = create_c(); ui.set_can_reply(true); assert!(ui.get_can_reply()); }
#[test] fn u3() { let ui = create_c(); ui.set_can_social(false); assert!(!ui.get_can_social()); }
#[test] fn u4() { let ui = create_c(); ui.set_frequency_value(0.75); assert_eq!(ui.get_frequency_value(), 0.75); }
#[test] fn u5() { let ui = create_c(); ui.set_is_advanced(false); assert!(!ui.get_is_advanced()); }
#[test] fn u6() { let ui = create_c(); ui.set_show_toast(true); assert!(ui.get_show_toast()); }
test_v!(u7, get_selected_agent, set_selected_agent, "");
#[test] fn u8() { let ui = create_c(); ui.set_frequency_value(0.0); assert_eq!(ui.get_frequency_value(), 0.0); }
#[test] fn u9() { let ui = create_c(); ui.set_frequency_value(1.0); assert_eq!(ui.get_frequency_value(), 1.0); }
test_v!(u10, get_selected_agent, set_selected_agent, "DeepThought");

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

test_v!(u21, get_frequency_value, set_frequency_value, 0.21);
test_v!(u22, get_frequency_value, set_frequency_value, 0.22);
test_v!(u23, get_frequency_value, set_frequency_value, 0.23);
test_v!(u24, get_frequency_value, set_frequency_value, 0.24);
test_v!(u25, get_frequency_value, set_frequency_value, 0.25);
test_v!(u26, get_frequency_value, set_frequency_value, 0.26);
test_v!(u27, get_frequency_value, set_frequency_value, 0.27);
test_v!(u28, get_frequency_value, set_frequency_value, 0.28);
test_v!(u29, get_frequency_value, set_frequency_value, 0.29);
test_v!(u30, get_frequency_value, set_frequency_value, 0.30);

#[test] fn u31() { let ui = create_c(); ui.set_can_reply(true); ui.set_can_social(true); assert!(ui.get_can_reply()); assert!(ui.get_can_social()); }
#[test] fn u32() { let ui = create_c(); ui.set_can_reply(false); ui.set_can_social(false); assert!(!ui.get_can_reply()); assert!(!ui.get_can_social()); }
#[test] fn u33() { let ui = create_c(); ui.set_is_advanced(true); ui.set_show_toast(true); assert!(ui.get_is_advanced()); assert!(ui.get_show_toast()); }
#[test] fn u34() { let ui = create_c(); ui.set_is_advanced(false); ui.set_show_toast(false); assert!(!ui.get_is_advanced()); assert!(!ui.get_show_toast()); }
test_v!(u35, get_selected_agent, set_selected_agent, "Agent with 🤖 Emoji");
test_v!(u36, get_selected_agent, set_selected_agent, "Agent with 'Quote'");
test_v!(u37, get_selected_agent, set_selected_agent, "Agent with ; Semicolon");
test_v!(u38, get_frequency_value, set_frequency_value, 0.00001);
test_v!(u39, get_frequency_value, set_frequency_value, 0.99999);
test_v!(u40, get_selected_agent, set_selected_agent, "Long Long Long Long Long ");

test_v!(u41, get_selected_agent, set_selected_agent, "a41");
test_v!(u42, get_selected_agent, set_selected_agent, "a42");
test_v!(u43, get_selected_agent, set_selected_agent, "a43");
test_v!(u44, get_selected_agent, set_selected_agent, "a44");
test_v!(u45, get_selected_agent, set_selected_agent, "a45");
test_v!(u46, get_selected_agent, set_selected_agent, "a46");
test_v!(u47, get_selected_agent, set_selected_agent, "a47");
test_v!(u48, get_selected_agent, set_selected_agent, "a48");
test_v!(u49, get_selected_agent, set_selected_agent, "a49");
test_v!(u50, get_selected_agent, set_selected_agent, "a50");

test_v!(u51, get_frequency_value, set_frequency_value, 0.51);
test_v!(u52, get_frequency_value, set_frequency_value, 0.52);
test_v!(u53, get_frequency_value, set_frequency_value, 0.53);
test_v!(u54, get_frequency_value, set_frequency_value, 0.54);
test_v!(u55, get_frequency_value, set_frequency_value, 0.55);
test_v!(u56, get_frequency_value, set_frequency_value, 0.56);
test_v!(u57, get_frequency_value, set_frequency_value, 0.57);
test_v!(u58, get_frequency_value, set_frequency_value, 0.58);
test_v!(u59, get_frequency_value, set_frequency_value, 0.59);
test_v!(u60, get_frequency_value, set_frequency_value, 0.60);

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

test_v!(u71, get_frequency_value, set_frequency_value, 0.71);
test_v!(u72, get_frequency_value, set_frequency_value, 0.72);
test_v!(u73, get_frequency_value, set_frequency_value, 0.73);
test_v!(u74, get_frequency_value, set_frequency_value, 0.74);
test_v!(u75, get_frequency_value, set_frequency_value, 0.75);
test_v!(u76, get_frequency_value, set_frequency_value, 0.76);
test_v!(u77, get_frequency_value, set_frequency_value, 0.77);
test_v!(u78, get_frequency_value, set_frequency_value, 0.78);
test_v!(u79, get_frequency_value, set_frequency_value, 0.79);
test_v!(u80, get_frequency_value, set_frequency_value, 0.80);
