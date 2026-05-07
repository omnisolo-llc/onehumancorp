use std::sync::Arc;
use tokio::time::{sleep, Duration, Instant};
use crate::orchestration::queue::{TaskQueue, Job};
use crate::db::DB;
use chrono::Utc;
use async_trait::async_trait;
use crate::orchestration::mesh::TeammateMesh;
use opentelemetry::metrics::{Histogram, Counter};

#[async_trait]
pub trait SubAgentSpawner: Send + Sync {
    async fn spawn_isolated(&self, job: &Job) -> Result<(), String>;
}

pub struct DefaultSubAgentSpawner {
    db: Arc<DB>,
}

impl DefaultSubAgentSpawner {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl SubAgentSpawner for DefaultSubAgentSpawner {
    async fn spawn_isolated(&self, job: &Job) -> Result<(), String> {
        let timestamp = Utc::now().timestamp();
        let status_dir = ".agent-task/status";
        tokio::fs::create_dir_all(status_dir).await.map_err(|e| e.to_string())?;

        let status_file = format!("{}/{}.yml", status_dir, timestamp);
        let content = format!("job_id: {}\nstatus: RUNNING\nagent_role: {}\n", job.id, job.agent_role);
        tokio::fs::write(status_file, content).await.map_err(|e| e.to_string())?;

        // In a real implementation this would fork a process or dispatch a K8s job.
        // For the scope of this OS-level limits request, simulating the delay.
        sleep(Duration::from_millis(50)).await;

        Ok(())
    }
}

pub struct SubAgentWorker {
    queue: Arc<dyn TaskQueue>,
    spawner: Arc<dyn SubAgentSpawner>,
    db: Arc<DB>,
    mesh: Arc<dyn TeammateMesh>,
    roles: Vec<String>,
    histo: Histogram<f64>,
    counter: Counter<u64>,
}

impl SubAgentWorker {
    pub fn new(queue: Arc<dyn TaskQueue>, spawner: Arc<dyn SubAgentSpawner>, db: Arc<DB>, mesh: Arc<dyn TeammateMesh>, roles: Vec<String>) -> Self {
        let meter = opentelemetry::global::meter("ohc.sub_agent");
        let histo = meter.f64_histogram("ohc_sub_agent_execution_duration_seconds").build();
        let counter = meter.u64_counter("ohc_sub_agent_failures_total").build();

        Self { queue, spawner, db, mesh, roles, histo, counter }
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            self.loop_run().await;
        });
    }

    async fn loop_run(&self) {
        loop {
            self.poll_and_execute().await;
            sleep(Duration::from_secs(2)).await;
        }
    }

    async fn poll_and_execute(&self) {
        if let Ok(Some(job)) = self.queue.dequeue(self.roles.clone()).await {
            let start = Instant::now();
            let _ = self.transition_job_state(&job, "RUNNING").await;
            self.emit_mesh_event(&job, "RUNNING").await;

            match self.spawner.spawn_isolated(&job).await {
                Ok(_) => {
                    self.histo.record(start.elapsed().as_secs_f64(), &[]);
                    let _ = self.complete_job(&job).await;
                }
                Err(err) => {
                    self.histo.record(start.elapsed().as_secs_f64(), &[]);
                    let _ = self.fail_job(&job, &err).await;
                }
            }
        }
    }

    async fn transition_job_state(&self, job: &Job, new_state: &str) -> Result<(), String> {
        let trans_id = uuid::Uuid::new_v4().to_string();

        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;

                let old_status: Option<String> = sqlx::query_scalar(
                    "SELECT status FROM shared_tasks_decomposition WHERE id = $1 FOR UPDATE"
                )
                .bind(&job.parent_task_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(os) = old_status {
                    sqlx::query("UPDATE shared_tasks_decomposition SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2")
                        .bind(new_state)
                        .bind(&job.parent_task_id)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                    sqlx::query(
                        r#"
                        INSERT INTO state_machine_transitions (id, task_id, from_state, to_state, agent_id, transitioned_at)
                        VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP)
                        "#
                    )
                    .bind(trans_id)
                    .bind(&job.parent_task_id)
                    .bind(os)
                    .bind(new_state)
                    .bind(&job.agent_role)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
                }
                tx.commit().await.map_err(|e| e.to_string())?;
            }
            crate::db::DbStore::Sqlite(pool) => {
                let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

                let old_status: Option<String> = sqlx::query_scalar(
                    "SELECT status FROM shared_tasks_decomposition WHERE id = ?"
                )
                .bind(&job.parent_task_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(os) = old_status {
                    sqlx::query("UPDATE shared_tasks_decomposition SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(new_state)
                        .bind(&job.parent_task_id)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| e.to_string())?;

                    sqlx::query(
                        r#"
                        INSERT INTO state_machine_transitions (id, task_id, from_state, to_state, agent_id, transitioned_at)
                        VALUES (?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
                        "#
                    )
                    .bind(trans_id)
                    .bind(&job.parent_task_id)
                    .bind(os)
                    .bind(new_state)
                    .bind(&job.agent_role)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
                }
                tx.commit().await.map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    async fn complete_job(&self, job: &Job) -> Result<(), String> {
        self.queue.complete(&job.id).await?;
        self.emit_mesh_event(job, "COMPLETED").await;
        self.transition_job_state(job, "COMPLETED").await
    }

    pub async fn fail_job_for_test(&self, job: &Job, reason: &str) -> Result<(), String> {
        self.fail_job(job, reason).await
    }

    async fn fail_job(&self, job: &Job, _reason: &str) -> Result<(), String> {
        self.counter.add(1, &[]);
        self.queue.fail(&job.id, _reason).await?;
        self.emit_mesh_event(job, "FAILED").await;
        self.transition_job_state(job, "FAILED").await
    }

    async fn emit_mesh_event(&self, job: &Job, status: &str) {
        let payload = serde_json::json!({
            "job_id": job.id,
            "parent_task_id": job.parent_task_id,
            "status": status,
            "agent_role": job.agent_role,
        }).to_string().into_bytes();

        let _ = self.mesh.publish("subagent.transition", payload).await;
    }
}
