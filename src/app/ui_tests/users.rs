use crate::app;
use slint::Model;
use std::rc::Rc;

fn create() -> app::UserManagement { crate::ui_tests::init(); app::UserManagement::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn users_xss_email() {
    let ui = create();
    let xss = "<script>alert('email')</script>";
    let users = slint::VecModel::from(vec![
        app::UiUser {
            id: "1".into(),
            username: "Admin".into(),
            email: xss.into(),
            role: "Admin".into(),
            joined_at: "now".into(),
            avatar_letter: "A".into(),
        }
    ]);
    ui.set_users(Rc::new(users).into());
    assert_eq!(ui.get_users().row_data(0).unwrap().email, xss);
}

#[test] fn users_injection_username() {
    let ui = create();
    let inj = "user'); DROP TABLE users; --";
    let users = slint::VecModel::from(vec![
        app::UiUser {
            id: "2".into(),
            username: inj.into(),
            email: "test@test.com".into(),
            role: "User".into(),
            joined_at: "today".into(),
            avatar_letter: "U".into(),
        }
    ]);
    ui.set_users(Rc::new(users).into());
    assert_eq!(ui.get_users().row_data(0).unwrap().username, inj);
}

#[test] fn users_avatar_emoji() {
    let ui = create();
    let emoji = "👨‍💻";
    let users = slint::VecModel::from(vec![
        app::UiUser {
            id: "3".into(),
            username: "Dev".into(),
            email: "dev@dev.com".into(),
            role: "Dev".into(),
            joined_at: "yesterday".into(),
            avatar_letter: emoji.into(),
        }
    ]);
    ui.set_users(Rc::new(users).into());
    assert_eq!(ui.get_users().row_data(0).unwrap().avatar_letter, emoji);
}

// --- Interaction / Flow Tests ---

#[test] fn users_flow_delete_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c = called.clone();
    ui.on_delete_user(move |id| { *c.borrow_mut() = id.to_string(); });
    ui.invoke_delete_user("user-99".into());
    assert_eq!(*called.borrow(), "user-99");
}

#[test] fn users_flow_generate_viral_invite_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_generate_viral_invite(move || { *c.borrow_mut() = true; });
    ui.invoke_generate_viral_invite();
    assert!(*called.borrow());
}

#[test] fn users_flow_open_referrals_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_open_referrals(move || { *c.borrow_mut() = true; });
    ui.invoke_open_referrals();
    assert!(*called.borrow());
}

#[test] fn users_create_verify_users_list() {
    let ui = create();
    let users = slint::VecModel::from(vec![
        app::UiUser {
            id: "u1".into(),
            username: "u1".into(),
            email: "u1@test.com".into(),
            role: "Role".into(),
            joined_at: "now".into(),
            avatar_letter: "U".into(),
        }
    ]);
    ui.set_users(Rc::new(users).into());
    assert_eq!(ui.get_users().row_count(), 1);
    let r = ui.get_users().row_data(0).unwrap();
    assert_eq!(r.id, "u1");
}

#[test] fn users_create_verify_delete_user() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c = called.clone();
    ui.on_delete_user(move |id| { *c.borrow_mut() = id.to_string(); });
    ui.invoke_delete_user("u2".into());
    assert_eq!(*called.borrow(), "u2");
}

#[test] fn users_create_verify_generate_viral_invite() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_generate_viral_invite(move || { *c.borrow_mut() = true; });
    ui.invoke_generate_viral_invite();
    assert!(*called.borrow());
}

#[test] fn users_create_verify_open_referrals() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_open_referrals(move || { *c.borrow_mut() = true; });
    ui.invoke_open_referrals();
    assert!(*called.borrow());
}


// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---
