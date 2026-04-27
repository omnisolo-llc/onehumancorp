use ohc::orchestration::hub_service_client::HubServiceClient;
use ohc::orchestration::RegisterAgentRequest;
use ohc::orchestration::Agent;

pub mod ohc {
    pub mod orchestration {
        tonic::include_proto!("ohc.orchestration");
    }
}

slint::include_modules!();

use serde::{Deserialize, Serialize};

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

    let ui = Dashboard::new()?;
    let ui_handle = ui.as_weak();

    tokio::spawn(async move {
        let client = reqwest::Client::new();
        loop {
            match client.get("http://127.0.0.1:18080/api/dashboard").send().await {
                Ok(res) => {
                    if res.status().is_success() {
                        match res.json::<DashboardSnapshot>().await {
                            Ok(data) => {
                                println!("Fetched dashboard data");
                                if let Some(ui) = ui_handle.upgrade() {
                                    ui.set_active_agents(data.agents.len().to_string().into());
                                    ui.set_active_tasks(data.statuses.iter().map(|s| s.count).sum::<i32>().to_string().into());
                                    ui.set_scheduled_calls(data.meetings.len().to_string().into());
                                    ui.set_team_members(data.organization.members.len().to_string().into());
                                }
                            }
                            Err(e) => println!("Failed to deserialize dashboard data: {:?}", e),
                        }
                    } else {
                        println!("API error: {:?}", res.status());
                    }
                }
                Err(e) => println!("Failed to fetch dashboard data: {:?}", e),
            }
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    });

    ui.run()?;
    
    Ok(())
}
