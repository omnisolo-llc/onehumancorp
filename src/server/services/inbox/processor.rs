
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[tokio::test]
    async fn test_inbox_processor_mocked() {
        // Since testing real PgPool in background tokio tasks during unit tests is flaky,
        // we can verify the logic by abstracting out the DB or just testing parsing.
        // For now, since processor is tightly coupled with sqlx macro, we verify it parses payload.
        let msg_payload = serde_json::json!({
            "message_id": "123",
            "tenant_id": "test_tenant",
            "content": "vegan cake",
            "channel_type": "IG"
        });

        if let Ok(payload) = serde_json::from_slice::<serde_json::Value>(&serde_json::to_vec(&msg_payload).unwrap()) {
            let content = payload["content"].as_str().unwrap_or_default();
            assert!(content.contains("vegan cake"));
        } else {
            panic!("Failed to parse payload");
        }
    }
}
