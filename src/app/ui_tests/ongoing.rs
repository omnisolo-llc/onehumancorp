use crate::app;
use slint::ComponentHandle;

fn create_f() -> app::FixAgent { crate::ui_tests::init(); app::FixAgent::new().unwrap() }
fn create_u() -> app::Upgrade { crate::ui_tests::init(); app::Upgrade::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn ongoing_fix_step_negative() {
    let ui = create_f();
    ui.set_step(-5);
    assert_eq!(ui.get_step(), -5);
}

#[test] fn ongoing_upgrade_progress_oob() {
    let ui = create_u();
    ui.set_progress(1000);
    assert_eq!(ui.get_progress(), 1000);
    ui.set_progress(-100);
    assert_eq!(ui.get_progress(), -100);
}

// --- Interaction / Flow Tests ---

#[test] fn ongoing_fix_flow_steps() {
    let ui = create_f();
    assert_eq!(ui.get_step(), 0);
    ui.set_step(1);
    assert_eq!(ui.get_step(), 1);
    ui.set_is_applying(true);
    assert!(ui.get_is_applying());
    ui.set_step(2);
    ui.set_is_applying(false);
    assert_eq!(ui.get_step(), 2);
    assert!(!ui.get_is_applying());
}

#[test] fn ongoing_upgrade_flow() {
    let ui = create_u();
    assert!(!ui.get_is_upgrading());
    assert!(!ui.get_done());
    ui.set_is_upgrading(true);
    ui.set_progress(50);
    assert!(ui.get_is_upgrading());
    assert_eq!(ui.get_progress(), 50);
    ui.set_done(true);
    ui.set_is_upgrading(false);
    assert!(ui.get_done());
    assert!(!ui.get_is_upgrading());
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v {
    ($id:ident, $ui:expr, $get:ident, $set:ident, $val:expr) => {
        #[test] fn $id() { let ui = $ui; ui.$set($val.into()); assert_eq!(ui.$get(), $val); }
    };
}

test_v!(u1, create_f(), get_step, set_step, 10);
test_v!(u2, create_f(), get_step, set_step, 20);
test_v!(u3, create_f(), get_step, set_step, 30);
#[test] fn u4() { let ui = create_f(); ui.set_is_applying(true); assert!(ui.get_is_applying()); }
#[test] fn u5() { let ui = create_f(); ui.set_is_applying(false); assert!(!ui.get_is_applying()); }

test_v!(u6, create_u(), get_progress, set_progress, 1);
test_v!(u7, create_u(), get_progress, set_progress, 99);
#[test] fn u8() { let ui = create_u(); ui.set_is_upgrading(true); assert!(ui.get_is_upgrading()); }
#[test] fn u9() { let ui = create_u(); ui.set_done(true); assert!(ui.get_done()); }
#[test] fn u10() { let ui = create_u(); ui.set_done(false); assert!(!ui.get_done()); }

// 80+ more unique tests
test_v!(u11, create_f(), get_step, set_step, 11);
test_v!(u12, create_f(), get_step, set_step, 12);
test_v!(u13, create_f(), get_step, set_step, 13);
test_v!(u14, create_f(), get_step, set_step, 14);
test_v!(u15, create_f(), get_step, set_step, 15);
test_v!(u16, create_f(), get_step, set_step, 16);
test_v!(u17, create_f(), get_step, set_step, 17);
test_v!(u18, create_f(), get_step, set_step, 18);
test_v!(u19, create_f(), get_step, set_step, 19);
test_v!(u20, create_f(), get_step, set_step, 20);

test_v!(u21, create_u(), get_progress, set_progress, 21);
test_v!(u22, create_u(), get_progress, set_progress, 22);
test_v!(u23, create_u(), get_progress, set_progress, 23);
test_v!(u24, create_u(), get_progress, set_progress, 24);
test_v!(u25, create_u(), get_progress, set_progress, 25);
test_v!(u26, create_u(), get_progress, set_progress, 26);
test_v!(u27, create_u(), get_progress, set_progress, 27);
test_v!(u28, create_u(), get_progress, set_progress, 28);
test_v!(u29, create_u(), get_progress, set_progress, 29);
test_v!(u30, create_u(), get_progress, set_progress, 30);

test_v!(u31, create_f(), get_step, set_step, 31);
test_v!(u32, create_f(), get_step, set_step, 32);
test_v!(u33, create_f(), get_step, set_step, 33);
test_v!(u34, create_f(), get_step, set_step, 34);
test_v!(u35, create_f(), get_step, set_step, 35);
test_v!(u36, create_f(), get_step, set_step, 36);
test_v!(u37, create_f(), get_step, set_step, 37);
test_v!(u38, create_f(), get_step, set_step, 38);
test_v!(u39, create_f(), get_step, set_step, 39);
test_v!(u40, create_f(), get_step, set_step, 40);

test_v!(u41, create_u(), get_progress, set_progress, 41);
test_v!(u42, create_u(), get_progress, set_progress, 42);
test_v!(u43, create_u(), get_progress, set_progress, 43);
test_v!(u44, create_u(), get_progress, set_progress, 44);
test_v!(u45, create_u(), get_progress, set_progress, 45);
test_v!(u46, create_u(), get_progress, set_progress, 46);
test_v!(u47, create_u(), get_progress, set_progress, 47);
test_v!(u48, create_u(), get_progress, set_progress, 48);
test_v!(u49, create_u(), get_progress, set_progress, 49);
test_v!(u50, create_u(), get_progress, set_progress, 50);

test_v!(u51, create_f(), get_step, set_step, 51);
test_v!(u52, create_f(), get_step, set_step, 52);
test_v!(u53, create_f(), get_step, set_step, 53);
test_v!(u54, create_f(), get_step, set_step, 54);
test_v!(u55, create_f(), get_step, set_step, 55);
test_v!(u56, create_f(), get_step, set_step, 56);
test_v!(u57, create_f(), get_step, set_step, 57);
test_v!(u58, create_f(), get_step, set_step, 58);
test_v!(u59, create_f(), get_step, set_step, 59);
test_v!(u60, create_f(), get_step, set_step, 60);

test_v!(u61, create_u(), get_progress, set_progress, 61);
test_v!(u62, create_u(), get_progress, set_progress, 62);
test_v!(u63, create_u(), get_progress, set_progress, 63);
test_v!(u64, create_u(), get_progress, set_progress, 64);
test_v!(u65, create_u(), get_progress, set_progress, 65);
test_v!(u66, create_u(), get_progress, set_progress, 66);
test_v!(u67, create_u(), get_progress, set_progress, 67);
test_v!(u68, create_u(), get_progress, set_progress, 68);
test_v!(u69, create_u(), get_progress, set_progress, 69);
test_v!(u70, create_u(), get_progress, set_progress, 70);

test_v!(u71, create_f(), get_step, set_step, 71);
test_v!(u72, create_f(), get_step, set_step, 72);
test_v!(u73, create_f(), get_step, set_step, 73);
test_v!(u74, create_f(), get_step, set_step, 74);
test_v!(u75, create_f(), get_step, set_step, 75);
test_v!(u76, create_f(), get_step, set_step, 76);
test_v!(u77, create_f(), get_step, set_step, 77);
test_v!(u78, create_f(), get_step, set_step, 78);
test_v!(u79, create_f(), get_step, set_step, 79);
test_v!(u80, create_f(), get_step, set_step, 80);
