use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticRoutingRequest {
    pub prompt: String,
    pub tenant_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticRoutingResponse {
    pub department: String,
    pub confidence: f32,
}

pub struct SemanticRouter {
    intent_centroids: HashMap<String, Vec<f32>>,
}

impl SemanticRouter {
    pub fn new() -> Self {
        let mut centroids = HashMap::new();
        centroids.insert("Operations".to_string(), vec![0.1, 0.2, 0.3]);
        centroids.insert("Marketing".to_string(), vec![0.8, 0.1, 0.1]);
        centroids.insert("Sales".to_string(), vec![0.2, 0.8, 0.1]);
        centroids.insert("Customer Success".to_string(), vec![0.1, 0.8, 0.2]);
        centroids.insert("Finance".to_string(), vec![0.1, 0.1, 0.8]);
        centroids.insert("Legal".to_string(), vec![0.1, 0.8, 0.8]);
        centroids.insert("Business Advisory".to_string(), vec![0.8, 0.8, 0.1]);

        Self {
            intent_centroids: centroids,
        }
    }

    pub async fn route(&self, req: SemanticRoutingRequest) -> Result<SemanticRoutingResponse, String> {
        if req.tenant_id.trim().is_empty() {
            return Err("tenant_id is required for RLS multi-tenant awareness".to_string());
        }

        let mock_embedding = self.mock_embedding(&req.prompt);
        let mut best_department = "Operations".to_string();
        let mut best_score = -1.0;

        for (dept, centroid) in &self.intent_centroids {
            let score = Self::cosine_similarity(&mock_embedding, centroid);
            if score > best_score {
                best_score = score;
                best_department = dept.clone();
            }
        }

        Ok(SemanticRoutingResponse {
            department: best_department,
            confidence: best_score,
        })
    }

    fn mock_embedding(&self, prompt: &str) -> Vec<f32> {
        let lower = prompt.to_lowercase();

        // Edge case: "refund request" vs "pricing strategy question"
        // If it contains refund, prioritize finance over strategy. If it contains pricing strategy, prioritize business advisory.
        if lower.contains("pricing strategy") {
            return vec![0.8, 0.8, 0.1]; // Business Advisory
        }
        if lower.contains("refund") {
            return vec![0.1, 0.1, 0.8]; // Finance
        }

        if lower.contains("website") || lower.contains("seo") || lower.contains("social") || lower.contains("promote") || lower.contains("design") {
            vec![0.8, 0.1, 0.1] // Marketing
        } else if lower.contains("quote") || lower.contains("sales") || lower.contains("lead") {
            vec![0.2, 0.8, 0.1] // Sales
        } else if lower.contains("pay") || lower.contains("deposit") || lower.contains("finance") || lower.contains("revenue") || lower.contains("price") {
            vec![0.1, 0.1, 0.8] // Finance
        } else if lower.contains("customer") || lower.contains("message") || lower.contains("support") || lower.contains("chat") {
            vec![0.1, 0.8, 0.2] // Customer Success
        } else if lower.contains("contract") || lower.contains("legal") || lower.contains("policy") || lower.contains("terms") || lower.contains("compliance") {
            vec![0.1, 0.8, 0.8] // Legal
        } else if lower.contains("report") || lower.contains("advice") || lower.contains("health") || lower.contains("trend") || lower.contains("strategy") {
            vec![0.8, 0.8, 0.1] // Business Advisory
        } else {
            vec![0.1, 0.2, 0.3] // Operations (Default)
        }
    }

    fn cosine_similarity(v1: &[f32], v2: &[f32]) -> f32 {
        let mut dot = 0.0;
        let mut norm1 = 0.0;
        let mut norm2 = 0.0;
        for (a, b) in v1.iter().zip(v2.iter()) {
            dot += a * b;
            norm1 += a * a;
            norm2 += b * b;
        }
        if norm1 == 0.0 || norm2 == 0.0 {
            return 0.0;
        }
        dot / (norm1.sqrt() * norm2.sqrt())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_semantic_router_table_driven() {
        let router = SemanticRouter::new();

        let cases = vec![
            ("Help me design a website for my custom cakes", "Marketing"),
            ("How do I accept deposit payments via Stripe?", "Finance"),
            ("A customer sent a message asking about my working hours.", "Customer Success"),
            ("I need to draft a contract policy for my cleaning services.", "Legal"),
            ("What is the trend for my weekly report strategy?", "Business Advisory"),
            ("I need to manage my inventory and schedule deliveries.", "Operations"),
            ("Can you help me process a refund request?", "Finance"),
            ("I need help with my pricing strategy.", "Business Advisory"),
        ];

        for (prompt, expected_dept) in cases {
            let req = SemanticRoutingRequest {
                prompt: prompt.to_string(),
                tenant_id: "tenant1".to_string(),
            };
            let res = router.route(req).await.unwrap();
            assert_eq!(res.department, expected_dept, "Failed for prompt: {}", prompt);
        }
    }

    #[tokio::test]
    async fn test_semantic_router_missing_tenant_id() {
        let router = SemanticRouter::new();
        let req = SemanticRoutingRequest {
            prompt: "Help me design a website".to_string(),
            tenant_id: "   ".to_string(),
        };
        let res = router.route(req).await;
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "tenant_id is required for RLS multi-tenant awareness");
    }
}
