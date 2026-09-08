#![allow(clippy::collapsible_if)]
use ohc_builtin_agent_core::types::ToolError;
use ohc_builtin_agent_llm::LlmClient;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

use super::Tool;
use super::pydantic::{PydanticAdapter, PydanticToolExecutor};

#[derive(Deserialize)]
struct LlmJudgeArgs {
    output: String,
    task_description: String,
}

#[derive(Deserialize)]
struct JudgeEvaluation {
    status: String,
    reason: String,
    confidence: f32,
    #[serde(default)]
    missing_elements: Vec<String>,
    #[serde(default)]
    suggested_fixes: Vec<String>,
}

struct LlmJudgeExecutor {
    llm: Arc<dyn LlmClient>,
    model: String,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<LlmJudgeArgs> for LlmJudgeExecutor {
    async fn execute_typed(&self, args: LlmJudgeArgs) -> Result<String, ToolError> {
        use ohc_builtin_agent_core::types::{ChatRequest, Message};

        let prompt = format!(
            "You are an LLM Judge.\n\nTask Description:\n{}\n\nOutput to Evaluate:\n{}\n\nEvaluate the output against the task description. Is it acceptable and correct?",
            args.task_description, args.output
        );

        let req = ChatRequest {
            model: self.model.clone(),
            system: "You are an expert LLM judge. Evaluate the output strictly. Provide your evaluation in structured JSON using the `structured_output` tool with fields `status` (APPROVE or REJECT), `reason`, `confidence` (float 0-1), `missing_elements` (list of strings), and `suggested_fixes` (list of strings).".to_string(),
            messages: vec![Message::user(prompt)],
            tools: vec![],
            max_tokens: 1024,
            temperature: 0.0,
        };

        struct ParserAdapter {
            llm: Arc<dyn LlmClient>,
        }
        #[async_trait::async_trait]
        impl ohc_builtin_agent_core::output_parser::LlmClientForParser for ParserAdapter {
            async fn chat(
                &self,
                req: ChatRequest,
            ) -> Result<
                ohc_builtin_agent_core::types::ChatResponse,
                Box<dyn std::error::Error + Send + Sync>,
            > {
                self.llm.chat(req).await
            }
        }
        let parser_client = Arc::new(ParserAdapter {
            llm: self.llm.clone(),
        })
            as Arc<dyn ohc_builtin_agent_core::output_parser::LlmClientForParser>;

        match ohc_builtin_agent_core::output_parser::parse_structured_output::<JudgeEvaluation>(
            &parser_client,
            req,
            3,
        )
        .await
        {
            Ok(eval) => {
                if eval.status.to_uppercase() == "APPROVE" && eval.confidence >= 0.7 {
                    Ok(json!({
                        "status": "APPROVED",
                        "message": "The LLM Judge approved the output."
                    })
                    .to_string())
                } else {
                    let mut err_msg =
                        format!("LLM Judge REJECTED the output.\nReason: {}", eval.reason);

                    if !eval.missing_elements.is_empty() {
                        err_msg.push_str(&format!(
                            "\nMissing Elements: {}",
                            eval.missing_elements.join(", ")
                        ));
                    }
                    if !eval.suggested_fixes.is_empty() {
                        err_msg.push_str(&format!(
                            "\nSuggested Fixes:\n- {}",
                            eval.suggested_fixes.join("\n- ")
                        ));
                    }

                    Err(ToolError::LlmRecoverable(err_msg))
                }
            }
            Err(e) => Err(e),
        }
    }
}

pub fn llm_judge_tool(llm: Arc<dyn LlmClient>, model: String) -> Tool {
    Tool {
        name: "llm_judge".to_string(),
        description: "Evaluate the quality and accuracy of an intermediate output using an LLM-as-judge subagent. Use this tool to proactively verify your work against the task requirements.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "output": {
                    "type": "string",
                    "description": "The text output you want to evaluate."
                },
                "task_description": {
                    "type": "string",
                    "description": "The original task description or criteria to evaluate against."
                }
            },
            "required": ["output", "task_description"]
        }),
        execute: Arc::new(PydanticAdapter::new(LlmJudgeExecutor { llm, model })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role, Usage};

    struct MockLlmClient {
        response_text: String,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let tool_call = ohc_builtin_agent_core::types::ToolCall {
                id: "call_1".to_string(),
                name: "structured_output".to_string(),
                arguments: serde_json::json!({
                    "data": serde_json::from_str::<serde_json::Value>(&self.response_text).unwrap_or(serde_json::json!({}))
                }),
            };

            let msg = Message {
                role: Role::Assistant,
                content: "".to_string(),
                tool_calls: vec![tool_call],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            };
            Ok(ChatResponse {
                response_id: Some("test".to_string()),
                stop_reason: "".to_string(),
                message: msg,
                usage: Usage::default(),
            })
        }
    }

    #[tokio::test]
    async fn test_llm_judge_tool_approve() {
        let pass_llm = Arc::new(MockLlmClient {
            response_text: r#"{"status": "APPROVE", "reason": "Looks good", "confidence": 0.9, "missing_elements": [], "suggested_fixes": []}"#.to_string(),
        });
        let tool = llm_judge_tool(pass_llm, "test-model".to_string());

        let args = json!({
            "output": "The sky is blue.",
            "task_description": "What color is the sky?"
        });

        let res = tool.execute.execute(args).await;
        assert!(res.is_ok());
        let result_str = res.unwrap();
        assert!(result_str.contains("APPROVED"));
    }

    #[tokio::test]
    async fn test_llm_judge_tool_reject() {
        let fail_llm = Arc::new(MockLlmClient {
            response_text: r#"{"status": "REJECT", "reason": "Bad answer", "confidence": 0.8, "missing_elements": ["Correct color"], "suggested_fixes": ["Say blue"]}"#.to_string(),
        });
        let tool = llm_judge_tool(fail_llm, "test-model".to_string());

        let args = json!({
            "output": "The sky is green.",
            "task_description": "What color is the sky?"
        });

        let res = tool.execute.execute(args).await;
        assert!(res.is_err());
        match res.unwrap_err() {
            ToolError::LlmRecoverable(msg) => {
                assert!(msg.contains("LLM Judge REJECTED the output"));
                assert!(msg.contains("Reason: Bad answer"));
                assert!(msg.contains("Say blue"));
            }
            _ => panic!("Expected LlmRecoverable error"),
        }
    }
}
