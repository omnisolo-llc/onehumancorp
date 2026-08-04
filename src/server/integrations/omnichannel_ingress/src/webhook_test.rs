use crate::webhook::*;

#[test]
fn test_parse_meta_payload() {
    let payload = r#"{
        "object": "page",
        "entry": [
            {
                "id": "12345",
                "time": 1234567890,
                "messaging": [
                    {
                        "sender": { "id": "user1" },
                        "recipient": { "id": "page1" },
                        "timestamp": 1234567890,
                        "message": {
                            "mid": "mid.12345",
                            "text": "Hello world"
                        }
                    }
                ]
            }
        ]
    }"#;

    let result = parse_meta_payload(payload).unwrap();
    assert_eq!(result.object, "page");
    assert_eq!(result.entry.len(), 1);
    let entry = &result.entry[0];
    assert_eq!(entry.id, "12345");
    assert_eq!(entry.time, 1234567890);

    let messaging = entry.messaging.as_ref().unwrap();
    assert_eq!(messaging.len(), 1);
    let msg = &messaging[0];
    assert_eq!(msg.sender.id, "user1");
    assert_eq!(msg.recipient.id, "page1");
    assert_eq!(msg.timestamp, 1234567890);
    let text_msg = msg.message.as_ref().unwrap();
    assert_eq!(text_msg.mid, "mid.12345");
    assert_eq!(text_msg.text.as_ref().unwrap(), "Hello world");
}
