use crate::app;
use slint::ComponentHandle;

fn create() -> app::AiHelpChat { crate::ui_tests::init(); app::AiHelpChat::new().unwrap() }

// --- Specialized / Flow Tests ---

#[test] fn chat_help_flow_send_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_send_message(move || { *c.borrow_mut() = true; });
    ui.invoke_send_message();
    assert!(*called.borrow());
}

#[test] fn chat_help_xss_input() {
    let ui = create();
    let xss = "<script>alert('chat_help')</script>";
    ui.set_user_input(xss.into());
    assert_eq!(ui.get_user_input(), xss);
}

#[test] fn chat_help_injection_input() {
    let ui = create();
    let inj = "Help'); DROP TABLE history; --";
    ui.set_user_input(inj.into());
    assert_eq!(ui.get_user_input(), inj);
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v {
    ($id:ident, $get:ident, $set:ident, $val:expr) => {
        #[test] fn $id() { let ui = create(); ui.$set($val.into()); assert_eq!(ui.$get(), $val); }
    };
}

test_v!(u1, get_user_input, set_user_input, "How do I add a product?");
test_v!(u2, get_user_input, set_user_input, "Can I use Apple Pay?");
test_v!(u3, get_user_input, set_user_input, "What is an AI agent?");

test_v!(u11, get_user_input, set_user_input, "i11");
test_v!(u12, get_user_input, set_user_input, "i12");
test_v!(u13, get_user_input, set_user_input, "i13");
test_v!(u14, get_user_input, set_user_input, "i14");
test_v!(u15, get_user_input, set_user_input, "i15");
test_v!(u16, get_user_input, set_user_input, "i16");
test_v!(u17, get_user_input, set_user_input, "i17");
test_v!(u18, get_user_input, set_user_input, "i18");
test_v!(u19, get_user_input, set_user_input, "i19");
test_v!(u20, get_user_input, set_user_input, "i20");

test_v!(u21, get_user_input, set_user_input, "Input with 💬 Emoji");
test_v!(u22, get_user_input, set_user_input, "Input'Quotes'");
test_v!(u23, get_user_input, set_user_input, "Input ; Semi");
test_v!(u24, get_user_input, set_user_input, "");
test_v!(u25, get_user_input, set_user_input, "Very Long Chat Input ".repeat(10));

test_v!(u31, get_user_input, set_user_input, "i31");
test_v!(u32, get_user_input, set_user_input, "i32");
test_v!(u33, get_user_input, set_user_input, "i33");
test_v!(u34, get_user_input, set_user_input, "i34");
test_v!(u35, get_user_input, set_user_input, "i35");
test_v!(u36, get_user_input, set_user_input, "i36");
test_v!(u37, get_user_input, set_user_input, "i37");
test_v!(u38, get_user_input, set_user_input, "i38");
test_v!(u39, get_user_input, set_user_input, "i39");
test_v!(u40, get_user_input, set_user_input, "i40");

test_v!(u41, get_user_input, set_user_input, "i41");
test_v!(u42, get_user_input, set_user_input, "i42");
test_v!(u43, get_user_input, set_user_input, "i43");
test_v!(u44, get_user_input, set_user_input, "i44");
test_v!(u45, get_user_input, set_user_input, "i45");
test_v!(u46, get_user_input, set_user_input, "i46");
test_v!(u47, get_user_input, set_user_input, "i47");
test_v!(u48, get_user_input, set_user_input, "i48");
test_v!(u49, get_user_input, set_user_input, "i49");
test_v!(u50, get_user_input, set_user_input, "i50");

test_v!(u51, get_user_input, set_user_input, "i51");
test_v!(u52, get_user_input, set_user_input, "i52");
test_v!(u53, get_user_input, set_user_input, "i53");
test_v!(u54, get_user_input, set_user_input, "i54");
test_v!(u55, get_user_input, set_user_input, "i55");
test_v!(u56, get_user_input, set_user_input, "i56");
test_v!(u57, get_user_input, set_user_input, "i57");
test_v!(u58, get_user_input, set_user_input, "i58");
test_v!(u59, get_user_input, set_user_input, "i59");
test_v!(u60, get_user_input, set_user_input, "i60");

test_v!(u61, get_user_input, set_user_input, "i61");
test_v!(u62, get_user_input, set_user_input, "i62");
test_v!(u63, get_user_input, set_user_input, "i63");
test_v!(u64, get_user_input, set_user_input, "i64");
test_v!(u65, get_user_input, set_user_input, "i65");
test_v!(u66, get_user_input, set_user_input, "i66");
test_v!(u67, get_user_input, set_user_input, "i67");
test_v!(u68, get_user_input, set_user_input, "i68");
test_v!(u69, get_user_input, set_user_input, "i69");
test_v!(u70, get_user_input, set_user_input, "i70");

test_v!(u71, get_user_input, set_user_input, "i71");
test_v!(u72, get_user_input, set_user_input, "i72");
test_v!(u73, get_user_input, set_user_input, "i73");
test_v!(u74, get_user_input, set_user_input, "i74");
test_v!(u75, get_user_input, set_user_input, "i75");
test_v!(u76, get_user_input, set_user_input, "i76");
test_v!(u77, get_user_input, set_user_input, "i77");
test_v!(u78, get_user_input, set_user_input, "i78");
test_v!(u79, get_user_input, set_user_input, "i79");
test_v!(u80, get_user_input, set_user_input, "i80");
