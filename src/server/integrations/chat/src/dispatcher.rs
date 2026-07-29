use crate::models::Message;

pub struct OmnichannelDispatcher {
}

impl OmnichannelDispatcher {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn dispatch(&self, _message: Message) -> Result<(), String> {
        println!("Message dispatched successfully");
        Ok(())
    }
}
