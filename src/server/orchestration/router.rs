use serde::{Deserialize, Serialize};
use crate::orchestration::departments::types::DepartmentType;
use opentelemetry::global;
use opentelemetry::KeyValue;
use opentelemetry::metrics::Counter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticRoutingRequest {
    pub tenant_id: String,
    pub prompt: String,
    // Add embedding if passed externally, else generated internally (mocked for now)
    pub embedding: Option<Vec<f32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticRoutingResponse {
    pub tenant_id: String,
    pub target_department: DepartmentType,
    pub confidence_score: f32,
}

pub struct SemanticRouter {
    route_counter: Counter<u64>,
}

impl SemanticRouter {
    pub fn new() -> Self {
        let meter = global::meter("ohc.orchestration.router");
        let route_counter = meter.u64_counter("semantic_route.count").build();
        Self { route_counter }
    }

    fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() || a.is_empty() {
            return 0.0;
        }
        let (dot, na, nb) = a.iter().zip(b.iter()).fold(
            (0.0f32, 0.0f32, 0.0f32),
            |(dot, na, nb), (&x, &y)| (dot + x * y, na + x * x, nb + y * y),
        );
        if na == 0.0 || nb == 0.0 {
            return 0.0;
        }
        dot / (na.sqrt() * nb.sqrt())
    }

    // Mock embedding generator for the given prompt
    fn generate_embedding(&self, prompt: &str) -> Vec<f32> {
        let text = prompt.to_lowercase();
        // A simple heuristic for tests
        if text.contains("website") || text.contains("design") || text.contains("marketing") || text.contains("seo") {
            vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
        } else if text.contains("refund") || text.contains("inventory") || text.contains("order") || text.contains("operations") {
            vec![0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]
        } else if text.contains("price") || text.contains("pricing") || text.contains("tax") || text.contains("finance") || text.contains("accountant") {
            vec![0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0]
        } else if text.contains("quote") || text.contains("proposal") || text.contains("lead") || text.contains("sales") {
            vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0]
        } else if text.contains("contract") || text.contains("policy") || text.contains("legal") || text.contains("terms") {
            vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0]
        } else if text.contains("customer") || text.contains("support") || text.contains("chat") {
            vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0]
        } else if text.contains("trend") || text.contains("report") || text.contains("advisory") || text.contains("health") {
            vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0]
        } else {
            // default fallback
            vec![0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1]
        }
    }

    pub fn route(&self, request: &SemanticRoutingRequest) -> Result<SemanticRoutingResponse, String> {
        if request.tenant_id.is_empty() {
            return Err("tenant_id is required".to_string());
        }

        let embedding = request.embedding.clone().unwrap_or_else(|| self.generate_embedding(&request.prompt));

        let centroids = vec![
            (DepartmentType::Marketing, vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
            (DepartmentType::Operations, vec![0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
            (DepartmentType::Finance, vec![0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0]),
            (DepartmentType::Sales, vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0]),
            (DepartmentType::Legal, vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
            (DepartmentType::CustomerSuccess, vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0]),
            (DepartmentType::BusinessAdvisory, vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0]),
        ];

        let mut best_dept = DepartmentType::Operations; // default
        let mut best_score = -1.0;

        for (dept, centroid) in centroids {
            let score = Self::cosine_similarity(&embedding, &centroid);
            if score > best_score {
                best_score = score;
                best_dept = dept;
            }
        }

        self.route_counter.add(1, &[
            KeyValue::new("tenant_id", request.tenant_id.clone()),
            KeyValue::new("department", best_dept.to_string()),
        ]);

        Ok(SemanticRoutingResponse {
            tenant_id: request.tenant_id.clone(),
            target_department: best_dept,
            confidence_score: best_score,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_routing() {
        let router = SemanticRouter::new();

        let cases = vec![
            ("I need a website designed", DepartmentType::Marketing),
            ("Can you help me process a refund for order 123", DepartmentType::Operations),
            ("What is my revenue this week and pricing strategy?", DepartmentType::Finance),
            ("Can you generate a quote for this lead?", DepartmentType::Sales),
            ("I need to draft terms of service", DepartmentType::Legal),
            ("Respond to this customer chat", DepartmentType::CustomerSuccess),
            ("What are the seasonal trends for my business?", DepartmentType::BusinessAdvisory),
        ];

        for (prompt, expected_dept) in cases {
            let req = SemanticRoutingRequest {
                tenant_id: "tenant_123".to_string(),
                prompt: prompt.to_string(),
                embedding: None,
            };

            let res = router.route(&req).expect("Routing failed");
            assert_eq!(res.tenant_id, "tenant_123");
            assert_eq!(res.target_department, expected_dept, "Failed for prompt: {}", prompt);
        }
    }

    #[test]
    fn test_missing_tenant_id() {
        let router = SemanticRouter::new();
        let req = SemanticRoutingRequest {
            tenant_id: "".to_string(),
            prompt: "hello".to_string(),
            embedding: None,
        };
        assert!(router.route(&req).is_err());
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundMessage {
    pub source: String,
    pub sender: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftReply {
    pub final_draft: String,
    pub operations_context: Option<String>,
    pub sales_context: Option<String>,
    pub customer_context: Option<String>,
}

pub struct OmniContextRouter {
    // LLM backed router for Omni-Context Sub-Agent Routing
}

impl OmniContextRouter {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn route_and_synthesize(&self, msg: &InboundMessage) -> Result<DraftReply, String> {
        let prompt = format!(
            "You are an Omni-Context Work Triage agent. Analyze the following inbound message and route to the appropriate sub-agents (Operations, Sales, Customer Success). Then synthesize their outputs into a unified DraftReply.
Message from {}: '{}'
Source: {}

Return strict JSON:
{{
  \"operations_context\": \"<details if scheduling or inventory needed, else null>\",
  \"sales_context\": \"<details if quote or pricing needed, else null>\",
  \"customer_context\": \"<drafted polite reply addressing the customer>\",
  \"final_draft\": \"<The combined final message to send to the customer>\"
}}",
            msg.sender, msg.content, msg.source
        );
        let compressed_prompt = crate::pricing::compression::reduce_tokens(&prompt);

        let max_retries = 3;
        let mut retry_count = 0;
        let mut result = DraftReply {
            final_draft: "Thanks for reaching out! We will review this and get back to you soon.".to_string(),
            operations_context: None,
            sales_context: None,
            customer_context: None,
        };

        let mut success = false;
        while retry_count < max_retries {
            let compressed_prompt_clone = compressed_prompt.clone();
            let llm_call = async {
                match std::env::var("OHC_INBOX_DRAFT_LLM_PROVIDER")
                    .or_else(|_| std::env::var("OHC_LLM_PROVIDER"))
                    .as_deref()
                {
                    Ok("minimax") => {
                        let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_else(|_| "fake-key".to_string());
                        if !api_key.is_empty() {
                            crate::minimax::MinimaxClient::new(api_key).reason(&compressed_prompt_clone).await
                        } else {
                            crate::minimax::LocalLLMClient::new().reason(&compressed_prompt_clone).await
                        }
                    }
                    _ => crate::minimax::LocalLLMClient::new().reason(&compressed_prompt_clone).await,
                }
            };

            match tokio::time::timeout(std::time::Duration::from_secs(60), llm_call).await {
                Ok(Ok(reply)) => {
                    let cleaned = reply.replace("```json", "").replace("```", "").trim().to_string();
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&cleaned) {
                        if json.get("final_draft").is_some() {
                            result.final_draft = json.get("final_draft").unwrap().as_str().unwrap_or("Thanks for reaching out! We will review this and get back to you soon.").to_string();
                            result.operations_context = json.get("operations_context").and_then(|v| v.as_str()).map(|s| s.to_string());
                            result.sales_context = json.get("sales_context").and_then(|v| v.as_str()).map(|s| s.to_string());
                            result.customer_context = json.get("customer_context").and_then(|v| v.as_str()).map(|s| s.to_string());
                            success = true;
                            break;
                        }
                    }
                    retry_count += 1;
                }
                Ok(Err(_)) | Err(_) => {
                    retry_count += 1;
                }
            }
        }

        if !success {
            return Err("AI Agent Service Unavailable".to_string());
        }

        Ok(result)
    }
}

#[cfg(test)]
mod omni_tests {
    use super::*;

    #[tokio::test]
    async fn test_omni_context_router_scheduling() {
        unsafe { std::env::set_var("CI", "1"); }
        let router = OmniContextRouter::new();
        let msg = InboundMessage {
            source: "Instagram".to_string(),
            sender: "maya".to_string(),
            content: "Can I schedule a cake delivery?".to_string(),
        };

        let result = router.route_and_synthesize(&msg).await.unwrap();
        assert!(result.operations_context.is_some());
        assert!(result.sales_context.is_none());
        assert!(result.final_draft.contains("schedule") || result.final_draft.contains("Thanks for reaching out!"));
    }

    #[tokio::test]
    async fn test_omni_context_router_quote() {
        unsafe { std::env::set_var("CI", "1"); }
        let router = OmniContextRouter::new();
        let msg = InboundMessage {
            source: "Email".to_string(),
            sender: "carlos".to_string(),
            content: "Need a quote for fixing a sink.".to_string(),
        };

        let result = router.route_and_synthesize(&msg).await.unwrap();
        assert!(result.operations_context.is_none());
        assert!(result.sales_context.is_some());
        assert!(result.final_draft.contains("quote") || result.final_draft.contains("Thanks for reaching out!"));
    }
}
