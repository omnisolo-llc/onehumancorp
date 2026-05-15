use std::collections::{HashMap, HashSet, BinaryHeap, VecDeque};
use std::cmp::Ordering;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NodeId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EdgeId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Concept,
    Entity,
    Action,
    Observation,
    Belief,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryNode {
    pub id: NodeId,
    pub node_type: NodeType,
    pub label: String,
    pub description: String,
    pub importance_score: u32,
    pub embedding: Option<Vec<f32>>,
    pub created_at: i64,
    pub last_accessed: i64,
    pub access_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdgeType {
    RelatesTo,
    Causes,
    Implies,
    PartOf,
    Requires,
    Contradicts,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEdge {
    pub id: EdgeId,
    pub source: NodeId,
    pub target: NodeId,
    pub edge_type: EdgeType,
    pub weight: f32,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct ShortestPathResult {
    pub path: Vec<NodeId>,
    pub total_weight: f32,
}

#[derive(Clone)]
struct State {
    cost: f32,
    position: NodeId,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.position == other.position
    }
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
            .then_with(|| self.position.0.cmp(&other.position.0))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct AgentKnowledgeGraph {
    nodes: RwLock<HashMap<NodeId, MemoryNode>>,
    edges: RwLock<HashMap<EdgeId, MemoryEdge>>,
    adjacency_list: RwLock<HashMap<NodeId, Vec<EdgeId>>>,
    reverse_adjacency: RwLock<HashMap<NodeId, Vec<EdgeId>>>,
    max_nodes: usize,
}

impl AgentKnowledgeGraph {
    pub fn new(max_nodes: usize) -> Self {
        Self {
            nodes: RwLock::new(HashMap::new()),
            edges: RwLock::new(HashMap::new()),
            adjacency_list: RwLock::new(HashMap::new()),
            reverse_adjacency: RwLock::new(HashMap::new()),
            max_nodes,
        }
    }

    pub fn add_node(&self, node: MemoryNode) -> Result<(), String> {
        let mut nodes = self.nodes.write().map_err(|_| "Poisoned lock".to_string())?;

        if nodes.len() >= self.max_nodes && !nodes.contains_key(&node.id) {
            self.evict_least_important_unlocked(&mut nodes)?;
        }

        let mut adj = self.adjacency_list.write().map_err(|_| "Poisoned lock".to_string())?;
        let mut rev_adj = self.reverse_adjacency.write().map_err(|_| "Poisoned lock".to_string())?;

        if !adj.contains_key(&node.id) {
            adj.insert(node.id.clone(), Vec::new());
        }
        if !rev_adj.contains_key(&node.id) {
            rev_adj.insert(node.id.clone(), Vec::new());
        }

        nodes.insert(node.id.clone(), node);
        Ok(())
    }

    fn evict_least_important_unlocked(&self, nodes: &mut HashMap<NodeId, MemoryNode>) -> Result<(), String> {
        if nodes.is_empty() {
            return Ok(());
        }

        // Combine importance score, access count, and recency for an eviction heuristic
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let mut least_important_id = None;
        let mut lowest_score = f32::MAX;

        for (id, node) in nodes.iter() {
            let age = (current_time - node.last_accessed) as f32;
            let score = (node.importance_score as f32) * (node.access_count as f32) / (age + 1.0);

            if score < lowest_score {
                lowest_score = score;
                least_important_id = Some(id.clone());
            }
        }

        if let Some(id) = least_important_id {
            nodes.remove(&id);
            self.remove_all_edges_for_node(&id)?;
        }

        Ok(())
    }

    fn remove_all_edges_for_node(&self, node_id: &NodeId) -> Result<(), String> {
        let mut edges = self.edges.write().map_err(|_| "Poisoned lock".to_string())?;
        let mut adj = self.adjacency_list.write().map_err(|_| "Poisoned lock".to_string())?;
        let mut rev_adj = self.reverse_adjacency.write().map_err(|_| "Poisoned lock".to_string())?;

        let outgoing = adj.remove(node_id).unwrap_or_default();
        let incoming = rev_adj.remove(node_id).unwrap_or_default();

        for edge_id in outgoing.iter().chain(incoming.iter()) {
            edges.remove(edge_id);
        }

        // Clean up references in other nodes' adjacency lists
        for edge_id in incoming {
            if let Some(edge) = edges.get(&edge_id) {
                if let Some(list) = adj.get_mut(&edge.source) {
                    list.retain(|e| e != &edge_id);
                }
            }
        }

        for edge_id in outgoing {
            if let Some(edge) = edges.get(&edge_id) {
                if let Some(list) = rev_adj.get_mut(&edge.target) {
                    list.retain(|e| e != &edge_id);
                }
            }
        }

        Ok(())
    }

    pub fn add_edge(&self, edge: MemoryEdge) -> Result<(), String> {
        let nodes = self.nodes.read().map_err(|_| "Poisoned lock".to_string())?;
        if !nodes.contains_key(&edge.source) || !nodes.contains_key(&edge.target) {
            return Err("Source or target node does not exist".to_string());
        }

        let mut edges = self.edges.write().map_err(|_| "Poisoned lock".to_string())?;
        let mut adj = self.adjacency_list.write().map_err(|_| "Poisoned lock".to_string())?;
        let mut rev_adj = self.reverse_adjacency.write().map_err(|_| "Poisoned lock".to_string())?;

        adj.entry(edge.source.clone()).or_default().push(edge.id.clone());
        rev_adj.entry(edge.target.clone()).or_default().push(edge.id.clone());
        edges.insert(edge.id.clone(), edge);

        Ok(())
    }

    pub fn get_node(&self, id: &NodeId) -> Result<Option<MemoryNode>, String> {
        let mut nodes = self.nodes.write().map_err(|_| "Poisoned lock".to_string())?;
        if let Some(node) = nodes.get_mut(id) {
            node.access_count += 1;
            node.last_accessed = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;
            Ok(Some(node.clone()))
        } else {
            Ok(None)
        }
    }

    pub fn search_by_label(&self, query: &str) -> Result<Vec<MemoryNode>, String> {
        let nodes = self.nodes.read().map_err(|_| "Poisoned lock".to_string())?;
        let mut results = Vec::new();
        let query_lower = query.to_lowercase();

        for node in nodes.values() {
            if node.label.to_lowercase().contains(&query_lower) ||
               node.description.to_lowercase().contains(&query_lower) {
                results.push(node.clone());
            }
        }

        results.sort_by(|a, b| b.importance_score.cmp(&a.importance_score));
        Ok(results)
    }

    pub fn find_shortest_path(&self, start: &NodeId, goal: &NodeId) -> Result<Option<ShortestPathResult>, String> {
        let nodes = self.nodes.read().map_err(|_| "Poisoned lock".to_string())?;
        let edges = self.edges.read().map_err(|_| "Poisoned lock".to_string())?;
        let adj = self.adjacency_list.read().map_err(|_| "Poisoned lock".to_string())?;

        if !nodes.contains_key(start) || !nodes.contains_key(goal) {
            return Ok(None);
        }

        let mut distances: HashMap<NodeId, f32> = HashMap::new();
        let mut previous: HashMap<NodeId, NodeId> = HashMap::new();
        let mut heap = BinaryHeap::new();

        for node_id in nodes.keys() {
            distances.insert(node_id.clone(), f32::MAX);
        }

        distances.insert(start.clone(), 0.0);
        heap.push(State { cost: 0.0, position: start.clone() });

        while let Some(State { cost, position }) = heap.pop() {
            if position == *goal {
                let mut path = Vec::new();
                let mut current = goal.clone();
                while current != *start {
                    path.push(current.clone());
                    current = previous.get(&current).unwrap().clone();
                }
                path.push(start.clone());
                path.reverse();

                return Ok(Some(ShortestPathResult {
                    path,
                    total_weight: cost,
                }));
            }

            if cost > *distances.get(&position).unwrap_or(&f32::MAX) {
                continue;
            }

            if let Some(outgoing_edges) = adj.get(&position) {
                for edge_id in outgoing_edges {
                    if let Some(edge) = edges.get(edge_id) {
                        let next = State {
                            cost: cost + edge.weight,
                            position: edge.target.clone(),
                        };

                        if next.cost < *distances.get(&next.position).unwrap_or(&f32::MAX) {
                            heap.push(next.clone());
                            distances.insert(next.position.clone(), next.cost);
                            previous.insert(next.position.clone(), position.clone());
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    pub fn get_neighborhood(&self, center: &NodeId, depth: u32) -> Result<Vec<MemoryNode>, String> {
        let nodes = self.nodes.read().map_err(|_| "Poisoned lock".to_string())?;
        let adj = self.adjacency_list.read().map_err(|_| "Poisoned lock".to_string())?;
        let edges = self.edges.read().map_err(|_| "Poisoned lock".to_string())?;

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        queue.push_back((center.clone(), 0));
        visited.insert(center.clone());

        while let Some((current_id, current_depth)) = queue.pop_front() {
            if let Some(node) = nodes.get(&current_id) {
                result.push(node.clone());
            }

            if current_depth < depth {
                if let Some(outgoing) = adj.get(&current_id) {
                    for edge_id in outgoing {
                        if let Some(edge) = edges.get(edge_id) {
                            if !visited.contains(&edge.target) {
                                visited.insert(edge.target.clone());
                                queue.push_back((edge.target.clone(), current_depth + 1));
                            }
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    pub fn calculate_pagerank(&self, iterations: usize, damping_factor: f32) -> Result<HashMap<NodeId, f32>, String> {
        let nodes = self.nodes.read().map_err(|_| "Poisoned lock".to_string())?;
        let edges = self.edges.read().map_err(|_| "Poisoned lock".to_string())?;
        let rev_adj = self.reverse_adjacency.read().map_err(|_| "Poisoned lock".to_string())?;
        let adj = self.adjacency_list.read().map_err(|_| "Poisoned lock".to_string())?;

        let num_nodes = nodes.len() as f32;
        if num_nodes == 0.0 {
            return Ok(HashMap::new());
        }

        let initial_rank = 1.0 / num_nodes;
        let mut ranks: HashMap<NodeId, f32> = nodes.keys().map(|id| (id.clone(), initial_rank)).collect();
        let mut out_degree: HashMap<NodeId, usize> = HashMap::new();

        for (id, outgoing) in adj.iter() {
            out_degree.insert(id.clone(), outgoing.len());
        }

        for _ in 0..iterations {
            let mut new_ranks: HashMap<NodeId, f32> = HashMap::new();
            let mut sink_rank = 0.0;

            for (node_id, rank) in ranks.iter() {
                if *out_degree.get(node_id).unwrap_or(&0) == 0 {
                    sink_rank += rank;
                }
            }

            for node_id in nodes.keys() {
                let mut rank_sum = 0.0;

                if let Some(incoming_edges) = rev_adj.get(node_id) {
                    for edge_id in incoming_edges {
                        if let Some(edge) = edges.get(edge_id) {
                            let source = &edge.source;
                            let source_out_degree = *out_degree.get(source).unwrap_or(&1) as f32;
                            rank_sum += ranks.get(source).unwrap_or(&0.0) / source_out_degree;
                        }
                    }
                }

                let new_rank = (1.0 - damping_factor) / num_nodes
                             + damping_factor * (rank_sum + sink_rank / num_nodes);
                new_ranks.insert(node_id.clone(), new_rank);
            }

            ranks = new_ranks;
        }

        Ok(ranks)
    }

    pub fn cosine_similarity(vec1: &[f32], vec2: &[f32]) -> f32 {
        if vec1.len() != vec2.len() || vec1.is_empty() {
            return 0.0;
        }

        let mut dot_product = 0.0;
        let mut norm1 = 0.0;
        let mut norm2 = 0.0;

        for (a, b) in vec1.iter().zip(vec2.iter()) {
            dot_product += a * b;
            norm1 += a * a;
            norm2 += b * b;
        }

        if norm1 == 0.0 || norm2 == 0.0 {
            return 0.0;
        }

        dot_product / (norm1.sqrt() * norm2.sqrt())
    }

    pub fn semantic_search(&self, query_embedding: &[f32], top_k: usize) -> Result<Vec<(MemoryNode, f32)>, String> {
        let nodes = self.nodes.read().map_err(|_| "Poisoned lock".to_string())?;
        let mut results = Vec::new();

        for node in nodes.values() {
            if let Some(embedding) = &node.embedding {
                let sim = Self::cosine_similarity(query_embedding, embedding);
                results.push((node.clone(), sim));
            }
        }

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
        results.truncate(top_k);
        Ok(results)
    }

    pub fn clear(&self) -> Result<(), String> {
        self.nodes.write().map_err(|_| "Poisoned lock".to_string())?.clear();
        self.edges.write().map_err(|_| "Poisoned lock".to_string())?.clear();
        self.adjacency_list.write().map_err(|_| "Poisoned lock".to_string())?.clear();
        self.reverse_adjacency.write().map_err(|_| "Poisoned lock".to_string())?.clear();
        Ok(())
    }

    pub fn export_subgraph(&self, root_nodes: &[NodeId], max_depth: u32) -> Result<(Vec<MemoryNode>, Vec<MemoryEdge>), String> {
        let nodes = self.nodes.read().map_err(|_| "Poisoned lock".to_string())?;
        let edges = self.edges.read().map_err(|_| "Poisoned lock".to_string())?;
        let adj = self.adjacency_list.read().map_err(|_| "Poisoned lock".to_string())?;

        let mut visited_nodes = HashSet::new();
        let mut included_nodes = Vec::new();
        let mut included_edges = Vec::new();

        let mut queue = VecDeque::new();
        for root in root_nodes {
            queue.push_back((root.clone(), 0));
            visited_nodes.insert(root.clone());
        }

        while let Some((node_id, depth)) = queue.pop_front() {
            if let Some(node) = nodes.get(&node_id) {
                included_nodes.push(node.clone());
            }

            if depth < max_depth {
                if let Some(outgoing) = adj.get(&node_id) {
                    for edge_id in outgoing {
                        if let Some(edge) = edges.get(edge_id) {
                            included_edges.push(edge.clone());

                            if !visited_nodes.contains(&edge.target) {
                                visited_nodes.insert(edge.target.clone());
                                queue.push_back((edge.target.clone(), depth + 1));
                            }
                        }
                    }
                }
            }
        }

        Ok((included_nodes, included_edges))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_node(id: &str, importance: u32) -> MemoryNode {
        MemoryNode {
            id: NodeId(id.to_string()),
            node_type: NodeType::Concept,
            label: id.to_string(),
            description: format!("Node {}", id),
            importance_score: importance,
            embedding: None,
            created_at: 0,
            last_accessed: 0,
            access_count: 0,
        }
    }

    fn create_test_edge(id: &str, source: &str, target: &str, weight: f32) -> MemoryEdge {
        MemoryEdge {
            id: EdgeId(id.to_string()),
            source: NodeId(source.to_string()),
            target: NodeId(target.to_string()),
            edge_type: EdgeType::RelatesTo,
            weight,
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn test_graph_add_and_get() {
        let graph = AgentKnowledgeGraph::new(100);
        let node = create_test_node("A", 10);
        graph.add_node(node.clone()).unwrap();

        let retrieved = graph.get_node(&NodeId("A".to_string())).unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id, node.id);
    }

    #[test]
    fn test_graph_eviction() {
        let graph = AgentKnowledgeGraph::new(2);

        graph.add_node(create_test_node("A", 10)).unwrap();
        graph.add_node(create_test_node("B", 5)).unwrap();

        // Node C should trigger eviction of B (lowest importance)
        graph.add_node(create_test_node("C", 20)).unwrap();

        let a = graph.get_node(&NodeId("A".to_string())).unwrap();
        let b = graph.get_node(&NodeId("B".to_string())).unwrap();
        let c = graph.get_node(&NodeId("C".to_string())).unwrap();

        assert!(a.is_some());
        assert!(b.is_none());
        assert!(c.is_some());
    }

    #[test]
    fn test_shortest_path() {
        let graph = AgentKnowledgeGraph::new(100);

        graph.add_node(create_test_node("A", 10)).unwrap();
        graph.add_node(create_test_node("B", 10)).unwrap();
        graph.add_node(create_test_node("C", 10)).unwrap();

        graph.add_edge(create_test_edge("e1", "A", "B", 1.0)).unwrap();
        graph.add_edge(create_test_edge("e2", "B", "C", 2.0)).unwrap();
        graph.add_edge(create_test_edge("e3", "A", "C", 4.0)).unwrap();

        let path = graph.find_shortest_path(&NodeId("A".to_string()), &NodeId("C".to_string())).unwrap();
        assert!(path.is_some());

        let result = path.unwrap();
        assert_eq!(result.total_weight, 3.0);
        assert_eq!(result.path, vec![NodeId("A".to_string()), NodeId("B".to_string()), NodeId("C".to_string())]);
    }
}

// --- Extended Graph Algorithms & Analytics ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cluster {
    pub id: String,
    pub nodes: HashSet<NodeId>,
    pub centroid: Option<Vec<f32>>,
}

impl AgentKnowledgeGraph {
    /// Computes the Betweenness Centrality for all nodes to identify key concepts or bottlenecks
    /// in the agent's memory representation.
    pub fn betweenness_centrality(&self) -> Result<HashMap<NodeId, f32>, String> {
        let nodes = self.nodes.read().map_err(|_| "Poisoned lock".to_string())?;
        let adj = self.adjacency_list.read().map_err(|_| "Poisoned lock".to_string())?;
        let edges = self.edges.read().map_err(|_| "Poisoned lock".to_string())?;

        let mut centrality = HashMap::new();
        for node_id in nodes.keys() {
            centrality.insert(node_id.clone(), 0.0);
        }

        // Brandes' algorithm for unweighted betweenness centrality
        for s in nodes.keys() {
            let mut stack = Vec::new();
            let mut paths: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
            let mut sigma: HashMap<NodeId, f32> = HashMap::new();
            let mut d: HashMap<NodeId, i32> = HashMap::new();

            for v in nodes.keys() {
                paths.insert(v.clone(), Vec::new());
                sigma.insert(v.clone(), 0.0);
                d.insert(v.clone(), -1);
            }

            sigma.insert(s.clone(), 1.0);
            d.insert(s.clone(), 0);

            let mut queue = VecDeque::new();
            queue.push_back(s.clone());

            while let Some(v) = queue.pop_front() {
                stack.push(v.clone());

                if let Some(outgoing) = adj.get(&v) {
                    for edge_id in outgoing {
                        if let Some(edge) = edges.get(edge_id) {
                            let w = &edge.target;

                            if *d.get(w).unwrap() < 0 {
                                queue.push_back(w.clone());
                                d.insert(w.clone(), d.get(&v).unwrap() + 1);
                            }

                            if *d.get(w).unwrap() == d.get(&v).unwrap() + 1 {
                                let new_sigma = sigma.get(w).unwrap() + sigma.get(&v).unwrap();
                                sigma.insert(w.clone(), new_sigma);
                                paths.get_mut(w).unwrap().push(v.clone());
                            }
                        }
                    }
                }
            }

            let mut delta: HashMap<NodeId, f32> = HashMap::new();
            for v in nodes.keys() {
                delta.insert(v.clone(), 0.0);
            }

            while let Some(w) = stack.pop() {
                if let Some(pred) = paths.get(&w) {
                    for v in pred {
                        let c = (sigma.get(v).unwrap() / sigma.get(&w).unwrap()) * (1.0 + delta.get(&w).unwrap());
                        let new_delta = delta.get(v).unwrap() + c;
                        delta.insert(v.clone(), new_delta);
                    }
                }

                if w != *s {
                    let new_cent = centrality.get(&w).unwrap() + delta.get(&w).unwrap();
                    centrality.insert(w.clone(), new_cent);
                }
            }
        }

        Ok(centrality)
    }

    /// Performs K-Means clustering on the graph nodes based on their embeddings.
    /// This allows the agent to segment memories into thematic groups.
    pub fn cluster_nodes_kmeans(&self, k: usize, max_iterations: usize) -> Result<Vec<Cluster>, String> {
        let nodes = self.nodes.read().map_err(|_| "Poisoned lock".to_string())?;

        let mut embed_nodes: Vec<(&NodeId, &Vec<f32>)> = Vec::new();
        for (id, node) in nodes.iter() {
            if let Some(embed) = &node.embedding {
                embed_nodes.push((id, embed));
            }
        }

        if embed_nodes.is_empty() || k == 0 {
            return Ok(Vec::new());
        }

        let k = k.min(embed_nodes.len());
        let embed_dim = embed_nodes[0].1.len();

        // Initialize centroids
        let mut centroids = Vec::new();
        for i in 0..k {
            centroids.push(embed_nodes[i % embed_nodes.len()].1.clone());
        }

        let mut clusters: Vec<HashSet<NodeId>> = vec![HashSet::new(); k];

        for _ in 0..max_iterations {
            let mut new_clusters: Vec<HashSet<NodeId>> = vec![HashSet::new(); k];
            let mut changed = false;

            // Assign points to nearest centroid
            for (id, embed) in &embed_nodes {
                let mut best_k = 0;
                let mut best_sim = -1.0;

                for (j, centroid) in centroids.iter().enumerate() {
                    let sim = Self::cosine_similarity(embed, centroid);
                    if sim > best_sim {
                        best_sim = sim;
                        best_k = j;
                    }
                }

                new_clusters[best_k].insert((*id).clone());
            }

            // Check convergence
            for j in 0..k {
                if new_clusters[j] != clusters[j] {
                    changed = true;
                    break;
                }
            }

            if !changed {
                break;
            }

            clusters = new_clusters;

            // Update centroids
            for j in 0..k {
                if clusters[j].is_empty() {
                    continue;
                }

                let mut new_centroid = vec![0.0; embed_dim];
                for id in &clusters[j] {
                    if let Some(node) = nodes.get(id) {
                        if let Some(embed) = &node.embedding {
                            for (i, val) in embed.iter().enumerate() {
                                new_centroid[i] += val;
                            }
                        }
                    }
                }

                let count = clusters[j].len() as f32;
                for val in &mut new_centroid {
                    *val /= count;
                }

                centroids[j] = new_centroid;
            }
        }

        let mut result = Vec::new();
        for (i, cluster_nodes) in clusters.into_iter().enumerate() {
            result.push(Cluster {
                id: format!("cluster_{}", i),
                nodes: cluster_nodes,
                centroid: Some(centroids[i].clone()),
            });
        }

        Ok(result)
    }

    /// Performs a spreading activation process (simulating how human memory retrieves associated concepts)
    pub fn spreading_activation(&self, initial_nodes: &[(NodeId, f32)], decay_factor: f32, steps: usize, threshold: f32) -> Result<HashMap<NodeId, f32>, String> {
        let nodes = self.nodes.read().map_err(|_| "Poisoned lock".to_string())?;
        let adj = self.adjacency_list.read().map_err(|_| "Poisoned lock".to_string())?;
        let edges = self.edges.read().map_err(|_| "Poisoned lock".to_string())?;

        let mut activation = HashMap::new();
        for (id, initial_act) in initial_nodes {
            if nodes.contains_key(id) {
                activation.insert(id.clone(), *initial_act);
            }
        }

        for _ in 0..steps {
            let mut next_activation = activation.clone();

            for (node_id, current_act) in activation.iter() {
                if *current_act < threshold {
                    continue;
                }

                if let Some(outgoing) = adj.get(node_id) {
                    let spread_amount = current_act * decay_factor / (outgoing.len() as f32).max(1.0);

                    for edge_id in outgoing {
                        if let Some(edge) = edges.get(edge_id) {
                            let target = &edge.target;
                            let current_target_act = next_activation.get(target).unwrap_or(&0.0);
                            let weight_factor = edge.weight.min(1.0).max(0.1);

                            next_activation.insert(target.clone(), current_target_act + (spread_amount * weight_factor));
                        }
                    }
                }
            }

            activation = next_activation;
        }

        // Filter out nodes below threshold
        activation.retain(|_, &mut v| v >= threshold);

        Ok(activation)
    }

    /// Finds all strongly connected components in the memory graph using Tarjan's algorithm.
    /// This is useful for finding tightly coupled concepts or cyclical reasoning.
    pub fn strongly_connected_components(&self) -> Result<Vec<Vec<NodeId>>, String> {
        let nodes = self.nodes.read().map_err(|_| "Poisoned lock".to_string())?;
        let adj = self.adjacency_list.read().map_err(|_| "Poisoned lock".to_string())?;
        let edges = self.edges.read().map_err(|_| "Poisoned lock".to_string())?;

        let mut index = 0;
        let mut stack: Vec<NodeId> = Vec::new();
        let mut indices: HashMap<NodeId, usize> = HashMap::new();
        let mut lowlink: HashMap<NodeId, usize> = HashMap::new();
        let mut on_stack: HashSet<NodeId> = HashSet::new();
        let mut sccs: Vec<Vec<NodeId>> = Vec::new();

        // Need to bypass Rust's borrowing rules for recursive algorithm by using a stack instead
        for v in nodes.keys() {
            if !indices.contains_key(v) {
                // Iterative Tarjan's implementation
                let mut call_stack = vec![(v.clone(), false)];

                while let Some((curr, visited)) = call_stack.pop() {
                    if !visited {
                        indices.insert(curr.clone(), index);
                        lowlink.insert(curr.clone(), index);
                        index += 1;
                        stack.push(curr.clone());
                        on_stack.insert(curr.clone());

                        call_stack.push((curr.clone(), true));

                        if let Some(outgoing) = adj.get(&curr) {
                            for edge_id in outgoing {
                                if let Some(edge) = edges.get(edge_id) {
                                    let w = &edge.target;
                                    if !indices.contains_key(w) {
                                        call_stack.push((w.clone(), false));
                                    }
                                }
                            }
                        }
                    } else {
                        // Post-visit
                        if let Some(outgoing) = adj.get(&curr) {
                            for edge_id in outgoing {
                                if let Some(edge) = edges.get(edge_id) {
                                    let w = &edge.target;
                                    if on_stack.contains(w) {
                                        let curr_lowlink = *lowlink.get(&curr).unwrap();
                                        let w_index = *indices.get(w).unwrap();
                                        if w_index < curr_lowlink {
                                            lowlink.insert(curr.clone(), w_index);
                                        }
                                    }
                                }
                            }
                        }

                        if lowlink.get(&curr) == indices.get(&curr) {
                            let mut scc = Vec::new();
                            while let Some(w) = stack.pop() {
                                on_stack.remove(&w);
                                scc.push(w.clone());
                                if w == curr {
                                    break;
                                }
                            }
                            sccs.push(scc);
                        }
                    }
                }
            }
        }

        Ok(sccs)
    }

    pub fn a_star_search(&self, start: &NodeId, goal: &NodeId, heuristic_fn: fn(&NodeId, &NodeId) -> f32) -> Result<Option<ShortestPathResult>, String> {
        let nodes = self.nodes.read().map_err(|_| "Poisoned lock".to_string())?;
        let edges = self.edges.read().map_err(|_| "Poisoned lock".to_string())?;
        let adj = self.adjacency_list.read().map_err(|_| "Poisoned lock".to_string())?;

        if !nodes.contains_key(start) || !nodes.contains_key(goal) {
            return Ok(None);
        }

        let mut g_score: HashMap<NodeId, f32> = HashMap::new();
        let mut f_score: HashMap<NodeId, f32> = HashMap::new();
        let mut previous: HashMap<NodeId, NodeId> = HashMap::new();
        let mut open_set = BinaryHeap::new();

        g_score.insert(start.clone(), 0.0);
        f_score.insert(start.clone(), heuristic_fn(start, goal));
        open_set.push(State { cost: f_score[start], position: start.clone() });

        while let Some(State { cost: _, position: current }) = open_set.pop() {
            if current == *goal {
                let mut path = Vec::new();
                let mut curr = goal.clone();
                while curr != *start {
                    path.push(curr.clone());
                    curr = previous.get(&curr).unwrap().clone();
                }
                path.push(start.clone());
                path.reverse();

                return Ok(Some(ShortestPathResult {
                    path,
                    total_weight: g_score[goal],
                }));
            }

            if let Some(outgoing_edges) = adj.get(&current) {
                for edge_id in outgoing_edges {
                    if let Some(edge) = edges.get(edge_id) {
                        let neighbor = &edge.target;
                        let tentative_g_score = g_score.get(&current).unwrap_or(&f32::MAX) + edge.weight;

                        if tentative_g_score < *g_score.get(neighbor).unwrap_or(&f32::MAX) {
                            previous.insert(neighbor.clone(), current.clone());
                            g_score.insert(neighbor.clone(), tentative_g_score);

                            let f = tentative_g_score + heuristic_fn(neighbor, goal);
                            f_score.insert(neighbor.clone(), f);
                            open_set.push(State { cost: -f, position: neighbor.clone() }); // Negate for min-heap behavior
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    /// Evaluates structural holes in the graph using Burt's constraint measure.
    /// This helps the agent identify concepts that bridge distinct clusters of knowledge.
    pub fn calculate_structural_holes(&self) -> Result<HashMap<NodeId, f32>, String> {
        let nodes = self.nodes.read().map_err(|_| "Poisoned lock".to_string())?;
        let adj = self.adjacency_list.read().map_err(|_| "Poisoned lock".to_string())?;

        let mut constraint: HashMap<NodeId, f32> = HashMap::new();

        for (i, i_outgoing) in adj.iter() {
            let mut i_constraint = 0.0;
            let i_degree = i_outgoing.len() as f32;

            if i_degree > 0.0 {
                for i_edge in i_outgoing {
                    let edges_guard = self.edges.read().unwrap(); let j_id = &edges_guard.get(i_edge).unwrap().target;

                    let mut p_ij = 1.0 / i_degree;

                    for q_edge in i_outgoing {
                        let q_id = &edges_guard.get(q_edge).unwrap().target;
                        if q_id != j_id && q_id != i {
                            if let Some(q_outgoing) = adj.get(q_id) {
                                let mut has_edge_q_j = false;
                                for edge in q_outgoing {
                                    if &edges_guard.get(edge).unwrap().target == j_id {
                                        has_edge_q_j = true;
                                        break;
                                    }
                                }

                                if has_edge_q_j {
                                    p_ij += (1.0 / i_degree) * (1.0 / q_outgoing.len() as f32);
                                }
                            }
                        }
                    }

                    i_constraint += p_ij * p_ij;
                }
            }

            constraint.insert(i.clone(), i_constraint);
        }

        Ok(constraint)
    }
}

#[cfg(test)]
mod advanced_tests {
    use super::*;

    fn create_test_node(id: &str) -> MemoryNode {
        MemoryNode {
            id: NodeId(id.to_string()),
            node_type: NodeType::Concept,
            label: id.to_string(),
            description: format!("Node {}", id),
            importance_score: 1,
            embedding: None,
            created_at: 0,
            last_accessed: 0,
            access_count: 0,
        }
    }

    fn create_test_edge(id: &str, source: &str, target: &str) -> MemoryEdge {
        MemoryEdge {
            id: EdgeId(id.to_string()),
            source: NodeId(source.to_string()),
            target: NodeId(target.to_string()),
            edge_type: EdgeType::RelatesTo,
            weight: 1.0,
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn test_scc() {
        let graph = AgentKnowledgeGraph::new(100);

        graph.add_node(create_test_node("0")).unwrap();
        graph.add_node(create_test_node("1")).unwrap();
        graph.add_node(create_test_node("2")).unwrap();
        graph.add_node(create_test_node("3")).unwrap();
        graph.add_node(create_test_node("4")).unwrap();

        graph.add_edge(create_test_edge("e1", "0", "1")).unwrap();
        graph.add_edge(create_test_edge("e2", "1", "2")).unwrap();
        graph.add_edge(create_test_edge("e3", "2", "0")).unwrap(); // Cycle 0-1-2

        graph.add_edge(create_test_edge("e4", "1", "3")).unwrap();

        graph.add_edge(create_test_edge("e5", "3", "4")).unwrap();
        graph.add_edge(create_test_edge("e6", "4", "3")).unwrap(); // Cycle 3-4

        let sccs = graph.strongly_connected_components().unwrap();
        assert_eq!(sccs.len(), 2);
    }

    #[test]
    fn test_centrality() {
        let graph = AgentKnowledgeGraph::new(100);

        graph.add_node(create_test_node("A")).unwrap();
        graph.add_node(create_test_node("B")).unwrap();
        graph.add_node(create_test_node("C")).unwrap();
        graph.add_node(create_test_node("D")).unwrap();
        graph.add_node(create_test_node("E")).unwrap();

        // Star graph centered on C
        graph.add_edge(create_test_edge("e1", "A", "C")).unwrap();
        graph.add_edge(create_test_edge("e2", "B", "C")).unwrap();
        graph.add_edge(create_test_edge("e3", "D", "C")).unwrap();
        graph.add_edge(create_test_edge("e4", "E", "C")).unwrap();

        graph.add_edge(create_test_edge("e5", "C", "A")).unwrap();
        graph.add_edge(create_test_edge("e6", "C", "B")).unwrap();
        graph.add_edge(create_test_edge("e7", "C", "D")).unwrap();
        graph.add_edge(create_test_edge("e8", "C", "E")).unwrap();

        let cent = graph.betweenness_centrality().unwrap();
        let c_score = cent.get(&NodeId("C".to_string())).unwrap();
        let a_score = cent.get(&NodeId("A".to_string())).unwrap();

        assert!(*c_score > *a_score);
    }
}
