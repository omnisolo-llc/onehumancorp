use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PydanticValidateRequest {
    pub tool_name: String,
    pub arguments: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct PydanticValidateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default)]
    pub is_recoverable: bool,
}

pub fn router<S>() -> Router<S> where S: Clone + Send + Sync + 'static {
    Router::new().route("/", post(validate_pydantic))
}

async fn validate_pydantic(
    Json(payload): Json<PydanticValidateRequest>,
) -> axum::response::Response {
use axum::response::IntoResponse;

use ohc_builtin_agent::types::format_pydantic_error;



    let mut err_msg = None;
    let mut is_recoverable = false;

    match payload.tool_name.as_str() {
        "TopicRetrieve" => {
            #[derive(Deserialize)]
            #[allow(dead_code)]
            struct Args { topic_name: String }
            if let Err(e) = serde_json::from_value::<Args>(payload.arguments.clone()) {
                err_msg = Some(format_pydantic_error(&e, Some(&payload.arguments.to_string()), None));
                is_recoverable = true;
            }
        }
        "TranscriptSearch" => {
            #[derive(Deserialize)]
            #[allow(dead_code)]
            struct Args { query: String }
            if let Err(e) = serde_json::from_value::<Args>(payload.arguments.clone()) {
                err_msg = Some(format_pydantic_error(&e, Some(&payload.arguments.to_string()), None));
                is_recoverable = true;
            }
        }
        "TopicWrite" => {
            #[derive(Deserialize)]
            #[allow(dead_code)]
            struct Args { topic_name: String, content: String }
            if let Err(e) = serde_json::from_value::<Args>(payload.arguments.clone()) {
                err_msg = Some(format_pydantic_error(&e, Some(&payload.arguments.to_string()), None));
                is_recoverable = true;
            }
        }
        "Bash" => {
            #[derive(Deserialize)]
            #[allow(dead_code)]
            struct Args { command: String }
            if let Err(e) = serde_json::from_value::<Args>(payload.arguments.clone()) {
                err_msg = Some(format_pydantic_error(&e, Some(&payload.arguments.to_string()), None));
                is_recoverable = true;
            }
        }
        _ => {
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(PydanticValidateResponse {
                    result: None,
                    error: Some("Unknown tool".to_string()),
                    is_recoverable: false,
                }),
            )
                .into_response();
        }
    }

    if let Some(err) = err_msg {
        (
            axum::http::StatusCode::BAD_REQUEST,
            Json(PydanticValidateResponse {
                result: None,
                error: Some(err),
                is_recoverable,
            }),
        )
            .into_response()
    } else {
        (
            axum::http::StatusCode::OK,
            Json(PydanticValidateResponse {
                result: Some("Tool payload validated successfully against the schema.".to_string()),
                error: None,
                is_recoverable: false,
            }),
        )
            .into_response()
    }
}
