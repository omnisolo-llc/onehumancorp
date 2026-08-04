use crate::models::Message;

pub struct ChatService;

impl ChatService {
    pub async fn process_incoming_message(_msg: Message) -> Result<(), String> {
        // Log to database, publish to Redis, etc.
        Ok(())
    }
}
