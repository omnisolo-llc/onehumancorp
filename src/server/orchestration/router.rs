use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::minimax::LocalLLMClient;
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SemanticRoutingRequest {
    pub prompt: String,
    pub tenant_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SemanticRoutingResponse {
    pub department: String,
    pub confidence: f32,
}

#[derive(Debug, Clone)]
pub struct DepartmentIntent {
    pub name: String,
    pub description: String,
    pub embedding: Vec<f32>,
}


#[async_trait::async_trait]
pub trait EmbeddingProvider: Send + Sync {
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, String>;
}

#[async_trait::async_trait]
impl EmbeddingProvider for LocalLLMClient {
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, String> {
        self.generate_embedding(text).await
    }
}

pub struct SemanticRouter {
    llm_client: Arc<dyn EmbeddingProvider>,
    intents: Vec<DepartmentIntent>,
    db_pool: Option<PgPool>,
}


impl SemanticRouter {
    pub async fn new(llm_client: Arc<dyn EmbeddingProvider>, db_pool: Option<PgPool>) -> Result<Self, String> {
        let departments = vec![
            ("Operations", "Handles the day-to-day execution of orders, bookings, inventory, deliveries, and refund requests."),
            ("Marketing", "Designs and publishes the business website, SEO, social media posts, promotional content."),
            ("Sales", "Generates quotes, follows up with prospects, manages lead pipeline and customer inquiries."),
            ("Customer Success", "Responds to customer messages, order confirmations, reviews, re-engages customers."),
            ("Finance", "Processes online payments, pricing strategy, tracks revenue, deposits, subscriptions, financial reports."),
            ("Legal", "Terms of service, privacy policies, custom order contracts, business licenses."),
            ("Advisory", "Business health reports, performance analysis, market data, pricing adjustments."),
        ];

        let mut intents = Vec::new();
        for (name, desc) in departments {
            let embedding = llm_client.generate_embedding(desc).await.unwrap_or_else(|_| vec![0.0; 1536]);
            intents.push(DepartmentIntent {
                name: name.to_string(),
                description: desc.to_string(),
                embedding,
            });
        }

        Ok(Self { llm_client, intents, db_pool })
    }

    pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot_product / (norm_a * norm_b)
        }
    }

    pub async fn route(&self, req: SemanticRoutingRequest) -> Result<SemanticRoutingResponse, String> {
        if let Some(pool) = &self.db_pool {
            // Future DB calls in this route will execute using a transaction to maintain RLS
            let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
            let _ = sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
                .bind(&req.tenant_id)
                .execute(&mut *tx)
                .await;
            // Example real-world embedding lookup could happen here
            let _ = tx.commit().await;
        }

        let embedding = self.llm_client.generate_embedding(&req.prompt).await.unwrap_or_else(|_| vec![0.0; 1536]);

        let mut best_department = "Operations".to_string(); // default
        let mut best_score = -1.0;

        for intent in &self.intents {
            let score = Self::cosine_similarity(&embedding, &intent.embedding);
            if score > best_score {
                best_score = score;
                best_department = intent.name.clone();
            }
        }

        Ok(SemanticRoutingResponse {
            department: best_department,
            confidence: best_score,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        let c = vec![0.0, 1.0, 0.0];

        assert_eq!(SemanticRouter::cosine_similarity(&a, &b), 1.0);
        assert_eq!(SemanticRouter::cosine_similarity(&a, &c), 0.0);
    }
}
