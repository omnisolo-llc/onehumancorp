use std::sync::Arc;
use crate::db::DB;
use tokio::time::{interval, Duration};

pub struct AgentMissionsWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}

impl AgentMissionsWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(5),
        }
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            let mut interval = interval(self.poll_interval);
            loop {
                interval.tick().await;
                if let Err(e) = self.poll().await {
                    tracing::error!("AgentMissionsWorker: poll error: {}", e);
                }
            }
        });
    }

    pub async fn poll(&self) -> Result<(), Box<dyn std::error::Error>> {
        let missions = self.db.get_pending_agent_missions().await?;
        for mission in missions {
            // "Drain the OHC 'Hybrid Agentic OS' mission queue"
            tracing::info!("Draining mission {}", mission.id);
            self.db.update_agent_mission_status(&mission.id, "COMPLETED").await?;
        }
        Ok(())
    }
}
