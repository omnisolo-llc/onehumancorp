use crate::app;
use slint::ComponentHandle;

fn create() -> app::Dashboard { crate::ui_tests::init(); app::Dashboard::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn dash_negative_orders() {
    let ui = create();
    ui.set_new_orders_count(-1);
    assert_eq!(ui.get_new_orders_count(), -1);
}

#[test] fn dash_overflow_helpers() {
    let ui = create();
    ui.set_active_helpers_count(2147483647);
    assert_eq!(ui.get_active_helpers_count(), 2147483647);
}

#[test] fn dash_xss_milestone_title() {
    let ui = create();
    let xss = "<svg/onload=alert(1)>";
    ui.set_milestone_title(xss.into());
    assert_eq!(ui.get_milestone_title(), xss);
}

#[test] fn dash_currency_injection() {
    let ui = create();
    let val = "$9,999,999.99'; DROP TABLE sales; --";
    ui.set_todays_sales(val.into());
    assert_eq!(ui.get_todays_sales(), val);
}

// --- Interaction / Logic Flows ---

#[test] fn dash_milestone_visibility_flow() {
    let ui = create();
    ui.set_show_milestone(false);
    ui.set_milestone_title("Hidden".into());
    assert!(!ui.get_show_milestone());
    ui.set_show_milestone(true);
    assert_eq!(ui.get_milestone_title(), "Hidden");
}

#[test] fn dash_mass_property_update() {
    let ui = create();
    for i in 0..100 {
        ui.set_new_orders_count(i);
        ui.set_active_helpers_count(i * 2);
        assert_eq!(ui.get_new_orders_count(), i);
        assert_eq!(ui.get_active_helpers_count(), i * 2);
    }
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v {
    ($id:ident, $get:ident, $set:ident, $val:expr) => {
        #[test] fn $id() { let ui = create(); ui.$set($val.into()); assert_eq!(ui.$get(), $val); }
    };
}

test_v!(u1, get_todays_sales, set_todays_sales, "FREE");
test_v!(u2, get_todays_sales, set_todays_sales, "N/A");
test_v!(u3, get_todays_sales, set_todays_sales, "0.00 €");
#[test] fn u4() { let ui = create(); ui.set_new_orders_count(0); assert_eq!(ui.get_new_orders_count(), 0); }
#[test] fn u5() { let ui = create(); ui.set_active_helpers_count(1); assert_eq!(ui.get_active_helpers_count(), 1); }
#[test] fn u6() { let ui = create(); ui.set_tasks_in_progress_count(100); assert_eq!(ui.get_tasks_in_progress_count(), 100); }
#[test] fn u7() { let ui = create(); ui.set_show_menu(true); assert!(ui.get_show_menu()); ui.set_show_menu(false); assert!(!ui.get_show_menu()); }
#[test] fn u8() { let ui = create(); ui.set_show_quick_actions_hint(true); assert!(ui.get_show_quick_actions_hint()); }
test_v!(u9, get_milestone_message, set_milestone_message, "First Order!");
test_v!(u10, get_milestone_title, set_milestone_title, "🏆 Achievement");

test_v!(u11, get_todays_sales, set_todays_sales, "s11");
test_v!(u12, get_todays_sales, set_todays_sales, "s12");
test_v!(u13, get_todays_sales, set_todays_sales, "s13");
test_v!(u14, get_todays_sales, set_todays_sales, "s14");
test_v!(u15, get_todays_sales, set_todays_sales, "s15");
test_v!(u16, get_todays_sales, set_todays_sales, "s16");
test_v!(u17, get_todays_sales, set_todays_sales, "s17");
test_v!(u18, get_todays_sales, set_todays_sales, "s18");
test_v!(u19, get_todays_sales, set_todays_sales, "s19");
test_v!(u20, get_todays_sales, set_todays_sales, "s20");

test_v!(u21, get_new_orders_count, set_new_orders_count, 21);
test_v!(u22, get_new_orders_count, set_new_orders_count, 22);
test_v!(u23, get_new_orders_count, set_new_orders_count, 23);
test_v!(u24, get_new_orders_count, set_new_orders_count, 24);
test_v!(u25, get_new_orders_count, set_new_orders_count, 25);
test_v!(u26, get_active_helpers_count, set_active_helpers_count, 26);
test_v!(u27, get_active_helpers_count, set_active_helpers_count, 27);
test_v!(u28, get_active_helpers_count, set_active_helpers_count, 28);
test_v!(u29, get_active_helpers_count, set_active_helpers_count, 29);
test_v!(u30, get_active_helpers_count, set_active_helpers_count, 30);

test_v!(u31, get_tasks_in_progress_count, set_tasks_in_progress_count, 31);
test_v!(u32, get_tasks_in_progress_count, set_tasks_in_progress_count, 32);
test_v!(u33, get_tasks_in_progress_count, set_tasks_in_progress_count, 33);
test_v!(u34, get_tasks_in_progress_count, set_tasks_in_progress_count, 34);
test_v!(u35, get_tasks_in_progress_count, set_tasks_in_progress_count, 35);
test_v!(u36, get_milestone_title, set_milestone_title, "mt36");
test_v!(u37, get_milestone_title, set_milestone_title, "mt37");
test_v!(u38, get_milestone_title, set_milestone_title, "mt38");
test_v!(u39, get_milestone_title, set_milestone_title, "mt39");
test_v!(u40, get_milestone_title, set_milestone_title, "mt40");

test_v!(u41, get_milestone_message, set_milestone_message, "mm41");
test_v!(u42, get_milestone_message, set_milestone_message, "mm42");
test_v!(u43, get_milestone_message, set_milestone_message, "mm43");
test_v!(u44, get_milestone_message, set_milestone_message, "mm44");
test_v!(u45, get_milestone_message, set_milestone_message, "mm45");
#[test] fn u46() { let ui = create(); ui.set_show_milestone(true); assert!(ui.get_show_milestone()); }
#[test] fn u47() { let ui = create(); ui.set_show_milestone(false); assert!(!ui.get_show_milestone()); }
#[test] fn u48() { let ui = create(); ui.set_show_quick_actions_hint(true); assert!(ui.get_show_quick_actions_hint()); }
#[test] fn u49() { let ui = create(); ui.set_show_quick_actions_hint(false); assert!(!ui.get_show_quick_actions_hint()); }
#[test] fn u50() { let ui = create(); ui.set_show_menu(true); assert!(ui.get_show_menu()); }

test_v!(u51, get_todays_sales, set_todays_sales, "1,000.00");
test_v!(u52, get_todays_sales, set_todays_sales, "99.99");
test_v!(u53, get_todays_sales, set_todays_sales, "0");
test_v!(u54, get_todays_sales, set_todays_sales, "-10.00");
test_v!(u55, get_todays_sales, set_todays_sales, "Infinity");
test_v!(u56, get_new_orders_count, set_new_orders_count, 1000);
test_v!(u57, get_new_orders_count, set_new_orders_count, 1000000);
test_v!(u58, get_active_helpers_count, set_active_helpers_count, 500);
test_v!(u59, get_tasks_in_progress_count, set_tasks_in_progress_count, 250);
test_v!(u60, get_milestone_title, set_milestone_title, "The Beginning");
test_v!(u61, get_milestone_message, set_milestone_message, "Welcome to the dashboard.");
#[test] fn u62() { let ui = create(); ui.set_show_menu(false); assert!(!ui.get_show_menu()); }
#[test] fn u63() { let ui = create(); ui.set_show_quick_actions_hint(true); assert!(ui.get_show_quick_actions_hint()); }
test_v!(u64, get_todays_sales, set_todays_sales, "€ 5,50");
test_v!(u65, get_todays_sales, set_todays_sales, "¥ 1000");
test_v!(u66, get_new_orders_count, set_new_orders_count, 7);
test_v!(u67, get_active_helpers_count, set_active_helpers_count, 3);
test_v!(u68, get_tasks_in_progress_count, set_tasks_in_progress_count, 1);
test_v!(u69, get_milestone_title, set_milestone_title, "Goal Reached");
test_v!(u70, get_milestone_message, set_milestone_message, "You have completed all tasks.");

test_v!(u71, get_todays_sales, set_todays_sales, "s71");
test_v!(u72, get_todays_sales, set_todays_sales, "s72");
test_v!(u73, get_todays_sales, set_todays_sales, "s73");
test_v!(u74, get_todays_sales, set_todays_sales, "s74");
test_v!(u75, get_todays_sales, set_todays_sales, "s75");
test_v!(u76, get_todays_sales, set_todays_sales, "s76");
test_v!(u77, get_todays_sales, set_todays_sales, "s77");
test_v!(u78, get_todays_sales, set_todays_sales, "s78");
test_v!(u79, get_todays_sales, set_todays_sales, "s79");
test_v!(u80, get_todays_sales, set_todays_sales, "s80");
