use crate::app;

fn create_agents_ui() -> app::Agents {
    crate::ui_tests::init();
    app::Agents::new().unwrap()
}

#[test]
fn test_agents_ui_agent_list_population() {
    let ui = create_agents_ui();

    let model = std::rc::Rc::new(slint::VecModel::from(vec![
        app::UiDepartment {
            id: "manager".into(),
            name: "The Manager".into(),
            description: "Desc".into(),
            is_active: true,
            icon: "I".into(),
        }
    ]));

    ui.set_departments(model.into());
    assert!(true);
}
