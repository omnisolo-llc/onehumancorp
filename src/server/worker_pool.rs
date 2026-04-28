use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::time::Duration;
use crate::queue::{QueueManager, SubAgentJob};
use crate::spawner::SubAgentSpawner;
use crate::hub::Hub;
use crate::tasks::SharedTask;
use ohc::orchestration::TeammateMeshEvent;

pub struct SubAgentWorkerPool {
    queue_manager: Arc<QueueManager>,
    spawner: Arc<dyn SubAgentSpawner>,
    hub: Arc<Hub>,
    worker_count: usize,
}

impl SubAgentWorkerPool {
    pub fn new(
        queue_manager: Arc<QueueManager>,
        spawner: Arc<dyn SubAgentSpawner>,
        hub: Arc<Hub>,
        worker_count: usize,
    ) -> Self {
        Self {
            queue_manager,
            spawner,
            hub,
            worker_count,
        }
    }

    pub async fn start(&self, shutdown_rx: broadcast::Sender<()>) {
        for i in 0..self.worker_count {
            let queue = self.queue_manager.clone();
            let spawner = self.spawner.clone();
            let hub = self.hub.clone();
            let mut rx = shutdown_rx.subscribe();
            let worker_id = format!("worker-{}", i);

            tokio::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_millis(100));

                loop {
                    tokio::select! {
                        _ = interval.tick() => {
                            match queue.poll(&worker_id).await {
                                Ok(Some(job)) => {
                                    Self::process_job(&job, &queue, &spawner, &hub, &worker_id).await;
                                }
                                Ok(None) => {
                                    // No job available
                                }
                                Err(e) => {
                                    eprintln!("Worker {} failed to poll: {}", worker_id, e);
                                }
                            }
                        }
                        _ = rx.recv() => {
                            break;
                        }
                    }
                }
            });
        }
    }

    async fn process_job(
        job: &SubAgentJob,
        queue: &QueueManager,
        spawner: &Arc<dyn SubAgentSpawner>,
        hub: &Hub,
        worker_id: &str,
    ) {
        // Broadcast START
        let _ = hub.publish_teammate_event(
            "sub_agent_mesh".to_string(),
            TeammateMeshEvent {
                agent_id: worker_id.to_string(),
                action: "START".to_string(),
                status: "RUNNING".to_string(),
                payload: job.id.clone().into_bytes(),
            },
        );

        let shared_task = SharedTask {
            id: job.id.clone(),
            organization_id: job.organization_id.clone(),
            mission_id: String::new(),
            parent_plan_id: job.parent_task_id.clone(),
            dependencies: vec![],
            title: "SubAgent Job".to_string(),
            description: None,
            assigned_agent_id: Some(worker_id.to_string()),
            status: "IN_PROGRESS".to_string(),
            priority: "P1".to_string(),
            payload: job.payload.to_string(),
            locked_until: None,
            ultraplan_phase: None,
            deliberation_log: None,
            depth: None,
            created_at: job.created_at,
            updated_at: job.updated_at,
            action_risk: None,
            approval_status: None,
            proposed_content: None,
        };

        match spawner.spawn(shared_task).await {
            Ok(_) => {
                let _ = queue.mark_completed(&job.id).await;
                let _ = hub.publish_teammate_event(
                    "sub_agent_mesh".to_string(),
                    TeammateMeshEvent {
                        agent_id: worker_id.to_string(),
                        action: "COMPLETE".to_string(),
                        status: "SUCCESS".to_string(),
                        payload: job.id.clone().into_bytes(),
                    },
                );
            }
            Err(e) => {
                let _ = queue.mark_failed(&job.id, &e).await;
                let _ = hub.publish_teammate_event(
                    "sub_agent_mesh".to_string(),
                    TeammateMeshEvent {
                        agent_id: worker_id.to_string(),
                        action: "COMPLETE".to_string(),
                        status: "FAIL".to_string(),
                        payload: e.into_bytes(),
                    },
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::sync::mpsc;
    use async_trait::async_trait;

    struct MockSpawner {
        should_fail: bool,
    }

    #[async_trait]
    impl SubAgentSpawner for MockSpawner {
        async fn spawn(&self, _task: SharedTask) -> Result<(), String> {
            if self.should_fail {
                Err("simulated failure".to_string())
            } else {
                Ok(())
            }
        }
    }

    async fn setup_test_db() -> sqlx::PgPool {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        sqlx::PgPool::connect(&db_url).await.unwrap()
    }

    #[tokio::test]
    #[ignore]
    async fn test_sub_agent_worker_pool_concurrent() {
        let pool = setup_test_db().await;

        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS sub_agent_queue (id TEXT PRIMARY KEY, organization_id TEXT NOT NULL, parent_task_id TEXT NOT NULL, payload TEXT NOT NULL, status TEXT NOT NULL DEFAULT 'PENDING', worker_id TEXT, created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP)")
            .execute(&pool)
            .await;
        let _ = sqlx::query("DELETE FROM sub_agent_queue").execute(&pool).await;

        let queue_manager = Arc::new(QueueManager::new(pool.clone()));
        let spawner = Arc::new(MockSpawner { should_fail: false });
        let (tx, _) = mpsc::channel(10);
        let hub = Arc::new(Hub::new(tx, pool.clone()));

        let worker_pool = SubAgentWorkerPool::new(queue_manager.clone(), spawner, hub.clone(), 5);
        let (shutdown_tx, _) = broadcast::channel(1);
        worker_pool.start(shutdown_tx.clone()).await;

        let mut rx = hub.subscribe_teammate_mesh("sub_agent_mesh".to_string());

        // Enqueue multiple jobs concurrently
        for i in 0..10 {
            let job = SubAgentJob {
                id: format!("job-{}", i),
                organization_id: "org1".to_string(),
                parent_task_id: "task1".to_string(),
                payload: serde_json::json!({"test": true}),
                status: "QUEUED".to_string(),
                worker_id: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            queue_manager.enqueue(job).await.unwrap();
        }

        let mut success_count = 0;
        let mut start_count = 0;

        let timeout = tokio::time::sleep(Duration::from_secs(5));
        tokio::pin!(timeout);

        loop {
            tokio::select! {
                Ok(event) = rx.recv() => {
                    if event.action == "START" {
                        start_count += 1;
                    } else if event.action == "COMPLETE" && event.status == "SUCCESS" {
                        success_count += 1;
                    }
                    if success_count == 10 && start_count == 10 {
                        break;
                    }
                }
                _ = &mut timeout => {
                    panic!("Test timed out waiting for events. start_count: {}, success_count: {}", start_count, success_count);
                }
            }
        }

        let _ = shutdown_tx.send(());

        let remaining = queue_manager.poll("test").await.unwrap();
        assert!(remaining.is_none());
    }
}
