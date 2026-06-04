use std::sync::Arc;
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

use crate::orchestration::departments::types::DepartmentType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticRoutingRequest {
    pub tenant_id: String,
    pub prompt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticRoutingResponse {
    pub tenant_id: String,
    pub original_prompt: String,
    pub routed_department: DepartmentType,
    pub confidence_score: f32,
}

#[async_trait]
pub trait Embedder: Send + Sync {
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, String>;
}

pub struct DepartmentIntent {
    pub department: DepartmentType,
    pub centroid: Vec<f32>,
}

pub struct SemanticRouter {
    embedder: Arc<dyn Embedder>,
    #[allow(dead_code)]
    pool: Option<PgPool>,
    intents: Vec<DepartmentIntent>,
}

impl SemanticRouter {
    pub fn new(embedder: Arc<dyn Embedder>, pool: Option<PgPool>) -> Self {
        // Initialize with default fallback centroids (mocked/randomized or hardcoded for now, typically 1536 dims, but keeping small for testing)
        // In a real scenario, these would be loaded from a DB or config, representing the center of embeddings for each department's queries.
        let intents = vec![
            // Operations: "The Manager" - process orders, calendar, inventory
            DepartmentIntent {
                department: DepartmentType::Operations,
                centroid: vec![0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            },
            // Marketing: "The Promoter" - website design, SEO, social media
            DepartmentIntent {
                department: DepartmentType::Marketing,
                centroid: vec![0.0, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0],
            },
            // Sales: "The Salesperson" - quotes, proposals, follow ups
            DepartmentIntent {
                department: DepartmentType::Sales,
                centroid: vec![0.0, 0.0, 0.1, 0.0, 0.0, 0.0, 0.0],
            },
            // Customer Success: "The Ambassador" - customer messages, refunds, reviews
            DepartmentIntent {
                department: DepartmentType::CustomerSuccess,
                centroid: vec![0.0, 0.0, 0.0, 0.1, 0.0, 0.0, 0.0],
            },
            // Finance: "The Accountant" - payments, pricing strategy, deposits, reports
            DepartmentIntent {
                department: DepartmentType::Finance,
                centroid: vec![0.0, 0.0, 0.0, 0.0, 0.1, 0.0, 0.0],
            },
            // Legal: "The Protector" - terms, contracts, GDPR
            DepartmentIntent {
                department: DepartmentType::Legal,
                centroid: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.1, 0.0],
            },
            // Business Advisory: "The Advisor" - performance analysis, trends, recommendations
            DepartmentIntent {
                department: DepartmentType::BusinessAdvisory,
                centroid: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.1],
            },
        ];

        Self {
            embedder,
            pool,
            intents,
        }
    }

    // Optional method to set custom centroids (e.g. for testing)
    pub fn with_intents(mut self, intents: Vec<DepartmentIntent>) -> Self {
        self.intents = intents;
        self
    }

    pub async fn route(&self, request: SemanticRoutingRequest) -> Result<SemanticRoutingResponse, String> {
        let embedding = self.embedder.generate_embedding(&request.prompt).await?;

        // Ensure tenant isolation if using DB later
        if let Some(pool) = &self.pool {
            // Check db vector similarity if available
            // Note: In an actual implementation with pgvector we could do a similarity search:
            // "SELECT department, 1 - (embedding <=> $1) as score FROM intent_centroids WHERE tenant_id = $2 OR tenant_id = 'system' ORDER BY score DESC LIMIT 1"
            // For now, we fallback to in-memory cosine similarity
        }

        let mut best_department = DepartmentType::Operations; // Fallback
        let mut best_score = -1.0;

        for intent in &self.intents {
            let score = cosine_similarity(&embedding, &intent.centroid);
            if score > best_score {
                best_score = score;
                best_department = intent.department;
            }
        }

        Ok(SemanticRoutingResponse {
            tenant_id: request.tenant_id,
            original_prompt: request.prompt,
            routed_department: best_department,
            confidence_score: best_score,
        })
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let mut dot_product = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    for (val_a, val_b) in a.iter().zip(b.iter()) {
        dot_product += val_a * val_b;
        norm_a += val_a * val_a;
        norm_b += val_b * val_b;
    }

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot_product / (norm_a.sqrt() * norm_b.sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockEmbedder {
        // Maps prompt to an expected returned embedding
        embedding: Vec<f32>,
    }

    #[async_trait]
    impl Embedder for MockEmbedder {
        async fn generate_embedding(&self, _text: &str) -> Result<Vec<f32>, String> {
            Ok(self.embedding.clone())
        }
    }

    #[tokio::test]
    async fn test_semantic_routing() {
        let intents = vec![
            DepartmentIntent {
                department: DepartmentType::CustomerSuccess,
                centroid: vec![1.0, 0.0, 0.0],
            },
            DepartmentIntent {
                department: DepartmentType::Finance,
                centroid: vec![0.0, 1.0, 0.0],
            },
            DepartmentIntent {
                department: DepartmentType::Marketing,
                centroid: vec![0.0, 0.0, 1.0],
            },
        ];

        let test_cases = vec![
            (
                "refund request",
                vec![0.9, 0.1, 0.0], // Close to CustomerSuccess
                DepartmentType::CustomerSuccess,
            ),
            (
                "pricing strategy question",
                vec![0.1, 0.9, 0.0], // Close to Finance
                DepartmentType::Finance,
            ),
            (
                "help me design a website",
                vec![0.0, 0.1, 0.9], // Close to Marketing
                DepartmentType::Marketing,
            ),
            (
                "general query",
                vec![0.5, 0.5, 0.5], // Equidistant to all, should pick one based on best score iteration
                DepartmentType::CustomerSuccess, // First checked has best or equal score
            ),
        ];

        for (prompt, embedding, expected_department) in test_cases {
            let embedder = Arc::new(MockEmbedder { embedding });
            let router = SemanticRouter::new(embedder, None).with_intents(intents.iter().map(|i| DepartmentIntent { department: i.department, centroid: i.centroid.clone() }).collect());

            let req = SemanticRoutingRequest {
                tenant_id: "tenant-123".to_string(),
                prompt: prompt.to_string(),
            };

            let resp = router.route(req).await.unwrap();

            assert_eq!(
                resp.routed_department, expected_department,
                "Failed routing for prompt: {}",
                prompt
            );
        }
    }
}
