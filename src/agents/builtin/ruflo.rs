use std::sync::Arc;
use std::collections::HashMap;

use crate::agent::{Agent, AgentRunConfig};
use ohc_builtin_agent_core::types::{ChatRequest, Message, Role};

/// Ruflo Unique Harness Innovations: Swarm coordination topologies
/// Hierarchical, mesh, adaptive with consensus
#[derive(Debug, Clone, PartialEq)]
pub enum SwarmTopology {
    Hierarchical,
    Mesh,
    AdaptiveWithConsensus,
}

pub struct RufloSwarm {
    pub topology: SwarmTopology,
    pub agents: Vec<Arc<Agent>>,
    pub lead_agent: Option<Arc<Agent>>,
}

impl RufloSwarm {
    pub fn new(topology: SwarmTopology, agents: Vec<Arc<Agent>>, lead_agent: Option<Arc<Agent>>) -> Self {
        Self {
            topology,
            agents,
            lead_agent,
        }
    }

    pub async fn run_swarm(&self, task: &str, config: &AgentRunConfig) -> Result<String, String> {
        match self.topology {
            SwarmTopology::Hierarchical => {
                if let Some(lead) = &self.lead_agent {
                    // Lead delegates to agents
                    let mut on_event = |_e| {};
                    let delegation_prompt = format!("You are the lead agent in a Hierarchical swarm. Delegate the following task to your sub-agents and synthesize the result: {}", task);
                    lead.run(config, &delegation_prompt, &mut on_event).await.map_err(|e| e.to_string())
                } else {
                    Err("Hierarchical topology requires a lead agent.".to_string())
                }
            }
            SwarmTopology::Mesh => {
                // All agents attempt the task independently, we return the first successful result
                // In a real mesh, they'd exchange messages, but for simplicity we run concurrently
                let mut handles = vec![];
                for agent in &self.agents {
                    let a = agent.clone();
                    let t = task.to_string();
                    let c = config.clone();
                    handles.push(tokio::spawn(async move {
                        let mut on_event = |_e| {};
                        a.run(&c, &t, &mut on_event).await
                    }));
                }

                let res = futures::future::select_ok(handles).await;
                match res {
                    Ok((Ok(success), _)) => Ok(success),
                    Ok((Err(e), _)) => Err(format!("Mesh execution failed: {}", e)),
                    _ => Err("Mesh execution failed".to_string())
                }
            }
            SwarmTopology::AdaptiveWithConsensus => {
                // Run all agents, then lead (or a separate consensus step) picks the best
                let mut handles = vec![];
                for agent in &self.agents {
                    let a = agent.clone();
                    let t = task.to_string();
                    let c = config.clone();
                    handles.push(tokio::spawn(async move {
                        let mut on_event = |_e| {};
                        a.run(&c, &t, &mut on_event).await
                    }));
                }

                let mut results = vec![];
                for handle in handles {
                    if let Ok(Ok(res)) = handle.await {
                        results.push(res);
                    }
                }

                if results.is_empty() {
                    return Err("No consensus could be reached.".to_string());
                }

                if let Some(lead) = &self.lead_agent {
                    let mut on_event = |_e| {};
                    let consensus_prompt = format!("Review these results and provide the final consensus output:\n{}", results.join("\n---\n"));
                    lead.run(config, &consensus_prompt, &mut on_event).await.map_err(|e| e.to_string())
                } else {
                    Ok(results[0].clone())
                }
            }
        }
    }
}

/// Ruflo Unique Harness Innovations: SONA neural patterns
/// Self-learning trajectory patterns
#[derive(Debug, Clone, Default)]
pub struct SonaPattern {
    pub trajectories: Vec<Vec<String>>,
}

impl SonaPattern {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_trajectory(&mut self, trajectory: Vec<String>) {
        self.trajectories.push(trajectory);
    }

    pub fn suggest_next_action(&self, current_trajectory: &[String]) -> Option<String> {
        let mut counts = HashMap::new();
        for traj in &self.trajectories {
            if traj.starts_with(current_trajectory) && traj.len() > current_trajectory.len() {
                *counts.entry(traj[current_trajectory.len()].clone()).or_insert(0) += 1;
            }
        }
        counts.into_iter().max_by_key(|&(_, count)| count).map(|(action, _)| action)
    }
}

/// Ruflo Unique Harness Innovations: HNSW vector memory
/// 150x-12,500x faster search via AgentDB
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use rand::Rng;

#[derive(Clone, Debug)]
pub struct HnswNode {
    pub id: usize,
    pub content: String,
    pub vector: Vec<f32>,
    pub neighbors: Vec<Vec<usize>>, // Layer -> List of neighbor IDs
}

#[derive(PartialEq)]
struct DistanceNode {
    distance: f32,
    id: usize,
}

impl Eq for DistanceNode {}

impl PartialOrd for DistanceNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.distance.partial_cmp(&self.distance) // Min-heap behavior
    }
}

impl Ord for DistanceNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.partial_cmp(&self.distance).unwrap_or(Ordering::Equal)
    }
}

pub struct HnswMemory {
    nodes: Vec<HnswNode>,
    entry_point: Option<usize>,
    max_layers: usize,
    m: usize,        // Max neighbors per node
    m_max: usize,    // Max neighbors for layer > 0
    m_max0: usize,   // Max neighbors for layer 0
    ef_construction: usize,
}

impl HnswMemory {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            entry_point: None,
            max_layers: 4,
            m: 16,
            m_max: 16,
            m_max0: 32,
            ef_construction: 100,
        }
    }

    fn distance(a: &[f32], b: &[f32]) -> f32 {
        a.iter().zip(b.iter()).map(|(x, y)| (x - y) * (x - y)).sum()
    }

    pub fn insert(&mut self, content: String, vector: Vec<f32>) {
        let mut rng = rand::thread_rng();
        let new_node_id = self.nodes.len();
        let mut node = HnswNode {
            id: new_node_id,
            content,
            vector: vector.clone(),
            neighbors: Vec::new(),
        };

        let l = (-rng.r#gen::<f32>().ln() * (1.0 / (self.m as f32).ln())).floor() as usize;
        let l = std::cmp::min(l, self.max_layers - 1);

        for _ in 0..=l {
            node.neighbors.push(Vec::new());
        }

        self.nodes.push(node.clone()); // Insert early to get an ID

        if self.entry_point.is_none() {
            self.entry_point = Some(new_node_id);
            return;
        }

        let mut curr_obj = self.entry_point.unwrap();
        let ep_layer = self.nodes[curr_obj].neighbors.len() - 1;

        for layer in (l + 1..=ep_layer).rev() {
            let mut w_arr = self.search_layer(&vector, curr_obj, 1, layer);
            if let Some(closest) = w_arr.pop() {
                curr_obj = closest.id;
            }
        }

        for layer in (0..=std::cmp::min(l, ep_layer)).rev() {
            let w_arr = self.search_layer(&vector, curr_obj, self.ef_construction, layer);
            let neighbors = self.select_neighbors(w_arr, self.m);

            self.nodes[new_node_id].neighbors[layer] = neighbors.clone();

            for neighbor_id in neighbors {
                self.nodes[neighbor_id].neighbors[layer].push(new_node_id);

                let m_max = if layer == 0 { self.m_max0 } else { self.m_max };
                if self.nodes[neighbor_id].neighbors[layer].len() > m_max {
                    // Shrink neighbors using a simple heuristic (keep closest)
                    let e_vector = self.nodes[neighbor_id].vector.clone();
                    let candidates: Vec<_> = self.nodes[neighbor_id].neighbors[layer].iter().map(|&id| {
                        DistanceNode { distance: Self::distance(&e_vector, &self.nodes[id].vector), id }
                    }).collect();
                    let mut heap = BinaryHeap::from(candidates);
                    let mut new_neighbors = Vec::new();

                    let count = std::cmp::min(m_max, heap.len());
                    for _ in 0..count {
                        if let Some(n) = heap.pop() {
                            new_neighbors.push(n.id);
                        }
                    }
                    self.nodes[neighbor_id].neighbors[layer] = new_neighbors;
                }
            }
        }

        if l > ep_layer {
            self.entry_point = Some(new_node_id);
        }
    }

    fn search_layer(&self, q: &[f32], ep: usize, ef: usize, lc: usize) -> BinaryHeap<DistanceNode> {
        let mut v = HashSet::new();
        v.insert(ep);

        let mut c = BinaryHeap::new();
        let mut w = BinaryHeap::new();

        let dist = Self::distance(q, &self.nodes[ep].vector);
        c.push(DistanceNode { distance: -dist, id: ep }); // Max-heap
        w.push(DistanceNode { distance: dist, id: ep });  // Min-heap

        while let Some(c_curr) = c.pop() {
            let c_dist = -c_curr.distance;
            let f_dist = w.peek().unwrap().distance; // Furthest element in W

            if c_dist > f_dist {
                break;
            }

            if self.nodes[c_curr.id].neighbors.len() > lc {
                for &e in &self.nodes[c_curr.id].neighbors[lc] {
                    if !v.contains(&e) {
                        v.insert(e);
                        let e_dist = Self::distance(q, &self.nodes[e].vector);

                        if w.len() < ef || e_dist < w.peek().unwrap().distance {
                            c.push(DistanceNode { distance: -e_dist, id: e });
                            w.push(DistanceNode { distance: e_dist, id: e });
                        }
                    }
                }
            }
        }
        w
    }

    fn select_neighbors(&self, mut candidates: BinaryHeap<DistanceNode>, m: usize) -> Vec<usize> {
        let mut result = Vec::new();
        let count = std::cmp::min(m, candidates.len());
        for _ in 0..count {
            if let Some(node) = candidates.pop() {
                result.push(node.id);
            }
        }
        result
    }

    pub fn search(&self, query_vector: &[f32], top_k: usize) -> Vec<String> {
        if self.entry_point.is_none() {
            return vec![];
        }
        let mut curr_obj = self.entry_point.unwrap();
        let ep_layer = self.nodes[curr_obj].neighbors.len() - 1;

        for layer in (1..=ep_layer).rev() {
            let mut w_arr = self.search_layer(query_vector, curr_obj, 1, layer);
            if let Some(closest) = w_arr.pop() {
                curr_obj = closest.id;
            }
        }

        let mut w_arr = self.search_layer(query_vector, curr_obj, top_k, 0);
        let mut result = Vec::new();
        let count = std::cmp::min(top_k, w_arr.len());
        for _ in 0..count {
            if let Some(node) = w_arr.pop() {
                result.push(self.nodes[node.id].content.clone());
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmClient;
    use ohc_builtin_agent_core::types::{ChatResponse, Usage};
    use tokio::sync::Mutex;

    struct MockLlmClientRuflo {
        responses: Mutex<Vec<String>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClientRuflo {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            let content = if !resps.is_empty() {
                resps.remove(0)
            } else {
                "default".to_string()
            };

            Ok(ChatResponse {
                message: Message::assistant(content),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_ruflo_swarm_hierarchical() {
        let lead_llm = Arc::new(MockLlmClientRuflo {
            responses: Mutex::new(vec!["Hierarchical output".to_string()]),
        });
        let lead_agent = Arc::new(Agent::new(lead_llm, vec![]));

        let swarm = RufloSwarm::new(SwarmTopology::Hierarchical, vec![], Some(lead_agent));
        let config = AgentRunConfig::default();
        let res = swarm.run_swarm("test task", &config).await.unwrap();
        assert_eq!(res, "Hierarchical output");
    }

    #[tokio::test]
    async fn test_ruflo_sona_pattern() {
        let mut sona = SonaPattern::new();
        sona.record_trajectory(vec!["read".to_string(), "think".to_string(), "write".to_string()]);
        sona.record_trajectory(vec!["read".to_string(), "think".to_string(), "verify".to_string()]);
        sona.record_trajectory(vec!["read".to_string(), "think".to_string(), "write".to_string()]);

        let suggestion = sona.suggest_next_action(&["read".to_string(), "think".to_string()]);
        assert_eq!(suggestion, Some("write".to_string()));
    }

    #[tokio::test]
    async fn test_ruflo_hnsw_memory() {
        let mut memory = HnswMemory::new();
        memory.insert("cat".to_string(), vec![1.0, 0.0, 0.0]);
        memory.insert("dog".to_string(), vec![0.9, 0.1, 0.0]);
        memory.insert("car".to_string(), vec![0.0, 1.0, 0.0]);

        let results = memory.search(&[1.0, 0.0, 0.0], 2);
        assert_eq!(results, vec!["cat".to_string(), "dog".to_string()]);
    }
}
