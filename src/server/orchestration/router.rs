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
