use crate::app;

fn create() -> app::Experiments {
    crate::ui_tests::init();
    app::Experiments::new().unwrap()
}

#[test]
fn test_experiments_creation() {
    let ui = create();
    assert_eq!(ui.window().is_visible(), false); // Ensure it can be created without panic
}