use super::*;

impl Agent {
    pub fn new(llm: Arc<dyn LlmClient>, tools: Vec<Tool>) -> Self {
        Self {
            llm,
            tools,
            progress: Arc::new(AgentProgress::default()),
            memory_store: None,
            checkpointer: None,
            observation_store: Arc::new(dashmap::DashMap::new()),
        }
    }

    pub fn with_memory_store(
        mut self,
        store: Arc<dyn crate::memory_store::LongTermMemory>,
    ) -> Self {
        self.memory_store = Some(store);
        self
    }

    pub fn with_checkpointer(
        mut self,
        checkpointer: Arc<dyn crate::checkpointer::CheckpointSaver>,
    ) -> Self {
        self.checkpointer = Some(checkpointer);
        self
    }

    pub fn add_tool(&mut self, tool: Tool) {
        self.tools.push(tool);
    }

    pub fn query(
        self: Arc<Self>,
        cfg: AgentRunConfig,
        initial_message: String,
    ) -> tokio::sync::mpsc::UnboundedReceiver<AgentEvent> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        tokio::spawn(async move {
            let mut on_event = |event: AgentEvent| {
                // We use an unbounded channel so send does not block or drop events if the consumer is slow.
                let _ = tx.send(event);
            };

            if let Err(e) = self.run(&cfg, &initial_message, &mut on_event).await {
                // Propagate the error through the stream so it is not silently swallowed.
                let _ = tx.send(AgentEvent::TaskError {
                    error: format!("Agent run failed: {}", e),
                });
            }
        });

        rx
    }

}
