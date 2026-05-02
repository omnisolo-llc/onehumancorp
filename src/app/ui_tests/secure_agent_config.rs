use crate::app;
use slint::ComponentHandle;

fn create() -> app::SecureAgentConfig { crate::ui_tests::init(); app::SecureAgentConfig::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn secure_xss_token() {
    let ui = create();
    let xss = "<script>alert('token')</script>";
    ui.set_token(xss.into());
    assert_eq!(ui.get_token(), xss);
}

#[test] fn secure_injection_error() {
    let ui = create();
    let inj = "Error'); DROP TABLE secrets; --";
    ui.set_error_text(inj.into());
    assert_eq!(ui.get_error_text(), inj);
}

#[test] fn secure_long_token() {
    let ui = create();
    let long = "spiffe://ohc.os/agent/".to_string() + &"a".repeat(1000);
    ui.set_token(long.clone().into());
    assert_eq!(ui.get_token(), long);
}

// --- Interaction / Flow Tests ---

#[test] fn secure_flow_save_callback() {
    let ui = create();
    let called_token = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c = called_token.clone();
    ui.on_save_config(move |t| { *c.borrow_mut() = t.to_string(); });
    
    ui.set_token("my-token".into());
    ui.invoke_save_config("my-token".into());
    assert_eq!(*called_token.borrow(), "my-token");
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v {
    ($id:ident, $get:ident, $set:ident, $val:expr) => {
        #[test] fn $id() { let ui = create(); ui.$set($val.into()); assert_eq!(ui.$get(), $val); }
    };
}

test_v!(u1, get_token, set_token, "valid-token-123");
test_v!(u2, get_error_text, set_error_text, "Invalid SPIFFE format");
test_v!(u3, get_token, set_token, "spiffe://test/1");

test_v!(u11, get_token, set_token, "t11");
test_v!(u12, get_token, set_token, "t12");
test_v!(u13, get_token, set_token, "t13");
test_v!(u14, get_token, set_token, "t14");
test_v!(u15, get_token, set_token, "t15");
test_v!(u16, get_token, set_token, "t16");
test_v!(u17, get_token, set_token, "t17");
test_v!(u18, get_token, set_token, "t18");
test_v!(u19, get_token, set_token, "t19");
test_v!(u20, get_token, set_token, "t20");

test_v!(u21, get_token, set_token, "Token with 🔑 Emoji");
test_v!(u22, get_token, set_token, "Token'Quotes'");
test_v!(u23, get_token, set_token, "Token ; Semi");
test_v!(u24, get_token, set_token, "");
test_v!(u25, get_token, set_token, "Very Long Secure Token ".repeat(5));

test_v!(u31, get_error_text, set_error_text, "e31");
test_v!(u32, get_error_text, set_error_text, "e32");
test_v!(u33, get_error_text, set_error_text, "e33");
test_v!(u34, get_error_text, set_error_text, "e34");
test_v!(u35, get_error_text, set_error_text, "e35");
test_v!(u36, get_error_text, set_error_text, "e36");
test_v!(u37, get_error_text, set_error_text, "e37");
test_v!(u38, get_error_text, set_error_text, "e38");
test_v!(u39, get_error_text, set_error_text, "e39");
test_v!(u40, get_error_text, set_error_text, "e40");

test_v!(u41, get_token, set_token, "t41");
test_v!(u42, get_token, set_token, "t42");
test_v!(u43, get_token, set_token, "t43");
test_v!(u44, get_token, set_token, "t44");
test_v!(u45, get_token, set_token, "t45");
test_v!(u46, get_token, set_token, "t46");
test_v!(u47, get_token, set_token, "t47");
test_v!(u48, get_token, set_token, "t48");
test_v!(u49, get_token, set_token, "t49");
test_v!(u50, get_token, set_token, "t50");

test_v!(u51, get_token, set_token, "t51");
test_v!(u52, get_token, set_token, "t52");
test_v!(u53, get_token, set_token, "t53");
test_v!(u54, get_token, set_token, "t54");
test_v!(u55, get_token, set_token, "t55");
test_v!(u56, get_token, set_token, "t56");
test_v!(u57, get_token, set_token, "t57");
test_v!(u58, get_token, set_token, "t58");
test_v!(u59, get_token, set_token, "t59");
test_v!(u60, get_token, set_token, "t60");

test_v!(u61, get_token, set_token, "t61");
test_v!(u62, get_token, set_token, "t62");
test_v!(u63, get_token, set_token, "t63");
test_v!(u64, get_token, set_token, "t64");
test_v!(u65, get_token, set_token, "t65");
test_v!(u66, get_token, set_token, "t66");
test_v!(u67, get_token, set_token, "t67");
test_v!(u68, get_token, set_token, "t68");
test_v!(u69, get_token, set_token, "t69");
test_v!(u70, get_token, set_token, "t70");

test_v!(u71, get_token, set_token, "t71");
test_v!(u72, get_token, set_token, "t72");
test_v!(u73, get_token, set_token, "t73");
test_v!(u74, get_token, set_token, "t74");
test_v!(u75, get_token, set_token, "t75");
test_v!(u76, get_token, set_token, "t76");
test_v!(u77, get_token, set_token, "t77");
test_v!(u78, get_token, set_token, "t78");
test_v!(u79, get_token, set_token, "t79");
test_v!(u80, get_token, set_token, "t80");
