use crate::app;
use slint::ComponentHandle;

fn create() -> app::SwarmMemory { crate::ui_tests::init(); app::SwarmMemory::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn memory_xss_activity() {
    let ui = create();
    let xss = "<script>alert('mesh')</script>";
    ui.set_mesh_activity(xss.into());
    assert_eq!(ui.get_mesh_activity(), xss);
}

#[test] fn memory_velocity_overflow() {
    let ui = create();
    ui.set_velocity_score(2147483647);
    assert_eq!(ui.get_velocity_score(), 2147483647);
}

#[test] fn memory_velocity_negative() {
    let ui = create();
    ui.set_velocity_score(-999);
    assert_eq!(ui.get_velocity_score(), -999);
}

// --- Interaction / Flow Tests ---

#[test] fn memory_flow_walkthrough_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_view_walkthrough(move || { *c.borrow_mut() = true; });
    ui.invoke_view_walkthrough();
    assert!(*called.borrow());
}

#[test] fn memory_flow_sync_loop() {
    let ui = create();
    for i in 0..50 {
        let act = format!("Activity {}", i);
        ui.set_mesh_activity(act.clone().into());
        ui.set_velocity_score(i);
        assert_eq!(ui.get_mesh_activity(), act);
        assert_eq!(ui.get_velocity_score(), i);
    }
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v {
    ($id:ident, $get:ident, $set:ident, $val:expr) => {
        #[test] fn $id() { let ui = create(); ui.$set($val.into()); assert_eq!(ui.$get(), $val); }
    };
}

test_v!(u1, get_durable_memory, set_durable_memory, "Cached State");
test_v!(u2, get_durable_memory, set_durable_memory, "Cloud Sync Active");
test_v!(u3, get_durable_memory, set_durable_memory, "Offline Mode");
#[test] fn u4() { let ui = create(); ui.set_velocity_score(100); assert_eq!(ui.get_velocity_score(), 100); }
#[test] fn u5() { let ui = create(); ui.set_velocity_score(0); assert_eq!(ui.get_velocity_score(), 0); }

test_v!(u11, get_mesh_activity, set_mesh_activity, "a11");
test_v!(u12, get_mesh_activity, set_mesh_activity, "a12");
test_v!(u13, get_mesh_activity, set_mesh_activity, "a13");
test_v!(u14, get_mesh_activity, set_mesh_activity, "a14");
test_v!(u15, get_mesh_activity, set_mesh_activity, "a15");
test_v!(u16, get_mesh_activity, set_mesh_activity, "a16");
test_v!(u17, get_mesh_activity, set_mesh_activity, "a17");
test_v!(u18, get_mesh_activity, set_mesh_activity, "a18");
test_v!(u19, get_mesh_activity, set_mesh_activity, "a19");
test_v!(u20, get_mesh_activity, set_mesh_activity, "a20");

test_v!(u21, get_velocity_score, set_velocity_score, 21);
test_v!(u22, get_velocity_score, set_velocity_score, 22);
test_v!(u23, get_velocity_score, set_velocity_score, 23);
test_v!(u24, get_velocity_score, set_velocity_score, 24);
test_v!(u25, get_velocity_score, set_velocity_score, 25);
test_v!(u26, get_velocity_score, set_velocity_score, 26);
test_v!(u27, get_velocity_score, set_velocity_score, 27);
test_v!(u28, get_velocity_score, set_velocity_score, 28);
test_v!(u29, get_velocity_score, set_velocity_score, 29);
test_v!(u30, get_velocity_score, set_velocity_score, 30);

test_v!(u31, get_mesh_activity, set_mesh_activity, "Activity with 🧠 Emoji");
test_v!(u32, get_mesh_activity, set_mesh_activity, "Activity with 'Quotes'");
test_v!(u33, get_mesh_activity, set_mesh_activity, "Activity with ; Semi");
test_v!(u34, get_mesh_activity, set_mesh_activity, "");
test_v!(u35, get_mesh_activity, set_mesh_activity, "Very Long Activity Name ".repeat(5));

test_v!(u41, get_durable_memory, set_durable_memory, "m41");
test_v!(u42, get_durable_memory, set_durable_memory, "m42");
test_v!(u43, get_durable_memory, set_durable_memory, "m43");
test_v!(u44, get_durable_memory, set_durable_memory, "m44");
test_v!(u45, get_durable_memory, set_durable_memory, "m45");
test_v!(u46, get_durable_memory, set_durable_memory, "m46");
test_v!(u47, get_durable_memory, set_durable_memory, "m47");
test_v!(u48, get_durable_memory, set_durable_memory, "m48");
test_v!(u49, get_durable_memory, set_durable_memory, "m49");
test_v!(u50, get_durable_memory, set_durable_memory, "m50");

test_v!(u51, get_mesh_activity, set_mesh_activity, "s51");
test_v!(u52, get_mesh_activity, set_mesh_activity, "s52");
test_v!(u53, get_mesh_activity, set_mesh_activity, "s53");
test_v!(u54, get_mesh_activity, set_mesh_activity, "s54");
test_v!(u55, get_mesh_activity, set_mesh_activity, "s55");
test_v!(u56, get_mesh_activity, set_mesh_activity, "s56");
test_v!(u57, get_mesh_activity, set_mesh_activity, "s57");
test_v!(u58, get_mesh_activity, set_mesh_activity, "s58");
test_v!(u59, get_mesh_activity, set_mesh_activity, "s59");
test_v!(u60, get_mesh_activity, set_mesh_activity, "s60");

test_v!(u61, get_velocity_score, set_velocity_score, 61);
test_v!(u62, get_velocity_score, set_velocity_score, 62);
test_v!(u63, get_velocity_score, set_velocity_score, 63);
test_v!(u64, get_velocity_score, set_velocity_score, 64);
test_v!(u65, get_velocity_score, set_velocity_score, 65);
test_v!(u66, get_velocity_score, set_velocity_score, 66);
test_v!(u67, get_velocity_score, set_velocity_score, 67);
test_v!(u68, get_velocity_score, set_velocity_score, 68);
test_v!(u69, get_velocity_score, set_velocity_score, 69);
test_v!(u70, get_velocity_score, set_velocity_score, 70);

test_v!(u71, get_durable_memory, set_durable_memory, "m71");
test_v!(u72, get_durable_memory, set_durable_memory, "m72");
test_v!(u73, get_durable_memory, set_durable_memory, "m73");
test_v!(u74, get_durable_memory, set_durable_memory, "m74");
test_v!(u75, get_durable_memory, set_durable_memory, "m75");
test_v!(u76, get_durable_memory, set_durable_memory, "m76");
test_v!(u77, get_durable_memory, set_durable_memory, "m77");
test_v!(u78, get_durable_memory, set_durable_memory, "m78");
test_v!(u79, get_durable_memory, set_durable_memory, "m79");
test_v!(u80, get_durable_memory, set_durable_memory, "m80");
