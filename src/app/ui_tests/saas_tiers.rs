
use slint::ComponentHandle;


#[test]
fn test_dashboard_product_limit_soft_paywall() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    crate::ui_tests::init();

    let dashboard_ui = crate::app::Dashboard::new().unwrap();
    let dashboard_handle_add_product = dashboard_ui.as_weak();

    dashboard_ui.on_action_add_product(move || {
        if let Some(ui) = dashboard_handle_add_product.upgrade() {
            ui.set_upgrade_prompt_message("You've added 10 products! Upgrade to our Pro plan to list even more items and grow your store.".into());
            ui.set_show_upgrade_prompt(true);
        }
    });

    dashboard_ui.invoke_action_add_product();
    assert!(dashboard_ui.get_show_upgrade_prompt(), "Upgrade prompt should show when adding product beyond free tier limit");
}

#[test]
fn test_agents_limit_soft_paywall() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    crate::ui_tests::init();

    let agents_ui = crate::app::Agents::new().unwrap();
    let agents_ui_handle = agents_ui.as_weak();
    agents_ui.on_hire_agent(move || {
        if let Some(ui) = agents_ui_handle.upgrade() {
            ui.set_upgrade_prompt_message("Your first helper is working hard! Upgrade to Pro to hire more helpers and automate more of your business.".into());
            ui.set_show_upgrade_prompt(true);
        }
    });

    agents_ui.invoke_hire_agent();
    assert!(agents_ui.get_show_upgrade_prompt(), "Upgrade prompt should show when hiring agent beyond free tier limit");
}
