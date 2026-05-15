pub enum AgentEvent {
    RunStarted { iteration: i32 },
    TextChunk { content: String },
    ToolCall { name: String, args_json: String, result: String, iteration: i32 },
    TaskComplete { content: String },
    TaskError { error: String },
    UserInterventionRequired { error: String },
    IterationStarted { iteration: i32, message_count: usize },
    CheckpointSaved { iteration: i32, path: String },
    Handoff { target_agent: String },
    RewindOccurred { iteration: i32, checkpoint_id: String, reason: String },
}