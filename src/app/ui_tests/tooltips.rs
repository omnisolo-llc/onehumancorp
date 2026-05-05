use crate::app;
use slint::ComponentHandle;

#[test]
fn test_tooltip_registry_callback() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let ui = app::Dashboard::new().unwrap();

    ui.global::<app::TooltipRegistry>().on_request_tooltip_text(|id| {
        static TOOLTIPS: std::sync::OnceLock<std::collections::HashMap<String, String>> = std::sync::OnceLock::new();
        let tooltips = TOOLTIPS.get_or_init(|| serde_json::from_str(include_str!("../tooltips.json")).unwrap_or_default());
        tooltips.get(id.as_str()).cloned().unwrap_or_default().into()
    });

    let tr = ui.global::<app::TooltipRegistry>();
    tr.invoke_show_tooltip("ask_ai".into(), 10.0, 10.0);

    assert_eq!(tr.get_is_visible(), true);
    assert_eq!(tr.get_active_text(), slint::SharedString::from("Ask the AI assistant for help or to perform tasks."));

    tr.invoke_hide_tooltip();
    assert_eq!(tr.get_is_visible(), false);
}
