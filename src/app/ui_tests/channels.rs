use crate::app;
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

// --- Consolidated Verified Tests ---
