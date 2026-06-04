use super::router::*;
use async_trait::async_trait;
use std::sync::Arc;

struct MockEmbeddingProvider;

#[async_trait]
impl EmbeddingProvider for MockEmbeddingProvider {
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, String> {
        let text_lower = text.to_lowercase();
        // Give explicit mock embeddings based on text match to simulate accurate routing
        // Order of intents:
        // 0: Operations, 1: Marketing, 2: Sales, 3: Customer Success, 4: Finance, 5: Legal, 6: Advisory

        if text_lower.contains("deposits") || text_lower.contains("financial") {
            Ok(vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0]) // Match Finance
        } else if text_lower.contains("refund") || text_lower.contains("inventory") || text_lower.contains("vegan cake") || text_lower.contains("execution") {
            Ok(vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) // Match Operations
        } else if text_lower.contains("website") || text_lower.contains("logo") || text_lower.contains("promotional") {
            Ok(vec![0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]) // Match Marketing
        } else if text_lower.contains("instagram dm") || text_lower.contains("messages") || text_lower.contains("responds") {
            Ok(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0]) // Match Customer Success
        } else if text_lower.contains("terms of service") || text_lower.contains("privacy") || text_lower.contains("licenses") {
            Ok(vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0]) // Match Legal
        } else if text_lower.contains("sales down") || text_lower.contains("reports") || text_lower.contains("performance analysis") {
            Ok(vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0]) // Match Advisory
        } else if text_lower.contains("quotes") || text_lower.contains("prospects") || text_lower.contains("generates quotes") {
             Ok(vec![0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0]) // Match Sales
        } else {
             Ok(vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]) // default to Operations
        }
    }
}

#[tokio::test]
async fn test_semantic_router_table_driven() {
    let mock_provider = Arc::new(MockEmbeddingProvider);
    let router = SemanticRouter::new(mock_provider.clone(), None).await.unwrap();

    let test_cases = vec![
        ("Help me set up taking deposits for custom cakes.", "Finance"),
        ("I need to add a new vegan cake to my menu.", "Operations"),
        ("Can you help me design a new logo and put it on my website?", "Marketing"),
        ("A customer wants a refund for their late order.", "Operations"),
        ("How do I respond to this Instagram DM asking about prices?", "Customer Success"),
        ("What should my terms of service say?", "Legal"),
        ("Why are my sales down this month?", "Advisory"),
        ("Generate quotes for my new prospects", "Sales"),
    ];

    for (prompt, expected_department) in test_cases {
        let req = SemanticRoutingRequest {
            prompt: prompt.to_string(),
            tenant_id: "test_tenant".to_string(),
        };

        let res = router.route(req).await.unwrap();
        assert_eq!(res.department, expected_department, "Failed for prompt: '{}'", prompt);
    }
}
