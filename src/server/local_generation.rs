//! Observed Ollama generation, separate from legacy text-only helpers.
//! Quantities come from provider fields, never text length. This local path does
//! not price/rebill customer inference or invent a provider request identity.
use ::server_harness::middleware::usage_ledger::TokenCounts;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ObservedGeneration {
    pub text: String,
    pub model: String,
    pub counts: Option<TokenCounts>,
    pub duration_ns: Option<u64>,
    pub stop_reason: String,
}

#[derive(Deserialize)]
struct WireGeneration {
    response: String,
    model: String,
    done: bool,
    done_reason: Option<String>,
    prompt_eval_count: Option<i64>,
    eval_count: Option<i64>,
    prompt_eval_cached_count: Option<i64>,
    total_duration: Option<u64>,
}

pub fn parse_generation(body: &[u8], expected_model: &str) -> Result<ObservedGeneration, String> {
    let value: WireGeneration =
        serde_json::from_slice(body).map_err(|_| "Local model response is malformed")?;
    if !value.done || value.response.trim().is_empty() || value.response.len() > 8 * 1024 * 1024 {
        return Err("Local model did not return a completed nonempty response".into());
    }
    if value.model != expected_model && value.model != format!("{expected_model}:latest") {
        return Err("Local model response does not match the requested model".into());
    }
    let cached = value.prompt_eval_cached_count.unwrap_or(0);
    let counts = match (value.prompt_eval_count, value.eval_count) {
        (Some(input), Some(output))
            if input >= 0 && output >= 0 && cached >= 0 && cached <= input =>
        {
            Some(TokenCounts {
                input,
                output,
                cached_input: cached,
            })
        }
        _ => None,
    };
    Ok(ObservedGeneration {
        text: value.response,
        model: value.model,
        counts,
        duration_ns: value.total_duration,
        stop_reason: value.done_reason.unwrap_or_else(|| "unknown".into()),
    })
}

pub async fn generate(
    endpoint: &str,
    model: &str,
    prompt: &str,
    maximum_output: i32,
) -> Result<ObservedGeneration, String> {
    if model.trim().is_empty()
        || model.len() > 255
        || model.chars().any(char::is_control)
        || prompt.trim().is_empty()
        || prompt.len() > 1_048_576
        || !(1..=1_000_000).contains(&maximum_output)
    {
        return Err("A model, bounded prompt and positive output limit are required".into());
    }
    let url = reqwest::Url::parse(endpoint).map_err(|_| "Invalid local model endpoint")?;
    let loopback = url
        .host_str()
        .is_some_and(|host| matches!(host, "localhost" | "127.0.0.1" | "[::1]"));
    if (url.scheme() != "https" && !(url.scheme() == "http" && loopback))
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
    {
        return Err("Model endpoint requires HTTPS or an explicit loopback service without embedded credentials".into());
    }
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(60))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|_| "Cannot configure local model transport")?;
    // One request only: an unknown response may already have consumed resources.
    // A caller retry must be an explicit new attempt, not invisible extra work.
    let mut response = client
        .post(url)
        .json(&serde_json::json!({"model":model,"prompt":prompt,
        "stream":false,"options":{"num_predict":maximum_output}}))
        .send()
        .await
        .map_err(|_| "Local model outcome is unknown; no automatic retry was made")?;
    if !response.status().is_success() {
        return Err(format!(
            "Local model returned HTTP {}",
            response.status().as_u16()
        ));
    }
    let mut body = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| "Local model response was interrupted; usage is unknown")?
    {
        if body.len().saturating_add(chunk.len()) > 10 * 1024 * 1024 {
            return Err("Local model response exceeded the safety limit".into());
        }
        body.extend_from_slice(&chunk);
    }
    parse_generation(&body, model)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn actual_provider_counts_and_cache_are_preserved_without_guesses() {
        let result = parse_generation(br#"{"model":"model:latest","response":"result","done":true,"done_reason":"stop","prompt_eval_count":23,"eval_count":4,"prompt_eval_cached_count":7,"total_duration":100}"#, "model").unwrap();
        assert_eq!(
            result.counts,
            Some(TokenCounts {
                input: 23,
                output: 4,
                cached_input: 7
            })
        );
        assert_eq!(result.duration_ns, Some(100));
        assert_eq!(result.stop_reason, "stop");
        for counts in [
            serde_json::json!({}),
            serde_json::json!({"prompt_eval_count":3,"eval_count":-1}),
            serde_json::json!({"prompt_eval_count":3,"eval_count":1,"prompt_eval_cached_count":4}),
        ] {
            let mut value =
                serde_json::json!({"model":"model","response":"not a token counter","done":true});
            value
                .as_object_mut()
                .unwrap()
                .extend(counts.as_object().unwrap().clone());
            assert!(
                parse_generation(&serde_json::to_vec(&value).unwrap(), "model")
                    .unwrap()
                    .counts
                    .is_none()
            );
        }
    }
    #[test]
    fn incomplete_foreign_and_malformed_results_do_not_look_completed() {
        for body in [
            br#"{"model":"other","response":"hello","done":true}"#.as_slice(),
            br#"{"model":"model","response":"hello","done":false}"#,
            br#"{"model":"model","response":"","done":true}"#,
            b"not json",
        ] {
            assert!(parse_generation(body, "model").is_err());
        }
    }
    #[tokio::test]
    async fn actual_transport_carries_bound_and_uses_provider_usage() {
        use axum::{Json, Router, routing::post};
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}/api/generate", listener.local_addr().unwrap());
        let app = Router::new().route("/api/generate", post(|Json(request): Json<serde_json::Value>| async move {
            assert_eq!(request["options"]["num_predict"], 125);
            assert_eq!(request["stream"], false);
            Json(serde_json::json!({"model":"model","response":"local contract fixture","done":true,
                "prompt_eval_count":9,"eval_count":5}))
        }));
        let task = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let result = generate(&url, "model", "contract input", 125)
            .await
            .unwrap();
        task.abort();
        assert_eq!(result.counts.unwrap().output, 5);
        assert!(
            generate("http://example.invalid/generate", "model", "input", 125)
                .await
                .is_err()
        );
    }
}
