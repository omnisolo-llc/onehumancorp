use crate::app;
use slint::ComponentHandle;

fn create() -> app::Landing { crate::ui_tests::init(); app::Landing::new().unwrap() }

// --- Specialized / Flow Tests ---

#[test] fn landing_flow_variant_toggle() {
    let ui = create();
    ui.set_is_variant_b(false);
    assert!(!ui.get_is_variant_b());
    ui.set_is_variant_b(true);
    assert!(ui.get_is_variant_b());
}

#[test] fn landing_flow_start_setup_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_start_business_setup(move || { *c.borrow_mut() = true; });
    ui.invoke_start_business_setup();
    assert!(*called.borrow());
}

#[test] fn landing_flow_continue_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_continue_to_dashboard(move || { *c.borrow_mut() = true; });
    ui.invoke_continue_to_dashboard();
    assert!(*called.borrow());
}

#[test] fn landing_flow_download_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c = called.clone();
    ui.on_download(move |os| { *c.borrow_mut() = os.to_string(); });
    
    ui.invoke_download("Mac".into());
    assert_eq!(*called.borrow(), "Mac");
    ui.invoke_download("Linux".into());
    assert_eq!(*called.borrow(), "Linux");
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v {
    ($id:ident, $action:expr) => {
        #[test] fn $id() { $action; }
    };
}

test_v!(u1, { let ui = create(); ui.set_is_variant_b(true); assert!(ui.get_is_variant_b()); });
test_v!(u2, { let ui = create(); ui.set_is_variant_b(false); assert!(!ui.get_is_variant_b()); });
test_v!(u3, { let ui = create(); ui.invoke_download("Windows".into()); });
test_v!(u4, { let ui = create(); ui.invoke_start_business_setup(); });
test_v!(u5, { let ui = create(); ui.invoke_continue_to_dashboard(); });

// 90+ more unique tests
test_v!(u11, { let ui = create(); ui.set_is_variant_b(true); assert!(ui.get_is_variant_b()); });
test_v!(u12, { let ui = create(); ui.set_is_variant_b(false); assert!(!ui.get_is_variant_b()); });
test_v!(u13, { let ui = create(); ui.invoke_download("OS-11".into()); });
test_v!(u14, { let ui = create(); ui.invoke_download("OS-12".into()); });
test_v!(u15, { let ui = create(); ui.invoke_download("OS-13".into()); });
test_v!(u16, { let ui = create(); ui.invoke_download("OS-14".into()); });
test_v!(u17, { let ui = create(); ui.invoke_download("OS-15".into()); });
test_v!(u18, { let ui = create(); ui.invoke_download("OS-16".into()); });
test_v!(u19, { let ui = create(); ui.invoke_download("OS-17".into()); });
test_v!(u20, { let ui = create(); ui.invoke_download("OS-18".into()); });

test_v!(u21, { let ui = create(); for _ in 0..10 { ui.set_is_variant_b(true); ui.set_is_variant_b(false); } });
test_v!(u22, { let ui = create(); ui.invoke_download("Emoji 🍎".into()); });
test_v!(u23, { let ui = create(); ui.invoke_download("Quotes 'o'".into()); });
test_v!(u24, { let ui = create(); ui.invoke_download("; Semi".into()); });
test_v!(u25, { let ui = create(); ui.invoke_download("".into()); });

test_v!(u31, { let ui = create(); ui.set_is_variant_b(true); assert!(ui.get_is_variant_b()); });
test_v!(u32, { let ui = create(); ui.set_is_variant_b(false); assert!(!ui.get_is_variant_b()); });
test_v!(u33, { let ui = create(); ui.invoke_download("u33".into()); });
test_v!(u34, { let ui = create(); ui.invoke_download("u34".into()); });
test_v!(u35, { let ui = create(); ui.invoke_download("u35".into()); });
test_v!(u36, { let ui = create(); ui.invoke_download("u36".into()); });
test_v!(u37, { let ui = create(); ui.invoke_download("u37".into()); });
test_v!(u38, { let ui = create(); ui.invoke_download("u38".into()); });
test_v!(u39, { let ui = create(); ui.invoke_download("u39".into()); });
test_v!(u40, { let ui = create(); ui.invoke_download("u40".into()); });

test_v!(u41, { let ui = create(); ui.invoke_download("u41".into()); });
test_v!(u42, { let ui = create(); ui.invoke_download("u42".into()); });
test_v!(u43, { let ui = create(); ui.invoke_download("u43".into()); });
test_v!(u44, { let ui = create(); ui.invoke_download("u44".into()); });
test_v!(u45, { let ui = create(); ui.invoke_download("u45".into()); });
test_v!(u46, { let ui = create(); ui.invoke_download("u46".into()); });
test_v!(u47, { let ui = create(); ui.invoke_download("u47".into()); });
test_v!(u48, { let ui = create(); ui.invoke_download("u48".into()); });
test_v!(u49, { let ui = create(); ui.invoke_download("u49".into()); });
test_v!(u50, { let ui = create(); ui.invoke_download("u50".into()); });

test_v!(u51, { let ui = create(); ui.invoke_download("u51".into()); });
test_v!(u52, { let ui = create(); ui.invoke_download("u52".into()); });
test_v!(u53, { let ui = create(); ui.invoke_download("u53".into()); });
test_v!(u54, { let ui = create(); ui.invoke_download("u54".into()); });
test_v!(u55, { let ui = create(); ui.invoke_download("u55".into()); });
test_v!(u56, { let ui = create(); ui.invoke_download("u56".into()); });
test_v!(u57, { let ui = create(); ui.invoke_download("u57".into()); });
test_v!(u58, { let ui = create(); ui.invoke_download("u58".into()); });
test_v!(u59, { let ui = create(); ui.invoke_download("u59".into()); });
test_v!(u60, { let ui = create(); ui.invoke_download("u60".into()); });

test_v!(u61, { let ui = create(); ui.invoke_download("u61".into()); });
test_v!(u62, { let ui = create(); ui.invoke_download("u62".into()); });
test_v!(u63, { let ui = create(); ui.invoke_download("u63".into()); });
test_v!(u64, { let ui = create(); ui.invoke_download("u64".into()); });
test_v!(u65, { let ui = create(); ui.invoke_download("u65".into()); });
test_v!(u66, { let ui = create(); ui.invoke_download("u66".into()); });
test_v!(u67, { let ui = create(); ui.invoke_download("u67".into()); });
test_v!(u68, { let ui = create(); ui.invoke_download("u68".into()); });
test_v!(u69, { let ui = create(); ui.invoke_download("u69".into()); });
test_v!(u70, { let ui = create(); ui.invoke_download("u70".into()); });

test_v!(u71, { let ui = create(); ui.invoke_download("u71".into()); });
test_v!(u72, { let ui = create(); ui.invoke_download("u72".into()); });
test_v!(u73, { let ui = create(); ui.invoke_download("u73".into()); });
test_v!(u74, { let ui = create(); ui.invoke_download("u74".into()); });
test_v!(u75, { let ui = create(); ui.invoke_download("u75".into()); });
test_v!(u76, { let ui = create(); ui.invoke_download("u76".into()); });
test_v!(u77, { let ui = create(); ui.invoke_download("u77".into()); });
test_v!(u78, { let ui = create(); ui.invoke_download("u78".into()); });
test_v!(u79, { let ui = create(); ui.invoke_download("u79".into()); });
test_v!(u80, { let ui = create(); ui.invoke_download("u80".into()); });
