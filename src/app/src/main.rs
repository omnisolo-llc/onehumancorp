use ohc::orchestration::hub_service_client::HubServiceClient;
use ohc::orchestration::RegisterAgentRequest;
use ohc::orchestration::Agent;

pub mod ohc {
    pub mod orchestration {
        tonic::include_proto!("ohc.orchestration");
    }
}

pub mod tooltip_registry;
use tooltip_registry::TooltipRegistry;

use slint::ComponentHandle;

pub mod app {
    include!(concat!(env!("OUT_DIR"), "/app.rs"));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("App starting...");

    let dashboard_ui = app::Dashboard::new()?;
    let dashboard_handle = dashboard_ui.as_weak();

    let tooltip_registry = TooltipRegistry::new();

    // Initialize tooltips
    if let Some(ui) = dashboard_handle.upgrade() {
        ui.set_tt_active_agents(tooltip_registry.get_tooltip("dashboard_active_agents").unwrap_or_default().into());
        ui.set_tt_active_tasks(tooltip_registry.get_tooltip("dashboard_active_tasks").unwrap_or_default().into());
        ui.set_tt_scheduled_calls(tooltip_registry.get_tooltip("dashboard_scheduled_calls").unwrap_or_default().into());
        ui.set_tt_team_members(tooltip_registry.get_tooltip("dashboard_team_members").unwrap_or_default().into());
    }

    dashboard_ui.on_open_help_center({
        move || {
            let help_center = app::HelpCenter::new().expect("Failed to create HelpCenter");
            help_center.show().expect("Failed to show HelpCenter");
        }
    });

    dashboard_ui.on_open_ai_chat({
        move || {
            let ai_chat = app::AiHelpChat::new().expect("Failed to create AiHelpChat");
            ai_chat.show().expect("Failed to show AiHelpChat");
        }
    });

    dashboard_ui.on_open_release_notes({
        move || {
            let release_notes = app::ReleaseNotes::new().expect("Failed to create ReleaseNotes");
            release_notes.show().expect("Failed to show ReleaseNotes");
        }
    });

    dashboard_ui.on_start_walkthrough({
        move || {
            let walkthrough = app::InteractiveWalkthrough::new().expect("Failed to create InteractiveWalkthrough");
            walkthrough.show().expect("Failed to show InteractiveWalkthrough");
        }
    });

    dashboard_ui.run()?;
    
    Ok(())
}

#[cfg(test)]
mod e2e_tests {
    use super::*;

    #[test]
    fn test_login_password_visibility_toggle() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        let ui = app::Login::new().unwrap();
        ui.set_password("secret".into());
        assert_eq!(ui.get_password(), "secret");
    }
}
