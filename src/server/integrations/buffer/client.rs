pub struct BufferClient {
    pub access_token: String,
}

impl BufferClient {
    pub fn new(access_token: String) -> Self {
        BufferClient { access_token }
    }
}

impl BufferClient {
    pub async fn get_messages(&self) -> Result<Vec<String>, String> {
        // Mock get messages
        Ok(vec!["msg1".to_string(), "msg2".to_string()])
    }

    pub async fn reply_message(&self, message_id: &str, reply: &str) -> Result<(), String> {
        // Mock reply
        Ok(())
    }
}
