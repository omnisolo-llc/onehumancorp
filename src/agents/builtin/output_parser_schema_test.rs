use super::*;
use crate::types::{Role, ToolCall, ToolDefinition, Usage};
use serde::Deserialize;
use serde_json::{Value, json};
use tokio::sync::Mutex;

#[derive(Debug, Deserialize, JsonSchema, PartialEq)]
struct PathRow {
    count: u64,
}

#[derive(Debug, Deserialize, JsonSchema, PartialEq)]
struct PathRows {
    items: Vec<PathRow>,
}

fn diagnostic_object(error: &str) -> Value {
    let reason = error.split_once("Reason: ").unwrap().1;
    let mut objects = serde_json::Deserializer::from_str(reason).into_iter::<Value>();
    objects.next().unwrap().unwrap()[0].clone()
}

fn assert_original_reason_and_input<T: DeserializeOwned + std::fmt::Debug>(
    data: &Value,
    observed: &str,
) {
    let original_error = T::deserialize(data).unwrap_err();
    let input = serde_json::to_string(data).unwrap();
    let original = crate::types::format_pydantic_error(
        &original_error,
        Some(&input),
        Some(
            "Please strictly follow the Pydantic-first tool schema and try again. Also ensure all enum variants are exact string matches.",
        ),
    );
    let mut before = diagnostic_object(&original);
    let mut after = diagnostic_object(observed);
    before.as_object_mut().unwrap().remove("loc");
    after.as_object_mut().unwrap().remove("loc");
    assert_eq!(after, before, "only the diagnostic location may change");
}

#[test]
fn serde_path_nested_array_location_preserves_reason_and_input_for_both_parsers() {
    let data = json!({"items":[{"count":1},{"count":"bad-value"}]});
    let msg = native(data.clone());
    let advanced = AdvancedPydanticOutputParser::<PathRows>::new()
        .parse_message(&msg)
        .unwrap_err();
    let structured = StructuredOutputParser::<PathRows>::new()
        .parse_message(&msg)
        .unwrap_err();
    for error in [advanced, structured] {
        assert_eq!(
            diagnostic_object(&error)["loc"],
            json!(["data", "items", 1, "count"])
        );
        assert_original_reason_and_input::<PathRows>(&data, &error);
    }
}

#[test]
fn serde_path_missing_field_reports_containing_object_without_guessing_a_leaf() {
    let data = json!({"items":[{}]});
    let error = AdvancedPydanticOutputParser::<PathRows>::new()
        .parse_message(&native(data.clone()))
        .unwrap_err();
    assert_eq!(
        diagnostic_object(&error)["loc"],
        json!(["data", "items", 0])
    );
    assert!(error.contains("missing field `count`"));
    assert_original_reason_and_input::<PathRows>(&data, &error);
}

#[test]
fn serde_path_root_type_error_has_only_the_envelope_root() {
    let data = json!(false);
    let error = StructuredOutputParser::<PathRows>::new()
        .parse_message(&native(data.clone()))
        .unwrap_err();
    assert_eq!(diagnostic_object(&error)["loc"], json!(["data"]));
    assert_original_reason_and_input::<PathRows>(&data, &error);
}

#[test]
fn serde_path_enum_reason_remains_actionable() {
    let data = json!({"mode":"not-a-mode","choice":1,"code":"A"});
    let error = AdvancedPydanticOutputParser::<Shapes>::new()
        .parse_message(&native(data.clone()))
        .unwrap_err();
    assert_eq!(diagnostic_object(&error)["loc"], json!(["data", "mode"]));
    assert!(error.contains("unknown variant"));
    assert!(error.contains("read_only"));
    assert_original_reason_and_input::<Shapes>(&data, &error);
}

#[test]
fn serde_path_map_segments_are_json_values_and_restrict_control_or_oversized_keys() {
    use std::collections::BTreeMap;
    for (key, expected) in [
        (
            "field.with[punctuation]\"\\".to_owned(),
            "field.with[punctuation]\"\\".to_owned(),
        ),
        ("étiquette".to_owned(), "étiquette".to_owned()),
        ("x".repeat(128), "x".repeat(128)),
        ("é".repeat(64), "é".repeat(64)),
        ("é".repeat(65), "<redacted>".to_owned()),
        ("secret\nkey".to_owned(), "<redacted>".to_owned()),
        ("secret\u{0}key".to_owned(), "<redacted>".to_owned()),
        ("x".repeat(129), "<redacted>".to_owned()),
    ] {
        let data = json!({key.clone(): "bad-value"});
        let error = StructuredOutputParser::<BTreeMap<String, u64>>::new()
            .parse_message(&native(data.clone()))
            .unwrap_err();
        assert_eq!(diagnostic_object(&error)["loc"], json!(["data", expected]));
        // Input/reason already belong to the existing same-model feedback contract.
        assert_original_reason_and_input::<BTreeMap<String, u64>>(&data, &error);
    }
}

#[test]
fn serde_path_external_enum_keeps_variant_location_and_custom_errors_keep_reason() {
    #[derive(Debug, Deserialize)]
    enum External {
        Selected(PathRow),
    }
    let valid = StructuredOutputParser::<External>::new()
        .parse_message(&native(json!({"Selected":{"count":9}})))
        .unwrap();
    let External::Selected(row) = valid;
    assert_eq!(row.count, 9);
    let data = json!({"Selected":{"count":"bad-value"}});
    let error = StructuredOutputParser::<External>::new()
        .parse_message(&native(data.clone()))
        .unwrap_err();
    assert_eq!(
        diagnostic_object(&error)["loc"],
        json!(["data", "Selected", "count"])
    );
    assert_original_reason_and_input::<External>(&data, &error);

    fn reject<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
        let value = String::deserialize(deserializer)?;
        Err(serde::de::Error::custom(format!(
            "custom rejection: {value}"
        )))
    }
    #[derive(Debug, Deserialize)]
    struct Custom {
        #[serde(deserialize_with = "reject")]
        payload: String,
    }
    let data = json!({"payload":"existing-feedback-canary"});
    let error = StructuredOutputParser::<Custom>::new()
        .parse_message(&native(data.clone()))
        .map(|value| value.payload)
        .unwrap_err();
    assert_eq!(diagnostic_object(&error)["loc"], json!(["data", "payload"]));
    assert_original_reason_and_input::<Custom>(&data, &error);
}

#[tokio::test]
async fn serde_path_terminal_error_remains_recoverable_and_corrections_stay_capped() {
    let data = json!({"items":[{"count":"bad-value"}]});
    let client = client(vec![native(data.clone()); 3]);
    let result: Result<PathRows, _> = parse(&client, request(), usize::MAX).await;
    let Err(ToolError::LlmRecoverable(error)) = result else {
        panic!("expected the existing recoverable terminal category");
    };
    assert_eq!(client.requests.lock().await.len(), 3);
    assert_eq!(
        diagnostic_object(&error)["loc"],
        json!(["data", "items", 0, "count"])
    );
    assert_original_reason_and_input::<PathRows>(&data, &error);
}

#[test]
fn serde_path_deep_location_is_explicitly_truncated() {
    #[derive(Debug, Deserialize)]
    struct Branch(Vec<Branch>);
    let empty = AdvancedPydanticOutputParser::<Branch>::new()
        .parse_message(&native(json!([])))
        .unwrap();
    assert!(empty.0.is_empty());
    let mut data = json!("not-an-array");
    for _ in 0..40 {
        data = json!([data]);
    }
    let error = AdvancedPydanticOutputParser::<Branch>::new()
        .parse_message(&native(data.clone()))
        .unwrap_err();
    let mut expected = vec![json!("data")];
    expected.extend(std::iter::repeat_n(json!(0), 32));
    expected.push(json!("<truncated>"));
    assert_eq!(diagnostic_object(&error)["loc"], Value::Array(expected));
    assert_original_reason_and_input::<Branch>(&data, &error);
}

#[test]
fn serde_path_valid_nested_data_preserves_serde_result() {
    let data = json!({"items":[{"count":0},{"count":u64::MAX}],"additive":true});
    let expected: PathRows = serde_json::from_value(data.clone()).unwrap();
    assert_eq!(
        StructuredOutputParser::<PathRows>::new()
            .parse_message(&native(data))
            .unwrap(),
        expected
    );
}

#[test]
fn serde_path_plain_and_trailing_json_text_cannot_bypass_native_value_contract() {
    for text in [r#"{"items":[]}"#, r#"{"items":[]} trailing"#] {
        let parser = AdvancedPydanticOutputParser::<PathRows>::new();
        assert!(
            parser
                .parse_message(&Message::assistant(text))
                .unwrap_err()
                .contains("Expected native tool_calls")
        );
        let error = parser.parse_message(&native(json!(text))).unwrap_err();
        assert_eq!(diagnostic_object(&error)["type"], "type_error");
    }
}

#[tokio::test]
async fn serde_path_actual_correction_feedback_keeps_ids_reason_and_request_limit() {
    let failed_data = json!({"items":[{"count":"bad-value"}]});
    let mut failed = native(failed_data.clone());
    failed.tool_calls[0].id = "owned-call-id".into();
    failed.response_id = Some("owned-response-id".into());
    let client = client(vec![failed, native(json!({"items":[{"count":7}]}))]);
    let parsed: PathRows = parse(&client, request(), 2).await.unwrap();
    assert_eq!(parsed.items[0].count, 7);
    let requests = client.requests.lock().await;
    assert_eq!(requests.len(), 2);
    let feedback = requests[1].messages.last().unwrap();
    assert_eq!(
        feedback.previous_response_id.as_deref(),
        Some("owned-response-id")
    );
    assert_eq!(feedback.tool_results[0].tool_call_id, "owned-call-id");
    assert_eq!(
        diagnostic_object(&feedback.tool_results[0].error)["loc"],
        json!(["data", "items", 0, "count"])
    );
    assert_original_reason_and_input::<PathRows>(&failed_data, &feedback.tool_results[0].error);
}

struct CaptureClient {
    requests: Mutex<Vec<ChatRequest>>,
    responses: Mutex<Vec<Message>>,
}

#[async_trait]
impl LlmClientForParser for CaptureClient {
    async fn chat(
        &self,
        req: ChatRequest,
    ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        self.requests.lock().await.push(req);
        let mut responses = self.responses.lock().await;
        assert!(!responses.is_empty(), "unexpected extra model request");
        Ok(ChatResponse {
            message: responses.remove(0),
            usage: Usage::default(),
            stop_reason: "tool_calls".to_owned(),
            response_id: None,
        })
    }
}

fn client(messages: Vec<Message>) -> Arc<CaptureClient> {
    Arc::new(CaptureClient {
        requests: Mutex::new(vec![]),
        responses: Mutex::new(messages),
    })
}

fn request() -> ChatRequest {
    ChatRequest {
        model: "test".into(),
        system: String::new(),
        messages: vec![],
        tools: vec![],
        max_tokens: 100,
        temperature: 0.0,
    }
}

fn native(data: Value) -> Message {
    Message {
        role: Role::Assistant,
        content: String::new(),
        tool_calls: vec![ToolCall {
            id: "call_1".into(),
            name: "structured_output".into(),
            arguments: json!({"data": data}),
        }],
        tool_results: vec![],
        response_id: None,
        previous_response_id: None,
    }
}

async fn parse<T: DeserializeOwned + JsonSchema + Send + Sync>(
    client: &Arc<CaptureClient>,
    request: ChatRequest,
    retries: usize,
) -> Result<T, ToolError> {
    parse_structured_output(
        &(client.clone() as Arc<dyn LlmClientForParser>),
        request,
        retries,
    )
    .await
}

#[derive(Debug, Deserialize, JsonSchema, PartialEq)]
struct ResultDto {
    result: String,
}

#[tokio::test]
async fn actual_vec_result_is_advertised_as_array_and_succeeds_once() {
    let data = json!([{"tool":"lookup","args":{"nested":[1,true,null]}}]);
    let client = client(vec![native(data.clone())]);
    let parsed: Vec<Value> = parse(&client, request(), 2).await.unwrap();
    assert_eq!(json!(parsed), data);
    let requests = client.requests.lock().await;
    assert_eq!(requests.len(), 1);
    let schema = &requests[0].tools[0].parameters;
    assert_eq!(schema["type"], "object");
    assert_eq!(schema["required"], json!(["data"]));
    assert_eq!(schema["properties"]["data"]["type"], "array");
    assert!(
        schema["properties"]["data"]["items"] == json!({})
            || schema["properties"]["data"]["items"] == json!(true)
    );
}

#[tokio::test]
async fn object_result_advertises_required_fields_without_changing_unrelated_tools() {
    let unrelated = ToolDefinition {
        name: "lookup".into(),
        description: "preserve".into(),
        parameters: json!({"type":"object","properties":{"x":{"type":"string"}}}),
    };
    let mut req = request();
    req.tools.push(unrelated.clone());
    let client = client(vec![native(json!({"result":"done","additive":true}))]);
    let value: ResultDto = parse(&client, req, 0).await.unwrap();
    assert_eq!(value.result, "done");
    let requests = client.requests.lock().await;
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].tools[0].name, unrelated.name);
    assert_eq!(requests[0].tools[0].description, unrelated.description);
    assert_eq!(requests[0].tools[0].parameters, unrelated.parameters);
    let data = &requests[0].tools[1].parameters["properties"]["data"];
    assert_eq!(data["type"], "object");
    assert_eq!(data["properties"]["result"]["type"], "string");
    assert_eq!(data["required"], json!(["result"]));
    assert_ne!(data["additionalProperties"], json!(false));
}

#[tokio::test]
async fn conflicting_reserved_tool_fails_before_model_io() {
    let mut req = request();
    req.tools.push(ToolDefinition {
        name: "structured_output".into(),
        description: "conflicting".into(),
        parameters: json!({"type":"string"}),
    });
    let client = client(vec![native(json!({"result":"ignored"}))]);
    let result: Result<ResultDto, _> = parse(&client, req, 2).await;
    assert!(result.is_err());
    assert!(client.requests.lock().await.is_empty());
}

#[tokio::test]
async fn identical_single_reserved_tool_is_preserved_but_duplicates_fail_before_io() {
    let first = client(vec![native(json!({"result":"first"}))]);
    let _: ResultDto = parse(&first, request(), 0).await.unwrap();
    let canonical = first.requests.lock().await[0].tools[0].clone();
    let mut req = request();
    req.tools.push(canonical.clone());
    let second = client(vec![native(json!({"result":"second"}))]);
    let _: ResultDto = parse(&second, req.clone(), 0).await.unwrap();
    let requests = second.requests.lock().await;
    assert_eq!(requests[0].tools.len(), 1);
    assert_eq!(requests[0].tools[0].name, canonical.name);
    assert_eq!(requests[0].tools[0].description, canonical.description);
    assert_eq!(requests[0].tools[0].parameters, canonical.parameters);
    drop(requests);
    req.tools.push(canonical);
    let duplicate = client(vec![native(json!({"result":"must not request"}))]);
    let result: Result<ResultDto, _> = parse(&duplicate, req, 2).await;
    assert!(result.is_err());
    assert!(duplicate.requests.lock().await.is_empty());
}

#[tokio::test]
async fn typed_schema_stays_identical_across_native_correction() {
    let client = client(vec![
        native(json!({"result":1})),
        native(json!({"result":"fixed"})),
    ]);
    let value: ResultDto = parse(&client, request(), 2).await.unwrap();
    assert_eq!(value.result, "fixed");
    let requests = client.requests.lock().await;
    assert_eq!(requests.len(), 2);
    assert_eq!(
        requests[0].tools[0].parameters,
        requests[1].tools[0].parameters
    );
    assert_eq!(
        requests[0].tools[0].parameters["properties"]["data"]["properties"]["result"]["type"],
        "string"
    );
    assert!(
        requests[1].messages.last().unwrap().tool_results[0]
            .error
            .contains("Validation Error")
    );
}

#[tokio::test]
async fn plain_text_and_missing_data_never_bypass_native_contract_or_retry_limit() {
    let mut missing = native(Value::Null);
    missing.tool_calls[0].arguments = json!({});
    let client = client(vec![
        Message::assistant("```json\n{\"result\":\"text\"}\n```"),
        missing,
        native(json!({"result":false})),
    ]);
    let result: Result<ResultDto, _> = parse(&client, request(), usize::MAX).await;
    assert!(matches!(result, Err(ToolError::LlmRecoverable(_))));
    assert_eq!(client.requests.lock().await.len(), 3);
}

#[derive(Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
enum Mode {
    ReadOnly,
    Write,
}

#[derive(Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(untagged)]
enum Choice {
    Text(String),
    Count(u32),
}

#[derive(Debug, Deserialize, JsonSchema, PartialEq)]
struct FlatFields {
    code: String,
}

#[derive(Debug, Deserialize, JsonSchema, PartialEq)]
struct Shapes {
    mode: Mode,
    choice: Choice,
    #[serde(rename = "wire_name")]
    optional_name: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(flatten)]
    flat: FlatFields,
}

#[tokio::test]
async fn serde_defaults_rename_nullable_enum_untagged_and_flatten_remain_authoritative() {
    let input = json!({"mode":"read_only","choice":3,"code":"A","additive":true});
    let expected: Shapes = serde_json::from_value(input.clone()).unwrap();
    let client = client(vec![native(input)]);
    let observed: Shapes = parse(&client, request(), 0).await.unwrap();
    assert_eq!(observed, expected);
    assert_eq!(observed.optional_name, None);
    assert!(observed.tags.is_empty());
    let requests = client.requests.lock().await;
    let data = &requests[0].tools[0].parameters["properties"]["data"];
    assert_eq!(
        data["properties"]["mode"]["enum"],
        json!(["read_only", "write"])
    );
    assert_eq!(
        data["properties"]["choice"]["anyOf"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert!(data["properties"].get("wire_name").is_some());
    assert!(data["properties"].get("optional_name").is_none());
    assert!(
        data["properties"]["wire_name"]["type"]
            .as_array()
            .unwrap()
            .contains(&json!("null"))
    );
    let required = data["required"].as_array().unwrap();
    for key in ["mode", "choice", "code"] {
        assert!(required.contains(&json!(key)));
    }
    for key in ["wire_name", "tags"] {
        assert!(!required.contains(&json!(key)));
    }
}

#[derive(Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
struct Strict {
    result: String,
}

#[tokio::test]
async fn deny_unknown_fields_is_advertised_and_enforced_by_serde() {
    let client = client(vec![native(json!({"result":"valid","unexpected":true}))]);
    let result: Result<Strict, _> = parse(&client, request(), 0).await;
    assert!(matches!(result, Err(ToolError::LlmRecoverable(_))));
    let requests = client.requests.lock().await;
    assert_eq!(requests.len(), 1);
    assert_eq!(
        requests[0].tools[0].parameters["properties"]["data"]["additionalProperties"],
        false
    );
}

#[tokio::test]
async fn nullable_result_still_requires_explicit_outer_data_key() {
    let client = client(vec![native(Value::Null)]);
    let result: Option<ResultDto> = parse(&client, request(), 0).await.unwrap();
    assert_eq!(result, None);
    let requests = client.requests.lock().await;
    assert_eq!(requests[0].tools[0].parameters["required"], json!(["data"]));
}

#[derive(Debug, Deserialize, JsonSchema, PartialEq)]
struct RecursiveNode {
    name: String,
    #[serde(default)]
    children: Vec<RecursiveNode>,
}

#[tokio::test]
async fn recursive_references_resolve_from_complete_tool_envelope() {
    fn check_references(value: &Value, root: &Value, count: &mut usize) {
        match value {
            Value::Object(object) => {
                if let Some(reference) = object.get("$ref") {
                    let pointer = reference.as_str().unwrap().strip_prefix('#').unwrap();
                    assert!(
                        root.pointer(pointer).is_some(),
                        "unresolved schema reference: {reference}"
                    );
                    *count += 1;
                }
                for child in object.values() {
                    check_references(child, root, count);
                }
            }
            Value::Array(array) => {
                for child in array {
                    check_references(child, root, count);
                }
            }
            _ => {}
        }
    }
    let input = json!({"name":"parent","children":[{"name":"child"}]});
    let client = client(vec![native(input.clone())]);
    let value: RecursiveNode = parse(&client, request(), 0).await.unwrap();
    assert_eq!(value, serde_json::from_value(input).unwrap());
    let requests = client.requests.lock().await;
    let schema = &requests[0].tools[0].parameters;
    let mut references = 0;
    check_references(schema, schema, &mut references);
    assert!(references > 0);
    assert!(schema.get("$schema").is_none());
}
