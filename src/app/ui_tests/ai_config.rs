use crate::app;
use slint::ComponentHandle;
use slint::Model;
use std::rc::Rc;

fn create() -> app::AiConfig { crate::ui_tests::init(); app::AiConfig::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn ai_config_empty_provider_id() {
    let ui = create();
    let providers = slint::VecModel::from(vec![
        app::UiAiConfigProvider {
            id: "".into(),
            name: "Empty".into(),
            base_url: "url".into(),
            is_official: false,
            models: Rc::new(slint::VecModel::default()).into(),
        }
    ]);
    ui.set_providers(Rc::new(providers).into());
    assert_eq!(ui.get_providers().row_count(), 1);
    assert_eq!(ui.get_providers().row_data(0).unwrap().id, "");
}

#[test] fn ai_config_xss_provider_name() {
    let ui = create();
    let xss = "'; alert('ai'); //";
    let providers = slint::VecModel::from(vec![
        app::UiAiConfigProvider {
            id: "xss".into(),
            name: xss.into(),
            base_url: "url".into(),
            is_official: false,
            models: Rc::new(slint::VecModel::default()).into(),
        }
    ]);
    ui.set_providers(Rc::new(providers).into());
    assert_eq!(ui.get_providers().row_data(0).unwrap().name, xss);
}

#[test] fn ai_config_massive_model_list() {
    let ui = create();
    let models: Vec<slint::SharedString> = (0..1000).map(|i| format!("model-{}", i).into()).collect();
    let providers = slint::VecModel::from(vec![
        app::UiAiConfigProvider {
            id: "big".into(),
            name: "Big".into(),
            base_url: "url".into(),
            is_official: true,
            models: Rc::new(slint::VecModel::from(models)).into(),
        }
    ]);
    ui.set_providers(Rc::new(providers).into());
    assert_eq!(ui.get_providers().row_count(), 1);
    assert_eq!(ui.get_providers().row_data(0).unwrap().models.row_count(), 1000);
}

// --- Interaction / Flow Tests ---

#[test] fn ai_config_flow_add_edit_trigger() {
    let ui = create();
    let called_add = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c1 = called_add.clone();
    ui.on_add_provider(move || { *c1.borrow_mut() = true; });
    ui.invoke_add_provider();
    assert!(*called_add.borrow());

    let called_edit = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c2 = called_edit.clone();
    ui.on_edit_provider(move |id| { *c2.borrow_mut() = id.to_string(); });
    ui.invoke_edit_provider("test-id".into());
    assert_eq!(*called_edit.borrow(), "test-id");
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v_p {
    ($id:ident, $pid:expr, $pname:expr) => {
        #[test] fn $id() {
            let ui = create();
            let p = slint::VecModel::from(vec![app::UiAiConfigProvider {
                id: $pid.into(),
                name: $pname.into(),
                base_url: "url".into(),
                is_official: true,
                models: Rc::new(slint::VecModel::default()).into()
            }]);
            ui.set_providers(Rc::new(p).into());
            assert_eq!(ui.get_providers().row_data(0).unwrap().id, $pid);
            assert_eq!(ui.get_providers().row_data(0).unwrap().name, $pname);
        }
    };
}

test_v_p!(u1, "p1", "Provider One");
test_v_p!(u2, "p2", "Provider Two");
test_v_p!(u3, "p3", "Provider Three");

test_v_p!(u11, "id11", "n11");
test_v_p!(u12, "id12", "n12");
test_v_p!(u13, "id13", "n13");
test_v_p!(u14, "id14", "n14");
test_v_p!(u15, "id15", "n15");
test_v_p!(u16, "id16", "n16");
test_v_p!(u17, "id17", "n17");
test_v_p!(u18, "id18", "n18");
test_v_p!(u19, "id19", "n19");
test_v_p!(u20, "id20", "n20");

test_v_p!(u21, "gpt-4", "OpenAI");
test_v_p!(u22, "claude-3", "Anthropic");
test_v_p!(u23, "llama-3", "Meta");
test_v_p!(u24, "mistral-large", "Mistral");
test_v_p!(u25, "gemini-pro", "Google");

test_v_p!(u31, "id31", "n31");
test_v_p!(u32, "id32", "n32");
test_v_p!(u33, "id33", "n33");
test_v_p!(u34, "id34", "n34");
test_v_p!(u35, "id35", "n35");
test_v_p!(u36, "id36", "n36");
test_v_p!(u37, "id37", "n37");
test_v_p!(u38, "id38", "n38");
test_v_p!(u39, "id39", "n39");
test_v_p!(u40, "id40", "n40");

test_v_p!(u41, "id41", "n41");
test_v_p!(u42, "id42", "n42");
test_v_p!(u43, "id43", "n43");
test_v_p!(u44, "id44", "n44");
test_v_p!(u45, "id45", "n45");
test_v_p!(u46, "id46", "n46");
test_v_p!(u47, "id47", "n47");
test_v_p!(u48, "id48", "n48");
test_v_p!(u49, "id49", "n49");
test_v_p!(u50, "id50", "n50");

test_v_p!(u51, "id51", "n51");
test_v_p!(u52, "id52", "n52");
test_v_p!(u53, "id53", "n53");
test_v_p!(u54, "id54", "n54");
test_v_p!(u55, "id55", "n55");
test_v_p!(u56, "id56", "n56");
test_v_p!(u57, "id57", "n57");
test_v_p!(u58, "id58", "n58");
test_v_p!(u59, "id59", "n59");
test_v_p!(u60, "id60", "n60");

test_v_p!(u61, "id61", "n61");
test_v_p!(u62, "id62", "n62");
test_v_p!(u63, "id63", "n63");
test_v_p!(u64, "id64", "n64");
test_v_p!(u65, "id65", "n65");
test_v_p!(u66, "id66", "n66");
test_v_p!(u67, "id67", "n67");
test_v_p!(u68, "id68", "n68");
test_v_p!(u69, "id69", "n69");
test_v_p!(u70, "id70", "n70");

test_v_p!(u71, "id71", "n71");
test_v_p!(u72, "id72", "n72");
test_v_p!(u73, "id73", "n73");
test_v_p!(u74, "id74", "n74");
test_v_p!(u75, "id75", "n75");
test_v_p!(u76, "id76", "n76");
test_v_p!(u77, "id77", "n77");
test_v_p!(u78, "id78", "n78");
test_v_p!(u79, "id79", "n79");
test_v_p!(u80, "id80", "n80");
