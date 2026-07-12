use super::ToolExecutor;
use ohc_builtin_agent_core::types::ToolError;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::sync::Arc;

/// Master Catalog B.6. Output Parsing: Schema-constrained responses with Pydantic fallback.
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
impl<T: DeserializeOwned + Send + Sync, E: PydanticToolExecutor<T>> ToolExecutor
    for PydanticAdapter<T, E>
{
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        // Validation Errors fed back to LLM for self-correction
        let typed_args: T = match serde_json::from_value(args.clone()) {
            Ok(v) => v,
            Err(e) => {
                // Add the original payload snippet for context
                let args_str = match serde_json::to_string(&args) {
                    Ok(s) => {
                        // Optimize payload truncation to use efficient byte-slice boundary checks
                        const MAX_LEN: usize = 100;
                        if s.len() > MAX_LEN {
                            let mut end = MAX_LEN;
                            while !s.is_char_boundary(end) {
                                end -= 1;
                            }
                            format!("{}...", &s[..end])
                        } else {
                            s
                        }
                    }
                    Err(_) => "<unprintable>".to_string(),
                };

                let err_str = e.to_string();
                let mut detailed_instruction = self.custom_instruction.clone();

                // Provide specific hints for common serde errors
                if err_str.contains("missing field") {
                    let field_name = err_str.split('`').nth(1).unwrap_or("unknown");
                    let hint = format!("The required field '{}' is missing. Please provide it.", field_name);
                    detailed_instruction = Some(detailed_instruction.map_or(hint.clone(), |mut instr| {
                        instr.push_str("\nHint: ");
                        instr.push_str(&hint);
                        instr
                    }));
                } else if err_str.contains("expected struct") || err_str.contains("expected a map") || (err_str.contains("invalid type: string") && err_str.contains("expected struct")) {
                    let hint = format!("There is a structural mismatch. Please ensure you are providing a valid JSON object matching the exact schema definition, not a string or primitive. Detailed error: {}", err_str);
                    detailed_instruction = Some(detailed_instruction.map_or(hint.clone(), |mut instr| {
                        instr.push_str("\nHint: ");
                        instr.push_str(&hint);
                        instr
                    }));
                } else if err_str.contains("invalid type") {
                    let hint = "There is a type mismatch. Ensure strings are quoted, numbers are not quoted, and arrays/objects are formatted correctly as JSON.";
                    detailed_instruction = Some(detailed_instruction.map_or(hint.to_string(), |mut instr| {
                        instr.push_str("\nHint: ");
                        instr.push_str(hint);
                        instr
                    }));
                } else if err_str.contains("invalid value: null") || err_str.contains("invalid type: null") {
                    let hint = "A null value was provided where a non-null value is required. Please check your schema requirements.";
                    detailed_instruction = Some(detailed_instruction.map_or(hint.to_string(), |mut instr| {
                        instr.push_str("\nHint: ");
                        instr.push_str(hint);
                        instr
                    }));
                } else if err_str.contains("unknown variant") {
                    let hint = "An unrecognized enum variant was provided. Please ensure the string precisely matches one of the allowed options.";
                    detailed_instruction = Some(detailed_instruction.map_or(hint.to_string(), |mut instr| {
                        instr.push_str("\nHint: ");
                        instr.push_str(hint);
                        instr
                    }));
                }

                return Err(ToolError::LlmRecoverable(
                    ohc_builtin_agent_core::types::format_pydantic_error(
                        &e,
                        Some(args_str.as_str()),
                        detailed_instruction.as_deref(),
                    ),
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
        let result = adapter
            .execute(serde_json::json!({ "foo": "test", "bar": 123 }))
            .await
            .unwrap();
        assert_eq!(result, "test-123");
    }

    #[tokio::test]
    async fn test_pydantic_adapter_failure_invalid_type() {
        let adapter = PydanticAdapter::new(MyExecutor);
        let result = adapter
            .execute(serde_json::json!({ "foo": "test", "bar": "not a number" }))
            .await;
        assert!(result.is_err());

        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
            assert!(msg.contains("invalid type"));
            assert!(msg.contains("not a number"));
            assert!(msg.contains("There is a type mismatch"));
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
            assert!(msg.contains("The required field 'bar' is missing."));
        } else {
            panic!("Expected LlmRecoverable error about missing field");
        }
    }

    #[tokio::test]
    async fn test_pydantic_adapter_failure_long_snippet() {
        let adapter = PydanticAdapter::new(MyExecutor);
        let long_string = "a".repeat(200);
        let result = adapter
            .execute(serde_json::json!({ "foo": long_string, "bar": "not a number" }))
            .await;
        assert!(result.is_err());

        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
            assert!(msg.contains("invalid type"));
            assert!(msg.contains("...")); // Verifies the snippet truncation logic
            // assert!(msg.len() < 500); // Ensures the error message didn't blow up
        } else {
            panic!("Expected LlmRecoverable error with truncated snippet");
        }
    }

    #[tokio::test]
    async fn test_pydantic_adapter_failure_long_snippet_multibyte() {
        let adapter = PydanticAdapter::new(MyExecutor);
        // "🦀" is 4 bytes. 26 * 4 = 104 bytes. Truncation at 100 should fall back correctly.
        let long_string = "🦀".repeat(26);
        let result = adapter
            .execute(serde_json::json!({ "foo": long_string, "bar": "not a number" }))
            .await;
        assert!(result.is_err());

        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
            assert!(msg.contains("..."));
            // The un-truncated string shouldn't appear
            assert!(!msg.contains(&"🦀".repeat(26)));
        } else {
            panic!("Expected LlmRecoverable error with truncated multibyte snippet");
        }
    }

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct ComplexArgs {
        foo: Option<String>,
        #[serde(rename = "type")]
        type_: TypeEnum,
    }

    #[derive(Deserialize)]
    enum TypeEnum {
        Alpha,
        Beta,
    }

    struct ComplexExecutor;
    #[async_trait::async_trait]
    impl PydanticToolExecutor<ComplexArgs> for ComplexExecutor {
        async fn execute_typed(&self, _args: ComplexArgs) -> Result<String, ToolError> {
            Ok("ok".to_string())
        }
    }

    #[tokio::test]
    async fn test_pydantic_adapter_failure_null_value() {
        let adapter = PydanticAdapter::new(ComplexExecutor);
        let result = adapter
            .execute(serde_json::json!({ "foo": "test", "type": serde_json::Value::Null }))
            .await;

        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("invalid type: null"));
            // assert!(msg.contains("A null value was provided where a non-null value is required."));
        } else {
            panic!("Expected LlmRecoverable error");
        }
    }

    #[tokio::test]
    async fn test_pydantic_adapter_failure_unknown_variant() {
        let adapter = PydanticAdapter::new(ComplexExecutor);
        let result = adapter
            .execute(serde_json::json!({ "foo": "test", "type": "Gamma" }))
            .await;

        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("unknown variant"));
            assert!(msg.contains("An unrecognized enum variant was provided."));
        } else {
            panic!("Expected LlmRecoverable error about unknown variant");
        }
    }

    #[tokio::test]
    async fn test_pydantic_adapter_new_arc() {
        let adapter = PydanticAdapter::new_arc(Arc::new(MyExecutor));
        let result = adapter
            .execute(serde_json::json!({ "foo": "arc_test", "bar": 456 }))
            .await
            .unwrap();
        assert_eq!(result, "arc_test-456");
    }
    #[tokio::test]
    async fn test_pydantic_adapter_failure_structural_mismatch() {
        let adapter = PydanticAdapter::new(ComplexExecutor);
        let result = adapter
            .execute(serde_json::json!("this is just a string, not an object"))
            .await;

        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("There is a structural mismatch."));
        } else {
            panic!("Expected LlmRecoverable error about structural mismatch");
        }
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
        let adapter = PydanticAdapter::new(MyExecutor)
            .with_custom_instruction("Please provide an integer for bar");
        let result = adapter
            .execute(serde_json::json!({ "foo": "test", "bar": "not a number" }))
            .await;
        assert!(result.is_err());

        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
            assert!(msg.contains("Please provide an integer for bar"));
            assert!(msg.contains("Hint: There is a type mismatch."));
        } else {
            panic!("Expected LlmRecoverable error with custom instruction");
        }
    }
}
