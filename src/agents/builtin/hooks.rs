use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, ToolCall};
use async_trait::async_trait;

/// Claude Code Agent SDK Architecture: Intercept and control agent behavior with hooks.
/// Hooks allow developers to inject custom logic at key points in the agent's orchestration loop.
#[async_trait]
pub trait AgentHook: Send + Sync + std::fmt::Debug {
    /// Called before the agent sends a request to the LLM.
    async fn before_turn(&self, _request: &mut ChatRequest) -> Result<(), String> {
        Ok(())
    }

    /// Called immediately after the LLM responds.
    async fn after_turn(&self, _response: &mut ChatResponse) -> Result<(), String> {
        Ok(())
    }

    /// Called right before a specific tool is executed.
    async fn before_tool(&self, _tool_call: &ToolCall) -> Result<(), String> {
        Ok(())
    }

    /// Called right after a tool finishes execution.
    async fn after_tool(&self, _tool_call: &ToolCall, _result: &mut Result<String, ohc_builtin_agent_core::types::ToolError>) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::Message;

    #[derive(Debug)]
    struct TestHook;
    #[async_trait]
    impl AgentHook for TestHook {
        async fn before_turn(&self, request: &mut ChatRequest) -> Result<(), String> {
            request.messages.push(Message::system("Hook injected context"));
            Ok(())
        }

        async fn after_turn(&self, response: &mut ChatResponse) -> Result<(), String> {
            response.message.content.push_str(" (hooked)");
            Ok(())
        }

        async fn before_tool(&self, tool_call: &ToolCall) -> Result<(), String> {
            if tool_call.name == "forbidden" {
                return Err("Tool blocked by hook".to_string());
            }
            Ok(())
        }

        async fn after_tool(&self, _tool_call: &ToolCall, result: &mut Result<String, ohc_builtin_agent_core::types::ToolError>) -> Result<(), String> {
            if let Ok(res) = result {
                res.push_str(" (hooked)");
            }
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_agent_hook_lifecycle() {
        let hook = TestHook;
        let mut req = ChatRequest {
            model: "test".to_string(),
            system: "sys".to_string(),
            messages: vec![],
            tools: vec![],
            max_tokens: 10,
            temperature: 0.0,
        };
        hook.before_turn(&mut req).await.unwrap();
        assert_eq!(req.messages.len(), 1);
        assert_eq!(req.messages[0].content, "Hook injected context");

        let mut resp = ChatResponse {
            message: Message::assistant("Hello"),
            usage: Default::default(),
            stop_reason: "stop".to_string(),
            response_id: None,
        };
        hook.after_turn(&mut resp).await.unwrap();
        assert_eq!(resp.message.content, "Hello (hooked)");
    }
}
