use std::collections::{HashMap, HashSet};

/// Ruflo Unique Harness Innovations: HNSW vector memory: 150x-12,500x faster search via AgentDB
/// A basic but functionally illustrative implementation of HNSW (Hierarchical Navigable Small World)
/// for vector storage and retrieval to represent the HNSW memory standard.

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

    pub fn distance(&self, other: &Vector) -> f32 {
        // HNSW usually uses distance. We use 1.0 - cosine_similarity.
        // For identical vectors, cos_sim = 1.0 -> dist = 0.0
        // For opposite vectors, cos_sim = -1.0 -> dist = 2.0
        1.0 - self.cosine_similarity(other)
    }
}

pub struct AgentDB {
    vectors: HashMap<String, Vector>,

    // HNSW graph: Layer -> Node ID -> List of neighbor Node IDs
    layers: Vec<HashMap<String, Vec<String>>>,
    entry_point: Option<String>,
    max_layers: usize,
    m: usize, // Max neighbors per node
    ef_construction: usize,
    rng_state: usize,
}

impl AgentDB {
    pub fn new() -> Self {
        Self {
            vectors: HashMap::new(),
            layers: vec![HashMap::new()], // Always at least layer 0
            entry_point: None,
            max_layers: 4,
            m: 16,
            ef_construction: 32,
            rng_state: 42,
        }
    }

    fn pseudo_random_level(&mut self) -> usize {
        self.rng_state = (self.rng_state.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7FFFFFFF;
        let r = (self.rng_state as f64) / (0x7FFFFFFF as f64 + 1.0);
        let m_l = 1.0 / (self.m as f64).ln();
        let level = (-r.ln() * m_l) as usize;
        std::cmp::min(level, self.max_layers - 1)
    }

    pub fn insert(&mut self, id: String, values: Vec<f32>, metadata: String) {
        let node = Vector::new(id.clone(), values, metadata);
        self.vectors.insert(id.clone(), node.clone());

        let node_level = self.pseudo_random_level();

        while self.layers.len() <= node_level {
            self.layers.push(HashMap::new());
        }

        if self.entry_point.is_none() {
            self.entry_point = Some(id.clone());
            for l in 0..=node_level {
                self.layers[l].insert(id.clone(), vec![]);
            }
            return;
        }

        let mut curr_obj = self.entry_point.clone().unwrap();
        let top_layer = self.layers.len() - 1;

        // Phase 1: Search top layers to find the entry point for the target layer
        for l in (node_level + 1..=top_layer).rev() {
            let res = self.search_layer(&node, curr_obj.clone(), 1, l);
            if !res.is_empty() {
                curr_obj = res[0].clone();
            }
        }

        // Phase 2: Insert into node_level down to 0
        for l in (0..=node_level).rev() {
            let neighbors = self.search_layer(&node, curr_obj.clone(), self.ef_construction, l);

            // Link node to its neighbors
            let selected_neighbors = self.select_neighbors(neighbors, self.m);
            self.layers[l].insert(id.clone(), selected_neighbors.clone());

            // Link neighbors to node
            for neighbor in &selected_neighbors {
                if let Some(n_links) = self.layers[l].get_mut(neighbor) {
                    n_links.push(id.clone());
                    // Shrink connections if exceeded m_max
                    if n_links.len() > self.m {
                        let mut n_links_cloned = n_links.clone();
                        n_links_cloned.sort_by(|a, b| {
                            let dist_a = self.vectors[neighbor].distance(&self.vectors[a]);
                            let dist_b = self.vectors[neighbor].distance(&self.vectors[b]);
                            dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
                        });
                        n_links_cloned.truncate(self.m);
                        *n_links = n_links_cloned;
                    }
                }
            }

            let res = self.search_layer(&node, curr_obj.clone(), 1, l);
            if !res.is_empty() {
                curr_obj = res[0].clone();
            }
        }

        if node_level == top_layer {
            self.entry_point = Some(id);
        }
    }

    fn search_layer(&self, query: &Vector, entry_point: String, ef: usize, layer: usize) -> Vec<String> {
        let mut visited = HashSet::new();
        let mut candidates = Vec::new();
        let mut results = Vec::new();

        visited.insert(entry_point.clone());
        let dist = query.distance(&self.vectors[&entry_point]);
        candidates.push((dist, entry_point.clone()));
        results.push((dist, entry_point.clone()));

        while !candidates.is_empty() {
            candidates.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            let (c_dist, c) = candidates.remove(0);

            results.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            let (f_dist, _) = results.last().unwrap();

            if c_dist > *f_dist && results.len() >= ef {
                break; // all elements in candidates are further than the furthest in results
            }

            if let Some(neighbors) = self.layers[layer].get(&c) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        let n_dist = query.distance(&self.vectors[neighbor]);

                        let (f_dist_results, _) = results.last().unwrap();
                        if results.len() < ef || n_dist < *f_dist_results {
                            candidates.push((n_dist, neighbor.clone()));
                            results.push((n_dist, neighbor.clone()));
                            results.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
                            if results.len() > ef {
                                results.pop();
                            }
                        }
                    }
                }
            }
        }

        results.into_iter().map(|(_, id)| id).collect()
    }

    fn select_neighbors(&self, candidates: Vec<String>, m: usize) -> Vec<String> {
        // Simplistic selection: just take the closest ones (which they already are, since search_layer returns them sorted)
        candidates.into_iter().take(m).collect()
    }

    pub fn search(&self, query: &Vec<f32>, top_k: usize) -> Vec<Vector> {
        if self.entry_point.is_none() {
            return vec![];
        }

        let query_vec = Vector::new("query".to_string(), query.clone(), "".to_string());
        let mut curr_obj = self.entry_point.clone().unwrap();
        let top_layer = self.layers.len() - 1;

        // Phase 1: Descend down to layer 1
        for l in (1..=top_layer).rev() {
            let res = self.search_layer(&query_vec, curr_obj.clone(), 1, l);
            if !res.is_empty() {
                curr_obj = res[0].clone();
            }
        }

        // Phase 2: Search layer 0
        let ef = std::cmp::max(top_k, self.ef_construction);
        let nearest_neighbors = self.search_layer(&query_vec, curr_obj, ef, 0);

        nearest_neighbors.into_iter().take(top_k).map(|id| self.vectors[&id].clone()).collect()
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

    #[test]
    fn test_agentdb_large_scale() {
        let mut db = AgentDB::new();
        for i in 0..100 {
            db.insert(format!("node_{}", i), vec![(i as f32) / 100.0, 1.0 - (i as f32) / 100.0], format!("meta_{}", i));
        }

        let results = db.search(&vec![0.5, 0.5], 5);
        assert_eq!(results.len(), 5);
        // node_50 should be identical to 0.5, 0.5
        assert_eq!(results[0].id, "node_50");
    }
}
