use crate::app;
use slint::ComponentHandle;

fn create() -> app::BusinessShare { crate::ui_tests::init(); app::BusinessShare::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn share_xss_name() {
    let ui = create();
    let xss = "<script>alert('share')</script>";
    ui.set_business_name(xss.into());
    assert_eq!(ui.get_business_name(), xss);
}

#[test] fn share_injection_tagline() {
    let ui = create();
    let inj = "Best Store'); DROP TABLE stores; --";
    ui.set_business_tagline(inj.into());
    assert_eq!(ui.get_business_tagline(), inj);
}

#[test] fn share_long_link() {
    let ui = create();
    let long = "ohc://share?b=".to_string() + &"f".repeat(1000);
    ui.set_share_link(long.clone().into());
    assert_eq!(ui.get_share_link(), long);
}

// --- Interaction / Flow Tests ---

#[test] fn share_flow_copy_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_copy_link(move || { *c.borrow_mut() = true; });
    ui.invoke_copy_link();
    assert!(*called.borrow());
}

#[test] fn share_flow_insta_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_share_to_instagram(move || { *c.borrow_mut() = true; });
    ui.invoke_share_to_instagram();
    assert!(*called.borrow());
}

#[test] fn share_flow_x_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_share_to_x(move || { *c.borrow_mut() = true; });
    ui.invoke_share_to_x();
    assert!(*called.borrow());
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v {
    ($id:ident, $get:ident, $set:ident, $val:expr) => {
        #[test] fn $id() { let ui = create(); ui.$set($val.into()); assert_eq!(ui.$get(), $val); }
    };
}

test_v!(u1, get_business_name, set_business_name, "Alpha Store");
test_v!(u2, get_business_tagline, set_business_tagline, "Quality First");
test_v!(u3, get_share_link, set_share_link, "https://link.com");
#[test] fn u4() { let ui = create(); ui.invoke_close(); }

test_v!(u11, get_business_name, set_business_name, "s11");
test_v!(u12, get_business_name, set_business_name, "s12");
test_v!(u13, get_business_name, set_business_name, "s13");
test_v!(u14, get_business_name, set_business_name, "s14");
test_v!(u15, get_business_name, set_business_name, "s15");
test_v!(u16, get_business_name, set_business_name, "s16");
test_v!(u17, get_business_name, set_business_name, "s17");
test_v!(u18, get_business_name, set_business_name, "s18");
test_v!(u19, get_business_name, set_business_name, "s19");
test_v!(u20, get_business_name, set_business_name, "s20");

test_v!(u21, get_business_tagline, set_business_tagline, "t21");
test_v!(u22, get_business_tagline, set_business_tagline, "t22");
test_v!(u23, get_business_tagline, set_business_tagline, "t23");
test_v!(u24, get_business_tagline, set_business_tagline, "t24");
test_v!(u25, get_business_tagline, set_business_tagline, "t25");
test_v!(u26, get_business_tagline, set_business_tagline, "t26");
test_v!(u27, get_business_tagline, set_business_tagline, "t27");
test_v!(u28, get_business_tagline, set_business_tagline, "t28");
test_v!(u29, get_business_tagline, set_business_tagline, "t29");
test_v!(u30, get_business_tagline, set_business_tagline, "t30");

test_v!(u31, get_business_name, set_business_name, "Store with 🛍️ Emoji");
test_v!(u32, get_business_name, set_business_name, "Store'Quotes'");
test_v!(u33, get_business_name, set_business_name, "Store;Semi");
test_v!(u34, get_business_name, set_business_name, "");
test_v!(u35, get_business_name, set_business_name, "Very Long Business Name ".repeat(5));

test_v!(u41, get_share_link, set_share_link, "l41");
test_v!(u42, get_share_link, set_share_link, "l42");
test_v!(u43, get_share_link, set_share_link, "l43");
test_v!(u44, get_share_link, set_share_link, "l44");
test_v!(u45, get_share_link, set_share_link, "l45");
test_v!(u46, get_share_link, set_share_link, "l46");
test_v!(u47, get_share_link, set_share_link, "l47");
test_v!(u48, get_share_link, set_share_link, "l48");
test_v!(u49, get_share_link, set_share_link, "l49");
test_v!(u50, get_share_link, set_share_link, "l50");

test_v!(u51, get_business_name, set_business_name, "n51");
test_v!(u52, get_business_name, set_business_name, "n52");
test_v!(u53, get_business_name, set_business_name, "n53");
test_v!(u54, get_business_name, set_business_name, "n54");
test_v!(u55, get_business_name, set_business_name, "n55");
test_v!(u56, get_business_name, set_business_name, "n56");
test_v!(u57, get_business_name, set_business_name, "n57");
test_v!(u58, get_business_name, set_business_name, "n58");
test_v!(u59, get_business_name, set_business_name, "n59");
test_v!(u60, get_business_name, set_business_name, "n60");

test_v!(u61, get_business_tagline, set_business_tagline, "t61");
test_v!(u62, get_business_tagline, set_business_tagline, "t62");
test_v!(u63, get_business_tagline, set_business_tagline, "t63");
test_v!(u64, get_business_tagline, set_business_tagline, "t64");
test_v!(u65, get_business_tagline, set_business_tagline, "t65");
test_v!(u66, get_business_tagline, set_business_tagline, "t66");
test_v!(u67, get_business_tagline, set_business_tagline, "t67");
test_v!(u68, get_business_tagline, set_business_tagline, "t68");
test_v!(u69, get_business_tagline, set_business_tagline, "t69");
test_v!(u70, get_business_tagline, set_business_tagline, "t70");

test_v!(u71, get_share_link, set_share_link, "l71");
test_v!(u72, get_share_link, set_share_link, "l72");
test_v!(u73, get_share_link, set_share_link, "l73");
test_v!(u74, get_share_link, set_share_link, "l74");
test_v!(u75, get_share_link, set_share_link, "l75");
test_v!(u76, get_share_link, set_share_link, "l76");
test_v!(u77, get_share_link, set_share_link, "l77");
test_v!(u78, get_share_link, set_share_link, "l78");
test_v!(u79, get_share_link, set_share_link, "l79");
test_v!(u80, get_share_link, set_share_link, "l80");
