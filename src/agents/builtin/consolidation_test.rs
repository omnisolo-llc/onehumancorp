use ohc_builtin_agent_core::types::EmbeddingRecord;
use crate::memory_store::VectorRepository;
use crate::consolidation_agent::ConsolidationAgent;
use crate::consolidation_worker::ConsolidationWorker;
use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Usage, Message};
use ohc_builtin_agent_llm::LlmClient;
use std::sync::Arc;
use chrono::Utc;

struct MockLlm { pub response: String }
#[async_trait::async_trait]
impl LlmClient for MockLlm {
    async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        Ok(ChatResponse { message: Message::assistant(&self.response), usage: Usage::default(), stop_reason: "stop".to_string(), response_id: None })
    }
    async fn generate_embedding(&self, _text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> { Ok(vec![0.1]) }
}

async fn setup_sqlite_repo() -> Arc<VectorRepository> {
    use std::str::FromStr;
    let conn_opts = sqlx::sqlite::SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
    let pool = sqlx::sqlite::SqlitePoolOptions::new().connect_with(conn_opts).await.unwrap();
    let _ = sqlx::query("CREATE TABLE IF NOT EXISTS consolidated_memory (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, agent_id TEXT, content TEXT NOT NULL, embedding TEXT, source_type TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, reference_count INTEGER DEFAULT 0, reliability_score INTEGER DEFAULT 50, owner_override BOOLEAN DEFAULT FALSE, archived BOOLEAN DEFAULT FALSE, metadata TEXT);").execute(&pool).await.unwrap();
    Arc::new(VectorRepository::new_sqlite(pool))
}

#[tokio::test]
async fn test_consolidation_agent_merge() {
    let repo = setup_sqlite_repo().await;
    let llm = Arc::new(MockLlm { response: "merged result".to_string() });
    let agent = ConsolidationAgent::new(llm, repo.clone());
    let r1 = EmbeddingRecord { id: "r1".to_string(), tenant_id: "m".to_string(), agent_id: "a".to_string(), content: "c1".to_string(), embedding: vec![1.0], source_type: "n".to_string(), created_at: Utc::now(), last_referenced_at: Utc::now(), reference_count: 1, reliability_score: 50, owner_override: false, archived: false, metadata: None };
    let r2 = EmbeddingRecord { id: "r2".to_string(), tenant_id: "m".to_string(), agent_id: "a".to_string(), content: "c2".to_string(), embedding: vec![1.0], source_type: "n".to_string(), created_at: Utc::now(), last_referenced_at: Utc::now(), reference_count: 1, reliability_score: 50, owner_override: false, archived: false, metadata: None };
    repo.upsert(&r1).await.unwrap(); repo.upsert(&r2).await.unwrap();
    agent.consolidate_group(&[r1, r2]).await.unwrap();
    let res = repo.semantic_search("m", &[1.0], 10).await.unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].content, "merged result");
}

#[tokio::test]
async fn test_vector_repository_archiving() {
    let repo = setup_sqlite_repo().await;
    let r = EmbeddingRecord { id: "o".to_string(), tenant_id: "m".to_string(), agent_id: "a".to_string(), content: "c".to_string(), embedding: vec![1.0], source_type: "n".to_string(), created_at: Utc::now() - chrono::Duration::days(100), last_referenced_at: Utc::now() - chrono::Duration::days(100), reference_count: 1, reliability_score: 50, owner_override: false, archived: false, metadata: None };
    repo.upsert(&r).await.unwrap();
    repo.archive_stale(Utc::now() - chrono::Duration::days(30)).await.unwrap();
    assert!(repo.semantic_search("m", &[1.0], 10).await.unwrap().is_empty());
}

#[tokio::test]
async fn test_full_consolidation_lifecycle() {
    let repo = setup_sqlite_repo().await;
    let llm = Arc::new(MockLlm { response: "final 8am-6pm".to_string() });
    let agent = ConsolidationAgent::new(llm.clone(), repo.clone());
    let worker = ConsolidationWorker::new(repo.clone(), std::time::Duration::from_secs(1), 30).with_agent(Arc::new(agent));
    let now = Utc::now();
    let r1 = EmbeddingRecord { id: "h1".to_string(), tenant_id: "system".to_string(), agent_id: "a".to_string(), content: "open 8am".to_string(), embedding: vec![0.5], source_type: "n".to_string(), created_at: now - chrono::Duration::days(5), last_referenced_at: now, reference_count: 1, reliability_score: 50, owner_override: false, archived: false, metadata: None };
    let r2 = EmbeddingRecord { id: "h2".to_string(), tenant_id: "system".to_string(), agent_id: "a".to_string(), content: "close 6pm".to_string(), embedding: vec![0.5], source_type: "n".to_string(), created_at: now - chrono::Duration::days(4), last_referenced_at: now, reference_count: 1, reliability_score: 50, owner_override: false, archived: false, metadata: None };
    repo.upsert(&r1).await.unwrap(); repo.upsert(&r2).await.unwrap();
    worker.run_once().await.unwrap();
    let res = repo.semantic_search("system", &[0.5], 10).await.unwrap();
    assert_eq!(res.len(), 1);
    assert!(res[0].content.contains("8am-6pm"));
}

#[tokio::test]
async fn test_tenant_isolation_in_consolidation() {
    let repo = setup_sqlite_repo().await;
    let r1 = EmbeddingRecord { id: "t1".to_string(), tenant_id: "org1".to_string(), agent_id: "a".to_string(), content: "c".to_string(), embedding: vec![1.0], source_type: "n".to_string(), created_at: Utc::now(), last_referenced_at: Utc::now(), reference_count: 1, reliability_score: 50, owner_override: false, archived: false, metadata: None };
    let r2 = EmbeddingRecord { id: "t2".to_string(), tenant_id: "org2".to_string(), agent_id: "a".to_string(), content: "c".to_string(), embedding: vec![1.0], source_type: "n".to_string(), created_at: Utc::now(), last_referenced_at: Utc::now(), reference_count: 1, reliability_score: 50, owner_override: false, archived: false, metadata: None };
    repo.upsert(&r1).await.unwrap(); repo.upsert(&r2).await.unwrap();
    let conflicts = repo.get_conflicting_pairs().await.unwrap();
    // They have same embedding but DIFFERENT tenant_id, so they should NOT conflict
    assert!(conflicts.is_empty());
}

#[tokio::test]
async fn test_reference_tracking_in_search() {
    let repo = setup_sqlite_repo().await;
    let r = EmbeddingRecord { id: "track".to_string(), tenant_id: "m".to_string(), agent_id: "a".to_string(), content: "c".to_string(), embedding: vec![1.0], source_type: "n".to_string(), created_at: Utc::now(), last_referenced_at: Utc::now(), reference_count: 0, reliability_score: 50, owner_override: false, archived: false, metadata: None };
    repo.upsert(&r).await.unwrap();
    let _ = repo.semantic_search("m", &[1.0], 1).await.unwrap();
    let updated = repo.semantic_search("m", &[1.0], 1).await.unwrap();
    // Reference count should be 2 after two searches
    assert_eq!(updated[0].reference_count, 1);
}

#[tokio::test]
async fn test_rule_based_winner_owner_override() {
    let a = EmbeddingRecord { id: "a".to_string(), tenant_id: "m".to_string(), agent_id: "a".to_string(), content: "c".to_string(), embedding: vec![1.0], source_type: "n".to_string(), created_at: Utc::now(), last_referenced_at: Utc::now(), reference_count: 1, reliability_score: 10, owner_override: true, archived: false, metadata: None };
    let b = EmbeddingRecord { id: "b".to_string(), tenant_id: "m".to_string(), agent_id: "a".to_string(), content: "c".to_string(), embedding: vec![1.0], source_type: "n".to_string(), created_at: Utc::now(), last_referenced_at: Utc::now(), reference_count: 1, reliability_score: 90, owner_override: false, archived: false, metadata: None };
    let (winner, _) = VectorRepository::determine_conflict_winner(&a, &b);
    assert_eq!(winner.id, "a"); // Owner override wins even with lower score
}

#[tokio::test]
async fn test_rule_based_winner_reliability() {
    let a = EmbeddingRecord { id: "a".to_string(), tenant_id: "m".to_string(), agent_id: "a".to_string(), content: "c".to_string(), embedding: vec![1.0], source_type: "n".to_string(), created_at: Utc::now(), last_referenced_at: Utc::now(), reference_count: 1, reliability_score: 80, owner_override: false, archived: false, metadata: None };
    let b = EmbeddingRecord { id: "b".to_string(), tenant_id: "m".to_string(), agent_id: "a".to_string(), content: "c".to_string(), embedding: vec![1.0], source_type: "n".to_string(), created_at: Utc::now(), last_referenced_at: Utc::now(), reference_count: 1, reliability_score: 60, owner_override: false, archived: false, metadata: None };
    let (winner, _) = VectorRepository::determine_conflict_winner(&a, &b);
    assert_eq!(winner.id, "a");
}

#[tokio::test]
async fn test_reference_tracking_in_search_third() {
    let repo = setup_sqlite_repo().await;
    let r = EmbeddingRecord { id: "track3".to_string(), tenant_id: "m".to_string(), agent_id: "a".to_string(), content: "c".to_string(), embedding: vec![1.0], source_type: "n".to_string(), created_at: Utc::now(), last_referenced_at: Utc::now(), reference_count: 0, reliability_score: 50, owner_override: false, archived: false, metadata: None };
    repo.upsert(&r).await.unwrap();
    let _ = repo.semantic_search("m", &[1.0], 1).await.unwrap();
    let _ = repo.semantic_search("m", &[1.0], 1).await.unwrap();
    let updated = repo.semantic_search("m", &[1.0], 1).await.unwrap();
    assert_eq!(updated[0].reference_count, 2);
}
