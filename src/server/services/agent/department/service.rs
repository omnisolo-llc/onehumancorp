use std::sync::Arc;
use crate::msgbus::{Bus, Message};

/// The `DepartmentService` struct acts as a primary component.
///
/// # Overview
/// This struct encapsulates the state necessary for execution.
///
/// # Thread Safety
/// Designed to be shared safely across async tokio tasks.
/// Uses types like `Arc` and `Mutex` to prevent race conditions.
///
/// # Performance
/// Optimized for low-latency operations.
///
/// # Usage Guidelines
/// - Created during initialization.
/// - Avoid holding synchronous locks across await points.
pub struct DepartmentService {
    bus: Arc<dyn Bus>,
}

impl DepartmentService {
    pub fn new(bus: Arc<dyn Bus>) -> Self {
        DepartmentService {
            bus,
        }
    }

    pub async fn start(&self) -> Result<(), String> {
        let bus_clone = self.bus.clone();

        let handler = Box::new(move |msg: Message| {
            if msg.topic == "system:order_received" {
                let bus = bus_clone.clone();
                tokio::spawn(async move {
                    let _ = bus.publish(Message {
                        topic: "system:activity".to_string(),
                        payload: "Operations processed OrderReceived".as_bytes().to_vec(),
                    }).await;

                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

                    let _ = bus.publish(Message {
                        topic: "system:activity".to_string(),
                        payload: "Customer Success drafted confirmation".as_bytes().to_vec(),
                    }).await;
                });
            }
        });

        let _ = self.bus.subscribe("system:order_received".to_string(), handler).await?;

        Ok(())
    }
}
