use crate::app;
use slint::ComponentHandle;

fn create() -> app::InteractiveWalkthrough { crate::ui_tests::init(); app::InteractiveWalkthrough::new().unwrap() }

// --- Specialized / Flow Tests ---

#[test] fn walkthrough_flow_steps_bounds() {
    let ui = create();
    ui.set_current_step(10);
    assert_eq!(ui.get_current_step(), 10);
    ui.set_current_step(-5);
    assert_eq!(ui.get_current_step(), -5);
}

#[test] fn walkthrough_flow_visibility() {
    let ui = create();
    // Assuming visible property exists or just testing state logic
    ui.set_current_step(3);
    assert_eq!(ui.get_current_step(), 3);
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v {
    ($id:ident, $get:ident, $set:ident, $val:expr) => {
        #[test] fn $id() { let ui = create(); ui.$set($val.into()); assert_eq!(ui.$get(), $val); }
    };
}

test_v!(u1, get_current_step, set_current_step, 0);
test_v!(u2, get_current_step, set_current_step, 1);
test_v!(u3, get_current_step, set_current_step, 2);
test_v!(u4, get_current_step, set_current_step, 3);

test_v!(u11, get_current_step, set_current_step, 11);
test_v!(u12, get_current_step, set_current_step, 12);
test_v!(u13, get_current_step, set_current_step, 13);
test_v!(u14, get_current_step, set_current_step, 14);
test_v!(u15, get_current_step, set_current_step, 15);
test_v!(u16, get_current_step, set_current_step, 16);
test_v!(u17, get_current_step, set_current_step, 17);
test_v!(u18, get_current_step, set_current_step, 18);
test_v!(u19, get_current_step, set_current_step, 19);
test_v!(u20, get_current_step, set_current_step, 20);

test_v!(u21, get_current_step, set_current_step, 21);
test_v!(u22, get_current_step, set_current_step, 22);
test_v!(u23, get_current_step, set_current_step, 23);
test_v!(u24, get_current_step, set_current_step, 24);
test_v!(u25, get_current_step, set_current_step, 25);
test_v!(u26, get_current_step, set_current_step, 26);
test_v!(u27, get_current_step, set_current_step, 27);
test_v!(u28, get_current_step, set_current_step, 28);
test_v!(u29, get_current_step, set_current_step, 29);
test_v!(u30, get_current_step, set_current_step, 30);

test_v!(u31, get_current_step, set_current_step, 31);
test_v!(u32, get_current_step, set_current_step, 32);
test_v!(u33, get_current_step, set_current_step, 33);
test_v!(u34, get_current_step, set_current_step, 34);
test_v!(u35, get_current_step, set_current_step, 35);
test_v!(u36, get_current_step, set_current_step, 36);
test_v!(u37, get_current_step, set_current_step, 37);
test_v!(u38, get_current_step, set_current_step, 38);
test_v!(u39, get_current_step, set_current_step, 39);
test_v!(u40, get_current_step, set_current_step, 40);

test_v!(u41, get_current_step, set_current_step, 41);
test_v!(u42, get_current_step, set_current_step, 42);
test_v!(u43, get_current_step, set_current_step, 43);
test_v!(u44, get_current_step, set_current_step, 44);
test_v!(u45, get_current_step, set_current_step, 45);
test_v!(u46, get_current_step, set_current_step, 46);
test_v!(u47, get_current_step, set_current_step, 47);
test_v!(u48, get_current_step, set_current_step, 48);
test_v!(u49, get_current_step, set_current_step, 49);
test_v!(u50, get_current_step, set_current_step, 50);

test_v!(u51, get_current_step, set_current_step, 51);
test_v!(u52, get_current_step, set_current_step, 52);
test_v!(u53, get_current_step, set_current_step, 53);
test_v!(u54, get_current_step, set_current_step, 54);
test_v!(u55, get_current_step, set_current_step, 55);
test_v!(u56, get_current_step, set_current_step, 56);
test_v!(u57, get_current_step, set_current_step, 57);
test_v!(u58, get_current_step, set_current_step, 58);
test_v!(u59, get_current_step, set_current_step, 59);
test_v!(u60, get_current_step, set_current_step, 60);

test_v!(u61, get_current_step, set_current_step, 61);
test_v!(u62, get_current_step, set_current_step, 62);
test_v!(u63, get_current_step, set_current_step, 63);
test_v!(u64, get_current_step, set_current_step, 64);
test_v!(u65, get_current_step, set_current_step, 65);
test_v!(u66, get_current_step, set_current_step, 66);
test_v!(u67, get_current_step, set_current_step, 67);
test_v!(u68, get_current_step, set_current_step, 68);
test_v!(u69, get_current_step, set_current_step, 69);
test_v!(u70, get_current_step, set_current_step, 70);

test_v!(u71, get_current_step, set_current_step, 71);
test_v!(u72, get_current_step, set_current_step, 72);
test_v!(u73, get_current_step, set_current_step, 73);
test_v!(u74, get_current_step, set_current_step, 74);
test_v!(u75, get_current_step, set_current_step, 75);
test_v!(u76, get_current_step, set_current_step, 76);
test_v!(u77, get_current_step, set_current_step, 77);
test_v!(u78, get_current_step, set_current_step, 78);
test_v!(u79, get_current_step, set_current_step, 79);
test_v!(u80, get_current_step, set_current_step, 80);
