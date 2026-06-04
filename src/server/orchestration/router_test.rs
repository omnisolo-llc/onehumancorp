use crate::orchestration::router::{SemanticRouter, SemanticRoutingRequest};

#[tokio::test]
async fn test_semantic_router_basic() {
    let router = SemanticRouter::new();

    let test_cases = vec![
        (
            "I need a new website design",
            "The Promoter",
        ),
        (
            "How do I process a refund?",
            "The Accountant",
        ),
        (
            "Can you help me check my inventory stock?",
            "The Manager",
        ),
        (
            "I want to send a proposal and quote to a new lead",
            "The Salesperson",
        ),
    ];

    for (prompt, expected_department) in test_cases {
        let req = SemanticRoutingRequest {
            tenant_id: "tenant_test_123".to_string(),
            prompt: prompt.to_string(),
        };

        let res = router.route(&req).await.unwrap();
        assert_eq!(
            res.target_department, expected_department,
            "Prompt '{}' routed to '{}', expected '{}'",
            prompt, res.target_department, expected_department
        );
        assert!(res.confidence > 0.0);
    }
}

#[tokio::test]
async fn test_semantic_router_missing_tenant() {
    let router = SemanticRouter::new();

    let req = SemanticRoutingRequest {
        tenant_id: "".to_string(),
        prompt: "I need a new website".to_string(),
    };

    let res = router.route(&req).await;
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "tenant_id is required");
}

#[tokio::test]
async fn test_cosine_similarity() {
    // Note: since cosine_similarity is private, we can't test it directly unless we test through the public API,
    // but we can test the fallback edge cases through route if needed, or make it pub(crate).
    // Let's add a test for a neutral prompt to see it falls back gracefully
    let router = SemanticRouter::new();
    let req = SemanticRoutingRequest {
        tenant_id: "tenant_test_123".to_string(),
        prompt: "hello".to_string(),
    };
    let res = router.route(&req).await.unwrap();
    // Neutral fallback vectors (0.25, 0.25, 0.25, 0.25) have some non-zero similarity with many departments
    // The exact winner might depend on iteration order or tie-breaking, but it should not crash.
    assert!(!res.target_department.is_empty());
}
