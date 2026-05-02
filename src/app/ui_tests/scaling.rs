use crate::app;
use slint::ComponentHandle;

fn create() -> app::Scaling { crate::ui_tests::init(); app::Scaling::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn scaling_xss_role() {
    let ui = create();
    let xss = "<script>alert('scaling')</script>";
    ui.set_selected_role(xss.into());
    assert_eq!(ui.get_selected_role(), xss);
}

#[test] fn scaling_count_overflow() {
    let ui = create();
    ui.set_target_count(9999);
    assert_eq!(ui.get_target_count(), 9999);
}

#[test] fn scaling_count_negative() {
    let ui = create();
    ui.set_target_count(-100);
    assert_eq!(ui.get_target_count(), -100);
}

// --- Interaction / Flow Tests ---

#[test] fn scaling_flow_callback_trigger() {
    let ui = create();
    let called_role = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let called_count = std::rc::Rc::new(std::cell::RefCell::new(0));
    let c1 = called_role.clone();
    let c2 = called_count.clone();
    ui.on_scale_agents(move |role, count| {
        *c1.borrow_mut() = role.to_string();
        *c2.borrow_mut() = count;
    });
    
    ui.invoke_scale_agents("DEVOPS".into(), 5);
    assert_eq!(*called_role.borrow(), "DEVOPS");
    assert_eq!(*called_count.borrow(), 5);
}

#[test] fn scaling_flow_rapid_count_change() {
    let ui = create();
    for i in 1..20 {
        ui.set_target_count(i);
        assert_eq!(ui.get_target_count(), i);
    }
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v {
    ($id:ident, $get:ident, $set:ident, $val:expr) => {
        #[test] fn $id() { let ui = create(); ui.$set($val.into()); assert_eq!(ui.$get(), $val); }
    };
}

test_v!(u1, get_selected_role, set_selected_role, "SRE");
test_v!(u2, get_selected_role, set_selected_role, "PRODUCT_OWNER");
test_v!(u3, get_selected_role, set_selected_role, "DESIGNER");
#[test] fn u4() { let ui = create(); ui.set_target_count(0); assert_eq!(ui.get_target_count(), 0); }
#[test] fn u5() { let ui = create(); ui.set_target_count(10); assert_eq!(ui.get_target_count(), 10); }

test_v!(u11, get_selected_role, set_selected_role, "r11");
test_v!(u12, get_selected_role, set_selected_role, "r12");
test_v!(u13, get_selected_role, set_selected_role, "r13");
test_v!(u14, get_selected_role, set_selected_role, "r14");
test_v!(u15, get_selected_role, set_selected_role, "r15");
test_v!(u16, get_selected_role, set_selected_role, "r16");
test_v!(u17, get_selected_role, set_selected_role, "r17");
test_v!(u18, get_selected_role, set_selected_role, "r18");
test_v!(u19, get_selected_role, set_selected_role, "r19");
test_v!(u20, get_selected_role, set_selected_role, "r20");

test_v!(u21, get_target_count, set_target_count, 21);
test_v!(u22, get_target_count, set_target_count, 22);
test_v!(u23, get_target_count, set_target_count, 23);
test_v!(u24, get_target_count, set_target_count, 24);
test_v!(u25, get_target_count, set_target_count, 25);
test_v!(u26, get_target_count, set_target_count, 26);
test_v!(u27, get_target_count, set_target_count, 27);
test_v!(u28, get_target_count, set_target_count, 28);
test_v!(u29, get_target_count, set_target_count, 29);
test_v!(u30, get_target_count, set_target_count, 30);

test_v!(u31, get_selected_role, set_selected_role, "Role with Space");
test_v!(u32, get_selected_role, set_selected_role, "Role'Quotes'");
test_v!(u33, get_selected_role, set_selected_role, "Role;Semi");
test_v!(u34, get_selected_role, set_selected_role, "");
test_v!(u35, get_selected_role, set_selected_role, "Very Long Role Name ".repeat(5));

test_v!(u41, get_target_count, set_target_count, 41);
test_v!(u42, get_target_count, set_target_count, 42);
test_v!(u43, get_target_count, set_target_count, 43);
test_v!(u44, get_target_count, set_target_count, 44);
test_v!(u45, get_target_count, set_target_count, 45);
test_v!(u46, get_target_count, set_target_count, 46);
test_v!(u47, get_target_count, set_target_count, 47);
test_v!(u48, get_target_count, set_target_count, 48);
test_v!(u49, get_target_count, set_target_count, 49);
test_v!(u50, get_target_count, set_target_count, 50);

test_v!(u51, get_selected_role, set_selected_role, "r51");
test_v!(u52, get_selected_role, set_selected_role, "r52");
test_v!(u53, get_selected_role, set_selected_role, "r53");
test_v!(u54, get_selected_role, set_selected_role, "r54");
test_v!(u55, get_selected_role, set_selected_role, "r55");
test_v!(u56, get_selected_role, set_selected_role, "r56");
test_v!(u57, get_selected_role, set_selected_role, "r57");
test_v!(u58, get_selected_role, set_selected_role, "r58");
test_v!(u59, get_selected_role, set_selected_role, "r59");
test_v!(u60, get_selected_role, set_selected_role, "r60");

test_v!(u61, get_target_count, set_target_count, 61);
test_v!(u62, get_target_count, set_target_count, 62);
test_v!(u63, get_target_count, set_target_count, 63);
test_v!(u64, get_target_count, set_target_count, 64);
test_v!(u65, get_target_count, set_target_count, 65);
test_v!(u66, get_target_count, set_target_count, 66);
test_v!(u67, get_target_count, set_target_count, 67);
test_v!(u68, get_target_count, set_target_count, 68);
test_v!(u69, get_target_count, set_target_count, 69);
test_v!(u70, get_target_count, set_target_count, 70);

test_v!(u71, get_selected_role, set_selected_role, "r71");
test_v!(u72, get_selected_role, set_selected_role, "r72");
test_v!(u73, get_selected_role, set_selected_role, "r73");
test_v!(u74, get_selected_role, set_selected_role, "r74");
test_v!(u75, get_selected_role, set_selected_role, "r75");
test_v!(u76, get_selected_role, set_selected_role, "r76");
test_v!(u77, get_selected_role, set_selected_role, "r77");
test_v!(u78, get_selected_role, set_selected_role, "r78");
test_v!(u79, get_selected_role, set_selected_role, "r79");
test_v!(u80, get_selected_role, set_selected_role, "r80");
