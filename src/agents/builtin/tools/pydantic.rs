use ohc_builtin_agent_core::types::ToolError;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::sync::Arc;

use super::ToolExecutor;

/// A typed executor that leverages Serde to automatically deserialize and validate `args`
/// before passing them to the inner typed execution function.
///
/// This implements the "Pydantic-first tool schema" mechanic from the Master Catalog.
/// Any deserialization or validation errors are immediately caught and fed back to the LLM
/// as a `ToolError::LlmRecoverable` for self-correction.
pub struct PydanticToolExecutor<T, F, Fut>
where
    T: DeserializeOwned + Send + Sync,
    F: Fn(T) -> Fut + Send + Sync,
    Fut: std::future::Future<Output = Result<String, ToolError>> + Send,
{
    executor_fn: F,
    _marker: std::marker::PhantomData<T>,
}

impl<T, F, Fut> PydanticToolExecutor<T, F, Fut>
where
    T: DeserializeOwned + Send + Sync,
    F: Fn(T) -> Fut + Send + Sync,
    Fut: std::future::Future<Output = Result<String, ToolError>> + Send,
{
    pub fn new(executor_fn: F) -> Self {
        Self {
            executor_fn,
            _marker: std::marker::PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<T, F, Fut> ToolExecutor for PydanticToolExecutor<T, F, Fut>
where
    T: DeserializeOwned + Send + Sync + 'static,
    F: Fn(T) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = Result<String, ToolError>> + Send + 'static,
{
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let typed_args: T = match serde_json::from_value(args.clone()) {
            Ok(parsed) => parsed,
            Err(e) => {
                // Pydantic-first tool schema: validation errors fed back to LLM for self-correction.
                let err_msg = format!("Schema validation failed for tool arguments. Error: {}. Please correct the arguments and try again. Provided args: {}", e, args);
                return Err(ToolError::LlmRecoverable(err_msg));
            }
        };

        (self.executor_fn)(typed_args).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct TestArgs {
        path: String,
        count: i32,
    }

    async fn test_executor(args: TestArgs) -> Result<String, ToolError> {
        Ok(format!("Success: {} - {}", args.path, args.count))
    }

    #[tokio::test]
    async fn test_pydantic_executor_success() {
        let executor = PydanticToolExecutor::new(test_executor);
        let args = serde_json::json!({
            "path": "/test",
            "count": 42
        });

        let result = executor.execute(args).await.unwrap();
        assert_eq!(result, "Success: /test - 42");
    }

    #[tokio::test]
    async fn test_pydantic_executor_validation_error() {
        let executor = PydanticToolExecutor::new(test_executor);
        let args = serde_json::json!({
            "path": "/test",
            // missing 'count' field
        });

        let result = executor.execute(args).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            ToolError::LlmRecoverable(msg) => {
                assert!(msg.contains("Schema validation failed"));
                assert!(msg.contains("missing field `count`"));
            }
            _ => panic!("Expected LlmRecoverable error"),
        }
    }
}
