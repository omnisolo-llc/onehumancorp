use slint::ComponentHandle;
use std::rc::Rc;
use crate::app;

#[test]
fn test_unified_inbox_meta_connection() {
    slint_testing::init_no_backend();

    let unified_inbox_ui = app::UnifiedInbox::new().unwrap();

    // Initial state
    assert_eq!(unified_inbox_ui.get_is_meta_connected(), false, "Meta should not be connected initially");
    assert_eq!(unified_inbox_ui.get_show_meta_banner(), true, "Meta banner should be visible initially");

    // Wire up the callbacks (using the same logic as desktop)
    let unified_inbox_handle_meta = unified_inbox_ui.as_weak();
    unified_inbox_ui.on_action_connect_meta(move || {
        if let Some(ui) = unified_inbox_handle_meta.upgrade() {
            ui.set_is_meta_connected(true);
            ui.set_show_meta_banner(false);

            // Add fake conversation
            let mut convs: Vec<app::UiConversation> = ui.get_conversations().iter().collect();
            convs.insert(0, app::UiConversation {
                id: "meta-1".into(),
                customer_name: "Maya (Instagram)".into(),
                channel_icon: "📸".into(),
                last_message: "Do you make vegan cakes?".into(),
                unread: true,
                time: "Just now".into(),
            });
            ui.set_conversations(slint::ModelRc::new(slint::VecModel::from(convs)));
        }
    });

    let unified_inbox_handle_dismiss = unified_inbox_ui.as_weak();
    unified_inbox_ui.on_dismiss_meta_banner(move || {
        if let Some(ui) = unified_inbox_handle_dismiss.upgrade() {
            ui.set_show_meta_banner(false);
        }
    });

    // Action 1: Connect Meta Accounts
    unified_inbox_ui.invoke_action_connect_meta();

    // Verify UI state after connection
    assert_eq!(unified_inbox_ui.get_is_meta_connected(), true, "Meta should be connected after invoking action");
    assert_eq!(unified_inbox_ui.get_show_meta_banner(), false, "Meta banner should be hidden after connection");

    // Verify a new conversation was added
    let convs = unified_inbox_ui.get_conversations();
    assert_eq!(convs.row_count(), 1, "Should have 1 conversation after connection");

    let first_conv = convs.row_data(0).unwrap();
    assert_eq!(first_conv.customer_name, "Maya (Instagram)", "New conversation should be from Maya");
    assert_eq!(first_conv.channel_icon, "📸", "New conversation should have Instagram icon");
}
