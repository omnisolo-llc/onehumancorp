use crate::app;
use slint::Model;
use std::rc::Rc;

fn create() -> app::AiConfig {
    crate::ui_tests::init();
    app::AiConfig::new().unwrap()
}

// --- Hacking / Corner Cases ---

#[test]
fn ai_config_empty_provider_id() {
    let ui = create();
    let providers = slint::VecModel::from(vec![app::UiAiConfigProvider {
        id: "".into(),
        name: "Empty".into(),
        base_url: "url".into(),
        is_official: false,
        models: Rc::new(slint::VecModel::default()).into(),
    }]);
    ui.set_providers(Rc::new(providers).into());
    assert_eq!(ui.get_providers().row_count(), 1);
    assert_eq!(ui.get_providers().row_data(0).unwrap().id, "");
}

#[test]
fn ai_config_xss_provider_name() {
    let ui = create();
    let xss = "'; alert('ai'); //";
    let providers = slint::VecModel::from(vec![app::UiAiConfigProvider {
        id: "xss".into(),
        name: xss.into(),
        base_url: "url".into(),
        is_official: false,
        models: Rc::new(slint::VecModel::default()).into(),
    }]);
    ui.set_providers(Rc::new(providers).into());
    assert_eq!(ui.get_providers().row_data(0).unwrap().name, xss);
}

#[test]
fn ai_config_massive_model_list() {
    let ui = create();
    let models: Vec<slint::SharedString> =
        (0..1000).map(|i| format!("model-{}", i).into()).collect();
    let providers = slint::VecModel::from(vec![app::UiAiConfigProvider {
        id: "big".into(),
        name: "Big".into(),
        base_url: "url".into(),
        is_official: true,
        models: Rc::new(slint::VecModel::from(models)).into(),
    }]);
    ui.set_providers(Rc::new(providers).into());
    assert_eq!(ui.get_providers().row_count(), 1);
    assert_eq!(
        ui.get_providers().row_data(0).unwrap().models.row_count(),
        1000
    );
}

// --- Interaction / Flow Tests ---

#[test]
fn ai_config_flow_add_edit_trigger() {
    let ui = create();
    let called_add = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c1 = called_add.clone();
    ui.on_add_provider(move || {
        *c1.borrow_mut() = true;
    });
    ui.invoke_add_provider();
    assert!(*called_add.borrow());

    let called_edit = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c2 = called_edit.clone();
    ui.on_edit_provider(move |id| {
        *c2.borrow_mut() = id.to_string();
    });
    ui.invoke_edit_provider("test-id".into());
    assert_eq!(*called_edit.borrow(), "test-id");
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

// --- Added tests for 100% coverage and 5 tests rule ---

#[test]
fn ai_config_test_set_is_advanced_true() {
    let ui = create();
    ui.set_is_advanced(true);
    assert_eq!(ui.get_is_advanced(), true);
}

#[test]
fn ai_config_test_set_is_advanced_false() {
    let ui = create();
    ui.set_is_advanced(false);
    assert_eq!(ui.get_is_advanced(), false);
}

#[test]
fn ai_config_test_toggle_advanced_callback() {
    let ui = create();
    let called_toggle = std::rc::Rc::new(std::cell::RefCell::new(false));
    let t1 = called_toggle.clone();
    ui.on_toggle_advanced(move || {
        *t1.borrow_mut() = true;
    });
    ui.invoke_toggle_advanced();
    assert!(*called_toggle.borrow());
}

#[test]
fn ai_config_test_get_providers_empty() {
    let ui = create();
    let providers = slint::VecModel::from(vec![]);
    ui.set_providers(Rc::new(providers).into());
    assert_eq!(ui.get_providers().row_count(), 0);
}

#[test]
fn ai_config_test_get_providers_multiple() {
    let ui = create();
    let providers = slint::VecModel::from(vec![
        app::UiAiConfigProvider {
            id: "1".into(),
            name: "A".into(),
            base_url: "urlA".into(),
            is_official: true,
            models: Rc::new(slint::VecModel::default()).into(),
        },
        app::UiAiConfigProvider {
            id: "2".into(),
            name: "B".into(),
            base_url: "urlB".into(),
            is_official: false,
            models: Rc::new(slint::VecModel::default()).into(),
        },
    ]);
    ui.set_providers(Rc::new(providers).into());
    assert_eq!(ui.get_providers().row_count(), 2);
    assert_eq!(ui.get_providers().row_data(0).unwrap().id, "1");
    assert_eq!(ui.get_providers().row_data(1).unwrap().id, "2");
}
