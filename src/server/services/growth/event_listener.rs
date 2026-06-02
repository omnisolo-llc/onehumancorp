use sqlx::PgPool;
use std::sync::Arc;
use crate::hub::Hub;
use crate::msgbus::Bus;
use crate::msgbus::Message;

pub fn spawn_event_listener(pool: PgPool, hub: Arc<Hub>) {
    tokio::spawn(async move {
        let bus = hub.msgbus();
        let _ = bus.subscribe("ServiceCompleted".to_string(), Box::new(move |msg: Message| {
            let payload_str = String::from_utf8_lossy(&msg.payload);
            tracing::info!(
                "Omnichannel Comm Engine: Sending SMS to customer regarding service completion: {}. 'How was your service? Reply 1-5.'",
                payload_str
            );
        })).await;

        let _ = bus.subscribe("OrderDelivered".to_string(), Box::new(move |msg: Message| {
            let payload_str = String::from_utf8_lossy(&msg.payload);
            tracing::info!(
                "Omnichannel Comm Engine: Sending SMS to customer regarding order delivery: {}. 'How was your order? Reply 1-5.'",
                payload_str
            );
        })).await;
    });
}
