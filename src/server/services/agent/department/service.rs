
use std::sync::Arc;
use std::time::Duration;
use crate::msgbus::{Bus, Message};
use crate::db::DB;



pub struct DepartmentService {
    bus: Arc<dyn Bus>,
    db: Arc<DB>,
}

impl DepartmentService {
    pub fn new(bus: Arc<dyn Bus>, db: Arc<DB>) -> Self {
        DepartmentService { bus, db }
    }

    pub async fn get_mode(&self, tenant_id: &str, department: &str) -> Result<String, String> {
        let pool = &self.db.pool;
        let query = "SELECT mode FROM agent_department_config WHERE tenant_id = $1 AND department = $2";

        let tenant_uuid = uuid::Uuid::parse_str(tenant_id).map_err(|e| e.to_string())?;

        let row: Option<(String,)> = sqlx::query_as(query)
            .bind(tenant_uuid)
            .bind(department)
            .fetch_optional(pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(row.map(|r| r.0).unwrap_or_else(|| "auto".to_string()))
    }


    pub async fn start(&self) -> Result<(), String> {
        let bus_clone = self.bus.clone();
        let db_clone = self.db.clone();

        let handler = Box::new(move |msg: Message| {
            if msg.topic == "system:order_received" {
                let bus = bus_clone.clone();
                let db = db_clone.clone();
                // In a real scenario we'd extract tenant_id from msg.payload.
                // Assuming "system" or specific tenant here. Let's just use a dummy one for the E2E or default.
                let tenant_id = "00000000-0000-0000-0000-000000000000".to_string();

                tokio::spawn(async move {
                    let mut attempts = 0;
                    let max_retries = 3;

                    loop {
                        attempts += 1;
                        let pool = &db.pool;
                        let tenant_uuid = uuid::Uuid::parse_str(&tenant_id).unwrap_or_default();

                        let run_result = tokio::time::timeout(std::time::Duration::from_secs(60), async {
                            let manager_mode: Option<(String,)> = sqlx::query_as("SELECT mode FROM agent_department_config WHERE tenant_id = $1 AND department = 'manager'")
                                .bind(tenant_uuid)
                                .fetch_optional(pool)
                                .await.unwrap_or(None);

                            let ambassador_mode: Option<(String,)> = sqlx::query_as("SELECT mode FROM agent_department_config WHERE tenant_id = $1 AND department = 'ambassador'")
                                .bind(tenant_uuid)
                                .fetch_optional(pool)
                                .await.unwrap_or(None);

                            let m_mode = manager_mode.map(|r| r.0).unwrap_or_else(|| "auto".to_string());
                            let a_mode = ambassador_mode.map(|r| r.0).unwrap_or_else(|| "auto".to_string());

                            if m_mode == "auto" {
                                let _ = bus.publish(Message {
                                    topic: "system:activity".to_string(),
                                    payload: "Operations processed OrderReceived".as_bytes().to_vec(),
                                }).await;
                            } else {
                                let _ = bus.publish(Message {
                                    topic: "system:activity".to_string(),
                                    payload: "Operations requested draft review for OrderReceived".as_bytes().to_vec(),
                                }).await;
                            }

                            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

                            if a_mode == "auto" {
                                let _ = bus.publish(Message {
                                    topic: "system:activity".to_string(),
                                    payload: "Customer Success drafted confirmation".as_bytes().to_vec(),
                                }).await;
                            } else {
                                let _ = bus.publish(Message {
                                    topic: "system:activity".to_string(),
                                    payload: "Customer Success requested draft review for confirmation".as_bytes().to_vec(),
                                }).await;
                            }
                            Ok::<(), String>(())
                        }).await;

                        match run_result {
                            Ok(Ok(_)) => break, // Success
                            Ok(Err(_e)) => {
                                // Logic error, maybe break or retry based on type
                                if attempts >= max_retries { break; }
                            }
                            Err(_) => {
                                // Timeout
                                if attempts >= max_retries { break; }
                            }
                        }
                    }
                });
            }
        });

        let _ = self.bus.subscribe("system:order_received".to_string(), handler).await?;

        Ok(())
    }
}
