use crate::app;
use slint::Model;
use std::rc::Rc;

fn create() -> app::Meetings { crate::ui_tests::init(); app::Meetings::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn meetings_xss_name() {
    let ui = create();
    let xss = "<script>alert('meeting')</script>";
    let meetings = slint::VecModel::from(vec![
        app::UiMeeting {
            name: xss.into(),
            status: "Active".into(),
            participants: 1,
            is_active: true,
        }
    ]);
    ui.set_meetings(Rc::new(meetings).into());
    assert_eq!(ui.get_meetings().row_data(0).unwrap().name, xss);
}

#[test] fn meetings_negative_participants() {
    let ui = create();
    let meetings = slint::VecModel::from(vec![
        app::UiMeeting {
            name: "Ghost Room".into(),
            status: "Spooky".into(),
            participants: -10,
            is_active: false,
        }
    ]);
    ui.set_meetings(Rc::new(meetings).into());
    assert_eq!(ui.get_meetings().row_data(0).unwrap().participants, -10);
}

#[test] fn meetings_injection_join() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c = called.clone();
    ui.on_join_room(move |name| { *c.borrow_mut() = name.to_string(); });
    
    let inj = "room'); DROP TABLE meetings; --";
    ui.invoke_join_room(inj.into());
    assert_eq!(*called.borrow(), inj);
}

// --- Interaction / Flow Tests ---

#[test] fn meetings_flow_mass_list() {
    let ui = create();
    let v: Vec<app::UiMeeting> = (0..100).map(|i| app::UiMeeting {
        name: format!("Room {}", i).into(),
        status: "Empty".into(),
        participants: 0,
        is_active: false,
    }).collect();
    ui.set_meetings(Rc::new(slint::VecModel::from(v)).into());
    assert_eq!(ui.get_meetings().row_count(), 100);
}

#[test] fn meetings_flow_new_room_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_new_room(move || { *c.borrow_mut() = true; });
    ui.invoke_new_room();
    assert!(*called.borrow());
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---
