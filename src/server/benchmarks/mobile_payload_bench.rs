use std::time::Instant;
use ::server_ohc::app::{DashboardSnapshot, MeetingRoom};
use ::server_ohc::organization::{Organization, TeamMember};
use ::server_ohc::agent::Agent;

pub async fn bench_mobile_payload() {
    tracing::info!("Benchmarking Mobile Payload Optimization...");

    let mut large_snapshot = DashboardSnapshot {
        organization: Some(Organization {
            id: "org1".to_string(),
            name: "Big Org".to_string(),
            domain: "bigorg.com".to_string(),
            ceo_id: "ceo1".to_string(),
            created_at_unix: 1234567890,
            members: vec![TeamMember {
                id: "member1".to_string(),
                organization_id: "org1".to_string(),
                name: "Member One".to_string(),
                manager_id: String::new(),
                is_human: true,
                role: 1,
            }; 100],
            role_profiles: vec![],
            tier: "enterprise".to_string(),
        }),
        agents: Vec::new(),
        meetings: Vec::new(),
        cost_summary: None,
        statuses: vec![],
        updated_at: String::new(),
        products: vec![],
        orders: vec![],
    };

    for i in 0..1000 {
        large_snapshot.agents.push(Agent {
            id: format!("agent{}", i),
            name: "A very detailed agent name with lots of text to test serialization".to_string(),
            role: 1,
            status: 1,
            organization_id: "org1".to_string(),
        });
    }

    for i in 0..500 {
        let mut transcript = Vec::new();
        for j in 0..50 {
            transcript.push(::server_ohc::agent::AgentMessage {
                id: format!("msg_{}_{}", i, j),
                from_agent_id: "agent1".to_string(),
                to_agent_id: "all".to_string(),
                message_type: "chat".to_string(),
                content: "This is a very long transcript message that takes up a lot of space in the payload to test network serialization times.".repeat(10),
                meeting_id: format!("meeting{}", i),
                occurred_at_unix: 0,
            });
        }
        large_snapshot.meetings.push(MeetingRoom {
            id: format!("meeting{}", i),
            participants: vec!["agent1".to_string()],
            transcript,
        });
    }

    let start_desktop = Instant::now();
    let mut encoded_desktop = Vec::new();
    prost::Message::encode(&large_snapshot, &mut encoded_desktop).unwrap();
    let desktop_duration = start_desktop.elapsed();
    let desktop_size = encoded_desktop.len();

    let start_mobile_opt = Instant::now();

    let mut mobile_snapshot = large_snapshot.clone();

    if let Some(mut org) = mobile_snapshot.organization.take() {
        org.domain = String::new();
        org.members = vec![];
        org.ceo_id = String::new();
        org.created_at_unix = 0;
        mobile_snapshot.organization = Some(org);
    }

    for agent in mobile_snapshot.agents.iter_mut() {
        agent.name = String::new();
    }

    for meeting in mobile_snapshot.meetings.iter_mut() {
        meeting.transcript = vec![];
    }

    let mut encoded_mobile = Vec::new();
    prost::Message::encode(&mobile_snapshot, &mut encoded_mobile).unwrap();
    let mobile_duration = start_mobile_opt.elapsed();
    let mobile_size = encoded_mobile.len();

    println!("Desktop Payload: Size {} bytes, Serialization Time {:?}", desktop_size, desktop_duration);
    println!("Mobile Payload: Size {} bytes, Serialization Time {:?}", mobile_size, mobile_duration);

    let size_reduction = 100.0 - ((mobile_size as f64 / desktop_size as f64) * 100.0);
    println!("Mobile Payload Optimization achieved {:.2}% size reduction.", size_reduction);

    assert!(mobile_size < desktop_size, "Mobile payload must be smaller than desktop payload");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bench_mobile_payload() {
        bench_mobile_payload().await;
    }
}
