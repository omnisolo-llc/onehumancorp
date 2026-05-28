use std::collections::HashMap;

/// Ruflo Unique Harness Innovations: HNSW vector memory: 150x-12,500x faster search via AgentDB
/// A mock/basic implementation of AgentDB using simple cosine similarity or brute-force
/// for vector storage and retrieval to represent the HNSW memory standard.

#[derive(Debug, Clone)]
pub struct Vector {
    pub id: String,
    pub values: Vec<f32>,
    pub metadata: String,
    pub neighbors: Vec<String>,
}

impl Vector {
    pub fn new(id: String, values: Vec<f32>, metadata: String) -> Self {
        Self { id, values, metadata, neighbors: Vec::new() }
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
}

pub struct AgentDB {
    vectors: HashMap<String, Vector>,
    entry_point: Option<String>,
    m: usize, // Max neighbors per node
}

impl AgentDB {
    pub fn new() -> Self {
        Self {
            vectors: HashMap::new(),
            entry_point: None,
            m: 16, // Typical value for HNSW max neighbors
        }
    }

    pub fn insert(&mut self, id: String, values: Vec<f32>, metadata: String) {
        let mut new_vec = Vector::new(id.clone(), values.clone(), metadata);

        if self.vectors.is_empty() {
            self.vectors.insert(id.clone(), new_vec);
            self.entry_point = Some(id);
            return;
        }

        // Find closest neighbors using greedy search
        let query_vec = Vector::new("query".to_string(), values.clone(), "".to_string());

        // Find candidate neighbors by searching the existing graph
        let best_candidates = self.search_internal(&query_vec, self.m);

        // Link bidirectional
        for candidate_id in &best_candidates {
            new_vec.neighbors.push(candidate_id.clone());
        }

        self.vectors.insert(id.clone(), new_vec);

        for candidate_id in best_candidates {
            // Check if we need to evict, but avoid mutable borrow spanning the immutable borrow
            let mut eviction_target = None;
            let mut neighbor_needs_update = false;

            if let Some(neighbor_vec) = self.vectors.get(&candidate_id) {
                if neighbor_vec.neighbors.len() < self.m {
                    neighbor_needs_update = true;
                } else {
                    // Simple eviction strategy
                    let mut worst_idx = 0;
                    let mut worst_sim = f32::MAX;

                    let new_sim = neighbor_vec.cosine_similarity(&Vector::new("".to_string(), values.clone(), "".to_string()));

                    for (i, n_id) in neighbor_vec.neighbors.iter().enumerate() {
                        if let Some(n_vec) = self.vectors.get(n_id) {
                            let sim = neighbor_vec.cosine_similarity(n_vec);
                            if sim < worst_sim {
                                worst_sim = sim;
                                worst_idx = i;
                            }
                        }
                    }

                    if new_sim > worst_sim {
                        eviction_target = Some(worst_idx);
                    }
                }
            }

            if neighbor_needs_update {
                if let Some(neighbor_vec) = self.vectors.get_mut(&candidate_id) {
                    neighbor_vec.neighbors.push(id.clone());
                }
            } else if let Some(worst_idx) = eviction_target {
                if let Some(neighbor_vec) = self.vectors.get_mut(&candidate_id) {
                    neighbor_vec.neighbors.remove(worst_idx);
                    neighbor_vec.neighbors.push(id.clone());
                }
            }
        }
    }

    fn search_internal(&self, query_vec: &Vector, top_k: usize) -> Vec<String> {
        let Some(entry_id) = &self.entry_point else {
            return Vec::new();
        };

        let current_node = entry_id.clone();
        let best_sim = self.vectors[&current_node].cosine_similarity(query_vec);

        let mut candidates = std::collections::BinaryHeap::new();
        let mut visited = std::collections::HashSet::new();

        // Max-heap to keep track of best top_k for greedy traversal (storing Float wrapper for f32)
        #[derive(PartialEq, Clone)]
        struct OrderedF32(f32);
        impl Eq for OrderedF32 {}
        impl PartialOrd for OrderedF32 {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }
        impl Ord for OrderedF32 {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
            }
        }

        candidates.push((OrderedF32(best_sim), current_node.clone()));
        visited.insert(current_node.clone());

        let mut results = Vec::new();
        results.push((best_sim, current_node.clone()));

        while let Some((_, curr)) = candidates.pop() {
            let Some(curr_vec) = self.vectors.get(&curr) else { continue };

            for neighbor_id in &curr_vec.neighbors {
                if visited.insert(neighbor_id.clone()) {
                    let neighbor_vec = &self.vectors[neighbor_id];
                    let sim = neighbor_vec.cosine_similarity(query_vec);

                    if results.len() < top_k || sim > results.last().unwrap().0 {
                        results.push((sim, neighbor_id.clone()));
                        results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
                        if results.len() > top_k {
                            results.pop();
                        }
                        candidates.push((OrderedF32(sim), neighbor_id.clone()));
                    }
                }
            }
        }

        // If we didn't find enough nodes via graph traversal (e.g., disconnected graph early on),
        // fall back to brute-force for the remaining (this handles edge cases in our naive implementation)
        if results.len() < top_k && self.vectors.len() > results.len() {
             let mut all_res: Vec<(&Vector, f32)> = self.vectors.values()
                .map(|v| (v, v.cosine_similarity(&query_vec)))
                .collect();
            all_res.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            return all_res.into_iter().take(top_k).map(|(v, _)| v.id.clone()).collect();
        }

        results.into_iter().map(|(_, id)| id).collect()
    }

    pub fn search(&self, query: &Vec<f32>, top_k: usize) -> Vec<Vector> {
        if self.vectors.is_empty() {
            return Vec::new();
        }
        let query_vec = Vector::new("query".to_string(), query.clone(), "".to_string());
        let best_ids = self.search_internal(&query_vec, top_k);
        best_ids.into_iter().filter_map(|id| self.vectors.get(&id).cloned()).collect()
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
        assert_eq!(results[0].id, "1");
        assert_eq!(results[1].id, "4");
    }
}

#[cfg(test)]
mod hnsw_additional_tests {
    use super::*;

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
        assert_eq!(results[0].id, "1"); // 1.0
        assert_eq!(results[1].id, "3"); // 0.99
        assert_eq!(results[2].id, "2"); // 0.0
        assert_eq!(results[3].id, "4"); // -1.0
    }
}
