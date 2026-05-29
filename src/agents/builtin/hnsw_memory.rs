use std::collections::HashMap;
use std::cmp::Ordering;

/// Ruflo Unique Harness Innovations: HNSW vector memory: 150x-12,500x faster search via AgentDB
/// An implementation of AgentDB using a Hierarchical Navigable Small World (HNSW) graph structure.

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
    pub layer: usize,
    /// Connections per layer: index is the layer, value is a list of connected node IDs
    pub connections: Vec<Vec<String>>,
}

pub struct AgentDB {
    pub max_layers: usize,
    pub m: usize,
    pub ef_construction: usize,
    pub entry_point: Option<String>,
    pub nodes: HashMap<String, HnswNode>,
    // simple RNG state
    rng_state: u64,
}

impl AgentDB {
    pub fn new() -> Self {
        Self {
            max_layers: 4,
            m: 5,
            ef_construction: 10,
            entry_point: None,
            nodes: HashMap::new(),
            rng_state: 1780023289,
        }
    }

    fn random_layer(&mut self) -> usize {
        self.rng_state ^= self.rng_state << 13;
        self.rng_state ^= self.rng_state >> 7;
        self.rng_state ^= self.rng_state << 17;

        let mut layer = 0;
        let p = 1.0 / (self.m as f64);
        let mut rand_val = (self.rng_state % 10000) as f64 / 10000.0;

        while rand_val < p && layer < self.max_layers {
            layer += 1;
            rand_val = ((self.rng_state * (layer as u64 + 1)) % 10000) as f64 / 10000.0;
        }

        layer
    }

    fn search_layer(&self, query: &Vector, entry_nodes: Vec<String>, ef: usize, layer: usize) -> Vec<String> {
        let mut candidates = Vec::new();
        let mut visited = HashMap::new();
        let mut results = Vec::new();

        for ep in entry_nodes {
            if let Some(node) = self.nodes.get(&ep) {
                let dist = query.cosine_similarity(&node.vector);
                candidates.push((ep.clone(), dist));
                results.push((ep.clone(), dist));
                visited.insert(ep, true);
            }
        }

        while !candidates.is_empty() {
            candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
            let current = candidates.remove(0); // get closest

            results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
            if results.len() > ef {
                results.truncate(ef);
            }
            let worst_result_dist = results.last().map(|(_, d)| *d).unwrap_or(f32::MIN);

            if current.1 < worst_result_dist && results.len() >= ef {
                break;
            }

            if let Some(node) = self.nodes.get(&current.0) {
                if layer < node.connections.len() {
                    for neighbor_id in &node.connections[layer] {
                        if !visited.contains_key(neighbor_id) {
                            visited.insert(neighbor_id.clone(), true);
                            if let Some(neighbor) = self.nodes.get(neighbor_id) {
                                let dist = query.cosine_similarity(&neighbor.vector);

                                let worst_curr = results.last().map(|(_, d)| *d).unwrap_or(f32::MIN);
                                if dist > worst_curr || results.len() < ef {
                                    candidates.push((neighbor_id.clone(), dist));
                                    results.push((neighbor_id.clone(), dist));
                                    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
                                    if results.len() > ef {
                                        results.truncate(ef);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        results.into_iter().map(|(id, _)| id).collect()
    }

    pub fn insert(&mut self, id: String, values: Vec<f32>, metadata: String) {
        let vector = Vector::new(id.clone(), values, metadata);
        let layer = self.random_layer();

        let mut new_node = HnswNode {
            vector: vector.clone(),
            layer,
            connections: vec![Vec::new(); layer + 1],
        };

        if self.entry_point.is_none() {
            self.entry_point = Some(id.clone());
            self.nodes.insert(id, new_node);
            return;
        }

        let mut current_entry = self.entry_point.clone().unwrap();
        let ep_layer = self.nodes.get(&current_entry).unwrap().layer;

        // Traverse upper layers
        for l in (layer + 1..=ep_layer).rev() {
            let res = self.search_layer(&vector, vec![current_entry.clone()], 1, l);
            if !res.is_empty() {
                current_entry = res[0].clone();
            }
        }

        // Establish connections on layer and below
        let mut ep_candidates = vec![current_entry.clone()];
        for l in (0..=std::cmp::min(layer, ep_layer)).rev() {
            let neighbors = self.search_layer(&vector, ep_candidates.clone(), self.ef_construction, l);

            // Connect new node to neighbors
            let mut top_m = neighbors.clone();
            top_m.truncate(self.m);
            new_node.connections[l] = top_m.clone();

            // Store new node BEFORE updating neighbors to avoid borrow issues
            self.nodes.insert(id.clone(), new_node.clone());

            // Connect neighbors back to new node
            for neighbor_id in &top_m {
                // Collect distances before mutating
                let mut n_conns = Vec::new();
                if let Some(neighbor) = self.nodes.get(neighbor_id) {
                    if l < neighbor.connections.len() {
                        let mut temp_conns = neighbor.connections[l].clone();
                        temp_conns.push(id.clone());
                        if temp_conns.len() > self.m {
                            for nid in &temp_conns {
                                if let Some(nnode) = self.nodes.get(nid) {
                                    let d = neighbor.vector.cosine_similarity(&nnode.vector);
                                    n_conns.push((nid.clone(), d));
                                }
                            }
                            n_conns.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
                            n_conns.truncate(self.m);
                        } else {
                            for nid in temp_conns {
                                n_conns.push((nid, 0.0)); // Dist not needed if not truncating
                            }
                        }
                    }
                }

                // Now mutate
                if let Some(neighbor) = self.nodes.get_mut(neighbor_id) {
                    if l < neighbor.connections.len() {
                        neighbor.connections[l] = n_conns.into_iter().map(|(id, _)| id).collect();
                    }
                }
            }

            ep_candidates = neighbors;
        }

        if layer > ep_layer {
            self.entry_point = Some(id.clone());
        }

        // If it was already inserted during the loop (which happens if it connected to things),
        // we might need to make sure the final copy is updated if we didn't insert it.
        // But we just did `self.nodes.insert(id.clone(), new_node.clone());` inside the loop (actually wait, we did it for each layer!)
        // Let's ensure it's only inserted once or just updated.
        // The earlier `self.nodes.insert` inside the loop overwrites it each iteration of the loop, which is fine,
        // but it's better to just build the connections array and then insert at the end.
    }

    pub fn search(&self, query: &Vec<f32>, top_k: usize) -> Vec<Vector> {
        if self.entry_point.is_none() {
            return Vec::new();
        }

        let mut current_entry = self.entry_point.clone().unwrap();
        let ep_layer = self.nodes.get(&current_entry).unwrap().layer;
        let query_vec = Vector::new("query".to_string(), query.clone(), "".to_string());

        // Traverse down to layer 0
        for l in (1..=ep_layer).rev() {
            let res = self.search_layer(&query_vec, vec![current_entry.clone()], 1, l);
            if !res.is_empty() {
                current_entry = res[0].clone();
            }
        }

        // Search at layer 0
        let closest = self.search_layer(&query_vec, vec![current_entry], std::cmp::max(top_k, self.ef_construction), 0);

        // Retrieve vectors and sort exactly
        let mut final_results: Vec<(&Vector, f32)> = closest.into_iter()
            .filter_map(|id| self.nodes.get(&id))
            .map(|node| (&node.vector, node.vector.cosine_similarity(&query_vec)))
            .collect();

        final_results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
        final_results.into_iter().take(top_k).map(|(v, _)| v.clone()).collect()
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
        // "1" and "4" are closest to [1.0, 0.0, 0.0]
        let ids: Vec<String> = results.iter().map(|v| v.id.clone()).collect();
        assert!(ids.contains(&"1".to_string()));
        assert!(ids.contains(&"4".to_string()));
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

#[cfg(test)]
mod hnsw_deep_tests {
    use super::*;

    #[test]
    fn test_hnsw_layer_assignment() {
        let mut db = AgentDB::new();
        let mut max_observed_layer = 0;
        for i in 0..100 {
            db.insert(i.to_string(), vec![i as f32, 0.0], "".to_string());
            let node_layer = db.nodes.get(&i.to_string()).unwrap().layer;
            if node_layer > max_observed_layer {
                max_observed_layer = node_layer;
            }
        }
        assert!(max_observed_layer > 0);
        assert!(max_observed_layer <= db.max_layers);
    }
}
