use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SemanticRoutingRequest {
    pub prompt: String,
    pub tenant_id: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SemanticRoutingResponse {
    pub assigned_department: String,
    pub confidence: f32,
}

pub trait EmbeddingGenerator: Send + Sync {
    fn generate_embedding<'a>(&'a self, text: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<f32>, String>> + Send + 'a>>;
}

pub struct SemanticRouter {
    embedding_generator: Arc<dyn EmbeddingGenerator>,
    centroids: HashMap<String, Vec<f32>>,
}

impl SemanticRouter {
    pub fn new(embedding_generator: Arc<dyn EmbeddingGenerator>, centroids: HashMap<String, Vec<f32>>) -> Self {
        Self {
            embedding_generator,
            centroids,
        }
    }

    pub async fn route(&self, req: &SemanticRoutingRequest) -> Result<SemanticRoutingResponse, String> {
        let prompt_embedding = self.embedding_generator.generate_embedding(&req.prompt).await?;

        if self.centroids.is_empty() {
            return Ok(SemanticRoutingResponse {
                assigned_department: "Operations".to_string(), // Fallback
                confidence: 0.0,
            });
        }

        let mut best_department = "Operations".to_string();
        let mut best_score = -1.0_f32;

        for (department, centroid) in &self.centroids {
            let score = cosine_similarity(&prompt_embedding, centroid);
            if score > best_score {
                best_score = score;
                best_department = department.clone();
            }
        }

        Ok(SemanticRoutingResponse {
            assigned_department: best_department,
            confidence: best_score,
        })
    }
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let mut dot_product = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    for i in 0..a.len() {
        dot_product += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot_product / (norm_a.sqrt() * norm_b.sqrt())
}

pub fn get_default_department_centroids() -> HashMap<String, Vec<f32>> {
    let centroids = HashMap::new();

    // In a real system these would be actual 1536d embeddings (e.g., from OpenAI/Minimax).
    // For standalone/fallback, we'll use a mocked 3-dimensional space for illustration
    // if using a mock generator, OR we assume the embedding generator produces vectors
    // that align with these. If the real generator is used, these need to be real embeddings
    // or we fetch them from DB.

    // For this implementation, we allow the caller to pass them in, but provide a default
    // empty map or dummy map. It's better to expect the caller to provide them for testing.

    centroids
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::pin::Pin;

    struct MockEmbeddingGenerator {
        // Map prompt content to a deterministic embedding vector
        mock_data: HashMap<String, Vec<f32>>,
    }

    impl EmbeddingGenerator for MockEmbeddingGenerator {
        fn generate_embedding<'a>(&'a self, text: &'a str) -> Pin<Box<dyn std::future::Future<Output = Result<Vec<f32>, String>> + Send + 'a>> {
            let embedding = self.mock_data.get(text).cloned().unwrap_or_else(|| vec![0.0; 3]);
            Box::pin(async move { Ok(embedding) })
        }
    }

    #[tokio::test]
    async fn test_semantic_routing() {
        let mut mock_data = HashMap::new();
        // Marketing vectors
        mock_data.insert("I need a website designed".to_string(), vec![1.0, 0.0, 0.0]);
        // Finance vectors
        mock_data.insert("How do I issue a refund?".to_string(), vec![0.0, 1.0, 0.0]);
        // Operations vectors
        mock_data.insert("How do I manage my inventory?".to_string(), vec![0.0, 0.0, 1.0]);

        let generator = Arc::new(MockEmbeddingGenerator { mock_data });

        let mut centroids = HashMap::new();
        centroids.insert("Marketing".to_string(), vec![1.0, 0.0, 0.0]);
        centroids.insert("Finance".to_string(), vec![0.0, 1.0, 0.0]);
        centroids.insert("Operations".to_string(), vec![0.0, 0.0, 1.0]);

        let router = SemanticRouter::new(generator, centroids);

        let cases = vec![
            ("I need a website designed", "Marketing"),
            ("How do I issue a refund?", "Finance"),
            ("How do I manage my inventory?", "Operations"),
        ];

        for (prompt, expected_department) in cases {
            let req = SemanticRoutingRequest {
                prompt: prompt.to_string(),
                tenant_id: "tenant1".to_string(),
            };

            let res = router.route(&req).await.unwrap();
            assert_eq!(res.assigned_department, expected_department, "Failed for prompt: {}", prompt);
            assert!(res.confidence > 0.9);
        }
    }

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert_eq!(cosine_similarity(&a, &b), 1.0);

        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];
        assert_eq!(cosine_similarity(&a, &b), 0.0);

        let a = vec![1.0, 1.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        // dot product = 1, norm_a = sqrt(2), norm_b = 1
        // result = 1 / sqrt(2) approx 0.707
        assert!((cosine_similarity(&a, &b) - 0.7071).abs() < 0.001);
    }
}
