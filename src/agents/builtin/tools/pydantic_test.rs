use ohc_builtin_agent_core::types::ToolError;
use serde::Deserialize;

use ohc_builtin_agent_tools::pydantic::{PydanticToolExecutor, PydanticAdapter};
use ohc_builtin_agent_tools::ToolExecutor;

#[derive(Deserialize)]
struct MyArgs {
    foo: String,
    bar: u32,
}

struct MyExecutor;

#[async_trait::async_trait]
impl PydanticToolExecutor<MyArgs> for MyExecutor {
    async fn execute_typed(&self, args: MyArgs) -> Result<String, ToolError> {
        Ok(format!("{}-{}", args.foo, args.bar))
    }
}

#[tokio::test]
async fn test_pydantic_adapter_success() {
    let adapter = PydanticAdapter::new(MyExecutor);
    let result = adapter.execute(serde_json::json!({ "foo": "test", "bar": 123 })).await.unwrap();
    assert_eq!(result, "test-123");
}

#[tokio::test]
async fn test_pydantic_adapter_failure() {
    let adapter = PydanticAdapter::new(MyExecutor);
    let result = adapter.execute(serde_json::json!({ "foo": "test", "bar": "not a number" })).await;
    assert!(result.is_err());

    if let Err(ToolError::LlmRecoverable(msg)) = result {
        assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
    } else {
        panic!("Expected LlmRecoverable error for self-correction");
    }
}
