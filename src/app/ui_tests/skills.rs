use crate::app;

fn create() -> app::Skills {
    crate::ui_tests::init();
    app::Skills::new().unwrap()
}

// --- Hacking / Corner Cases ---

#[test]
fn skills_xss_category() {
    let ui = create();
    let xss = "<textarea onload=alert(1)>";
    ui.set_selected_category(xss.into());
    assert_eq!(ui.get_selected_category(), xss);
}

#[test]
fn skills_injection_category() {
    let ui = create();
    let inj = "'; DELETE FROM skills; --";
    ui.set_selected_category(inj.into());
    assert_eq!(ui.get_selected_category(), inj);
}

#[test]
fn skills_empty_category() {
    let ui = create();
    ui.set_selected_category("".into());
    assert_eq!(ui.get_selected_category(), "");
}

// --- Interaction / Flow Tests ---

#[test]
fn skills_flow_rapid_category_switch() {
    let ui = create();
    let cats = ["Coding", "Design", "Writing", "Marketing", "Sales"];
    for _ in 0..20 {
        for c in cats {
            ui.set_selected_category(c.into());
            assert_eq!(ui.get_selected_category(), c);
        }
    }
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_selected_category() {
    let ui = create();
    ui.set_selected_category("Frontend".into());
    assert_eq!(ui.get_selected_category(), "Frontend");
    ui.set_selected_category("Backend".into());
    assert_eq!(ui.get_selected_category(), "Backend");
    ui.set_selected_category("Fullstack".into());
    assert_eq!(ui.get_selected_category(), "Fullstack");
}
