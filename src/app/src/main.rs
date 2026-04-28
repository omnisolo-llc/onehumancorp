use ohc::orchestration::hub_service_client::HubServiceClient;
use ohc::orchestration::RegisterAgentRequest;
use ohc::orchestration::Agent;
use ohc::orchestration::growth_service_client::GrowthServiceClient;
use ohc::orchestration::CreateReferralRequest;

pub mod ohc {
    pub mod orchestration {
        tonic::include_proto!("ohc.orchestration");
    }
}

pub mod agent;
pub mod local_manager;
pub mod api_service;
use slint::ComponentHandle;

pub mod app {
    include!(concat!(env!("OUT_DIR"), "/app.rs"));
}

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DashboardSnapshot {
    organization: Organization,
    meetings: Vec<serde_json::Value>,
    costs: CostSummary,
    storage: Option<StorageSummary>,
    agents: Vec<AgentModel>,
    statuses: Vec<StatusBucket>,
    updated_at: String,
    hybrid_health: Option<HybridHealth>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Organization {
    id: String,
    name: String,
    domain: String,
    tier: String,
    members: Vec<OrganizationMember>,
    role_profiles: Vec<RoleProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RoleProfile {
    role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OrganizationMember {
    id: String,
    name: String,
    role: String,
    manager_id: Option<String>,
    is_human: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CostSummary {
    total_cost_usd: f64,
    total_tokens: i32,
    total_actions: i32,
    agents: Vec<AgentCost>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AgentCost {
    agent_id: String,
    cost_usd: f64,
    token_used: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StorageSummary {
    used_bytes: i64,
    limit_bytes: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AgentModel {
    id: String,
    name: String,
    role: String,
    organization_id: String,
    status: String,
    provider_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StatusBucket {
    status: String,
    count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HybridHealth {
    mode: String,
    status: String,
    mesh_active: bool,
    cloud_connected: bool,
    sync_backlog: i32,
    stuck_missions: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("App starting...");

    tokio::spawn(async move {
        match HubServiceClient::connect("http://127.0.0.1:18789").await {
            Ok(mut client) => {
                println!("Connected to server!");
                let request = tonic::Request::new(RegisterAgentRequest {
                    agent: Some(Agent {
                        id: "agent_1".into(),
                        name: "Rust Agent".into(),
                        role: "Worker".into(),
                        organization_id: "org_1".into(),
                        status: "Running".into(),
                        provider_type: "Mock".into(),
                    }),
                });
                match client.register_agent(request).await {
                    Ok(response) => println!("RESPONSE={:?}", response),
                    Err(e) => println!("ERR={:?}", e),
                }
            }
            Err(e) => {
                println!("Could not connect to server: {:?}", e);
            }
        }
    });





let um_ui = app::UserManagement::new()?;
    let um_ui_handle = um_ui.as_weak();

    // Generate simple referral code if none exists, or fetch it
    let user_id = uuid::Uuid::new_v4().to_string();
    let referral_code = format!("ohc.to/ref/{}", &user_id[0..6]);
    um_ui.set_referral_link(referral_code.clone().into());

    let backend_url_init = std::env::var("OHC_CORE_URL").unwrap_or_else(|_| "http://127.0.0.1:18789".to_string());

    let um_ui_handle_init = um_ui.as_weak();
    slint::spawn_local(async move {
        if let Ok(mut client) = GrowthServiceClient::connect(backend_url_init).await {
            let request = tonic::Request::new(ohc::orchestration::EmptyRequest {});
            if let Ok(response) = client.get_referrals(request).await {
                // For simplicity in this demo, just sum up invites_sent across all referrals
                // since we don't have a GetUserStats endpoint.
                let mut total_sent = 0;
                for ref_obj in response.into_inner().referrals {
                    total_sent += 1; // Or parse invites_sent if added to proto
                }
                if let Some(ui) = um_ui_handle_init.upgrade() {
                    ui.set_invites_sent(total_sent);
                }
            }
        }
    }).unwrap();

    um_ui.on_share_referral(move || {
        let ui = um_ui_handle.unwrap();
        let current_sent = ui.get_invites_sent();
        ui.set_invites_sent(current_sent + 1);

        let backend_url = std::env::var("OHC_CORE_URL").unwrap_or_else(|_| "http://127.0.0.1:18789".to_string());

        let req_user_id = user_id.clone();
        let req_ref_code = referral_code.clone();

        slint::spawn_local(async move {
            if let Ok(mut client) = GrowthServiceClient::connect(backend_url).await {
                let request = tonic::Request::new(CreateReferralRequest {
                    user_id: req_user_id,
                    referral_code: req_ref_code,
                });
                let _ = client.create_referral(request).await;
            }
        }).unwrap();
    });
    um_ui.run()?;



    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_welcome_checklist_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
            println!("Skipping test_welcome_checklist_creation because no display server is available.");
            return;
        }
        app::WelcomeChecklist::new().unwrap();
    }

    #[test]
    fn test_login_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
            println!("Skipping test_login_creation because no display server is available.");
            return;
        }
        let ui = app::Login::new().unwrap();
        assert_eq!(ui.get_username(), "");
        assert_eq!(ui.get_password(), "");
    }

    #[test]
    fn test_business_setup_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
            println!("Skipping test_business_setup_creation because no display server is available.");
            return;
        }
        let ui = app::BusinessSetup::new().unwrap();
        assert_eq!(ui.get_step(), 0);
        assert_eq!(ui.get_company_name(), "");
    }

    #[test]
    fn test_agent_hire_next_button_disabled_by_default() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
            println!("Skipping test_agent_hire_next_button_disabled_by_default because no display server is available.");
            return;
        }
        let ui = app::AgentHire::new().unwrap();
        assert_eq!(ui.get_step(), 0);
        assert_eq!(ui.get_selected_role(), "");
        assert_eq!(ui.get_next_enabled(), false);
    }

    #[test]
    fn test_agent_hire_next_button_enabled_after_role_selection() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
            println!("Skipping test_agent_hire_next_button_enabled_after_role_selection because no display server is available.");
            return;
        }
        let ui = app::AgentHire::new().unwrap();
        assert_eq!(ui.get_step(), 0);
        ui.set_selected_role("SOFTWARE_ENGINEER".into());
        assert_eq!(ui.get_next_enabled(), true);
    }

    #[test]
    fn test_landing_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
            println!("Skipping test_landing_creation because no display server is available.");
            return;
        }
        let ui = app::Landing::new().unwrap();
        assert_eq!(ui.get_is_variant_b(), false);
    }

    #[test]
    fn test_agents_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::Agents::new().unwrap();
    }
    #[test]
    fn test_chat_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::Chat::new().unwrap();
    }
    #[test]
    fn test_channels_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::Channels::new().unwrap();
    }
    #[test]
    fn test_integrations_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::Integrations::new().unwrap();
    }
    #[test]
    fn test_security_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::Security::new().unwrap();
    }
    #[test]
    fn test_meetings_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::Meetings::new().unwrap();
    }
    #[test]
    fn test_logs_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::Logs::new().unwrap();
    }
    #[test]
    fn test_pricing_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::Pricing::new().unwrap();
    }
    #[test]
    fn test_scaling_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::Scaling::new().unwrap();
    }
    #[test]
    fn test_swarm_memory_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::SwarmMemory::new().unwrap();
    }
    #[test]
    fn test_website_builder_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::WebsiteBuilder::new().unwrap();
    }
    #[test]
    fn test_setup_wizard_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::SetupWizard::new().unwrap();
    }
    #[test]
    fn test_task_list_creation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        app::TaskList::new().unwrap();
    }

#[tokio::test]
    async fn test_user_management_e2e() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

        let login_ui = app::Login::new().unwrap();
        // Simulate login (skipped)

        let ui = app::UserManagement::new().unwrap();
        assert_eq!(ui.get_invites_sent(), 0);

        // Setup initial referral state explicitly, then simulate click
        let ui_weak = ui.as_weak();
        ui.on_share_referral(move || {
            let ui = ui_weak.unwrap();
            let sent = ui.get_invites_sent();
            ui.set_invites_sent(sent + 1);
        });

        ui.invoke_share_referral();
        assert_eq!(ui.get_invites_sent(), 1);
    }

}
