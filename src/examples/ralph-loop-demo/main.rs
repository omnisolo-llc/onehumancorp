use ohc_builtin_agent::agent::{Agent, AgentRunConfig};
use ohc_builtin_agent::llm::LlmClient;
use ohc_builtin_agent::ralph_loop::RalphLoop;
use ohc_builtin_agent::types::{ChatRequest, ChatResponse, Message, Usage};
use std::sync::Arc;

struct DemoLlmClient;

#[async_trait::async_trait]
impl LlmClient for DemoLlmClient {
    async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        // Simple mock behavior: if the prompt asks to break down the task, return a JSON array of features
        if req.messages.last().map(|m| m.content.contains("Break down the following task")).unwrap_or(false) {
            Ok(ChatResponse {
                message: Message::assistant(r#"["Setup Database", "Create API", "Write Frontend"]"#),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id_init".to_string()),
            })
        } else {
            // For feature implementation steps, simulate success
            let feature_name = "the requested feature"; // In a real mock we'd parse the prompt
            Ok(ChatResponse {
                message: Message::assistant(&format!("Successfully implemented {}", feature_name)),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id_feat".to_string()),
            })
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Starting Ralph Loop Demo...");

    // 1. Create a progress file in the current directory
    let progress_file = ".ralph_progress_demo.json";

    // Clear old progress if it exists to ensure a fresh run
    let _ = std::fs::remove_file(progress_file);

    // 2. Initialize LLM, Agent, and Config
    let llm = Arc::new(DemoLlmClient);
    let agent = Arc::new(Agent::new(llm, vec![]));
    let config = AgentRunConfig::default();

    // 3. Create the Ralph Loop
    let ralph = RalphLoop::new(agent.clone(), config.clone(), progress_file);

    let task = "Build a complete web application with a database, API, and frontend UI.";
    println!("Task: {}", task);

    // 4. Run the Ralph Loop
    match ralph.run(task).await {
        Ok(_) => {
            println!("Ralph Loop completed successfully!");
            if let Ok(content) = std::fs::read_to_string(progress_file) {
                println!("Final Progress File Content:\n{}", content);
            }
        }
        Err(e) => {
            eprintln!("Ralph Loop failed: {}", e);
        }
    }

    Ok(())
}
