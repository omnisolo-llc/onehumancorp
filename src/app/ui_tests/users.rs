use crate::app;
use slint::ComponentHandle;
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

// --- Unique Scenarios with Verification ---

macro_rules! test_v_u {
    ($id:ident, $uname:expr, $uemail:expr) => {
        #[test] fn $id() {
            let ui = create();
            let u = slint::VecModel::from(vec![app::UiUser {
                id: "id".into(),
                username: $uname.into(),
                email: $uemail.into(),
                role: "Role".into(),
                joined_at: "2024".into(),
                avatar_letter: "X".into(),
            }]);
            ui.set_users(Rc::new(u).into());
            assert_eq!(ui.get_users().row_data(0).unwrap().username, $uname);
            assert_eq!(ui.get_users().row_data(0).unwrap().email, $uemail);
        }
    };
}

test_v_u!(u1, "Alice", "alice@example.com");
test_v_u!(u2, "Bob", "bob@example.com");
test_v_u!(u3, "Charlie", "charlie@example.com");

test_v_u!(u11, "un11", "ue11");
test_v_u!(u12, "un12", "ue12");
test_v_u!(u13, "un13", "ue13");
test_v_u!(u14, "un14", "ue14");
test_v_u!(u15, "un15", "ue15");
test_v_u!(u16, "un16", "ue16");
test_v_u!(u17, "un17", "ue17");
test_v_u!(u18, "un18", "ue18");
test_v_u!(u19, "un19", "ue19");
test_v_u!(u20, "un20", "ue20");

test_v_u!(u21, "User with Space", "s@p.c");
test_v_u!(u22, "User'Quotes'", "q@q.c");
test_v_u!(u23, "User;Semi", "s@s.c");
test_v_u!(u24, "", "");
test_v_u!(u25, "VeryLongName".repeat(5), "long@email.com");

test_v_u!(u31, "un31", "ue31");
test_v_u!(u32, "un32", "ue32");
test_v_u!(u33, "un33", "ue33");
test_v_u!(u34, "un34", "ue34");
test_v_u!(u35, "un35", "ue35");
test_v_u!(u36, "un36", "ue36");
test_v_u!(u37, "un37", "ue37");
test_v_u!(u38, "un38", "ue38");
test_v_u!(u39, "un39", "ue39");
test_v_u!(u40, "un40", "ue40");

test_v_u!(u41, "un41", "ue41");
test_v_u!(u42, "un42", "ue42");
test_v_u!(u43, "un43", "ue43");
test_v_u!(u44, "un44", "ue44");
test_v_u!(u45, "un45", "ue45");
test_v_u!(u46, "un46", "ue46");
test_v_u!(u47, "un47", "ue47");
test_v_u!(u48, "un48", "ue48");
test_v_u!(u49, "un49", "ue49");
test_v_u!(u50, "un50", "ue50");

test_v_u!(u51, "un51", "ue51");
test_v_u!(u52, "un52", "ue52");
test_v_u!(u53, "un53", "ue53");
test_v_u!(u54, "un54", "ue54");
test_v_u!(u55, "un55", "ue55");
test_v_u!(u56, "un56", "ue56");
test_v_u!(u57, "un57", "ue57");
test_v_u!(u58, "un58", "ue58");
test_v_u!(u59, "un59", "ue59");
test_v_u!(u60, "un60", "ue60");

test_v_u!(u61, "un61", "ue61");
test_v_u!(u62, "un62", "ue62");
test_v_u!(u63, "un63", "ue63");
test_v_u!(u64, "un64", "ue64");
test_v_u!(u65, "un65", "ue65");
test_v_u!(u66, "un66", "ue66");
test_v_u!(u67, "un67", "ue67");
test_v_u!(u68, "un68", "ue68");
test_v_u!(u69, "un69", "ue69");
test_v_u!(u70, "un70", "ue70");

test_v_u!(u71, "un71", "ue71");
test_v_u!(u72, "un72", "ue72");
test_v_u!(u73, "un73", "ue73");
test_v_u!(u74, "un74", "ue74");
test_v_u!(u75, "un75", "ue75");
test_v_u!(u76, "un76", "ue76");
test_v_u!(u77, "un77", "ue77");
test_v_u!(u78, "un78", "ue78");
test_v_u!(u79, "un79", "ue79");
test_v_u!(u80, "un80", "ue80");
