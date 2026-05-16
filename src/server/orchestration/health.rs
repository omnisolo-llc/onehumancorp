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
                for agent in monitor_hub.get_agents().iter() {
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
#[cfg(test)]
mod tests {
    include!("health_test.rs");
}
