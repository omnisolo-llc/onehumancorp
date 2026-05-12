use slint::ComponentHandle;
use std::rc::Rc;
use crate::app::{ActionFeed, UiActionItem};

#[test]
fn test_action_feed_e2e() {
    crate::ui_tests::init();
    let ui = ActionFeed::new().unwrap();

    let items = vec![
        UiActionItem {
            id: "1".into(),
            agent_id: "The Ambassador".into(),
            action_type: "Draft Message Review".into(),
            payload: "Customer asking about vegan cake".into(),
            status: "pending".into(),
            timestamp: "2 mins ago".into(),
        },
        UiActionItem {
            id: "2".into(),
            agent_id: "The Vigilant Manager".into(),
            action_type: "Inventory Alert".into(),
            payload: "Low Stock: Restock organic flour".into(),
            status: "pending".into(),
            timestamp: "1 hour ago".into(),
        },
    ];
    let model = Rc::new(slint::VecModel::from(items));
    ui.set_actions(model.into());

    // Simulate tapping on action 1 to open the modal
    ui.set_selected_action_id("1".into());
    ui.invoke_select_action("1".into());
    assert_eq!(ui.get_selected_action_id(), "1");

    ui.on_approve_action(move |id| {
        assert_eq!(id, "1");
    });

    ui.invoke_approve_action("1".into());
}
