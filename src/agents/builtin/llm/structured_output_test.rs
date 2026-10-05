use omnisolo_builtin_agent_core::{
    output_parser::{LlmClientForParser, parse_structured_output},
    types::{ChatRequest, ChatResponse, Message, Role, ToolCall, Usage},
};
use serde_json::{Value, json};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Obtain a real generated request without hand-copying the schema under test.
pub(super) async fn array_request() -> ChatRequest {
    struct Capture(Mutex<Option<ChatRequest>>);
    #[async_trait::async_trait]
    impl LlmClientForParser for Capture {
        async fn chat(
            &self,
            request: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            assert!(self.0.lock().await.replace(request).is_none());
            Ok(ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content: String::new(),
                    tool_calls: vec![ToolCall {
                        id: "capture".into(),
                        name: "structured_output".into(),
                        arguments: json!({"data": []}),
                    }],
                    tool_results: vec![],
                    response_id: None,
                    previous_response_id: None,
                },
                usage: Usage::default(),
                stop_reason: "tool_calls".into(),
                response_id: None,
            })
        }
    }
    let capture = Arc::new(Capture(Mutex::new(None)));
    let client = capture.clone() as Arc<dyn LlmClientForParser>;
    let request = ChatRequest {
        model: "schema-fixture".into(),
        system: "fixed fixture".into(),
        messages: vec![Message::user("plan")],
        tools: vec![],
        max_tokens: 100,
        temperature: 0.0,
    };
    let _: Vec<Value> = parse_structured_output(&client, request, 0).await.unwrap();
    capture.0.lock().await.take().unwrap()
}
