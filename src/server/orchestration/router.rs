use crate::orchestration::departments::types::DepartmentType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticRoutingRequest {
    pub tenant_id: String,
    pub prompt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticRoutingResponse {
    pub department: DepartmentType,
    pub confidence: f32,
}

#[async_trait]
pub trait EmbeddingProvider: Send + Sync {
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, String>;
}

pub struct SemanticRouter {
    embedding_provider: Arc<dyn EmbeddingProvider>,
    intent_vectors: HashMap<DepartmentType, Vec<f32>>,
}

impl SemanticRouter {
    pub async fn new(provider: Arc<dyn EmbeddingProvider>) -> Result<Self, String> {
        let mut intent_vectors = HashMap::new();

        let intents = vec![
            (DepartmentType::Operations, "Process orders, manage inventory, coordinate pickups and deliveries, handle refund requests, bookings calendar, daily execution"),
            (DepartmentType::Marketing, "Design website, SEO, social media, promotional content, advertising, flyers, link-in-bio, get found on Google"),
            (DepartmentType::Sales, "Generate quotes, proposals, lead pipeline, upsell, cross-sell, follow up with prospects, win customers"),
            (DepartmentType::CustomerSuccess, "Respond to customer messages, chat, email, Instagram DM, reviews, re-engage customers, post-sale relationship"),
            (DepartmentType::Finance, "Process payments, online payments, Stripe, financial reports, taxes, subscriptions, pricing strategy, profit margins"),
            (DepartmentType::Legal, "Terms of service, privacy policies, contracts, GDPR, compliance, liability disclaimers, regulatory requirements"),
            (DepartmentType::BusinessAdvisory, "Business health reports, performance analysis, market data, seasonal trends, pricing adjustments, unusual patterns, consultant"),
        ];

        for (dept, description) in intents {
            let embedding = provider.generate_embedding(description).await?;
            intent_vectors.insert(dept, embedding);
        }

        Ok(Self {
            embedding_provider: provider,
            intent_vectors,
        })
    }

    pub async fn route(&self, request: &SemanticRoutingRequest) -> Result<SemanticRoutingResponse, String> {
        if request.tenant_id.is_empty() {
            return Err("tenant_id is required for multi-tenant isolation".to_string());
        }

        let prompt_embedding = self.embedding_provider.generate_embedding(&request.prompt).await?;

        let mut best_dept = DepartmentType::Operations;
        let mut best_score = -1.0_f32;

        for (dept, intent_vector) in &self.intent_vectors {
            let score = Self::cosine_similarity(&prompt_embedding, intent_vector);
            if score > best_score {
                best_score = score;
                best_dept = *dept;
            }
        }

        Ok(SemanticRoutingResponse {
            department: best_dept,
            confidence: best_score,
        })
    }

    fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() || a.is_empty() {
            return 0.0;
        }
        let (dot_product, norm_a, norm_b) = a.iter().zip(b.iter()).fold(
            (0.0f32, 0.0f32, 0.0f32),
            |(dot, na, nb), (&x, &y)| (dot + x * y, na + x * x, nb + y * y),
        );
        if norm_a == 0.0 || norm_b == 0.0 {
            return 0.0;
        }
        dot_product / (norm_a.sqrt() * norm_b.sqrt())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockEmbeddingProvider;

    #[async_trait]
    impl EmbeddingProvider for MockEmbeddingProvider {
        async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, String> {
            let t = text.to_lowercase();
            let mut emb = vec![0.0; 7];

            // Simple mock embedding: each dimension corresponds to a department
            if t.contains("refund") || t.contains("inventory") || t.contains("order") {
                emb[0] = 1.0; // Operations
            }
            if t.contains("website") || t.contains("seo") || t.contains("flyer") {
                emb[1] = 1.0; // Marketing
            }
            if t.contains("quote") || t.contains("lead") {
                emb[2] = 1.0; // Sales
            }
            if t.contains("message") || t.contains("review") {
                emb[3] = 1.0; // Customer Success
            }
            if t.contains("payment") || t.contains("price") || t.contains("strategy") || t.contains("tax") {
                emb[4] = 1.0; // Finance
            }
            if t.contains("contract") || t.contains("policy") || t.contains("gdpr") {
                emb[5] = 1.0; // Legal
            }
            if t.contains("trend") || t.contains("analysis") || t.contains("report") {
                emb[6] = 1.0; // Business Advisory
            }

            // If no match, add some default to Operations to prevent zero vector
            if emb.iter().all(|&x| x == 0.0) {
                emb[0] = 0.5;
            }

            Ok(emb)
        }
    }

    #[tokio::test]
    async fn test_semantic_router() {
        let provider = Arc::new(MockEmbeddingProvider);
        let router = SemanticRouter::new(provider).await.unwrap();

        let test_cases = vec![
            (
                "I need to process a refund for a customer",
                DepartmentType::Operations,
            ),
            (
                "Can you help me build a website for my bakery?",
                DepartmentType::Marketing,
            ),
            (
                "Generate a quote for a new plumbing job",
                DepartmentType::Sales,
            ),
            (
                "I have a bad review on Google, how should I respond?",
                DepartmentType::CustomerSuccess,
            ),
            (
                "What is my pricing strategy for the new vegan cake?",
                DepartmentType::Finance,
            ),
            (
                "I need a GDPR compliance policy",
                DepartmentType::Legal,
            ),
            (
                "Show me the business analysis and seasonal trends",
                DepartmentType::BusinessAdvisory,
            ),
        ];

        for (prompt, expected_dept) in test_cases {
            let req = SemanticRoutingRequest {
                tenant_id: "tenant_123".to_string(),
                prompt: prompt.to_string(),
            };
            let res = router.route(&req).await.unwrap();
            assert_eq!(res.department, expected_dept, "Failed for prompt: {}", prompt);
            assert!(res.confidence > 0.0);
        }
    }

    #[tokio::test]
    async fn test_semantic_router_tenant_enforcement() {
        let provider = Arc::new(MockEmbeddingProvider);
        let router = SemanticRouter::new(provider).await.unwrap();

        let req = SemanticRoutingRequest {
            tenant_id: "".to_string(), // Empty tenant ID
            prompt: "Process an order".to_string(),
        };

        let res = router.route(&req).await;
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "tenant_id is required for multi-tenant isolation");
    }
}
