use std::sync::Arc;
use crate::hub::Hub;
use ohc_builtin_agent::mesh::transport::MeshTransport;

pub async fn run_health_monitor(
    monitor_transport: Arc<dyn MeshTransport>,
    monitor_hub: Arc<Hub>,
) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
    loop {
        interval.tick().await;

        // 1. Get current presence list from mesh
        match monitor_transport.get_active_agents().await {
            Ok(agents) => {
                if agents.is_empty() {
                    tracing::warn!("HEALTH MONITOR: No active agents found in mesh presence.");
                }

                let mut active_agent_ids = std::collections::HashSet::new();
                for (agent_id, _status) in agents {
                    active_agent_ids.insert(agent_id.clone());
                }

                let mut to_fire = Vec::new();
                for agent in monitor_hub.get_agents().iter() {
                    // 2. Heartbeat check: Monitor verifies presence within last 60 seconds
                    // The get_active_agents call already filters by TTL in MemoryTransport/IpcTransport/RedisTransport
                    // so we just check if they are present in the set.
                    if !active_agent_ids.contains(&agent.id) {
                        to_fire.push(agent.id.clone());
                    }
                }

                for agent_id in to_fire {
                    tracing::warn!("HEALTH MONITOR: Agent {} heartbeat failed (missing from mesh presence). Firing and initiating reassignment.", agent_id);
                    monitor_hub.fire_agent(&agent_id);
                }
            }
            Err(e) => {
                tracing::error!("HEALTH MONITOR: Failed to get active agents from mesh: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Duration;
    use ohc_builtin_agent::mesh::transport::MemoryTransport;

    #[tokio::test]
    async fn test_health_monitor_fires_unresponsive_agent() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
        if !db_url.starts_with("sqlite") && std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let _pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect_lazy("sqlite::memory:")
            .unwrap();

        // We use casting to bypass postgres/sqlite types to instantiate a generic hub for test
        // Since Hub takes a PgPool, we have to supply one to construct it, even if unused in this isolated test
        let pg_pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://dummy")
            .unwrap();

        let (tx, _) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(Hub::new(tx, pg_pool));

        // Register an idle agent
        hub.register_agent(crate::ohc::orchestration::Agent {
            id: "agent_idle".to_string(),
            name: "Idle Agent".to_string(),
            role: "test".to_string(),
            organization_id: "org1".to_string(),
            status: "IDLE".to_string(),
            provider_type: "test".to_string(),
        });

        // Register a busy agent
        hub.register_agent(crate::ohc::orchestration::Agent {
            id: "agent_busy".to_string(),
            name: "Busy Agent".to_string(),
            role: "test".to_string(),
            organization_id: "org1".to_string(),
            status: "BUSY".to_string(),
            provider_type: "test".to_string(),
        });

        assert!(hub.get_agent("agent_idle").is_some());
        assert!(hub.get_agent("agent_busy").is_some());

        // We simulate a transport with NO active agents
        let transport = Arc::new(MemoryTransport::new());

        let monitor_transport: Arc<dyn MeshTransport> = transport.clone();
        let monitor_hub = hub.clone();

        let handle = tokio::spawn(async move {
            run_health_monitor(monitor_transport, monitor_hub).await;
        });

        // Let the monitor loop run once
        let _ = tokio::time::timeout(Duration::from_millis(50), handle).await;

        // Both agents should be fired (removed) since they weren't in the mesh transport active agents list
        assert!(hub.get_agent("agent_idle").is_none());
        assert!(hub.get_agent("agent_busy").is_none());
    }
}
