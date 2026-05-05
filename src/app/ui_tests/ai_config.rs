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
