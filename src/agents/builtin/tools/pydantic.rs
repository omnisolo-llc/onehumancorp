use ohc_builtin_agent_core::types::ToolError;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::sync::Arc;
use super::ToolExecutor;

/// SOTA Harness Pattern: Pydantic-first tool schema validation.
/// If validation fails, it generates a precise ToolError::LlmRecoverable containing the serde validation error
/// so the LLM can self-correct its arguments.

#[async_trait::async_trait]
pub trait PydanticToolExecutor<T: DeserializeOwned + Send + Sync>: Send + Sync {
    async fn execute_typed(&self, args: T) -> Result<String, ToolError>;
}

pub struct PydanticAdapter<T, E> {
    executor: Arc<E>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned + Send + Sync, E: PydanticToolExecutor<T>> PydanticAdapter<T, E> {
    pub fn new(executor: E) -> Self {
        Self {
            executor: Arc::new(executor),
            _marker: std::marker::PhantomData,
        }
    }
    pub fn new_arc(executor: Arc<E>) -> Self {
        Self {
            executor,
            _marker: std::marker::PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<T: DeserializeOwned + Send + Sync, E: PydanticToolExecutor<T>> ToolExecutor for PydanticAdapter<T, E> {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        // Validation Errors fed back to LLM for self-correction
        let typed_args: T = match serde_json::from_value(args.clone()) {
            Ok(v) => v,
            Err(e) => {
                // Enhance the error message based on the Serde Error classification
                let detail = if e.is_data() {
                    format!("Semantic validation failed: {}", e)
                } else if e.is_syntax() {
                    format!("JSON syntax error at line {}, column {}: {}", e.line(), e.column(), e)
                } else if e.is_eof() {
                    format!("Incomplete JSON structure (unexpected EOF): {}", e)
                } else {
                    format!("{}", e)
                };

                // Add the original payload snippet for context
                let args_str = match serde_json::to_string(&args) {
                    Ok(s) => if s.len() > 100 { format!("{}...", &s[..100]) } else { s },
                    Err(_) => "<unprintable>".to_string(),
                };

                return Err(ToolError::LlmRecoverable(format!(
                    "Validation Error (Pydantic-first tool schema): Failed to parse arguments.\nReason: {}\nProvided arguments snippet: {}\nPlease strictly follow the tool's JSON schema and try again.",
                    detail, args_str
                )));
            }
        };

        self.executor.execute_typed(typed_args).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

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
    async fn test_pydantic_adapter_failure_invalid_type() {
        let adapter = PydanticAdapter::new(MyExecutor);
        let result = adapter.execute(serde_json::json!({ "foo": "test", "bar": "not a number" })).await;
        assert!(result.is_err());

        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
            assert!(msg.contains("Semantic validation failed"));
            assert!(msg.contains("invalid type"));
            assert!(msg.contains("not a number"));
        } else {
            panic!("Expected LlmRecoverable error with detailed semantic validation feedback");
        }
    }

    #[tokio::test]
    async fn test_pydantic_adapter_failure_missing_field() {
        let adapter = PydanticAdapter::new(MyExecutor);
        let result = adapter.execute(serde_json::json!({ "foo": "test" })).await;
        assert!(result.is_err());

        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
            assert!(msg.contains("missing field `bar`"));
        } else {
            panic!("Expected LlmRecoverable error about missing field");
        }
    }
}
