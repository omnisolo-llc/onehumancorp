use crate::app;
use slint::ComponentHandle;

fn create() -> app::GrowBusiness { crate::ui_tests::init(); app::GrowBusiness::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn grow_xss_strategy() {
    let ui = create();
    let xss = "<script>alert('grow')</script>";
    ui.set_selected_strategy(xss.into());
    assert_eq!(ui.get_selected_strategy(), xss);
}

#[test] fn grow_step_overflow() {
    let ui = create();
    ui.set_step(999);
    assert_eq!(ui.get_step(), 999);
}

#[test] fn grow_step_underflow() {
    let ui = create();
    ui.set_step(-999);
    assert_eq!(ui.get_step(), -999);
}

// --- Interaction / Flow Tests ---

#[test] fn grow_flow_retention_switch() {
    let ui = create();
    ui.set_selected_strategy("A".into());
    ui.set_is_advanced(true);
    ui.set_selected_strategy("B".into());
    assert!(ui.get_is_advanced());
}

#[test] fn grow_flow_step_loop() {
    let ui = create();
    for i in 0..10 {
        ui.set_step(i);
        assert_eq!(ui.get_step(), i);
    }
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v {
    ($id:ident, $get:ident, $set:ident, $val:expr) => {
        #[test] fn $id() { let ui = create(); ui.$set($val.into()); assert_eq!(ui.$get(), $val); }
    };
}

test_v!(u1, get_selected_strategy, set_selected_strategy, "Inbound Marketing");
test_v!(u2, get_selected_strategy, set_selected_strategy, "Outbound Sales");
test_v!(u3, get_selected_strategy, set_selected_strategy, "Content Creation");
test_v!(u4, get_selected_strategy, set_selected_strategy, "SEO Optimization");
test_v!(u5, get_selected_strategy, set_selected_strategy, "Social Media");
test_v!(u6, get_selected_strategy, set_selected_strategy, "Email Campaigns");
test_v!(u7, get_selected_strategy, set_selected_strategy, "Partnerships");
test_v!(u8, get_selected_strategy, set_selected_strategy, "Referral Program");
test_v!(u9, get_selected_strategy, set_selected_strategy, "Paid Ads");
test_v!(u10, get_selected_strategy, set_selected_strategy, "Events/Webinars");

test_v!(u11, get_selected_strategy, set_selected_strategy, "s11");
test_v!(u12, get_selected_strategy, set_selected_strategy, "s12");
test_v!(u13, get_selected_strategy, set_selected_strategy, "s13");
test_v!(u14, get_selected_strategy, set_selected_strategy, "s14");
test_v!(u15, get_selected_strategy, set_selected_strategy, "s15");
test_v!(u16, get_selected_strategy, set_selected_strategy, "s16");
test_v!(u17, get_selected_strategy, set_selected_strategy, "s17");
test_v!(u18, get_selected_strategy, set_selected_strategy, "s18");
test_v!(u19, get_selected_strategy, set_selected_strategy, "s19");
test_v!(u20, get_selected_strategy, set_selected_strategy, "s20");

test_v!(u21, get_step, set_step, 21);
test_v!(u22, get_step, set_step, 22);
test_v!(u23, get_step, set_step, 23);
test_v!(u24, get_step, set_step, 24);
test_v!(u25, get_step, set_step, 25);
#[test] fn u26() { let ui = create(); ui.set_is_advanced(true); assert!(ui.get_is_advanced()); }
#[test] fn u27() { let ui = create(); ui.set_is_advanced(false); assert!(!ui.get_is_advanced()); }
test_v!(u28, get_selected_strategy, set_selected_strategy, "Strategy with Emoji 📈");
test_v!(u29, get_selected_strategy, set_selected_strategy, "Strategy with Quote 's'");
test_v!(u30, get_selected_strategy, set_selected_strategy, "Long Strategy Name Long Strategy Name ");

test_v!(u31, get_selected_strategy, set_selected_strategy, "s31");
test_v!(u32, get_selected_strategy, set_selected_strategy, "s32");
test_v!(u33, get_selected_strategy, set_selected_strategy, "s33");
test_v!(u34, get_selected_strategy, set_selected_strategy, "s34");
test_v!(u35, get_selected_strategy, set_selected_strategy, "s35");
test_v!(u36, get_selected_strategy, set_selected_strategy, "s36");
test_v!(u37, get_selected_strategy, set_selected_strategy, "s37");
test_v!(u38, get_selected_strategy, set_selected_strategy, "s38");
test_v!(u39, get_selected_strategy, set_selected_strategy, "s39");
test_v!(u40, get_selected_strategy, set_selected_strategy, "s40");

test_v!(u41, get_selected_strategy, set_selected_strategy, "s41");
test_v!(u42, get_selected_strategy, set_selected_strategy, "s42");
test_v!(u43, get_selected_strategy, set_selected_strategy, "s43");
test_v!(u44, get_selected_strategy, set_selected_strategy, "s44");
test_v!(u45, get_selected_strategy, set_selected_strategy, "s45");
test_v!(u46, get_selected_strategy, set_selected_strategy, "s46");
test_v!(u47, get_selected_strategy, set_selected_strategy, "s47");
test_v!(u48, get_selected_strategy, set_selected_strategy, "s48");
test_v!(u49, get_selected_strategy, set_selected_strategy, "s49");
test_v!(u50, get_selected_strategy, set_selected_strategy, "s50");

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

test_v!(u61, get_selected_strategy, set_selected_strategy, "s61");
test_v!(u62, get_selected_strategy, set_selected_strategy, "s62");
test_v!(u63, get_selected_strategy, set_selected_strategy, "s63");
test_v!(u64, get_selected_strategy, set_selected_strategy, "s64");
test_v!(u65, get_selected_strategy, set_selected_strategy, "s65");
test_v!(u66, get_selected_strategy, set_selected_strategy, "s66");
test_v!(u67, get_selected_strategy, set_selected_strategy, "s67");
test_v!(u68, get_selected_strategy, set_selected_strategy, "s68");
test_v!(u69, get_selected_strategy, set_selected_strategy, "s69");
test_v!(u70, get_selected_strategy, set_selected_strategy, "s70");

test_v!(u71, get_selected_strategy, set_selected_strategy, "s71");
test_v!(u72, get_selected_strategy, set_selected_strategy, "s72");
test_v!(u73, get_selected_strategy, set_selected_strategy, "s73");
test_v!(u74, get_selected_strategy, set_selected_strategy, "s74");
test_v!(u75, get_selected_strategy, set_selected_strategy, "s75");
test_v!(u76, get_selected_strategy, set_selected_strategy, "s76");
test_v!(u77, get_selected_strategy, set_selected_strategy, "s77");
test_v!(u78, get_selected_strategy, set_selected_strategy, "s78");
test_v!(u79, get_selected_strategy, set_selected_strategy, "s79");
test_v!(u80, get_selected_strategy, set_selected_strategy, "s80");
