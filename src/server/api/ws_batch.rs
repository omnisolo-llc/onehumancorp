/// Take one outgoing batch, preserving the single-message wire format.
pub(super) fn take_batch(batch: &mut Vec<String>) -> Option<String> {
    if batch.is_empty() {
        return None;
    }
    if batch.len() == 1 {
        return Some(batch.remove(0));
    }
    let items: Vec<_> = batch
        .drain(..batch.len().min(20))
        .map(|message| serde_json::from_str(&message).unwrap_or(serde_json::Value::String(message)))
        .collect();
    Some(serde_json::json!({"type":"batch","items":items}).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn batches_keep_objects_and_enforce_twenty_message_limit() {
        let mut messages: Vec<_> = (0..45)
            .map(|id| serde_json::json!({"id":id}).to_string())
            .collect();
        for (start, count) in [(0, 20), (20, 20), (40, 5)] {
            let value: serde_json::Value =
                serde_json::from_str(&take_batch(&mut messages).unwrap()).unwrap();
            let items = value["items"].as_array().unwrap();
            assert_eq!(items.len(), count);
            assert_eq!(items[0]["id"], start);
        }
        assert!(take_batch(&mut messages).is_none());
    }
    #[test]
    fn singletons_and_non_json_messages_remain_compatible() {
        assert_eq!(
            take_batch(&mut vec!["plain".into()]).as_deref(),
            Some("plain")
        );
        let value: serde_json::Value =
            serde_json::from_str(&take_batch(&mut vec!["plain".into(), "{}".into()]).unwrap())
                .unwrap();
        assert_eq!(value["items"][0], "plain");
        assert!(value["items"][1].is_object());
    }
}
