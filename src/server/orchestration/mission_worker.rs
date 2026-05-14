use std::sync::Arc;
use tokio::time::{sleep, Duration};
use crate::hub::Hub;
use crate::sip::SipDB;
use sqlx::Row;

pub struct MissionQueueDrainer {
    hub: Arc<Hub>,
    sip_db: Arc<SipDB>,
    poll_interval: Duration,
}

#[derive(Debug, Clone)]
pub struct Mission {
    pub id: String,
    pub status: String,
    pub payload: String,
    pub tenant_id: Option<String>,
}

impl MissionQueueDrainer {
    pub fn new(hub: Arc<Hub>, sip_db: Arc<SipDB>, poll_interval: Duration) -> Self {
        Self {
            hub,
            sip_db,
            poll_interval,
        }
    }

    pub async fn start(self: Arc<Self>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            self.run().await;
        })
    }

    pub async fn run(&self) {
        let mut interval = tokio::time::interval(self.poll_interval);

        loop {
            interval.tick().await;

            // Try to claim missions and process them
            match self.sip_db.claim_next_mission("drainer_agent").await {
                Ok(Some(mission)) => {
                    tracing::info!("Claimed mission: {}", mission.id);

                    // Simulate work for processing
                    tokio::time::sleep(Duration::from_millis(100)).await;

                    // Complete mission
                    if let Err(e) = self.sip_db.mark_mission_completed(&mission.id, "Successfully drained").await {
                        tracing::error!("Failed to mark mission {} as complete: {}", mission.id, e);
                    } else {
                        tracing::info!("Successfully processed and completed mission {}", mission.id);
                    }
                }
                Ok(None) => {
                    // No missions pending, just wait for next tick
                }
                Err(e) => {
                    tracing::error!("Error claiming mission: {}", e);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use chrono::Utc;

    async fn setup_db() -> sqlx::postgres::PgPool {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://dummy".to_string());
        sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .connect_lazy(&db_url)
            .unwrap()
    }

    #[tokio::test]
    async fn test_drainer_initialization() {
        let pool = setup_db().await;
        let sip_db = Arc::new(SipDB::new(pool.clone(), "test_org".to_string()));
        let (tx, _) = tokio::sync::mpsc::channel(1);
        let hub = Arc::new(Hub::new(tx, pool));

        let drainer = MissionQueueDrainer::new(hub, sip_db, Duration::from_millis(10));
        assert_eq!(drainer.poll_interval, Duration::from_millis(10));
    }
}
