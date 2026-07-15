use ohc_builtin_agent_core::types::ToolError;
use serde::Deserialize;

use ohc_builtin_agent_tools::pydantic::{PydanticToolExecutor, PydanticAdapter};
use ohc_builtin_agent_tools::ToolExecutor;

#[derive(Deserialize)]
struct NestedArgs {
    nested_str: String,
}

#[derive(Deserialize)]
struct MyArgs {
    foo: String,
    bar: u32,
    nested: Option<NestedArgs>,
}

struct MyExecutor;

#[async_trait::async_trait]
impl PydanticToolExecutor<MyArgs> for MyExecutor {
    async fn execute_typed(&self, args: MyArgs) -> Result<String, ToolError> {
        if let Some(n) = args.nested {
            Ok(format!("{}-{}-{}", args.foo, args.bar, n.nested_str))
        } else {
            Ok(format!("{}-{}", args.foo, args.bar))
        }
    }
}

#[tokio::test]
async fn test_pydantic_adapter_success() {
    let adapter = PydanticAdapter::new(MyExecutor);
    let result = adapter.execute(serde_json::json!({ "foo": "test", "bar": 123 })).await.unwrap();
    assert_eq!(result, "test-123");
}

#[tokio::test]
async fn test_pydantic_adapter_failure_missing_field() {
    let adapter = PydanticAdapter::new(MyExecutor);
    let result = adapter.execute(serde_json::json!({ "foo": "test" })).await;
    assert!(result.is_err());

    if let Err(ToolError::LlmRecoverable(msg)) = result {
        assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
        assert!(msg.contains("The required field 'bar' is missing. Please provide it."));
    } else {
        panic!("Expected LlmRecoverable error for self-correction");
    }
}

#[tokio::test]
async fn test_pydantic_adapter_failure_invalid_type() {
    let adapter = PydanticAdapter::new(MyExecutor);
    let result = adapter.execute(serde_json::json!({ "foo": "test", "bar": "not a number" })).await;
    assert!(result.is_err());

    if let Err(ToolError::LlmRecoverable(msg)) = result {
        assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
        assert!(msg.contains("There is a type mismatch. Ensure strings are quoted, numbers are not quoted, and arrays/objects are formatted correctly as JSON."));
    } else {
        panic!("Expected LlmRecoverable error for self-correction");
    }
}

#[tokio::test]
async fn test_pydantic_adapter_failure_nested_missing_field() {
    let adapter = PydanticAdapter::new(MyExecutor);
    let result = adapter.execute(serde_json::json!({ "foo": "test", "bar": 123, "nested": {} })).await;
    assert!(result.is_err());

    if let Err(ToolError::LlmRecoverable(msg)) = result {
        assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
        assert!(msg.contains("The required field 'nested_str' is missing. Please provide it."));
    } else {
        panic!("Expected LlmRecoverable error for self-correction");
    }
}

#[tokio::test]
async fn test_pydantic_adapter_failure_exact_line_col() {
    let adapter = PydanticAdapter::new(MyExecutor);
    let result = adapter.execute(serde_json::json!({ "foo": "test", "bar": "not a number" })).await;
    assert!(result.is_err());

    if let Err(ToolError::LlmRecoverable(msg)) = result {
        assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
        // The error should now be able to pinpoint the line and column, which wasn't possible with from_value
        // Because "not a number" triggers a syntax/type error inside the string parsing.
        // It should contain exact coordinate information instead of (line 0, col 0)
        assert!(msg.contains("line 1"));
        assert!(msg.contains("column 34"));
    } else {
        panic!("Expected LlmRecoverable error for self-correction");
    }
}
