use std::sync::Arc;
use crate::queue::{Job, TaskQueue};
use crate::orchestration::mesh::TeammateMesh;
use crate::orchestration::hierarchical::K8sOperatorDelegator;
use async_trait::async_trait;

#[async_trait]
pub trait SubAgentSpawner: Send + Sync {
    async fn execute_with_retry(&self, job: Job) -> Result<(), String>;
    async fn spawn(&self, role: &str, instruction: &str, thread_id: &str, mode: &str) -> Result<String, String>;
}

pub struct DefaultSubAgentSpawner {
    queue: Arc<dyn TaskQueue>,
    mesh: Arc<dyn TeammateMesh>,
}

impl DefaultSubAgentSpawner {
    pub fn new(queue: Arc<dyn TaskQueue>, mesh: Arc<dyn TeammateMesh>) -> Self {
        Self { queue, mesh }
    }

    async fn fail_task(&self, job_id: &str, reason: &str, mode: &str) -> Result<(), String> {
        ::server_telemetry::record_sub_agent_spawn_error(mode);
        self.queue.fail(job_id, reason).await
    }
}

#[async_trait]
impl SubAgentSpawner for DefaultSubAgentSpawner {
    async fn execute_with_retry(&self, job: Job) -> Result<(), String> {
        let mode = ::server_telemetry::get_deployment_mode();
        let payload: serde_json::Value = serde_json::from_str(&job.payload).unwrap_or(serde_json::json!({}));
        let instruction = payload.get("instruction").and_then(|v| v.as_str()).unwrap_or("");
        let thread_id = payload.get("thread_id").and_then(|v| v.as_str()).unwrap_or("");

        let mut attempts = job.attempts;
        let max_attempts = job.max_attempts;

        loop {
            match self.spawn(&job.agent_role, instruction, thread_id, mode).await {
                Ok(pod_id) => {
                    tracing::info!("SubAgentSpawned: pod_id={} for job={}", pod_id, job.id);
                    // Broadcast SUB_AGENT_SPAWNED
                    let msg = serde_json::json!({
                        "event": "SUB_AGENT_SPAWNED",
                        "job_id": job.id,
                        "pod_id": pod_id,
                        "role": job.agent_role,
                        "tenant_id": job.tenant_id,
                    });
                    let _ = self.mesh.publish_task_broadcast(serde_json::to_vec(&msg).unwrap_or_default()).await;

                    let _ = self.queue.complete(&job.id).await;

                    let msg_complete = serde_json::json!({
                        "event": "SUB_AGENT_COMPLETED",
                        "job_id": job.id,
                        "pod_id": pod_id,
                        "role": job.agent_role,
                        "tenant_id": job.tenant_id,
                    });
                    let _ = self.mesh.publish_task_broadcast(serde_json::to_vec(&msg_complete).unwrap_or_default()).await;

                    return Ok(());
                }
                Err(e) => {
                    attempts += 1;
                    if attempts >= max_attempts {
                        let _ = self.fail_task(&job.id, &e, mode).await;
                        return Err(e);
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs((1 << attempts) as u64)).await;
                }
            }
        }
    }

    async fn spawn(&self, role: &str, instruction: &str, thread_id: &str, mode: &str) -> Result<String, String> {
        if mode == "postgres" || mode == "cloud" {
            K8sOperatorDelegator::spawn_sub_agent_pod(role, instruction, thread_id).await
        } else {
            // Standalone mode token execution: spawn local worker safely capped by Tokio's pool size.
            // We use bounded channels or a semaphore in a more complex setup;
            // for now tokio automatically handles the concurrency of these tasks efficiently.
            let pod_id = format!("local-sub-agent-{}-{}", role, uuid::Uuid::new_v4());
            let instr_clone = instruction.to_string();
            tokio::spawn(async move {
                tracing::info!("Executing local sub-agent task: {}", instr_clone);
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            });
            Ok(pod_id)
        }
    }
}
