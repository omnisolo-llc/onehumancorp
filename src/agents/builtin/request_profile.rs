use crate::types::ChatRequest;

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct RequestProfile {
    pub system_chars: usize,
    pub history_chars: usize,
    pub tool_result_chars: usize,
    pub tool_schema_chars: usize,
    pub message_count: usize,
    pub tool_count: usize,
    pub estimated_input_tokens: usize,
}

pub fn profile_request(request: &ChatRequest) -> RequestProfile {
    let system_chars = request.system.chars().count();
    let history_chars = request
        .messages
        .iter()
        .map(|message| message.content.chars().count())
        .sum();
    let tool_result_chars = request
        .messages
        .iter()
        .flat_map(|message| &message.tool_results)
        .map(|result| result.content.chars().count() + result.error.chars().count())
        .sum();
    let tool_schema_chars = request
        .tools
        .iter()
        .map(|tool| {
            tool.name.chars().count()
                + tool.description.chars().count()
                + serde_json::to_string(&tool.parameters)
                    .expect("serializing serde_json::Value cannot fail")
                    .chars()
                    .count()
        })
        .sum();
    let total_chars = system_chars + history_chars + tool_result_chars + tool_schema_chars;

    RequestProfile {
        system_chars,
        history_chars,
        tool_result_chars,
        tool_schema_chars,
        message_count: request.messages.len(),
        tool_count: request.tools.len(),
        estimated_input_tokens: total_chars.div_ceil(4),
    }
}

#[cfg(test)]
mod tests {
    use super::{RequestProfile, profile_request};
    use crate::types::{ChatRequest, Message, Role, ToolDefinition, ToolResult};

    #[test]
    fn attributes_unicode_characters_without_retaining_request_content() {
        let secret_system = "system-秘密";
        let secret_history = "history-🧪";
        let secret_result = "result-✅";
        let secret_error = "error-🚫";
        let secret_tool_name = "lookup-α";
        let secret_tool_description = "description-β";
        let secret_schema_value = "schema-γ";
        let parameters = serde_json::json!({
            "type": "object",
            "properties": {
                "query": { "type": "string", "description": secret_schema_value }
            }
        });
        let serialized_parameters = serde_json::to_string(&parameters).unwrap();
        let request = ChatRequest {
            model: "fixture-model".to_string(),
            system: secret_system.to_string(),
            messages: vec![
                Message::user(secret_history),
                Message {
                    role: Role::Tool,
                    content: String::new(),
                    tool_calls: vec![],
                    tool_results: vec![ToolResult {
                        tool_call_id: "fixture-call".to_string(),
                        content: secret_result.to_string(),
                        error: secret_error.to_string(),
                    }],
                    response_id: None,
                    previous_response_id: None,
                },
            ],
            tools: vec![ToolDefinition {
                name: secret_tool_name.to_string(),
                description: secret_tool_description.to_string(),
                parameters,
            }],
            max_tokens: 256,
            temperature: 0.0,
        };

        let profile = profile_request(&request);
        let tool_schema_chars = secret_tool_name.chars().count()
            + secret_tool_description.chars().count()
            + serialized_parameters.chars().count();
        let total_chars = secret_system.chars().count()
            + secret_history.chars().count()
            + secret_result.chars().count()
            + secret_error.chars().count()
            + tool_schema_chars;

        assert_eq!(
            profile,
            RequestProfile {
                system_chars: secret_system.chars().count(),
                history_chars: secret_history.chars().count(),
                tool_result_chars: secret_result.chars().count() + secret_error.chars().count(),
                tool_schema_chars,
                message_count: 2,
                tool_count: 1,
                estimated_input_tokens: total_chars.div_ceil(4),
            }
        );

        let serialized_profile = serde_json::to_string(&profile).unwrap();
        for fixture_content in [
            secret_system,
            secret_history,
            secret_result,
            secret_error,
            secret_tool_name,
            secret_tool_description,
            secret_schema_value,
        ] {
            assert!(!serialized_profile.contains(fixture_content));
        }
    }
}
