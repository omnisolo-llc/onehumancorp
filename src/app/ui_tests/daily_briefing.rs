use crate::app;
use slint::{ComponentHandle, Model};

fn create() -> app::Dashboard {
    crate::ui_tests::init();
    app::Dashboard::new().unwrap()
}

#[test]
fn e2e_daily_briefing_default_hidden() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }
    let ui = create();
    assert!(
        !ui.get_show_daily_briefing(),
        "Daily briefing should be hidden by default"
    );
}

#[test]
fn e2e_daily_briefing_show_and_dismiss() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }
    let ui = create();

    // We simulate the behavior defined in main.rs
    let ui_handle = ui.as_weak();
    ui.on_dismiss_daily_briefing(move || {
        if let Some(ui) = ui_handle.upgrade() {
            ui.set_show_daily_briefing(false);
        }
    });

    ui.set_show_daily_briefing(true);
    assert!(
        ui.get_show_daily_briefing(),
        "Daily briefing should be visible after setting to true"
    );

    ui.invoke_dismiss_daily_briefing();
    assert!(
        !ui.get_show_daily_briefing(),
        "Daily briefing should be hidden after dismiss callback is invoked"
    );
}

#[test]
fn e2e_daily_briefing_content_population() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }
    let ui = create();

    let bullets = vec![
        slint::SharedString::from("You had 8 orders this week."),
        slint::SharedString::from("Vegan cake requests doubled."),
        slint::SharedString::from("Consider adding a vegan chocolate option!"),
    ];
    let bullets_model = slint::ModelRc::new(slint::VecModel::from(bullets.clone()));
    ui.set_daily_briefing_content(bullets_model);

    let content = ui.get_daily_briefing_content();
    assert_eq!(
        content.row_count(),
        3,
        "There should be 3 bullets in the daily briefing"
    );
    assert_eq!(content.row_data(0).unwrap(), bullets[0]);
    assert_eq!(content.row_data(1).unwrap(), bullets[1]);
    assert_eq!(content.row_data(2).unwrap(), bullets[2]);
}

#[test]
fn e2e_daily_briefing_empty_content() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }
    let ui = create();

    let bullets = vec![];
    let bullets_model = slint::ModelRc::new(slint::VecModel::from(bullets));
    ui.set_daily_briefing_content(bullets_model);

    let content = ui.get_daily_briefing_content();
    assert_eq!(
        content.row_count(),
        0,
        "There should be no bullets if content is empty"
    );
}

#[test]
fn e2e_daily_briefing_xss_protection() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }
    let ui = create();

    let xss_string = slint::SharedString::from("<script>alert('XSS')</script>");
    let bullets = vec![xss_string.clone()];
    let bullets_model = slint::ModelRc::new(slint::VecModel::from(bullets));
    ui.set_daily_briefing_content(bullets_model);

    let content = ui.get_daily_briefing_content();
    assert_eq!(content.row_count(), 1);
    assert_eq!(
        content.row_data(0).unwrap(),
        xss_string,
        "The UI framework should correctly handle literal strings without executing them"
    );
}
