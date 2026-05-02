use crate::app;
use slint::ComponentHandle;

fn create() -> app::SetupWizard { crate::ui_tests::init(); app::SetupWizard::new().unwrap() }

// --- Specialized / Hacking Cases ---

#[test] fn wizard_step_negative() {
    let ui = create();
    ui.set_step(-1);
    assert_eq!(ui.get_step(), -1);
}

#[test] fn wizard_step_overflow() {
    let ui = create();
    ui.set_step(1000);
    assert_eq!(ui.get_step(), 1000);
}

#[test] fn wizard_xss_company_name() {
    let ui = create();
    let xss = "<img src=x onerror=alert(1)>";
    ui.set_company_name(xss.into());
    assert_eq!(ui.get_company_name(), xss);
}

#[test] fn wizard_injection_bio() {
    let ui = create();
    let inj = "'); DROP TABLE users; --";
    ui.set_instant_bio(inj.into());
    assert_eq!(ui.get_instant_bio(), inj);
}

#[test] fn wizard_unicode_launch_status() {
    let ui = create();
    let status = "🚀 Deploying... 🛰️";
    ui.set_launch_status(status.into());
    assert_eq!(ui.get_launch_status(), status);
}

// --- Interaction / Flow Tests ---

#[test] fn wizard_flow_step_by_step_data_retention() {
    let ui = create();
    ui.set_step(1);
    ui.set_company_name("Acme".into());
    ui.set_step(2);
    assert_eq!(ui.get_company_name(), "Acme");
    ui.set_business_type("SaaS".into());
    ui.set_step(3);
    assert_eq!(ui.get_business_type(), "SaaS");
    assert_eq!(ui.get_company_name(), "Acme");
}

#[test] fn wizard_flow_toggle_all_checkboxes() {
    let ui = create();
    ui.set_sell_physical(true);
    ui.set_sell_digital(true);
    ui.set_sell_services(true);
    ui.set_sell_food(true);
    ui.set_sell_subscriptions(true);
    assert!(ui.get_sell_physical());
    assert!(ui.get_sell_digital());
    assert!(ui.get_sell_services());
    assert!(ui.get_sell_food());
    assert!(ui.get_sell_subscriptions());
}

#[test] fn wizard_flow_rapid_step_change() {
    let ui = create();
    for i in 0..50 {
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

test_v!(u1, get_company_name, set_company_name, "Globex Corp");
test_v!(u2, get_company_name, set_company_name, "Initech");
test_v!(u3, get_company_name, set_company_name, "Umbrella Corp");
test_v!(u4, get_company_description, set_company_description, "Leading provider of nothing.");
test_v!(u5, get_admin_email, set_admin_email, "admin@test.invalid");
test_v!(u6, get_admin_name, set_admin_name, "John Doe");
test_v!(u7, get_payment_pref, set_payment_pref, "Stripe");
test_v!(u8, get_payment_pref, set_payment_pref, "PayPal");
#[test] fn u9() { let ui = create(); ui.set_is_advanced(true); assert!(ui.get_is_advanced()); }
#[test] fn u10() { let ui = create(); ui.set_is_instant_build(true); assert!(ui.get_is_instant_build()); }

test_v!(u11, get_company_name, set_company_name, "C11");
test_v!(u12, get_company_name, set_company_name, "C12");
test_v!(u13, get_company_name, set_company_name, "C13");
test_v!(u14, get_company_name, set_company_name, "C14");
test_v!(u15, get_company_name, set_company_name, "C15");
test_v!(u16, get_company_name, set_company_name, "C16");
test_v!(u17, get_company_name, set_company_name, "C17");
test_v!(u18, get_company_name, set_company_name, "C18");
test_v!(u19, get_company_name, set_company_name, "C19");
test_v!(u20, get_company_name, set_company_name, "C20");

#[test] fn u21() { let ui = create(); ui.set_admin_name("Alice".into()); ui.set_admin_email("a@b.c".into()); assert_eq!(ui.get_admin_name(), "Alice"); assert_eq!(ui.get_admin_email(), "a@b.c"); }
#[test] fn u22() { let ui = create(); ui.set_admin_name("Bob".into()); ui.set_admin_email("b@c.d".into()); assert_eq!(ui.get_admin_name(), "Bob"); assert_eq!(ui.get_admin_email(), "b@c.d"); }
test_v!(u23, get_business_type, set_business_type, "Retail");
test_v!(u24, get_business_type, set_business_type, "Consulting");
test_v!(u25, get_business_type, set_business_type, "Food");
test_v!(u26, get_launch_status, set_launch_status, "Pending");
test_v!(u27, get_launch_status, set_launch_status, "Active");
test_v!(u28, get_launch_details, set_launch_details, "Logs...");
test_v!(u29, get_instant_bio, set_instant_bio, "Short bio");
test_v!(u30, get_instant_bio, set_instant_bio, "Very long bio...Very long bio...Very long bio...");

#[test] fn u31() { let ui = create(); ui.set_step(0); ui.set_launching(false); assert_eq!(ui.get_step(), 0); assert!(!ui.get_launching()); }
#[test] fn u32() { let ui = create(); ui.set_step(4); ui.set_launching(true); assert_eq!(ui.get_step(), 4); assert!(ui.get_launching()); }
#[test] fn u33() { let ui = create(); ui.set_sell_physical(false); ui.set_sell_digital(false); assert!(!ui.get_sell_physical()); assert!(!ui.get_sell_digital()); }
#[test] fn u34() { let ui = create(); ui.set_is_advanced(false); ui.set_is_instant_build(false); assert!(!ui.get_is_advanced()); assert!(!ui.get_is_instant_build()); }
test_v!(u35, get_admin_email, set_admin_email, "user@sub.domain.co.uk");
test_v!(u36, get_company_name, set_company_name, "Name with 123 numbers");
test_v!(u37, get_company_name, set_company_name, "Name with !@# symbols");
test_v!(u38, get_company_description, set_company_description, "Description\nwith\nnewlines");
test_v!(u39, get_payment_pref, set_payment_pref, "Crypto");
test_v!(u40, get_business_type, set_business_type, "Non-Profit");

test_v!(u41, get_step, set_step, 10);
test_v!(u42, get_step, set_step, 11);
test_v!(u43, get_step, set_step, 12);
test_v!(u44, get_step, set_step, 13);
test_v!(u45, get_step, set_step, 14);
test_v!(u46, get_step, set_step, 15);
test_v!(u47, get_step, set_step, 16);
test_v!(u48, get_step, set_step, 17);
test_v!(u49, get_step, set_step, 18);
test_v!(u50, get_step, set_step, 19);

test_v!(u51, get_company_name, set_company_name, "n51");
test_v!(u52, get_company_name, set_company_name, "n52");
test_v!(u53, get_company_name, set_company_name, "n53");
test_v!(u54, get_company_name, set_company_name, "n54");
test_v!(u55, get_company_name, set_company_name, "n55");
test_v!(u56, get_company_name, set_company_name, "n56");
test_v!(u57, get_company_name, set_company_name, "n57");
test_v!(u58, get_company_name, set_company_name, "n58");
test_v!(u59, get_company_name, set_company_name, "n59");
test_v!(u60, get_company_name, set_company_name, "n60");

test_v!(u61, get_admin_name, set_admin_name, "an61");
test_v!(u62, get_admin_name, set_admin_name, "an62");
test_v!(u63, get_admin_name, set_admin_name, "an63");
test_v!(u64, get_admin_name, set_admin_name, "an64");
test_v!(u65, get_admin_name, set_admin_name, "an65");
test_v!(u66, get_admin_email, set_admin_email, "ae66");
test_v!(u67, get_admin_email, set_admin_email, "ae67");
test_v!(u68, get_admin_email, set_admin_email, "ae68");
test_v!(u69, get_admin_email, set_admin_email, "ae69");
test_v!(u70, get_admin_email, set_admin_email, "ae70");

test_v!(u71, get_launch_status, set_launch_status, "ls71");
test_v!(u72, get_launch_status, set_launch_status, "ls72");
test_v!(u73, get_launch_status, set_launch_status, "ls73");
test_v!(u74, get_launch_status, set_launch_status, "ls74");
test_v!(u75, get_launch_status, set_launch_status, "ls75");
test_v!(u76, get_instant_bio, set_instant_bio, "ib76");
test_v!(u77, get_instant_bio, set_instant_bio, "ib77");
test_v!(u78, get_instant_bio, set_instant_bio, "ib78");
test_v!(u79, get_instant_bio, set_instant_bio, "ib79");
test_v!(u80, get_instant_bio, set_instant_bio, "ib80");
