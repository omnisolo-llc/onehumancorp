use std::collections::HashMap;
use std::sync::RwLock;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy)]
pub struct Price {
    pub input_per_million_usd: f64,
    pub output_per_million_usd: f64,
    pub cached_per_million_usd: f64,
}

pub struct Tracker {
    catalog: HashMap<String, Price>,
    usages: RwLock<Vec<Usage>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub agent_id: String,
    pub agent_role: String,
    pub organization_id: String,
    pub model: String,
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub cached_tokens: i64,
    pub is_action: bool,
    pub occurred_at: DateTime<Utc>,
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSummary {
    pub agent_id: String,
    pub cost_usd: f64,
    pub token_used: i64,
    pub total_actions: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    pub organization_id: String,
    pub total_cost_usd: f64,
    pub total_tokens: i64,
    pub total_actions: i64,
    pub projected_monthly_usd: f64,
    pub agents: Vec<AgentSummary>,
}

impl Tracker {
    pub fn new() -> Self {
        let mut catalog = HashMap::new();
        // Add all default prices from Go code
        // Anthropic — Claude 3 family
        catalog.insert("claude-3-opus".to_string(), Price { input_per_million_usd: 15.00, output_per_million_usd: 75.00, cached_per_million_usd: 0.0 });
        catalog.insert("claude-3-sonnet".to_string(), Price { input_per_million_usd: 3.00, output_per_million_usd: 15.00, cached_per_million_usd: 0.0 });
        catalog.insert("claude-3-haiku".to_string(), Price { input_per_million_usd: 0.25, output_per_million_usd: 1.25, cached_per_million_usd: 0.0 });
        // Anthropic — Claude 3.5 family
        catalog.insert("claude-3.5-sonnet".to_string(), Price { input_per_million_usd: 3.00, output_per_million_usd: 15.00, cached_per_million_usd: 0.30 });
        catalog.insert("claude-3.5-haiku".to_string(), Price { input_per_million_usd: 0.80, output_per_million_usd: 4.00, cached_per_million_usd: 0.08 });
        // Anthropic — Claude 3.7 family
        catalog.insert("claude-3.7-sonnet".to_string(), Price { input_per_million_usd: 3.00, output_per_million_usd: 15.00, cached_per_million_usd: 0.30 });
        // OpenAI — GPT-4 family
        catalog.insert("gpt-4".to_string(), Price { input_per_million_usd: 30.00, output_per_million_usd: 60.00, cached_per_million_usd: 0.0 });
        catalog.insert("gpt-4-turbo".to_string(), Price { input_per_million_usd: 10.00, output_per_million_usd: 30.00, cached_per_million_usd: 0.0 });
        catalog.insert("gpt-4o".to_string(), Price { input_per_million_usd: 5.00, output_per_million_usd: 15.00, cached_per_million_usd: 2.50 });
        catalog.insert("gpt-4o-mini".to_string(), Price { input_per_million_usd: 0.15, output_per_million_usd: 0.60, cached_per_million_usd: 0.075 });
        // OpenAI — GPT-4.1 family
        catalog.insert("gpt-4.1".to_string(), Price { input_per_million_usd: 2.00, output_per_million_usd: 8.00, cached_per_million_usd: 0.0 });
        catalog.insert("gpt-4.1-mini".to_string(), Price { input_per_million_usd: 0.40, output_per_million_usd: 1.60, cached_per_million_usd: 0.0 });
        catalog.insert("gpt-4.1-nano".to_string(), Price { input_per_million_usd: 0.10, output_per_million_usd: 0.40, cached_per_million_usd: 0.0 });
        // OpenAI — o-series reasoning models
        catalog.insert("o1".to_string(), Price { input_per_million_usd: 15.00, output_per_million_usd: 60.00, cached_per_million_usd: 0.0 });
        catalog.insert("o1-mini".to_string(), Price { input_per_million_usd: 3.00, output_per_million_usd: 12.00, cached_per_million_usd: 0.0 });
        catalog.insert("o3-mini".to_string(), Price { input_per_million_usd: 1.10, output_per_million_usd: 4.40, cached_per_million_usd: 0.0 });
        // Google — Gemini 1.5 family
        catalog.insert("gemini-1.5-pro".to_string(), Price { input_per_million_usd: 3.50, output_per_million_usd: 10.50, cached_per_million_usd: 0.0 });
        catalog.insert("gemini-1.5-flash".to_string(), Price { input_per_million_usd: 0.35, output_per_million_usd: 1.05, cached_per_million_usd: 0.0 });
        // Google — Gemini 2.0 family
        catalog.insert("gemini-2.0-flash".to_string(), Price { input_per_million_usd: 0.10, output_per_million_usd: 0.40, cached_per_million_usd: 0.0 });
        catalog.insert("gemini-2.0-flash-lite".to_string(), Price { input_per_million_usd: 0.075, output_per_million_usd: 0.30, cached_per_million_usd: 0.0 });
        // Google — Gemini 2.5 family
        catalog.insert("gemini-2.5-pro".to_string(), Price { input_per_million_usd: 1.25, output_per_million_usd: 10.00, cached_per_million_usd: 0.0 });
        catalog.insert("gemini-2.5-flash".to_string(), Price { input_per_million_usd: 0.15, output_per_million_usd: 0.60, cached_per_million_usd: 0.0 });
        // MiniMax — M2.7 family
        catalog.insert("minimax-m2.7".to_string(), Price { input_per_million_usd: 1.00, output_per_million_usd: 1.00, cached_per_million_usd: 0.0 });
        catalog.insert("minimax-m2.7-turbo".to_string(), Price { input_per_million_usd: 0.50, output_per_million_usd: 0.50, cached_per_million_usd: 0.0 });

        Tracker {
            catalog,
            usages: RwLock::new(Vec::new()),
        }
    }

    pub fn track(&self, mut usage: Usage) -> Result<Usage, String> {
        let price = self.catalog.get(&usage.model).ok_or("unknown model pricing")?;
        
        usage.cost_usd = (usage.prompt_tokens as f64 / 1_000_000.0) * price.input_per_million_usd +
            (usage.completion_tokens as f64 / 1_000_000.0) * price.output_per_million_usd +
            (usage.cached_tokens as f64 / 1_000_000.0) * price.cached_per_million_usd;
            
        usage.occurred_at = Utc::now();
        
        let mut usages = self.usages.write().unwrap();
        usages.push(usage.clone());
        
        Ok(usage)
    }

    pub fn summary(&self, organization_id: &str) -> Summary {
        let usages = self.usages.read().unwrap();
        
        let mut by_agent: HashMap<String, AgentSummary> = HashMap::new();
        let mut total_cost = 0.0;
        let mut total_tokens = 0;
        let mut total_actions = 0;
        
        for usage in usages.iter() {
            if usage.organization_id != organization_id {
                continue;
            }
            
            let agent = by_agent.entry(usage.agent_id.clone()).or_insert(AgentSummary {
                agent_id: usage.agent_id.clone(),
                cost_usd: 0.0,
                token_used: 0,
                total_actions: 0,
            });
            
            agent.cost_usd += usage.cost_usd;
            let tokens = usage.prompt_tokens + usage.completion_tokens + usage.cached_tokens;
            agent.token_used += tokens;
            if usage.is_action {
                agent.total_actions += 1;
                total_actions += 1;
            }
            
            total_cost += usage.cost_usd;
            total_tokens += tokens;
        }
        
        let mut agents: Vec<AgentSummary> = by_agent.into_values().collect();
        agents.sort_by(|a, b| a.agent_id.cmp(&b.agent_id));
        
        Summary {
            organization_id: organization_id.to_string(),
            total_cost_usd: total_cost,
            total_tokens,
            total_actions,
            projected_monthly_usd: total_cost * 30.0,
            agents,
        }
    }
}

impl Default for Tracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tracker_track_and_summary() {
        let t = Tracker::new();
        
        let usage = Usage {
            agent_id: "agent1".to_string(),
            agent_role: "viewer".to_string(),
            organization_id: "org1".to_string(),
            model: "gpt-4o".to_string(),
            prompt_tokens: 1000,
            completion_tokens: 2000,
            cached_tokens: 500,
            is_action: true,
            occurred_at: Utc::now(),
            cost_usd: 0.0,
        };
        
        let tracked = t.track(usage).unwrap();
        
        // Expected cost:
        // (1000 / 1_000_000) * 5.0 = 0.005
        // (2000 / 1_000_000) * 15.0 = 0.03
        // (500 / 1_000_000) * 2.5 = 0.00125
        // Total = 0.03625
        
        assert_eq!(tracked.cost_usd, 0.03625);
        
        let summary = t.summary("org1");
        assert_eq!(summary.total_cost_usd, 0.03625);
        assert_eq!(summary.total_tokens, 3500);
        assert_eq!(summary.total_actions, 1);
        assert_eq!(summary.agents.len(), 1);
        assert_eq!(summary.agents[0].agent_id, "agent1");
    }

    #[test]
    fn test_tracker_track_gemini_2_5_flash() {
        let t = Tracker::new();

        let usage = Usage {
            agent_id: "agent2".to_string(),
            agent_role: "editor".to_string(),
            organization_id: "org2".to_string(),
            model: "gemini-2.5-flash".to_string(),
            prompt_tokens: 1000000,
            completion_tokens: 1000000,
            cached_tokens: 0,
            is_action: false,
            occurred_at: Utc::now(),
            cost_usd: 0.0,
        };

        let tracked = t.track(usage).unwrap();

        // Expected cost for gemini-2.5-flash:
        // Input: $0.15 / 1M tokens
        // Output: $0.60 / 1M tokens
        // Total = 0.15 + 0.60 = 0.75

        assert_eq!(tracked.cost_usd, 0.75);
    }
}
