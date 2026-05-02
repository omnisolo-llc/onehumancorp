use crate::app;
use slint::ComponentHandle;
use slint::Model;
use std::rc::Rc;

fn create() -> app::Channels { crate::ui_tests::init(); app::Channels::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn channels_xss_name() {
    let ui = create();
    let xss = "<script>alert('channel')</script>";
    let model = slint::VecModel::from(vec![app::UiChatChannel {
        id: "1".into(),
        name: xss.into(),
        backend_name: "slack".into(),
        icon: "💬".into(),
        enabled: true,
    }]);
    ui.set_channels(Rc::new(model).into());
    assert_eq!(ui.get_channels().row_data(0).unwrap().name, xss);
}

#[test] fn channels_injection_backend() {
    let ui = create();
    let inj = "slack'); DROP TABLE channels; --";
    let model = slint::VecModel::from(vec![app::UiChatChannel {
        id: "1".into(),
        name: "Slack".into(),
        backend_name: inj.into(),
        icon: "💬".into(),
        enabled: true,
    }]);
    ui.set_channels(Rc::new(model).into());
    assert_eq!(ui.get_channels().row_data(0).unwrap().backend_name, inj);
}

#[test] fn channels_massive_list() {
    let ui = create();
    let v: Vec<app::UiChatChannel> = (0..500).map(|i| app::UiChatChannel {
        id: i.to_string().into(),
        name: format!("Chan {}", i).into(),
        backend_name: "test".into(),
        icon: "🔗".into(),
        enabled: i % 2 == 0,
    }).collect();
    ui.set_channels(Rc::new(slint::VecModel::from(v)).into());
    assert_eq!(ui.get_channels().row_count(), 500);
}

// --- Interaction / Flow Tests ---

#[test] fn channels_flow_add_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_add_channel(move || { *c.borrow_mut() = true; });
    ui.invoke_add_channel();
    assert!(*called.borrow());
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v_c {
    ($id:ident, $name:expr) => {
        #[test] fn $id() {
            let ui = create();
            let model = slint::VecModel::from(vec![app::UiChatChannel {
                id: "id".into(),
                name: $name.into(),
                backend_name: "be".into(),
                icon: "i".into(),
                enabled: true,
            }]);
            ui.set_channels(Rc::new(model).into());
            assert_eq!(ui.get_channels().row_data(0).unwrap().name, $name);
        }
    };
}

test_v_c!(u1, "Slack Integration");
test_v_c!(u2, "Discord Bot");
test_v_c!(u3, "Telegram Channel");

test_v_c!(u11, "c11");
test_v_c!(u12, "c12");
test_v_c!(u13, "c13");
test_v_c!(u14, "c14");
test_v_c!(u15, "c15");
test_v_c!(u16, "c16");
test_v_c!(u17, "c17");
test_v_c!(u18, "c18");
test_v_c!(u19, "c19");
test_v_c!(u20, "c20");

test_v_c!(u21, "Chan with 🚀 Emoji");
test_v_c!(u22, "Chan'Quotes'");
test_v_c!(u23, "Chan ; Semi");
test_v_c!(u24, "");
test_v_c!(u25, "Very Long Channel Name ".repeat(5));

test_v_c!(u31, "c31");
test_v_c!(u32, "c32");
test_v_c!(u33, "c33");
test_v_c!(u34, "c34");
test_v_c!(u35, "c35");
test_v_c!(u36, "c36");
test_v_c!(u37, "c37");
test_v_c!(u38, "c38");
test_v_c!(u39, "c39");
test_v_c!(u40, "c40");

test_v_c!(u41, "c41");
test_v_c!(u42, "c42");
test_v_c!(u43, "c43");
test_v_c!(u44, "c44");
test_v_c!(u45, "c45");
test_v_c!(u46, "c46");
test_v_c!(u47, "c47");
test_v_c!(u48, "c48");
test_v_c!(u49, "c49");
test_v_c!(u50, "c50");

test_v_c!(u51, "c51");
test_v_c!(u52, "c52");
test_v_c!(u53, "c53");
test_v_c!(u54, "c54");
test_v_c!(u55, "c55");
test_v_c!(u56, "c56");
test_v_c!(u57, "c57");
test_v_c!(u58, "c58");
test_v_c!(u59, "c59");
test_v_c!(u60, "c60");

test_v_c!(u61, "c61");
test_v_c!(u62, "c62");
test_v_c!(u63, "c63");
test_v_c!(u64, "c64");
test_v_c!(u65, "c65");
test_v_c!(u66, "c66");
test_v_c!(u67, "c67");
test_v_c!(u68, "c68");
test_v_c!(u69, "c69");
test_v_c!(u70, "c70");

test_v_c!(u71, "c71");
test_v_c!(u72, "c72");
test_v_c!(u73, "c73");
test_v_c!(u74, "c74");
test_v_c!(u75, "c75");
test_v_c!(u76, "c76");
test_v_c!(u77, "c77");
test_v_c!(u78, "c78");
test_v_c!(u79, "c79");
test_v_c!(u80, "c80");
