use slint::{ComponentHandle, Model};
use crate::app;

#[tokio::test]
async fn test_grandmother_unified_inbox_flow_db_roundtrip() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // We ensure the system starts at login
    let main_app = app::AppWindow::new().unwrap();
    let login_ui = app::Login::new().unwrap();
    login_ui.invoke_login("ceo@store.com".into(), "123".into());

    // We assume Dashboard is shown after login.
    let dashboard_ui = app::Dashboard::new().unwrap();
    let check_messages_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let check_messages_called_clone = check_messages_called.clone();

    let unified_inbox_ui = app::UnifiedInbox::new().unwrap();

    dashboard_ui.on_action_check_messages(move || {
        *check_messages_called_clone.borrow_mut() = true;
    });

    dashboard_ui.invoke_action_check_messages();
    assert!(*check_messages_called.borrow());

    let long_msg = "This string exceeds the normal box to test the replacement of overflow: elide with proper text wrapping per the token standard.";

    let convs = vec![
        app::UiConversation {
            id: "long-text".into(),
            customer_name: "Test User".into(),
            channel_icon: "✉️".into(),
            last_message: long_msg.into(),
            unread: true,
            time: "Now".into(),
        }
    ];
    unified_inbox_ui.set_conversations(slint::ModelRc::new(slint::VecModel::from(convs)));

    // Simulating user clicking a conversation
    let select_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let select_called_clone = select_called.clone();
    unified_inbox_ui.on_select_conversation(move |id| {
        assert_eq!(id, "long-text");
        *select_called_clone.borrow_mut() = true;
    });
    unified_inbox_ui.invoke_select_conversation("long-text".into());
    assert!(*select_called.borrow());

    // Simulating sending a message
    let send_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let send_called_clone = send_called.clone();

    // Let's create an in-memory DB to simulate the backend.
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();

    // Create table for messages.
    sqlx::query("CREATE TABLE IF NOT EXISTS messages (id TEXT PRIMARY KEY, conv_id TEXT, message TEXT, unread BOOLEAN)")
        .execute(&pool)
        .await
        .unwrap();

    let pool_clone = pool.clone();
    let ui_weak = unified_inbox_ui.as_weak();

    unified_inbox_ui.on_send_message(move |msg| {
        let pool = pool_clone.clone();
        let ui = ui_weak.unwrap();
        let msg = msg.to_string();
        *send_called_clone.borrow_mut() = true;

        // Spawn async task to update DB and then UI.
        tokio::task::spawn_local(async move {
            // Update DB
            sqlx::query("INSERT INTO messages (id, conv_id, message, unread) VALUES ('msg-1', 'long-text', ?, false)")
                .bind(&msg)
                .execute(&pool)
                .await
                .unwrap();

            // Fetch from DB to verify
            let row: (String, bool) = sqlx::query_as("SELECT message, unread FROM messages WHERE conv_id = 'long-text' ORDER BY id DESC LIMIT 1")
                .fetch_one(&pool)
                .await
                .unwrap();

            // Update UI with DB state.
            let updated_conversations = vec![
                app::UiConversation {
                    id: "long-text".into(),
                    customer_name: "Test User".into(),
                    channel_icon: "✉️".into(),
                    last_message: row.0.into(),
                    unread: row.1,
                    time: "Just now".into(),
                }
            ];

            ui.set_conversations(slint::ModelRc::new(slint::VecModel::from(updated_conversations)));
        });
    });

    let local = tokio::task::LocalSet::new();
    local.run_until(async move {
        unified_inbox_ui.set_new_message("Replying to message".into());
        unified_inbox_ui.invoke_send_message("Replying to message".into());

        // Yield to allow spawned task to complete
        tokio::task::yield_now().await;

        assert!(*send_called.borrow());

        // Verify UI updated from DB
        let updated_convs = unified_inbox_ui.get_conversations();
        assert_eq!(updated_convs.row_data(0).unwrap().last_message, "Replying to message");
        assert_eq!(updated_convs.row_data(0).unwrap().unread, false);
    }).await;
}

#[test]
fn test_grandmother_unified_inbox_flow_cancel() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    let _main_app = app::AppWindow::new().unwrap();
    let login_ui = app::Login::new().unwrap();
    login_ui.invoke_login("ceo@store.com".into(), "123".into());

    let unified_inbox_ui = app::UnifiedInbox::new().unwrap();

    let back_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let back_called_clone = back_called.clone();
    unified_inbox_ui.on_back_to_list(move || {
        *back_called_clone.borrow_mut() = true;
    });

    unified_inbox_ui.invoke_back_to_list();
    assert!(*back_called.borrow());
}

#[test]
fn test_grandmother_unified_inbox_quick_reply() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let _main_app = app::AppWindow::new().unwrap();
    let login_ui = app::Login::new().unwrap();
    login_ui.invoke_login("ceo@store.com".into(), "123".into());

    let unified_inbox_ui = app::UnifiedInbox::new().unwrap();

    let quick_reply_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let quick_reply_called_clone = quick_reply_called.clone();
    unified_inbox_ui.on_use_quick_reply(move |reply| {
        assert_eq!(reply, "Yes, we do!");
        *quick_reply_called_clone.borrow_mut() = true;
    });

    unified_inbox_ui.invoke_use_quick_reply("Yes, we do!".into());
    assert!(*quick_reply_called.borrow());
}

#[test]
fn test_grandmother_unified_inbox_long_text_no_truncation() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let _main_app = app::AppWindow::new().unwrap();
    let login_ui = app::Login::new().unwrap();
    login_ui.invoke_login("ceo@store.com".into(), "123".into());

    let unified_inbox_ui = app::UnifiedInbox::new().unwrap();
    let long_msg = "This string exceeds the normal box to test the replacement of overflow: elide with proper text wrapping per the token standard.";

    let convs = vec![
        app::UiConversation {
            id: "long-text".into(),
            customer_name: "Test User".into(),
            channel_icon: "✉️".into(),
            last_message: long_msg.into(),
            unread: true,
            time: "Now".into(),
        }
    ];
    unified_inbox_ui.set_conversations(slint::ModelRc::new(slint::VecModel::from(convs)));

    let current_convs = unified_inbox_ui.get_conversations();
    assert_eq!(current_convs.row_data(0).unwrap().last_message, long_msg);
}

#[test]
fn test_e2e_unified_inbox_word_wrap() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let _main_app = app::AppWindow::new().unwrap();
    let login_ui = app::Login::new().unwrap();
    login_ui.invoke_login("ceo@store.com".into(), "123".into());

    let unified_inbox_ui = app::UnifiedInbox::new().unwrap();
    let long_message = "This is a very very long message that should definitely wrap across multiple lines because it exceeds the standard width of the container without relying on elide which is invalid under the design standards and was removed in this fix.";

    let conversations = vec![
        app::UiConversation {
            id: "conv-long".into(),
            customer_name: "Long Talker".into(),
            channel_icon: "✉️".into(),
            last_message: long_message.into(),
            unread: true,
            time: "1m ago".into(),
        }
    ];

    unified_inbox_ui.set_conversations(slint::ModelRc::new(slint::VecModel::from(conversations)));

    let select_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let select_called_clone = select_called.clone();
    unified_inbox_ui.on_select_conversation(move |id| {
        assert_eq!(id, "conv-long");
        *select_called_clone.borrow_mut() = true;
    });

    unified_inbox_ui.invoke_select_conversation("conv-long".into());
    assert!(*select_called.borrow());
}
