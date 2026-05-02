use crate::app;
use slint::ComponentHandle;
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

macro_rules! test_v_m {
    ($id:ident, $mname:expr, $mstatus:expr) => {
        #[test] fn $id() {
            let ui = create();
            let m = slint::VecModel::from(vec![app::UiMeeting {
                name: $mname.into(),
                status: $mstatus.into(),
                participants: 1,
                is_active: true
            }]);
            ui.set_meetings(Rc::new(m).into());
            assert_eq!(ui.get_meetings().row_data(0).unwrap().name, $mname);
            assert_eq!(ui.get_meetings().row_data(0).unwrap().status, $mstatus);
        }
    };
}

test_v_m!(u1, "Room One", "Active");
test_v_m!(u2, "Room Two", "Pending");
test_v_m!(u3, "Room Three", "Full");

test_v_m!(u11, "n11", "s11");
test_v_m!(u12, "n12", "s12");
test_v_m!(u13, "n13", "s13");
test_v_m!(u14, "n14", "s14");
test_v_m!(u15, "n15", "s15");
test_v_m!(u16, "n16", "s16");
test_v_m!(u17, "n17", "s17");
test_v_m!(u18, "n18", "s18");
test_v_m!(u19, "n19", "s19");
test_v_m!(u20, "n20", "s20");

test_v_m!(u21, "🚀 Launchpad", "Ignition");
test_v_m!(u22, "Room 'Quoted'", "Quoted");
test_v_m!(u23, "Room ; Semicolon", "Semis");
test_v_m!(u24, "", "");
test_v_m!(u25, "Huge Room", "Crowded");

test_v_m!(u31, "n31", "s31");
test_v_m!(u32, "n32", "s32");
test_v_m!(u33, "n33", "s33");
test_v_m!(u34, "n34", "s34");
test_v_m!(u35, "n35", "s35");
test_v_m!(u36, "n36", "s36");
test_v_m!(u37, "n37", "s37");
test_v_m!(u38, "n38", "s38");
test_v_m!(u39, "n39", "s39");
test_v_m!(u40, "n40", "s40");

test_v_m!(u41, "n41", "s41");
test_v_m!(u42, "n42", "s42");
test_v_m!(u43, "n43", "s43");
test_v_m!(u44, "n44", "s44");
test_v_m!(u45, "n45", "s45");
test_v_m!(u46, "n46", "s46");
test_v_m!(u47, "n47", "s47");
test_v_m!(u48, "n48", "s48");
test_v_m!(u49, "n49", "s49");
test_v_m!(u50, "n50", "s50");

test_v_m!(u51, "n51", "s51");
test_v_m!(u52, "n52", "s52");
test_v_m!(u53, "n53", "s53");
test_v_m!(u54, "n54", "s54");
test_v_m!(u55, "n55", "s55");
test_v_m!(u56, "n56", "s56");
test_v_m!(u57, "n57", "s57");
test_v_m!(u58, "n58", "s58");
test_v_m!(u59, "n59", "s59");
test_v_m!(u60, "n60", "s60");

test_v_m!(u61, "n61", "s61");
test_v_m!(u62, "n62", "s62");
test_v_m!(u63, "n63", "s63");
test_v_m!(u64, "n64", "s64");
test_v_m!(u65, "n65", "s65");
test_v_m!(u66, "n66", "s66");
test_v_m!(u67, "n67", "s67");
test_v_m!(u68, "n68", "s68");
test_v_m!(u69, "n69", "s69");
test_v_m!(u70, "n70", "s70");

test_v_m!(u71, "n71", "s71");
test_v_m!(u72, "n72", "s72");
test_v_m!(u73, "n73", "s73");
test_v_m!(u74, "n74", "s74");
test_v_m!(u75, "n75", "s75");
test_v_m!(u76, "n76", "s76");
test_v_m!(u77, "n77", "s77");
test_v_m!(u78, "n78", "s78");
test_v_m!(u79, "n79", "s79");
test_v_m!(u80, "n80", "s80");
