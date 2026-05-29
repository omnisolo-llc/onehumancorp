use crate::agent::AgentRunConfig;
use std::path::Path;

// Prompt Construction: OpenAI Codex Mechanic
// 1. Server-controlled System Message (Highest Priority)
// 2. Tool Definitions
// 3. Developer Instructions
// 4. User Instructions (cascading AGENTS.md files, capped at 32 KiB)
// 5. Conversation History (happens at run loop)

pub async fn load_cascading_agents_md(start_dir: &std::path::Path) -> String {
    let mut current_dir = start_dir.to_path_buf();
    let mut contents = Vec::new();
    let mut max_depth = 50;

    loop {
        let agent_file = current_dir.join("AGENTS.md");
        if agent_file.exists() && agent_file.is_file() {
            if let Ok(content) = tokio::fs::read_to_string(&agent_file).await {
                contents.push(content);
            }
        }

        if !current_dir.pop() || max_depth == 0 {
            break;
        }
        max_depth -= 1;
    }

    // Order: more deeply-nested files take precedence
    let mut combined = String::new();
    for (i, content) in contents.iter().enumerate() {
        if i > 0 {
            combined.push_str("\n\n---\n\n");
        }
        combined.push_str(content);
    }

    let max_bytes = 32 * 1024;
    if combined.len() > max_bytes {
        let mut end_idx = max_bytes;
        while end_idx > 0 && !combined.is_char_boundary(end_idx) {
            end_idx -= 1;
        }
        combined.truncate(end_idx);
        combined.push_str("\n\n[System: AGENTS.md content truncated to 32KiB limit.]");
    }

    combined
}

/// A dedicated builder for the Hierarchical Priority Stack mechanic.
/// This fulfills the Master Catalog specification: Prompt Construction: OpenAI Codex Strict hierarchical priority stack
/// 1. Server-controlled System Message (Highest Priority)
/// 2. Tool Definitions
/// 3. Developer Instructions
/// 4. User Instructions (capped at 32 KiB)
pub struct HierarchicalPromptBuilder {
    server_system_message: String,
    tool_definitions: String,
    developer_instructions: String,
    user_instructions: String,
    enable_lost_in_the_middle_prevention: bool,
}

impl HierarchicalPromptBuilder {
    pub fn new(cfg: &AgentRunConfig, tools: &[crate::tools::Tool]) -> Self {
        let mut tool_defs = String::new();
        if !tools.is_empty() {
            for tool in tools {
                tool_defs.push_str(&format!("Tool: {}\n", tool.name));
                tool_defs.push_str(&format!("Description: {}\n", tool.description));
                tool_defs.push_str(&format!("Parameters: {}\n", tool.parameters));
            }
            tool_defs.pop(); // Remove trailing newline
        }

        let mut end_idx = 32768;
        if cfg.user_instructions.len() > 32768 {
            while end_idx > 0 && !cfg.user_instructions.is_char_boundary(end_idx) {
                end_idx -= 1;
            }
        } else {
            end_idx = cfg.user_instructions.len();
        }
        let user_instr = cfg.user_instructions[..end_idx].to_string();

        Self {
            server_system_message: cfg.server_system_message.clone(),
            tool_definitions: tool_defs,
            developer_instructions: cfg.developer_instructions.clone(),
            user_instructions: user_instr,
            enable_lost_in_the_middle_prevention: cfg.enable_lost_in_the_middle_prevention,
        }
    }

    pub fn build(&self) -> String {
        let mut combined_system = String::new();

        // Prompt Construction: OpenAI Codex Strict hierarchical priority stack
        // 1. Server-controlled System Message (Highest Priority) -> 2. Tool Definitions -> 3. Developer Instructions -> 4. User Instructions
        // 1. Server-controlled System Message (Highest Priority)
        if !self.server_system_message.is_empty() {
            combined_system.push_str("[Server System Message]\n");
            combined_system.push_str(&self.server_system_message);
        }

        // 2. Tool Definitions
        if !self.tool_definitions.is_empty() {
            if !combined_system.is_empty() {
                combined_system.push_str("\n\n");
            }
            combined_system.push_str("[Tool Definitions]\n");
            combined_system.push_str(&self.tool_definitions);
        }

        // 3. Developer Instructions
        if !self.developer_instructions.is_empty() {
            if !combined_system.is_empty() {
                combined_system.push_str("\n\n");
            }
            combined_system.push_str("[Developer Instructions]\n");
            combined_system.push_str(&self.developer_instructions);
        }

        // 4. User Instructions
        if !self.user_instructions.is_empty() {
            if !combined_system.is_empty() {
                combined_system.push_str("\n\n");
            }
            combined_system.push_str("[User Instructions]\n");
            combined_system.push_str(&self.user_instructions);
        }

        if self.enable_lost_in_the_middle_prevention && !self.server_system_message.is_empty() {
            if !combined_system.is_empty() {
                combined_system.push_str("\n\n");
            }
            combined_system.push_str("[CRITICAL REMINDER: High-Signal Context Repeated to prevent 'Lost in the Middle']\n");
            combined_system.push_str(&self.server_system_message);
        }

        combined_system
    }
}

pub fn build_hierarchical_system_prompt(cfg: &AgentRunConfig, tools: &[crate::tools::Tool]) -> String {
    HierarchicalPromptBuilder::new(cfg, tools).build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::Tool;
    use std::sync::Arc;
    use crate::types::{ChatResponse, Message, Role, ToolCall, Usage};
    use crate::llm::LlmClient;
    use ohc_builtin_agent_core::types::ChatRequest;


    struct RecordingLlmClient {
        responses: tokio::sync::Mutex<Vec<ChatResponse>>,
        pub received_requests: tokio::sync::Mutex<Vec<ChatRequest>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for RecordingLlmClient {
        async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            self.received_requests.lock().await.push(req);
            let mut resps = self.responses.lock().await;
            if !resps.is_empty() {
                Ok(resps.remove(0))
            } else {
                Ok(ChatResponse {
                    message: Message::assistant("default output"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("mock-id".to_string()),
                })
            }
        }
    }

    struct DummyExecutor;
    #[async_trait::async_trait]
    impl crate::tools::ToolExecutor for DummyExecutor {
        async fn execute(&self, _args: serde_json::Value) -> Result<String, crate::types::ToolError> {
            Ok("dummy".to_string())
        }
    }

#[tokio::test]
    async fn test_prompt_construction_lost_in_the_middle_prevention() {
        let client = Arc::new(RecordingLlmClient {
            responses: tokio::sync::Mutex::new(Vec::new()),
            received_requests: tokio::sync::Mutex::new(Vec::new()),
        });

        // Create an agent and we will inject some state so messages.len() > 3
        use crate::agent::Agent;
        let agent = Agent::new(client.clone(), vec![]);

        let mut cfg = AgentRunConfig::default();
        cfg.enable_lost_in_the_middle_prevention = true;
        cfg.enable_state_checkpointing = true;
        cfg.developer_instructions = "Developer instructions here.".to_string();
        cfg.user_instructions = "Super long user instructions that span many many words.".to_string();

        let scratchpad_path = format!(".test_checkpoint_litm_{}.json", uuid::Uuid::new_v4());
        cfg.state_scratchpad_path = Some(scratchpad_path.clone());

        // Pre-fill some messages to make len > 3
        let initial_msgs = vec![
            Message::user("Task: Do something"),
            Message::assistant("Thinking..."),
            Message::assistant("Still thinking..."),
            Message::user("Please continue"),
        ];
        tokio::fs::write(&scratchpad_path, serde_json::to_string(&initial_msgs).unwrap()).await.unwrap();

        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };

        let result = agent.run(&cfg, "Continue working", &mut on_event).await;
        assert!(result.is_ok());

        let reqs = client.received_requests.lock().await;
        let lr = reqs.last();
        let req = lr.as_ref().unwrap();
        let last_msg = req.messages.last().unwrap();

        assert_eq!(last_msg.role, Role::User);
        assert!(last_msg.content.contains("[System Reminder: Developer instructions here.]"));
        assert!(last_msg.content.contains("[System Reminder to combat 'Lost in the Middle' effect: Remember your core objective: Super long user instructions that span many many words....]"));

        let _ = tokio::fs::remove_file(&scratchpad_path).await;
    }

}
