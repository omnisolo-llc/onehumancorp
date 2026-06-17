use crate::llm::LlmClient;

pub struct Agent {
    client: Box<dyn LlmClient>,
}

impl Agent {
    pub fn new(client: Box<dyn LlmClient>) -> Self {
        Self { client }
    }
}
