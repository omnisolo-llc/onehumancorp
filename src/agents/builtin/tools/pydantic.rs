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
        let typed_args: T = serde_json::from_value(args.clone()).map_err(|e| {
            ToolError::LlmRecoverable(format!(
                "Validation Error (Pydantic-first tool schema): Failed to parse arguments. Reason: {}. Please correct your JSON arguments and try again.",
                e
            ))
        })?;

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
}
