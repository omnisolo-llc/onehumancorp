pub struct ManychatClient {
    pub api_key: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ManychatMessage {
    pub id: String,
    pub direction: String,
    pub sender_id: String,
    pub body: String,
    pub created_at_unix: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ManychatConversation {
    pub id: String,
    pub channel: String,
    pub external_customer_id: String,
    pub customer_name: String,
    pub status: String,
    pub messages: Vec<ManychatMessage>,
}

impl ManychatClient {
    pub fn new(api_key: String) -> Self {
        ManychatClient { api_key }
    }

    pub async fn fetch_conversations(&self) -> Result<Vec<ManychatConversation>, String> {
        Ok(vec![ManychatConversation {
            id: "manychat-thread-demo".to_string(),
            channel: "instagram".to_string(),
            external_customer_id: "manychat-contact-demo".to_string(),
            customer_name: "Test Customer".to_string(),
            status: "open".to_string(),
            messages: vec![ManychatMessage {
                id: "manychat-message-demo".to_string(),
                direction: "inbound".to_string(),
                sender_id: "manychat-contact-demo".to_string(),
                body: "Do you have vegan birthday cakes available this weekend?".to_string(),
                created_at_unix: 0,
            }],
        }])
    }
}
