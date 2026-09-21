use axum::{
    Json, Router,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
};
use omnisolo_builtin_agent::{
    guardrails::{ToolGuardrail, anthropic_hooks::AnthropicToolGater},
    types::ToolCall,
};
use serde::Deserialize;
use serde_json::json;

const MAX_TOOL_NAME_CHARS: usize = 128;
const MAX_TOOL_LIST_ITEMS: usize = 128;
const SAFE_TOOLS_FOR_UNTRUSTED: [&str; 2] = ["read_file", "list_files"];

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AnthropicGuardrailRequest {
    tool_name: String,
    project_trusted: bool,
    session_allowed_tools: Vec<String>,
    high_risk_tools: Vec<String>,
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new().route("/anthropic", post(evaluate_anthropic_guardrail))
}

fn valid_tool_name(value: &str) -> bool {
    let value = value.trim();
    !value.is_empty()
        && value.chars().count() <= MAX_TOOL_NAME_CHARS
        && value
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || "_-.:".contains(character))
}

fn valid_tool_list(values: &[String]) -> bool {
    values.len() <= MAX_TOOL_LIST_ITEMS && values.iter().all(|value| valid_tool_name(value))
}

fn evaluate(payload: AnthropicGuardrailRequest) -> Result<&'static str, (StatusCode, String)> {
    let tool_name = payload.tool_name.trim();
    if !valid_tool_name(tool_name)
        || !valid_tool_list(&payload.session_allowed_tools)
        || !valid_tool_list(&payload.high_risk_tools)
    {
        return Err((
            StatusCode::BAD_REQUEST,
            "invalid guardrail evaluation request".into(),
        ));
    }

    let gater = AnthropicToolGater::new(
        payload.project_trusted,
        SAFE_TOOLS_FOR_UNTRUSTED
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        payload.session_allowed_tools,
        payload.high_risk_tools,
    );
    let tool_call = ToolCall {
        id: "guardrail-evaluation".to_string(),
        name: tool_name.to_string(),
        arguments: json!({}),
    };

    gater
        .check_tool(&tool_call)
        .map(|_| "Validation passed successfully")
        .map_err(|message| (StatusCode::UNPROCESSABLE_ENTITY, message))
}

async fn evaluate_anthropic_guardrail(Json(payload): Json<AnthropicGuardrailRequest>) -> Response {
    match evaluate(payload) {
        Ok(result) => (StatusCode::OK, Json(json!({ "result": result }))).into_response(),
        Err((status, error)) => (status, Json(json!({ "error": error }))).into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(
        tool_name: &str,
        project_trusted: bool,
        session_allowed_tools: &[&str],
        high_risk_tools: &[&str],
    ) -> AnthropicGuardrailRequest {
        AnthropicGuardrailRequest {
            tool_name: tool_name.to_string(),
            project_trusted,
            session_allowed_tools: session_allowed_tools
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
            high_risk_tools: high_risk_tools
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
        }
    }

    #[test]
    fn untrusted_mutating_tool_fails_stage_one() {
        let error = evaluate(request(
            "execute_bash",
            false,
            &["read_file", "execute_bash"],
            &[],
        ))
        .unwrap_err();

        assert_eq!(error.0, StatusCode::UNPROCESSABLE_ENTITY);
        assert!(error.1.contains("Stage 1"));
    }

    #[test]
    fn trusted_disallowed_tool_fails_stage_two() {
        let error = evaluate(request("execute_bash", true, &["read_file"], &[])).unwrap_err();

        assert_eq!(error.0, StatusCode::UNPROCESSABLE_ENTITY);
        assert!(error.1.contains("Stage 2"));
    }

    #[test]
    fn high_risk_tool_fails_stage_three() {
        let error = evaluate(request(
            "execute_bash",
            true,
            &["read_file", "execute_bash"],
            &["execute_bash"],
        ))
        .unwrap_err();

        assert_eq!(error.0, StatusCode::UNPROCESSABLE_ENTITY);
        assert!(error.1.contains("Stage 3"));
    }

    #[test]
    fn safe_allowed_tool_passes_all_stages() {
        assert_eq!(
            evaluate(request(
                "read_file",
                true,
                &["read_file", "execute_bash"],
                &["execute_bash"],
            )),
            Ok("Validation passed successfully")
        );
    }

    #[test]
    fn invalid_tool_names_fail_closed() {
        for name in ["", " ", "../shell", "shell\nname"] {
            let error = evaluate(request(name, true, &[], &[])).unwrap_err();
            assert_eq!(error.0, StatusCode::BAD_REQUEST);
        }
    }
}
