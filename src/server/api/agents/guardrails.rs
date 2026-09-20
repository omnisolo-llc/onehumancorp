//! Read-only policy preview. Inputs never change a runtime policy or approve a tool.
use ::server_common::Claims;
use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Extension},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
};
use omnisolo_builtin_agent::{
    guardrails::{ToolGuardrail, anthropic_hooks::AnthropicToolGater},
    types::ToolCall,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PreviewRequest {
    tool_name: String,
    project_trusted: bool,
    session_allowed_tools: Vec<String>,
    high_risk_tools: Vec<String>,
}

#[derive(Serialize)]
struct PreviewResponse {
    preview: bool,
    executed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(preview))
        .layer(DefaultBodyLimit::max(32_768))
}

fn response(status: StatusCode, decision: Result<String, String>) -> Response {
    let (result, error) = match decision {
        Ok(message) => (Some(message), None),
        Err(message) => (None, Some(message)),
    };
    (
        status,
        [("cache-control", "private, no-store")],
        Json(PreviewResponse {
            preview: true,
            executed: false,
            result,
            error,
        }),
    )
        .into_response()
}

fn valid_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 128
        && name
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"_-.:/".contains(&c))
}

fn evaluate(payload: PreviewRequest) -> Result<(), String> {
    if !valid_name(&payload.tool_name)
        || payload.session_allowed_tools.len() > 256
        || payload.high_risk_tools.len() > 256
        || !payload
            .session_allowed_tools
            .iter()
            .chain(&payload.high_risk_tools)
            .all(|name| valid_name(name))
    {
        return Err("Invalid policy preview: bounded tool identifiers are required".to_string());
    }
    // Reuse runtime semantics; an explicitly supplied empty allowlist means
    // unrestricted in that evaluator. A missing allowlist fails deserialization.
    // Read-only preview cannot persist this policy or execute any tool.
    let gater = AnthropicToolGater::new(
        payload.project_trusted,
        vec!["read_file".to_string(), "list_files".to_string()],
        payload.session_allowed_tools,
        payload.high_risk_tools,
    );
    gater.check_tool(&ToolCall {
        id: "policy-preview-no-execution".to_string(),
        name: payload.tool_name,
        arguments: serde_json::json!({}),
    })
}

async fn preview(
    claims: Option<Extension<Claims>>,
    Json(payload): Json<PreviewRequest>,
) -> Response {
    let Some(Extension(claims)) = claims else {
        return response(
            StatusCode::UNAUTHORIZED,
            Err("Authentication required".to_string()),
        );
    };
    if claims
        .organization_id
        .as_deref()
        .is_none_or(|tenant| tenant.trim().is_empty())
    {
        return response(
            StatusCode::UNAUTHORIZED,
            Err("Tenant context required".to_string()),
        );
    }
    if !claims
        .roles
        .iter()
        .any(|role| role == "ADMIN" || role == "OWNER")
    {
        return response(
            StatusCode::FORBIDDEN,
            Err("Owner or administrator required".to_string()),
        );
    }
    match evaluate(payload) {
        Ok(()) => response(StatusCode::OK, Ok("Validation passed successfully for this policy preview. No tool was executed or approved.".to_string())),
        Err(error) => response(StatusCode::BAD_REQUEST, Err(error)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::{Body, to_bytes},
        http::Request,
    };
    use tower::ServiceExt;

    fn policy() -> serde_json::Value {
        serde_json::json!({"toolName":"read_file", "projectTrusted":false,
            "sessionAllowedTools":["read_file"], "highRiskTools":["execute_bash"]})
    }

    fn claims(role: &str, tenant: Option<&str>) -> Claims {
        Claims {
            sub: "actor".to_string(),
            exp: i64::MAX,
            iat: 1,
            organization_id: tenant.map(str::to_string),
            username: "owner".to_string(),
            email: String::new(),
            roles: vec![role.to_string()],
            session_id: None,
            jti: "test".to_string(),
        }
    }

    async fn send(value: serde_json::Value, actor: Option<Claims>) -> Response {
        let mut request = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/json")
            .body(Body::from(value.to_string()))
            .unwrap();
        if let Some(actor) = actor {
            request.extensions_mut().insert(actor);
        }
        router::<()>().oneshot(request).await.unwrap()
    }

    #[tokio::test]
    async fn requires_authenticated_admin_and_tenant() {
        for (actor, expected) in [
            (None, StatusCode::UNAUTHORIZED),
            (Some(claims("ADMIN", None)), StatusCode::UNAUTHORIZED),
            (Some(claims("ADMIN", Some(" "))), StatusCode::UNAUTHORIZED),
            (
                Some(claims("MEMBER", Some("tenant-a"))),
                StatusCode::FORBIDDEN,
            ),
        ] {
            assert_eq!(send(policy(), actor).await.status(), expected);
        }
    }

    #[tokio::test]
    async fn successful_preview_is_explicitly_not_execution_or_approval() {
        for tenant in ["tenant-a", "tenant-b"] {
            let response = send(policy(), Some(claims("ADMIN", Some(tenant)))).await;
            assert_eq!(response.status(), StatusCode::OK);
            let bytes = to_bytes(response.into_body(), 4096).await.unwrap();
            let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(body["preview"], true);
            assert_eq!(body["executed"], false);
            assert!(
                body["result"]
                    .as_str()
                    .unwrap()
                    .contains("No tool was executed or approved")
            );
        }
    }

    #[test]
    fn retains_all_three_runtime_stages() {
        let mut value = policy();
        value["toolName"] = serde_json::json!("execute_bash");
        assert!(
            evaluate(serde_json::from_value(value.clone()).unwrap())
                .unwrap_err()
                .contains("Stage 1")
        );
        value["projectTrusted"] = serde_json::json!(true);
        assert!(
            evaluate(serde_json::from_value(value.clone()).unwrap())
                .unwrap_err()
                .contains("Stage 2")
        );
        value["sessionAllowedTools"] = serde_json::json!(["execute_bash"]);
        assert!(
            evaluate(serde_json::from_value(value).unwrap())
                .unwrap_err()
                .contains("Stage 3")
        );
    }

    #[tokio::test]
    async fn rejects_missing_policy_forged_approval_and_unbounded_names() {
        for key in [
            "toolName",
            "projectTrusted",
            "sessionAllowedTools",
            "highRiskTools",
        ] {
            let mut value = policy();
            value.as_object_mut().unwrap().remove(key);
            assert_eq!(
                send(value, Some(claims("ADMIN", Some("tenant-a"))))
                    .await
                    .status(),
                StatusCode::UNPROCESSABLE_ENTITY
            );
        }
        let mut value = policy();
        value["approved"] = serde_json::json!(true);
        assert_eq!(
            send(value, Some(claims("ADMIN", Some("tenant-a"))))
                .await
                .status(),
            StatusCode::UNPROCESSABLE_ENTITY
        );
        let mut value = policy();
        value["toolName"] = serde_json::json!("x".repeat(129));
        assert_eq!(
            send(value, Some(claims("ADMIN", Some("tenant-a"))))
                .await
                .status(),
            StatusCode::BAD_REQUEST
        );
    }

    #[test]
    fn explicit_empty_allowlist_keeps_existing_preview_semantics() {
        let mut value = policy();
        value["sessionAllowedTools"] = serde_json::json!([]);
        assert!(evaluate(serde_json::from_value(value).unwrap()).is_ok());
    }
}
