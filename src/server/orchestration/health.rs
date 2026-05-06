use std::sync::Arc;
use crate::hub::Hub;
use ohc_builtin_agent::mesh::transport::MeshTransport;

pub async fn run_health_monitor(
    monitor_transport: Arc<dyn MeshTransport>,
    monitor_hub: Arc<Hub>,
    tick_duration: std::time::Duration,
    is_cloud: bool,
) {
    let mut interval = tokio::time::interval(tick_duration);
    let mut pending_fires: std::collections::HashMap<String, u8> = std::collections::HashMap::new();
    loop {
        interval.tick().await;
        let mut to_fire_now: Vec<String> = Vec::new();
        match tokio::time::timeout(std::time::Duration::from_secs(5), monitor_transport.get_active_agents()).await {
            Ok(Ok(agents)) => {
                if agents.is_empty() {
                    tracing::warn!("HEALTH MONITOR: No active agents found. Alerting / initiating task reassignment.");
                }

                let mut active_agent_ids = std::collections::HashSet::new();
                for (agent_id, _status) in agents {
                    active_agent_ids.insert(agent_id.clone());
                }

                let mut to_fire = Vec::new();
                for agent in monitor_hub.get_agents().iter() {
                    // Fire agents that are missing from active agents mesh list, regardless of their IDLE/BUSY status
                    if !active_agent_ids.contains(&agent.id) {
                        to_fire.push(agent.id.clone());
                    }
                }
                for agent_id in to_fire {
                    if is_cloud {
                        let count = pending_fires.entry(agent_id.clone()).or_insert(0);
                        *count += 1;
                        if *count >= 3 {
                            to_fire_now.push(agent_id.clone());
                        } else {
                            tracing::warn!("HEALTH MONITOR: Agent {} is unresponsive ({} failures). Retrying next tick.", agent_id, count);
                        }
                    } else {
                        to_fire_now.push(agent_id.clone());
                    }
                }
                pending_fires.retain(|k, _| !active_agent_ids.contains(k));
                for agent_id in to_fire_now {
                    tracing::warn!("HEALTH MONITOR: Agent {} is definitively unresponsive. Firing and initiating reassignment.", agent_id);
                    monitor_hub.fire_agent(&agent_id);
                    pending_fires.remove(&agent_id);
                }
            }
            Ok(Err(e)) => {
                tracing::error!("HEALTH MONITOR: Failed to get active agents: {}", e);
            }
            Err(_) => {
                tracing::error!("HEALTH MONITOR: Timed out waiting for active agents list from transport");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            run_health_monitor(monitor_transport, monitor_hub, std::time::Duration::from_millis(10), false).await;
        });

        // Let the monitor loop run once
        let _ = tokio::time::timeout(tokio::time::Duration::from_millis(50), handle).await;

        // Let it run a few more times to reach the retry limit (3)
        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;

        // Both agents should be fired (removed) after retries in standalone mode
        assert!(hub.get_agent("agent_idle").is_none());
        assert!(hub.get_agent("agent_busy").is_none());
    }

    #[tokio::test]
    async fn test_health_monitor_cloud_retry() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
        if !db_url.starts_with("sqlite") && std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let _pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect_lazy("sqlite::memory:")
            .unwrap();

        let pg_pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://dummy")
            .unwrap();

        let (tx, _) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(Hub::new(tx, pg_pool));

        hub.register_agent(crate::ohc::orchestration::Agent {
            id: "agent_cloud".to_string(),
            name: "Cloud Agent".to_string(),
            role: "test".to_string(),
            organization_id: "org1".to_string(),
            status: "IDLE".to_string(),
            provider_type: "test".to_string(),
        });

        let transport = ohc_builtin_agent::mesh::transport::create_transport(None, false).await.unwrap();
        let monitor_transport: Arc<dyn MeshTransport> = transport.clone();
        let monitor_hub = hub.clone();

        let handle = tokio::spawn(async move {
            run_health_monitor(monitor_transport, monitor_hub, std::time::Duration::from_millis(10), true).await;
        });

        tokio::time::sleep(std::time::Duration::from_millis(15)).await;
        // After 1 tick, the cloud agent should NOT be fired yet (retrying)
        assert!(hub.get_agent("agent_cloud").is_some(), "Agent should not be fired immediately in cloud mode");

        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        // After multiple ticks (3+), it should be fired
        assert!(hub.get_agent("agent_cloud").is_none(), "Agent should be fired after retries in cloud mode");
        handle.abort();
    }
}
