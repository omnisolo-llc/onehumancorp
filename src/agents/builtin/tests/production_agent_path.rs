use ohc_builtin_agent::agent::{Agent, AgentRunConfig};
use ohc_builtin_agent::llm::LlmClient;
use ohc_builtin_agent::types::{ChatRequest, ChatResponse, Message, Usage};
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct RecordingLlmClient {
    requests: Mutex<Vec<ChatRequest>>,
}

#[async_trait::async_trait]
impl LlmClient for RecordingLlmClient {
    async fn chat(
        &self,
        request: ChatRequest,
    ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        self.requests.lock().unwrap().push(request);
        Ok(ChatResponse {
            message: Message::assistant("The verified answer is 42."),
            usage: Usage {
                input_tokens: 120,
                output_tokens: 8,
                ..Default::default()
            },
            stop_reason: "stop".to_string(),
            response_id: Some("fixture-response".to_string()),
        })
    }
}

#[tokio::test]
async fn production_agent_completes_one_deterministic_turn() {
    let llm = Arc::new(RecordingLlmClient::default());
    let agent = Agent::new(llm.clone(), vec![]);
    let config = AgentRunConfig {
        max_iterations: 1,
        enable_lost_in_the_middle_prevention: false,
        ..Default::default()
    };
    let input = "What is six times seven?";

    let answer = agent
        .run_tao_orchestration_loop(&config, input, &[], &mut |_| {})
        .await
        .unwrap();

    assert_eq!(answer, "The verified answer is 42.");
    let requests = llm.requests.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert!(
        requests[0]
            .messages
            .last()
            .is_some_and(|message| message.content.contains(input))
    );
}
