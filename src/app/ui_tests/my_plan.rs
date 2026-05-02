use crate::app;
use slint::ComponentHandle;

fn create() -> app::MyPlan { crate::ui_tests::init(); app::MyPlan::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn myplan_xss_tier() {
    let ui = create();
    let xss = "<script>alert('plan')</script>";
    ui.set_tier(xss.into());
    assert_eq!(ui.get_tier(), xss);
}

#[test] fn myplan_injection_actions() {
    let ui = create();
    let inj = "1000'); DROP TABLE actions; --";
    ui.set_total_actions(inj.into());
    assert_eq!(ui.get_total_actions(), inj);
}

#[test] fn myplan_long_date() {
    let ui = create();
    let long = "Date ".repeat(200);
    ui.set_renewal_date(long.clone().into());
    assert_eq!(ui.get_renewal_date(), long);
}

// --- Interaction / Flow Tests ---

#[test] fn myplan_flow_upgrade_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_upgrade(move || { *c.borrow_mut() = true; });
    ui.invoke_upgrade();
    assert!(*called.borrow());
}

#[test] fn myplan_flow_cancel_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_cancel_subscription(move || { *c.borrow_mut() = true; });
    ui.invoke_cancel_subscription();
    assert!(*called.borrow());
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v {
    ($id:ident, $get:ident, $set:ident, $val:expr) => {
        #[test] fn $id() { let ui = create(); ui.$set($val.into()); assert_eq!(ui.$get(), $val); }
    };
}

test_v!(u1, get_tier, set_tier, "Enterprise");
test_v!(u2, get_plan_status, set_plan_status, "Past Due");
test_v!(u3, get_estimated_bill, set_estimated_bill, "$0.00");

test_v!(u11, get_tier, set_tier, "t11");
test_v!(u12, get_tier, set_tier, "t12");
test_v!(u13, get_tier, set_tier, "t13");
test_v!(u14, get_tier, set_tier, "t14");
test_v!(u15, get_tier, set_tier, "t15");
test_v!(u16, get_tier, set_tier, "t16");
test_v!(u17, get_tier, set_tier, "t17");
test_v!(u18, get_tier, set_tier, "t18");
test_v!(u19, get_tier, set_tier, "t19");
test_v!(u20, get_tier, set_tier, "t20");

test_v!(u21, get_total_actions, set_total_actions, "21");
test_v!(u22, get_total_actions, set_total_actions, "22");
test_v!(u23, get_total_actions, set_total_actions, "23");
test_v!(u24, get_total_actions, set_total_actions, "24");
test_v!(u25, get_total_actions, set_total_actions, "25");
test_v!(u26, get_total_actions, set_total_actions, "26");
test_v!(u27, get_total_actions, set_total_actions, "27");
test_v!(u28, get_total_actions, set_total_actions, "28");
test_v!(u29, get_total_actions, set_total_actions, "29");
test_v!(u30, get_total_actions, set_total_actions, "30");

test_v!(u31, get_tier, set_tier, "Tier with 💎 Emoji");
test_v!(u32, get_tier, set_tier, "Tier'Quotes'");
test_v!(u33, get_tier, set_tier, "Tier ; Semi");
test_v!(u34, get_tier, set_tier, "");
test_v!(u35, get_tier, set_tier, "Very Long Tier Name ".repeat(5));

test_v!(u41, get_used_storage, set_used_storage, "u41");
test_v!(u42, get_used_storage, set_used_storage, "u42");
test_v!(u43, get_used_storage, set_used_storage, "u43");
test_v!(u44, get_used_storage, set_used_storage, "u44");
test_v!(u45, get_used_storage, set_used_storage, "u45");
test_v!(u46, get_used_storage, set_used_storage, "u46");
test_v!(u47, get_used_storage, set_used_storage, "u47");
test_v!(u48, get_used_storage, set_used_storage, "u48");
test_v!(u49, get_used_storage, set_used_storage, "u49");
test_v!(u50, get_used_storage, set_used_storage, "u50");

test_v!(u51, get_estimated_bill, set_estimated_bill, "b51");
test_v!(u52, get_estimated_bill, set_estimated_bill, "b52");
test_v!(u53, get_estimated_bill, set_estimated_bill, "b53");
test_v!(u54, get_estimated_bill, set_estimated_bill, "b54");
test_v!(u55, get_estimated_bill, set_estimated_bill, "b55");
test_v!(u56, get_estimated_bill, set_estimated_bill, "b56");
test_v!(u57, get_estimated_bill, set_estimated_bill, "b57");
test_v!(u58, get_estimated_bill, set_estimated_bill, "b58");
test_v!(u59, get_estimated_bill, set_estimated_bill, "b59");
test_v!(u60, get_estimated_bill, set_estimated_bill, "b60");

test_v!(u61, get_renewal_date, set_renewal_date, "d61");
test_v!(u62, get_renewal_date, set_renewal_date, "d62");
test_v!(u63, get_renewal_date, set_renewal_date, "d63");
test_v!(u64, get_renewal_date, set_renewal_date, "d64");
test_v!(u65, get_renewal_date, set_renewal_date, "d65");
test_v!(u66, get_renewal_date, set_renewal_date, "d66");
test_v!(u67, get_renewal_date, set_renewal_date, "d67");
test_v!(u68, get_renewal_date, set_renewal_date, "d68");
test_v!(u69, get_renewal_date, set_renewal_date, "d69");
test_v!(u70, get_renewal_date, set_renewal_date, "d70");

test_v!(u71, get_plan_status, set_plan_status, "s71");
test_v!(u72, get_plan_status, set_plan_status, "s72");
test_v!(u73, get_plan_status, set_plan_status, "s73");
test_v!(u74, get_plan_status, set_plan_status, "s74");
test_v!(u75, get_plan_status, set_plan_status, "s75");
test_v!(u76, get_plan_status, set_plan_status, "s76");
test_v!(u77, get_plan_status, set_plan_status, "s77");
test_v!(u78, get_plan_status, set_plan_status, "s78");
test_v!(u79, get_plan_status, set_plan_status, "s79");
test_v!(u80, get_plan_status, set_plan_status, "s80");
