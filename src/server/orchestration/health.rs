use std::sync::Arc;
use crate::hub::Hub;
use crate::orchestration::mesh::TeammateMesh;

pub async fn run_health_monitor(
    monitor_mesh: Arc<dyn TeammateMesh>,
    monitor_hub: Arc<Hub>,
    is_cloud: bool,
    tick_duration: std::time::Duration,
) {
    let mut interval = tokio::time::interval(tick_duration);
    let mut pending_fires: std::collections::HashMap<String, u8> = std::collections::HashMap::new();
    loop {
        interval.tick().await;

        // Perform active probe
        let ping_ok = match tokio::time::timeout(std::time::Duration::from_millis(50), monitor_mesh.ping()).await {
            Ok(Ok(_)) => true,
            _ => false,
        };

        if !ping_ok {
            tracing::trace!("HEALTH MONITOR: Active probe (ping) failed or timed out.");
        }

        // Hybrid mode health check
        if let Ok(Ok(health)) = tokio::time::timeout(std::time::Duration::from_millis(50), monitor_hub.check_health()).await {
            if let Some(ready) = health.get("hybrid_mode_ready").and_then(|v| v.as_bool()) {
                if !ready {
                    tracing::trace!("HEALTH MONITOR: Hybrid mode is degraded.");
                }
            }
        }

        // New Health-check probe for local-to-cloud mission sync
        if let Ok(Ok(health)) = tokio::time::timeout(std::time::Duration::from_millis(50), monitor_hub.check_health()).await {
            if let Some(sync_errors) = health.get("sync_error_count").and_then(|v| v.as_i64()) {
                if sync_errors > 10 {
                    tracing::warn!("HEALTH MONITOR: High sync error count detected: {}", sync_errors);
                } else if sync_errors > 0 {
                    tracing::trace!("HEALTH MONITOR: Sync errors present but below threshold: {}", sync_errors);
                }
            }
        }

        let mut to_fire_now: Vec<String> = Vec::new();
        match tokio::time::timeout(std::time::Duration::from_millis(50), monitor_mesh.get_active_agents()).await {
            Ok(Ok(agents)) => {
                if agents.is_empty() {
                    tracing::trace!("HEALTH MONITOR: No active agents found."); // Reduced noise
                }

                let mut active_agent_ids = std::collections::HashSet::new();
                for (agent_id, _status) in agents {
                    active_agent_ids.insert(agent_id.clone());
                }

                let mut to_fire = Vec::new();
                for agent in monitor_hub.get_agents().await.iter() {
                    // Fire agents that are missing from active agents mesh list OR if ping failed
                    if !active_agent_ids.contains(&agent.id) || !ping_ok {
                        to_fire.push(agent.id.clone());
                    }
                }
                for agent_id in to_fire {
                    let count = pending_fires.entry(agent_id.clone()).or_insert(0);
                    *count += 1;
                    let threshold = if is_cloud { 3 } else { 1 };
                    if *count >= threshold {
                        to_fire_now.push(agent_id.clone());
                    } else {
                        tracing::trace!("HEALTH MONITOR: Agent {} is unresponsive ({} failures). Retrying next tick.", agent_id, count); // Reduced noise
                    }
                }
                pending_fires.retain(|k, _| !active_agent_ids.contains(k) || !ping_ok);
                for agent_id in to_fire_now {
                    tracing::info!("HEALTH MONITOR: Agent {} is definitively unresponsive. Firing and initiating reassignment.", agent_id);
                    monitor_hub.fire_agent(&agent_id);
                    pending_fires.remove(&agent_id);
                }
            }
            Ok(Err(e)) => {
                tracing::trace!("HEALTH MONITOR: Failed to get active agents: {}", e);
            }
            Err(_) => {
                tracing::trace!("HEALTH MONITOR: Timed out waiting for active agents list from transport");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ohc_builtin_agent::mesh::transport::InProcessTransport;

    #[tokio::test]
    async fn test_health_monitor_fires_unresponsive_agent() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
        if !db_url.starts_with("sqlite") && std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let _pool = sqlx::sqlite::SqlitePoolOptions::new().max_connections(1)
            .connect_lazy("sqlite::memory:")
            .unwrap();

        // We use casting to bypass postgres/sqlite types to instantiate a generic hub for test
        // Since Hub takes a PgPool, we have to supply one to construct it, even if unused in this isolated test
        let pg_pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .connect_lazy("postgres://dummy")
            .unwrap();

        let (tx, _) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(Hub::new(tx, pg_pool));

        // Register an idle agent
        hub.register_agent(::server_ohc::orchestration::Agent {
            id: "agent_idle".to_string(),
            name: "Idle Agent".to_string(),
            role: "test".to_string(),
            organization_id: "org1".to_string(),
            status: "IDLE".to_string(),
            provider_type: "test".to_string(),
        });

        // Register a busy agent
        hub.register_agent(::server_ohc::orchestration::Agent {
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
        let transport = Arc::new(InProcessTransport::new());
        let centrifuge_node = Arc::new(crate::orchestration::mesh::CentrifugeNode::new(transport));

        let monitor_mesh: Arc<dyn TeammateMesh> = centrifuge_node.clone();
        let monitor_hub = hub.clone();

        let handle = tokio::spawn(async move {
            run_health_monitor(monitor_mesh, monitor_hub, false, std::time::Duration::from_millis(10)).await;
        });

        // Let the monitor loop run once
        tokio::time::sleep(std::time::Duration::from_millis(1500)).await;

        // Both agents should be fired (removed) immediately in standalone mode
        assert!(hub.get_agent("agent_idle").is_none());
        assert!(hub.get_agent("agent_busy").is_none());
        handle.abort();
    }

    #[tokio::test]
    async fn test_health_monitor_cloud_retry() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
        if !db_url.starts_with("sqlite") && std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let _pool = sqlx::sqlite::SqlitePoolOptions::new().max_connections(1)
            .connect_lazy("sqlite::memory:")
            .unwrap();

        let pg_pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .connect_lazy("postgres://dummy")
            .unwrap();

        let (tx, _) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(Hub::new(tx, pg_pool));

        hub.register_agent(::server_ohc::orchestration::Agent {
            id: "agent_cloud".to_string(),
            name: "Cloud Agent".to_string(),
            role: "test".to_string(),
            organization_id: "org1".to_string(),
            status: "IDLE".to_string(),
            provider_type: "test".to_string(),
        });

        let transport = ohc_builtin_agent::mesh::transport::create_transport(None, false).await.unwrap();
        let centrifuge_node = Arc::new(crate::orchestration::mesh::CentrifugeNode::new(transport));
        let monitor_mesh: Arc<dyn TeammateMesh> = centrifuge_node.clone();
        let monitor_hub = hub.clone();

        let handle = tokio::spawn(async move {
            run_health_monitor(monitor_mesh, monitor_hub, true, std::time::Duration::from_millis(10)).await;
        });

        tokio::time::sleep(std::time::Duration::from_millis(2500)).await;
        assert!(hub.get_agent("agent_cloud").is_none(), "Agent should be fired after retries in cloud mode");
        handle.abort();
    }

    #[tokio::test]
    async fn test_health_monitor_sync_probe() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
        if !db_url.starts_with("sqlite") && std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let _pool = sqlx::sqlite::SqlitePoolOptions::new().max_connections(1)
            .connect_lazy("sqlite::memory:")
            .unwrap();

        let pg_pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .connect_lazy("postgres://dummy")
            .unwrap();

        let (tx, _) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(Hub::new(tx, pg_pool));

        let transport = ohc_builtin_agent::mesh::transport::create_transport(None, false).await.unwrap();
        let centrifuge_node = Arc::new(crate::orchestration::mesh::CentrifugeNode::new(transport));
        let monitor_mesh: Arc<dyn TeammateMesh> = centrifuge_node.clone();
        let monitor_hub = hub.clone();

        let handle = tokio::spawn(async move {
            run_health_monitor(monitor_mesh, monitor_hub, true, std::time::Duration::from_millis(10)).await;
        });

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        handle.abort();
    }
}
