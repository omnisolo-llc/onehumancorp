use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushNotificationPayload {
    pub title: String,
    pub body: String,
    pub action_type: String,
    pub action_id: String,
}

pub async fn dispatch_push_notification(tenant_id: &str, payload: PushNotificationPayload) -> Result<(), String> {
    // In a real implementation, this would connect to FCM or APNS.
    // For now, we log the push notification payload directly to simulate the dispatch.
    tracing::info!(
        "DISPATCHING PUSH NOTIFICATION to tenant {}: {:?}",
        tenant_id,
        payload
    );
    Ok(())
}
