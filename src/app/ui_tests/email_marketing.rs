use crate::app::EmailMarketing;
use slint::{ComponentHandle, SharedString, Timer, TimerMode};
use std::env;
use std::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_email_marketing_flow() {
    if env::var("DISPLAY").is_err() && env::var("WAYLAND_DISPLAY").is_err() {
        return;
    }

    let _ = crate::ui_tests::init();
    let app = EmailMarketing::new().unwrap();

    let app = app.as_weak().upgrade().unwrap();



    // Setup mock values via global callbacks
    app.on_generate_template({
        let app_weak = app.as_weak();
        move |template_name: SharedString| {
            let app = app_weak.upgrade().unwrap();
            let mut preview = format!("Draft for: {}", template_name);
            if template_name == "Flash sale" {
                preview = "Get 20% off all items! Limited time only.".to_string();
            }
            app.set_preview_text(preview.into());
        }
    });

    app.on_send_campaign({
        let app_weak = app.as_weak();
        move || {
            let app = app_weak.upgrade().unwrap();
            app.set_status_message("Sending...".into());

            let timer = Timer::default();
            timer.start(
                TimerMode::SingleShot,
                Duration::from_millis(50),
                {
                    let app_weak = app_weak.clone();
                    move || {
                        if let Some(app) = app_weak.upgrade() {
                            app.set_status_message("Sent!".into());
                            app.set_emails_sent(150);
                            app.set_open_rate("32%".into());
                        }
                    }
                },
            );
            // leak the timer so it doesn't get dropped immediately
            Box::leak(Box::new(timer));
        }
    });

    // Test 1: Generate template
    app.invoke_generate_template("Flash sale".into());
    assert_eq!(app.get_preview_text().as_str(), "Get 20% off all items! Limited time only.");

    // Test 2: Send Campaign
    app.invoke_send_campaign();
    assert_eq!(app.get_status_message().as_str(), "Sending...");

    // Simulate async delay
    std::thread::sleep(Duration::from_millis(100));
    slint::platform::update_timers_and_animations();

    // Verify results
    assert_eq!(app.get_status_message().as_str(), "Sent!");
    assert_eq!(app.get_emails_sent(), 150);
    assert_eq!(app.get_open_rate().as_str(), "32%");

    // Test 3: Multiple templates
    app.invoke_generate_template("New arrivals".into());
    assert_eq!(app.get_preview_text().as_str(), "Draft for: New arrivals");

    // Test 4: Another template
    app.invoke_generate_template("Thank you".into());
    assert_eq!(app.get_preview_text().as_str(), "Draft for: Thank you");

    // Test 5: Verify default audience metric
    assert_eq!(app.get_total_subscribers(), 150);
}
