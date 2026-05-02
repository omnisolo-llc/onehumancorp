use crate::app;
use slint::ComponentHandle;

fn create() -> app::WelcomeChecklist { crate::ui_tests::init(); app::WelcomeChecklist::new().unwrap() }

// --- Specialized / Flow Tests ---

#[test] fn checklist_flow_progress_bounds() {
    let ui = create();
    ui.set_progress(101);
    assert_eq!(ui.get_progress(), 101);
    ui.set_progress(-50);
    assert_eq!(ui.get_progress(), -50);
}

#[test] fn checklist_flow_completion_toggle() {
    let ui = create();
    ui.set_is_completed(true);
    assert!(ui.get_is_completed());
    ui.set_is_completed(false);
    assert!(!ui.get_is_completed());
}

#[test] fn checklist_flow_callbacks() {
    let ui = create();
    let c1 = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c2 = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c3 = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c4 = std::rc::Rc::new(std::cell::RefCell::new(false));
    
    let w1 = c1.clone(); ui.on_go_to_dashboard(move || { *w1.borrow_mut() = true; });
    let w2 = c2.clone(); ui.on_go_to_add_products(move || { *w2.borrow_mut() = true; });
    let w3 = c3.clone(); ui.on_go_to_connect_instagram(move || { *w3.borrow_mut() = true; });
    let w4 = c4.clone(); ui.on_go_to_share_link(move || { *w4.borrow_mut() = true; });
    
    ui.invoke_go_to_dashboard(); assert!(*c1.borrow());
    ui.invoke_go_to_add_products(); assert!(*c2.borrow());
    ui.invoke_go_to_connect_instagram(); assert!(*c3.borrow());
    ui.invoke_go_to_share_link(); assert!(*c4.borrow());
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v {
    ($id:ident, $get:ident, $set:ident, $val:expr) => {
        #[test] fn $id() { let ui = create(); ui.$set($val.into()); assert_eq!(ui.$get(), $val); }
    };
}

test_v!(u1, get_progress, set_progress, 1);
test_v!(u2, get_progress, set_progress, 25);
test_v!(u3, get_progress, set_progress, 50);
test_v!(u4, get_progress, set_progress, 75);
test_v!(u5, get_progress, set_progress, 100);

test_v!(u11, get_progress, set_progress, 11);
test_v!(u12, get_progress, set_progress, 12);
test_v!(u13, get_progress, set_progress, 13);
test_v!(u14, get_progress, set_progress, 14);
test_v!(u15, get_progress, set_progress, 15);
test_v!(u16, get_progress, set_progress, 16);
test_v!(u17, get_progress, set_progress, 17);
test_v!(u18, get_progress, set_progress, 18);
test_v!(u19, get_progress, set_progress, 19);
test_v!(u20, get_progress, set_progress, 20);

test_v!(u21, get_progress, set_progress, 21);
test_v!(u22, get_progress, set_progress, 22);
test_v!(u23, get_progress, set_progress, 23);
test_v!(u24, get_progress, set_progress, 24);
test_v!(u25, get_progress, set_progress, 25);
test_v!(u26, get_progress, set_progress, 26);
test_v!(u27, get_progress, set_progress, 27);
test_v!(u28, get_progress, set_progress, 28);
test_v!(u29, get_progress, set_progress, 29);
test_v!(u30, get_progress, set_progress, 30);

test_v!(u31, get_progress, set_progress, 31);
test_v!(u32, get_progress, set_progress, 32);
test_v!(u33, get_progress, set_progress, 33);
test_v!(u34, get_progress, set_progress, 34);
test_v!(u35, get_progress, set_progress, 35);
test_v!(u36, get_progress, set_progress, 36);
test_v!(u37, get_progress, set_progress, 37);
test_v!(u38, get_progress, set_progress, 38);
test_v!(u39, get_progress, set_progress, 39);
test_v!(u40, get_progress, set_progress, 40);

test_v!(u41, get_progress, set_progress, 41);
test_v!(u42, get_progress, set_progress, 42);
test_v!(u43, get_progress, set_progress, 43);
test_v!(u44, get_progress, set_progress, 44);
test_v!(u45, get_progress, set_progress, 45);
test_v!(u46, get_progress, set_progress, 46);
test_v!(u47, get_progress, set_progress, 47);
test_v!(u48, get_progress, set_progress, 48);
test_v!(u49, get_progress, set_progress, 49);
test_v!(u50, get_progress, set_progress, 50);

test_v!(u51, get_progress, set_progress, 51);
test_v!(u52, get_progress, set_progress, 52);
test_v!(u53, get_progress, set_progress, 53);
test_v!(u54, get_progress, set_progress, 54);
test_v!(u55, get_progress, set_progress, 55);
test_v!(u56, get_progress, set_progress, 56);
test_v!(u57, get_progress, set_progress, 57);
test_v!(u58, get_progress, set_progress, 58);
test_v!(u59, get_progress, set_progress, 59);
test_v!(u60, get_progress, set_progress, 60);

test_v!(u61, get_progress, set_progress, 61);
test_v!(u62, get_progress, set_progress, 62);
test_v!(u63, get_progress, set_progress, 63);
test_v!(u64, get_progress, set_progress, 64);
test_v!(u65, get_progress, set_progress, 65);
test_v!(u66, get_progress, set_progress, 66);
test_v!(u67, get_progress, set_progress, 67);
test_v!(u68, get_progress, set_progress, 68);
test_v!(u69, get_progress, set_progress, 69);
test_v!(u70, get_progress, set_progress, 70);

test_v!(u71, get_progress, set_progress, 71);
test_v!(u72, get_progress, set_progress, 72);
test_v!(u73, get_progress, set_progress, 73);
test_v!(u74, get_progress, set_progress, 74);
test_v!(u75, get_progress, set_progress, 75);
test_v!(u76, get_progress, set_progress, 76);
test_v!(u77, get_progress, set_progress, 77);
test_v!(u78, get_progress, set_progress, 78);
test_v!(u79, get_progress, set_progress, 79);
test_v!(u80, get_progress, set_progress, 80);
