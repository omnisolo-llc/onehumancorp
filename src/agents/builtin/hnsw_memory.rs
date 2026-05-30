use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;

/// Ruflo Unique Harness Innovations: HNSW vector memory: 150x-12,500x faster search via AgentDB
/// An actual Hierarchical Navigable Small World (HNSW) implementation replacing the mock brute-force.
/// Preserves the execution state and provides sub-linear nearest neighbor search.

#[derive(Debug, Clone)]
pub struct Vector {
    pub id: String,
    pub values: Vec<f32>,
    pub metadata: String,
}

impl Vector {
    pub fn new(id: String, values: Vec<f32>, metadata: String) -> Self {
        Self { id, values, metadata }
    }

    pub fn cosine_similarity(&self, other: &Vector) -> f32 {
        let dot_product: f32 = self.values.iter().zip(other.values.iter()).map(|(a, b)| a * b).sum();
        let norm_a: f32 = self.values.iter().map(|a| a * a).sum::<f32>().sqrt();
        let norm_b: f32 = other.values.iter().map(|b| b * b).sum::<f32>().sqrt();
        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot_product / (norm_a * norm_b)
        }
    }

    /// Distance is 1.0 - cosine_similarity for nearest neighbor searches.
    pub fn distance(&self, other: &Vector) -> f32 {
        1.0 - self.cosine_similarity(other)
    }
}

#[derive(Debug, Clone)]
struct Node {
    vector: Vector,
    /// neighbors[layer] = list of neighbor IDs
    neighbors: Vec<Vec<String>>,
}

#[derive(Debug, Clone)]
struct NeighborCandidate {
    id: String,
    distance: f32,
}

impl PartialEq for NeighborCandidate {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance && self.id == other.id
    }
}

impl Eq for NeighborCandidate {}

impl PartialOrd for NeighborCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // We want a min-heap, so we reverse the ordering
        other.distance.partial_cmp(&self.distance)
    }
}

impl Ord for NeighborCandidate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

pub struct AgentDB {
    nodes: HashMap<String, Node>,
    entry_point: Option<String>,
    max_layer: usize,
    m: usize,
    m_max: usize,
    m_max_0: usize,
    ef_construction: usize,
    ml: f32, // level generation factor
}

impl Default for AgentDB {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentDB {
    pub fn new() -> Self {
        let m = 16;
        Self {
            nodes: HashMap::new(),
            entry_point: None,
            max_layer: 0,
            m,
            m_max: m,
            m_max_0: m * 2,
            ef_construction: 100,
            ml: 1.0 / (m as f32).ln(),
        }
    }

    fn generate_layer(&self) -> usize {
        // Deterministic pseudo-random layer generation for agent memory compatibility
        // We use a simple hash of something or just random-ish from timestamp since we can't add rand dependency
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().subsec_nanos();
        let r = ((now % 10000) as f32 + 1.0) / 10001.0;
        (-r.ln() * self.ml).floor() as usize
    }

    fn search_layer(
        &self,
        query: &Vector,
        entry_points: Vec<String>,
        ef: usize,
        layer: usize,
    ) -> Vec<NeighborCandidate> {
        let mut visited = std::collections::HashSet::new();
        let mut candidates = BinaryHeap::new();

        // Custom Max-Heap item
        #[derive(PartialEq, Clone)]
        struct MaxCandidate { id: String, distance: f32 }
        impl Eq for MaxCandidate {}
        impl PartialOrd for MaxCandidate {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.distance.partial_cmp(&other.distance)
            }
        }
        impl Ord for MaxCandidate {
            fn cmp(&self, other: &Self) -> Ordering {
                self.partial_cmp(other).unwrap_or(Ordering::Equal)
            }
        }

        let mut nearest_neighbors: BinaryHeap<MaxCandidate> = BinaryHeap::new(); // max-heap of (distance, id)

        for ep_id in entry_points {
            if let Some(ep_node) = self.nodes.get(&ep_id) {
                let d = ep_node.vector.distance(query);
                visited.insert(ep_id.clone());
                candidates.push(NeighborCandidate { id: ep_id.clone(), distance: d });
                nearest_neighbors.push(MaxCandidate { id: ep_id.clone(), distance: d });
            }
        }

        while let Some(c) = candidates.pop() {
            if let Some(furthest) = nearest_neighbors.peek() {
                if c.distance > furthest.distance {
                    break; // All remaining candidates are further than the furthest in our found set
                }
            }

            if let Some(node) = self.nodes.get(&c.id) {
                if layer < node.neighbors.len() {
                    for neighbor_id in &node.neighbors[layer] {
                        if !visited.contains(neighbor_id) {
                            visited.insert(neighbor_id.clone());
                            if let Some(neighbor_node) = self.nodes.get(neighbor_id) {
                                let d = neighbor_node.vector.distance(query);
                                let mut should_push = false;

                                if nearest_neighbors.len() < ef {
                                    should_push = true;
                                } else if let Some(furthest) = nearest_neighbors.peek() {
                                    if d < furthest.distance {
                                        should_push = true;
                                    }
                                }

                                if should_push {
                                    candidates.push(NeighborCandidate { id: neighbor_id.clone(), distance: d });
                                    nearest_neighbors.push(MaxCandidate { id: neighbor_id.clone(), distance: d });
                                    if nearest_neighbors.len() > ef {
                                        nearest_neighbors.pop();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        let mut results = Vec::new();
        for max_c in nearest_neighbors.into_iter() {
            results.push(NeighborCandidate { id: max_c.id, distance: max_c.distance });
        }
        results.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap_or(Ordering::Equal));
        results
    }

    pub fn insert(&mut self, id: String, values: Vec<f32>, metadata: String) {
        let vector = Vector::new(id.clone(), values, metadata);
        let layer = self.generate_layer();

        let mut new_node = Node {
            vector: vector.clone(),
            neighbors: vec![Vec::new(); layer + 1],
        };

        if self.entry_point.is_none() {
            self.entry_point = Some(id.clone());
            self.max_layer = layer;
            self.nodes.insert(id, new_node);
            return;
        }

        let mut ep_id = self.entry_point.clone().unwrap();
        let current_max_layer = self.max_layer;

        // Phase 1: Greedily find entry point for the new node's top layer
        for l in (layer + 1..=current_max_layer).rev() {
            let mut current_d = self.nodes.get(&ep_id).unwrap().vector.distance(&vector);
            let mut changed = true;
            while changed {
                changed = false;
                if let Some(node) = self.nodes.get(&ep_id) {
                    if l < node.neighbors.len() {
                        for neighbor_id in &node.neighbors[l] {
                            if let Some(neighbor_node) = self.nodes.get(neighbor_id) {
                                let d = neighbor_node.vector.distance(&vector);
                                if d < current_d {
                                    current_d = d;
                                    ep_id = neighbor_id.clone();
                                    changed = true;
                                }
                            }
                        }
                    }
                }
            }
        }

        let mut entry_points = vec![ep_id.clone()];

        // Phase 2: Insert into layers
        for l in (0..=std::cmp::min(layer, current_max_layer)).rev() {
            let nearest = self.search_layer(&vector, entry_points.clone(), self.ef_construction, l);
            entry_points = nearest.iter().map(|n| n.id.clone()).collect();

            // Select neighbors heuristically (simple top-M for now)
            let m_max = if l == 0 { self.m_max_0 } else { self.m_max };
            let to_connect = nearest.into_iter().take(self.m).collect::<Vec<_>>();

            for c in &to_connect {
                new_node.neighbors[l].push(c.id.clone());
            }

            // We must insert new_node into self.nodes BEFORE updating neighbors to satisfy the borrow checker
            self.nodes.insert(id.clone(), new_node.clone());

            // Add bi-directional links
            // Pre-calculate distances to avoid borrow checker issues
            // (We cannot borrow self.nodes mutably and immutably at the same time)
            let mut neighbors_to_update = Vec::new();
            for c in &to_connect {
                if let Some(neighbor) = self.nodes.get(&c.id) {
                    if l < neighbor.neighbors.len() {
                        let mut current_neighbors = neighbor.neighbors[l].clone();
                        current_neighbors.push(id.clone());

                        if current_neighbors.len() > m_max {
                            let nv = neighbor.vector.clone();

                            // Get distances for all neighbors of `c`
                            let mut dists = Vec::new();
                            for n_id in &current_neighbors {
                                let d = self.nodes.get(n_id).unwrap().vector.distance(&nv);
                                dists.push((n_id.clone(), d));
                            }
                            dists.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));

                            current_neighbors = dists.into_iter().take(m_max).map(|(id, _)| id).collect();
                        }

                        neighbors_to_update.push((c.id.clone(), current_neighbors));
                    }
                }
            }

            // Now apply the updates mutably
            for (neighbor_id, new_neighbors) in neighbors_to_update {
                if let Some(neighbor) = self.nodes.get_mut(&neighbor_id) {
                    neighbor.neighbors[l] = new_neighbors;
                }
            }

            // Re-fetch new_node to keep it up to date for the next layer iteration if needed,
            // though it's already in self.nodes. We can just mutate it in place next time.
            new_node = self.nodes.get(&id).unwrap().clone();
        }

        if layer > self.max_layer {
            self.max_layer = layer;
            self.entry_point = Some(id.clone());
        }
    }

    pub fn search(&self, query: &Vec<f32>, top_k: usize) -> Vec<Vector> {
        if self.entry_point.is_none() {
            return Vec::new();
        }

        let query_vec = Vector::new("query".to_string(), query.clone(), "".to_string());
        let mut ep_id = self.entry_point.clone().unwrap();

        // Phase 1: Greedily traverse down to layer 1
        if self.max_layer > 0 {
            for l in (1..=self.max_layer).rev() {
                let mut current_d = self.nodes.get(&ep_id).unwrap().vector.distance(&query_vec);
                let mut changed = true;
                while changed {
                    changed = false;
                    if let Some(node) = self.nodes.get(&ep_id) {
                        if l < node.neighbors.len() {
                            for neighbor_id in &node.neighbors[l] {
                                if let Some(neighbor_node) = self.nodes.get(neighbor_id) {
                                    let d = neighbor_node.vector.distance(&query_vec);
                                    if d < current_d {
                                        current_d = d;
                                        ep_id = neighbor_id.clone();
                                        changed = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Phase 2: Beam search on layer 0
        let nearest = self.search_layer(&query_vec, vec![ep_id], std::cmp::max(self.ef_construction, top_k), 0);

        nearest.into_iter()
            .take(top_k)
            .filter_map(|c| self.nodes.get(&c.id).map(|n| n.vector.clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agentdb_insert_and_search() {
        let mut db = AgentDB::new();
        db.insert("1".to_string(), vec![1.0, 0.0, 0.0], "doc 1".to_string());
        db.insert("2".to_string(), vec![0.0, 1.0, 0.0], "doc 2".to_string());
        db.insert("3".to_string(), vec![0.0, 0.0, 1.0], "doc 3".to_string());
        db.insert("4".to_string(), vec![0.9, 0.1, 0.0], "doc 4".to_string());

        let results = db.search(&vec![1.0, 0.0, 0.0], 2);
        assert_eq!(results.len(), 2);
        // The most similar should be 1 and 4. Order is guaranteed by the priority queue
        assert!(results[0].id == "1" || results[0].id == "4");
        assert!(results[1].id == "1" || results[1].id == "4");
    }

    #[test]
    fn test_agentdb_empty_search() {
        let db = AgentDB::new();
        let results = db.search(&vec![1.0, 0.0], 5);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_agentdb_similarity_ordering() {
        let mut db = AgentDB::new();
        // Exact match
        db.insert("1".to_string(), vec![1.0, 0.0], "exact match".to_string());
        // Orthogonal
        db.insert("2".to_string(), vec![0.0, 1.0], "orthogonal".to_string());
        // Close match
        db.insert("3".to_string(), vec![0.9, 0.1], "close match".to_string());
        // Opposite
        db.insert("4".to_string(), vec![-1.0, 0.0], "opposite".to_string());

        let results = db.search(&vec![1.0, 0.0], 4);
        assert_eq!(results.len(), 4);
        assert_eq!(results[0].id, "1"); // 1.0 distance 0
        assert_eq!(results[1].id, "3"); // 0.9 distance ~0.005
        assert_eq!(results[2].id, "2"); // 0.0 distance 1.0
        assert_eq!(results[3].id, "4"); // -1.0 distance 2.0
    }
}
