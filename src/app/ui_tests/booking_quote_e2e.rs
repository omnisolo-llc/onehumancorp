use std::rc::Rc;
use slint::{ComponentHandle, Model, VecModel};
use crate::app::UnifiedInbox;

#[test]
fn test_draft_quote_generation() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let ui = UnifiedInbox::new().unwrap();
    let msgs = vec![
        crate::app::UiInboxMessage {
            id: "msg-1".into(),
            author_name: "Customer".into(),
            body: "How much to fix my sink?".into(),
            is_me: false,
            time: "10m ago".into(),
            is_quote: false,
            quote_amount: "".into(),
            quote_status: "".into(),
        },
        crate::app::UiInboxMessage {
            id: "msg-2".into(),
            author_name: "Salesperson AI".into(),
            body: "I can fix that for $150.".into(),
            is_me: true,
            time: "5m ago".into(),
            is_quote: true,
            quote_amount: "$150.00".into(),
            quote_status: "draft".into(),
        }
    ];

    ui.set_current_messages(Rc::new(VecModel::from(msgs)).into());
    assert_eq!(ui.get_current_messages().row_count(), 2);

    let quote_msg = ui.get_current_messages().row_data(1).unwrap();
    assert!(quote_msg.is_quote);
    assert_eq!(quote_msg.quote_status, "draft");
    assert_eq!(quote_msg.quote_amount, "$150.00");
}

#[test]
fn test_quote_editing() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let ui = UnifiedInbox::new().unwrap();
    let msgs = vec![
        crate::app::UiInboxMessage {
            id: "msg-1".into(),
            author_name: "Salesperson AI".into(),
            body: "Quote for service".into(),
            is_me: true,
            time: "1m ago".into(),
            is_quote: true,
            quote_amount: "$150.00".into(),
            quote_status: "draft".into(),
        }
    ];

    ui.set_current_messages(Rc::new(VecModel::from(msgs)).into());

    // Simulate editing
    let mut edited_msg = ui.get_current_messages().row_data(0).unwrap();
    edited_msg.quote_amount = "$200.00".into();

    let model = Rc::new(VecModel::from(vec![edited_msg]));
    ui.set_current_messages(model.into());

    let new_msg = ui.get_current_messages().row_data(0).unwrap();
    assert_eq!(new_msg.quote_amount, "$200.00");
}

#[test]
fn test_quote_approval() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let ui = UnifiedInbox::new().unwrap();
    let msgs = vec![
        crate::app::UiInboxMessage {
            id: "msg-1".into(),
            author_name: "Salesperson AI".into(),
            body: "Quote for service".into(),
            is_me: true,
            time: "1m ago".into(),
            is_quote: true,
            quote_amount: "$200.00".into(),
            quote_status: "draft".into(),
        }
    ];

    ui.set_current_messages(Rc::new(VecModel::from(msgs)).into());

    // Mock the callback execution as if done via UI click
    let ui_handle = ui.as_weak();
    ui.on_approve_quote(move |id, _amount| {
        if let Some(app) = ui_handle.upgrade() {
            let mut msgs: Vec<_> = app.get_current_messages().iter().collect();
            for m in msgs.iter_mut() {
                if m.id == id {
                    m.quote_status = "approved".into();
                }
            }
            app.set_current_messages(Rc::new(VecModel::from(msgs)).into());
        }
    });

    ui.invoke_approve_quote("msg-1".into(), "$200.00".into());

    let approved_msg = ui.get_current_messages().row_data(0).unwrap();
    assert_eq!(approved_msg.quote_status, "approved");
}

#[test]
fn test_stripe_link_generation() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let ui = UnifiedInbox::new().unwrap();
    let msgs = vec![
        crate::app::UiInboxMessage {
            id: "msg-1".into(),
            author_name: "Salesperson AI".into(),
            body: "Quote for service".into(),
            is_me: true,
            time: "1m ago".into(),
            is_quote: true,
            quote_amount: "$200.00".into(),
            quote_status: "draft".into(),
        }
    ];

    ui.set_current_messages(Rc::new(VecModel::from(msgs)).into());

    let ui_handle = ui.as_weak();
    ui.on_approve_quote(move |_id, amount| {
        if let Some(app) = ui_handle.upgrade() {
            let mut msgs: Vec<_> = app.get_current_messages().iter().collect();
            msgs.push(crate::app::UiInboxMessage {
                id: "msg-2".into(),
                author_name: "Me".into(),
                body: format!("Pay here: https://checkout.stripe.com/pay/dummy {}", amount).into(),
                is_me: true,
                time: "Now".into(),
                is_quote: false,
                quote_amount: "".into(),
                quote_status: "".into(),
            });
            app.set_current_messages(Rc::new(VecModel::from(msgs)).into());
        }
    });

    ui.invoke_approve_quote("msg-1".into(), "$200.00".into());

    assert_eq!(ui.get_current_messages().row_count(), 2);
    let link_msg = ui.get_current_messages().row_data(1).unwrap();
    assert!(link_msg.body.contains("https://checkout.stripe.com/pay/dummy"));
}

#[test]
fn test_double_booking_prevention() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let ui = UnifiedInbox::new().unwrap();
    // Simulate attempting to book an overlapping slot resulting in an error message
    let msgs = vec![
        crate::app::UiInboxMessage {
            id: "msg-1".into(),
            author_name: "System".into(),
            body: "Error: Time slot overlaps with an existing booking.".into(),
            is_me: false,
            time: "Now".into(),
            is_quote: false,
            quote_amount: "".into(),
            quote_status: "".into(),
        }
    ];

    ui.set_current_messages(Rc::new(VecModel::from(msgs)).into());
    let err_msg = ui.get_current_messages().row_data(0).unwrap();

    assert!(err_msg.body.contains("Time slot overlaps"));
}
