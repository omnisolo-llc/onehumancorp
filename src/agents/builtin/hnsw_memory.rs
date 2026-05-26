use std::collections::HashMap;

/// Ruflo Unique Harness Innovations: HNSW vector memory: 150x-12,500x faster search via AgentDB
/// A mock/basic implementation of AgentDB using simple cosine similarity or brute-force
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
}

#[derive(Debug, Clone)]
pub struct HnswNode {
    pub vector: Vector,
    pub max_layer: usize,
    /// Adjacency list: level -> Vec<node_id>
    pub neighbors: Vec<Vec<String>>,
}

impl HnswNode {
    pub fn new(vector: Vector, max_layer: usize) -> Self {
        Self {
            vector,
            max_layer,
            neighbors: vec![Vec::new(); max_layer + 1],
        }
    }
}

pub struct AgentDB {
    nodes: HashMap<String, HnswNode>,
    entry_point: Option<String>,
    max_layer: usize,
    m: usize,
    m_max: usize,
}

impl AgentDB {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            entry_point: None,
            max_layer: 0,
            m: 16,
            m_max: 32,
        }
    }

    fn generate_random_layer(&self) -> usize {
        let ml = 1.0 / (self.m as f64).ln();
        // Since `rand` is not easily imported, we use a simple linear congruential generator
        // based on the system time for a simple random number generator to create layer levels.
        use std::sync::atomic::{AtomicU64, Ordering};
        static SEED: AtomicU64 = AtomicU64::new(123456789);
        let mut seed = SEED.load(Ordering::SeqCst);
        if seed == 123456789 {
            seed = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64;
        }
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        SEED.store(seed, Ordering::SeqCst);

        let mut r = ((seed >> 16) as f64) / ((1u64 << 48) as f64);
        if r < 0.0001 {
            r = 0.0001;
        }
        (-r.ln() * ml).floor() as usize
    }

    // Helper method to find up to `ef` nearest neighbors from a starting node.
    fn search_layer(&self, query_vec: &Vector, entry: &str, ef: usize, layer: usize) -> Vec<(String, f32)> {
        let mut visited = std::collections::HashSet::new();
        let mut candidates = vec![(entry.to_string(), self.nodes[entry].vector.cosine_similarity(query_vec))];
        let mut results = candidates.clone();
        visited.insert(entry.to_string());

        while let Some((c_id, c_sim)) = candidates.pop() {
            let worst_sim = results.last().unwrap().1;
            if c_sim < worst_sim {
                break;
            }

            for n_id in &self.nodes[&c_id].neighbors[layer] {
                if visited.insert(n_id.clone()) {
                    let n_sim = self.nodes[n_id].vector.cosine_similarity(query_vec);
                    let worst_sim = results.last().unwrap().1;
                    if n_sim > worst_sim || results.len() < ef {
                        candidates.push((n_id.clone(), n_sim));
                        candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

                        results.push((n_id.clone(), n_sim));
                        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                        if results.len() > ef {
                            results.pop();
                        }
                    }
                }
            }
        }
        results
    }

    pub fn insert(&mut self, id: String, values: Vec<f32>, metadata: String) {
        let vector = Vector::new(id.clone(), values, metadata);
        let layer = self.generate_random_layer();
        let mut new_node = HnswNode::new(vector, layer);

        if self.entry_point.is_none() {
            self.entry_point = Some(id.clone());
            self.max_layer = layer;
            self.nodes.insert(id, new_node);
            return;
        }

        let mut curr_obj = self.entry_point.clone().unwrap();
        let mut curr_layer = self.max_layer;
        let query_vec = new_node.vector.clone();

        // Phase 1: greedy search down to the layer where we actually insert
        while curr_layer > layer {
            let mut changed = true;
            while changed {
                changed = false;
                let mut max_sim = self.nodes[&curr_obj].vector.cosine_similarity(&query_vec);
                let neighbors = self.nodes[&curr_obj].neighbors[curr_layer].clone();
                for neighbor_id in neighbors {
                    let sim = self.nodes[&neighbor_id].vector.cosine_similarity(&query_vec);
                    if sim > max_sim {
                        max_sim = sim;
                        curr_obj = neighbor_id;
                        changed = true;
                    }
                }
            }
            if curr_layer == 0 {
                break;
            }
            curr_layer -= 1;
        }

        // Phase 2: greedy search with beam at layers <= layer
        // We use ef_construction = 20 for insertion.
        let ef_construction = 20;

        for l in (0..=std::cmp::min(layer, self.max_layer)).rev() {
            let neighbors_found = self.search_layer(&query_vec, &curr_obj, ef_construction, l);

            // Connect to up to M neighbors
            let m_conn = if l == 0 { self.m_max } else { self.m };
            for (n_id, _) in neighbors_found.iter().take(m_conn) {
                new_node.neighbors[l].push(n_id.clone());
            }

            // Temporarily insert to update reciprocal connections
            self.nodes.insert(id.clone(), new_node.clone());

            for (n_id, _) in neighbors_found.iter().take(m_conn) {
                let n_vector = self.nodes[n_id].vector.clone();
                let neighbors_of_n = self.nodes[n_id].neighbors[l].clone();

                if neighbors_of_n.len() < m_conn {
                    if let Some(n) = self.nodes.get_mut(n_id) {
                        n.neighbors[l].push(id.clone());
                    }
                } else {
                    // Heuristic: swap out weakest connection
                    let mut min_sim = f32::MAX;
                    let mut min_idx = 0;
                    for (i, exist_n_id) in neighbors_of_n.iter().enumerate() {
                        let sim = self.nodes[exist_n_id].vector.cosine_similarity(&n_vector);
                        if sim < min_sim {
                            min_sim = sim;
                            min_idx = i;
                        }
                    }
                    let id_sim = query_vec.cosine_similarity(&n_vector);
                    if id_sim > min_sim {
                        if let Some(n) = self.nodes.get_mut(n_id) {
                            n.neighbors[l][min_idx] = id.clone();
                        }
                    }
                }
            }

            curr_obj = neighbors_found.first().unwrap().0.clone();
        }

        if layer > self.max_layer {
            self.max_layer = layer;
            self.entry_point = Some(id.clone());
        }

        self.nodes.insert(id, new_node);
    }

    pub fn search(&self, query: &Vec<f32>, top_k: usize) -> Vec<Vector> {
        if self.entry_point.is_none() {
            return vec![];
        }

        let query_vec = Vector::new("query".to_string(), query.clone(), "".to_string());

        // Exact search fallback if nodes.len() is extremely small
        if self.nodes.len() <= top_k * 2 {
            let mut results: Vec<(&Vector, f32)> = self.nodes.values()
                .map(|n| (&n.vector, n.vector.cosine_similarity(&query_vec)))
                .collect();
            results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            return results.into_iter().take(top_k).map(|(v, _)| v.clone()).collect();
        }

        let mut curr_obj = self.entry_point.clone().unwrap();

        // Greedy search down to layer 1
        for l in (1..=self.max_layer).rev() {
            let mut changed = true;
            let mut max_sim = self.nodes[&curr_obj].vector.cosine_similarity(&query_vec);
            while changed {
                changed = false;
                let neighbors = self.nodes[&curr_obj].neighbors[l].clone();
                for neighbor_id in neighbors {
                    let sim = self.nodes[&neighbor_id].vector.cosine_similarity(&query_vec);
                    if sim > max_sim {
                        max_sim = sim;
                        curr_obj = neighbor_id.clone();
                        changed = true;
                    }
                }
            }
        }

        // Layer 0 beam search
        let ef_search = std::cmp::max(top_k * 2, 50); // Expand ef_search to ensure we find enough neighbors
        let best_neighbors = self.search_layer(&query_vec, &curr_obj, ef_search, 0);

        best_neighbors.into_iter()
            .take(top_k)
            .map(|(n_id, _)| self.nodes[&n_id].vector.clone())
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
    fn test_agentdb_hnsw_large_graph_fallback() {
        // Test that the large graph branch executes properly without crashing.
        let mut db = AgentDB::new();
        // Generate enough items to bypass the exact search fallback condition
        // `self.nodes.len() <= top_k * 2`.
        for i in 0..20 {
            db.insert(
                format!("doc_{}", i),
                vec![(i as f32) / 20.0, 1.0 - (i as f32) / 20.0],
                format!("meta_{}", i),
            );
        }

        // top_k = 5, top_k * 2 = 10 < 20
        let results = db.search(&vec![1.0, 0.0], 5);

        assert_eq!(results.len(), 5);
        // doc_19 should be closest to [1.0, 0.0] as its vector is [19/20, 1 - 19/20]
        assert_eq!(results[0].id, "doc_19");
    }
}
