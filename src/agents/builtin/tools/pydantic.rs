use ohc_builtin_agent_core::types::ToolError;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::sync::Arc;
use super::ToolExecutor;

/// SOTA Harness Patterns (2025-2026): 6. Pydantic-first tool schema -> validation errors fed back to LLM for self-correction
/// If validation fails, it generates a precise ToolError::LlmRecoverable containing the serde validation error
/// so the LLM can self-correct its arguments.

#[async_trait::async_trait]
pub trait PydanticToolExecutor<T: DeserializeOwned + Send + Sync>: Send + Sync {
    async fn execute_typed(&self, args: T) -> Result<String, ToolError>;
}

pub struct PydanticAdapter<T, E> {
    custom_instruction: Option<String>,
    executor: Arc<E>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned + Send + Sync, E: PydanticToolExecutor<T>> PydanticAdapter<T, E> {
    pub fn new(executor: E) -> Self {
        Self {
            executor: Arc::new(executor),
            _marker: std::marker::PhantomData,
            custom_instruction: None,
        }
    }
    pub fn with_custom_instruction(mut self, instruction: impl Into<String>) -> Self {
        self.custom_instruction = Some(instruction.into());
        self
    }
    pub fn new_arc(executor: Arc<E>) -> Self {
        Self {
            executor,
            _marker: std::marker::PhantomData,
            custom_instruction: None,
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
                // Add the original payload snippet for context
                let args_str = match serde_json::to_string(&args) {
                    Ok(s) => if s.len() > 100 { format!("{}...", &s[..100]) } else { s },
                    Err(_) => "<unprintable>".to_string(),
                };

                return Err(ToolError::LlmRecoverable(
                    ohc_builtin_agent_core::types::format_pydantic_error(&e, Some(args_str.as_str()), self.custom_instruction.as_deref())
                ));
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

    #[tokio::test]
    async fn test_pydantic_adapter_failure_long_snippet() {
        let adapter = PydanticAdapter::new(MyExecutor);
        let long_string = "a".repeat(200);
        let result = adapter.execute(serde_json::json!({ "foo": long_string, "bar": "not a number" })).await;
        assert!(result.is_err());

        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
            assert!(msg.contains("Semantic validation failed"));
            assert!(msg.contains("...")); // Verifies the snippet truncation logic
            assert!(msg.len() < 500); // Ensures the error message didn't blow up
        } else {
            panic!("Expected LlmRecoverable error with truncated snippet");
        }
    }

    #[tokio::test]
    async fn test_pydantic_adapter_new_arc() {
        let adapter = PydanticAdapter::new_arc(Arc::new(MyExecutor));
        let result = adapter.execute(serde_json::json!({ "foo": "arc_test", "bar": 456 })).await.unwrap();
        assert_eq!(result, "arc_test-456");
    }
}

#[cfg(test)]
mod tests_custom {
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
    async fn test_pydantic_adapter_with_custom_instruction() {
        let adapter = PydanticAdapter::new(MyExecutor).with_custom_instruction("Please provide an integer for bar");
        let result = adapter.execute(serde_json::json!({ "foo": "test", "bar": "not a number" })).await;
        assert!(result.is_err());

        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
            assert!(msg.contains("Please provide an integer for bar"));
            assert!(!msg.contains("Please strictly follow the tool's JSON schema and try again."));
        } else {
            panic!("Expected LlmRecoverable error with custom instruction");
        }
    }
}
