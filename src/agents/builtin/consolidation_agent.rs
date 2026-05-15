use ohc_builtin_agent_core::types::EmbeddingRecord;
use crate::memory_store::VectorRepository;
use ohc_builtin_agent_core::types::{ChatRequest, Message};
use ohc_builtin_agent_llm::LlmClient;
use std::sync::Arc;
use chrono::Utc;

pub struct ConsolidationAgent {
    pub llm: Arc<dyn LlmClient>,
    pub repo: Arc<VectorRepository>,
}

impl ConsolidationAgent {
    pub fn new(llm: Arc<dyn LlmClient>, repo: Arc<VectorRepository>) -> Self {
        Self { llm, repo }
    }

    pub async fn consolidate_group(&self, records: &[EmbeddingRecord]) -> Result<(), String> {
        if records.len() < 2 { return Ok(()); }
        let tenant_id = records[0].tenant_id.clone();
        let mut context = String::new();
        for (i, rec) in records.iter().enumerate() {
            context.push_str(&format!("Record {}:\nContent: {}\nSource: {}\nCreated At: {}\nReliability: {}\nOwner Override: {}\n\n",
                i + 1, rec.content, rec.source_type, rec.created_at, rec.reliability_score, rec.owner_override));
        }

        let prompt = format!(
            "You are an expert knowledge consolidation agent. Your task is to merge the following pieces of memory into a single, high-quality 'Golden Record'.\n\n\
            RECORDS TO CONSOLIDATE:\n{}\n\n\
            INSTRUCTIONS:\n\
            1. Identify redundant information and merge it.\n\
            2. Resolve conflicts. Favor information from 'Owner Override' records or higher 'Reliability' scores. If recency is the only differentiator, favor the newest information.\n\
            3. The output should be a single, concise, and accurate summary that retains all unique and valuable facts.\n\
            4. Respond ONLY with the consolidated content text. No labels, no preamble.\n\n\
            CONSOLIDATED CONTENT:",
            context
        );

        let req = ChatRequest {
            model: "gpt-4o".to_string(),
            system: "You are a helpful knowledge consolidation agent.".to_string(),
            messages: vec![Message::user(prompt)],
            tools: vec![],
            max_tokens: 1024,
            temperature: 0.1,
        };

        let response = self.llm.chat(req).await.map_err(|e| e.to_string())?;
        let consolidated_content = response.message.content.trim().to_string();
        if consolidated_content.is_empty() { return Err("LLM returned empty consolidation content".to_string()); }

        let embedding = self.llm.generate_embedding(&consolidated_content).await.map_err(|e| e.to_string())?;
        let winner = records.iter().max_by(|a, b| {
            if a.owner_override != b.owner_override { a.owner_override.cmp(&b.owner_override) }
            else if a.reliability_score != b.reliability_score { a.reliability_score.cmp(&b.reliability_score) }
            else { a.created_at.cmp(&b.created_at) }
        }).unwrap();

        let golden_record = EmbeddingRecord {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            agent_id: "consolidation_agent".to_string(),
            content: consolidated_content,
            embedding,
            source_type: "GOLDEN_RECORD".to_string(),
            created_at: Utc::now(),
            last_referenced_at: Utc::now(),
            reference_count: records.iter().map(|r| r.reference_count).sum::<i32>() + 1,
            reliability_score: winner.reliability_score.max(90),
            owner_override: records.iter().any(|r| r.owner_override),
            archived: false,
            metadata: Some(format!(r#"{{"consolidated_from": {:?}}}"#, records.iter().map(|r| &r.id).collect::<Vec<_>>())),
        };

        self.repo.upsert(&golden_record).await?;
        for rec in records { self.repo.delete(&rec.id).await?; }
        Ok(())
    }

    pub async fn auto_consolidate(&self, tenant_id: &str) -> Result<usize, String> {
        let groups = self.repo.consolidate_records(tenant_id, 10).await?;
        let mut count = 0;
        for group in groups {
            if let Err(e) = self.consolidate_group(&group).await {
                tracing::error!("Failed to consolidate group for tenant {}: {}", tenant_id, e);
                continue;
            }
            count += 1;
        }
        Ok(count)
    }
}
