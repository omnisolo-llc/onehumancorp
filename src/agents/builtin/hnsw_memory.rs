#![allow(clippy::useless_vec)]
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

/// Ruflo Unique Harness Innovations: HNSW vector memory: 150x-12,500x faster search via AgentDB
/// A hierarchical navigable small world (HNSW) graph implementation for fast approximate nearest neighbor search.

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Vector {
    pub id: String,
    pub values: Vec<f32>,
    pub metadata: String,
}

impl Vector {
    pub fn new(id: String, values: Vec<f32>, metadata: String) -> Self {
        Self {
            id,
            values,
            metadata,
        }
    }

    /// Computes the cosine similarity. If the norms are 0, it returns 0.0.
    pub fn cosine_similarity(&self, other: &Vector) -> f32 {
        let dot_product: f32 = self
            .values
            .iter()
            .zip(other.values.iter())
            .map(|(a, b)| a * b)
            .sum();
        let norm_a: f32 = self.values.iter().map(|a| a * a).sum::<f32>().sqrt();
        let norm_b: f32 = other.values.iter().map(|b| b * b).sum::<f32>().sqrt();
        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot_product / (norm_a * norm_b)
        }
    }

    /// Computes angular distance (1.0 - cosine_similarity).
    /// Used for proximity (lower is closer).
    pub fn distance(&self, other: &Vector) -> f32 {
        1.0 - self.cosine_similarity(other)
    }
}

/// Helper struct for priority queues holding distance and node id.
/// Sorted such that smallest distance is popped first.
#[derive(Debug, Clone)]
struct DistNode {
    dist: f32,
    id: String,
}

impl PartialEq for DistNode {
    fn eq(&self, other: &Self) -> bool {
        self.dist.to_bits() == other.dist.to_bits() && self.id == other.id
    }
}

impl Eq for DistNode {}

impl PartialOrd for DistNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DistNode {
    fn cmp(&self, other: &Self) -> Ordering {
        if other.dist < self.dist { Ordering::Less } else if other.dist > self.dist { Ordering::Greater } else { Ordering::Equal }
    }
}

/// Helper struct for max-heaps (largest distance popped first).
#[derive(Debug, Clone)]
struct MaxDistNode {
    dist: f32,
    id: String,
}

impl PartialEq for MaxDistNode {
    fn eq(&self, other: &Self) -> bool {
        self.dist.to_bits() == other.dist.to_bits() && self.id == other.id
    }
}

impl Eq for MaxDistNode {}

impl PartialOrd for MaxDistNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MaxDistNode {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.dist < other.dist { Ordering::Less } else if self.dist > other.dist { Ordering::Greater } else { Ordering::Equal }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AgentDB {
    vectors: HashMap<String, Vector>,

    // HNSW specific parameters
    m: usize,               // max number of connections per layer
    m_max: usize,           // max number of connections at layer 0
    m_max0: usize,          // max number of connections for layer 0
    ef_construction: usize, // size of the dynamic candidate list
    ml: f32,                // level generation factor

    // Entry point
    enter_point: Option<String>,
    max_level: usize,

    // Layer structures: Graph edges at each level.
    // layers[l] maps node_id -> Vec<neighbor_id>
    layers: Vec<HashMap<String, Vec<String>>>,
}

impl Default for AgentDB {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentDB {
    pub fn new() -> Self {
        Self::with_params(16, 200)
    }

    pub fn with_params(m: usize, ef_construction: usize) -> Self {
        Self {
            vectors: HashMap::new(),
            m,
            m_max: m,
            m_max0: m * 2,
            ef_construction,
            ml: 1.0 / (m as f32).ln(),
            enter_point: None,
            max_level: 0,
            layers: vec![HashMap::new()], // Always have layer 0
        }
    }

    /// Randomly determines the max layer for a new node.
    fn random_level(&self) -> usize {
        // Since we are not using `rand` crate to avoid external deps issues,
        // we use a simple deterministic hash-based pseudo-random generator based on total count
        let count = self.vectors.len();
        // Just a simple linear congruential generator step
        let pseudo_rand =
            (count.wrapping_mul(1103515245).wrapping_add(12345) % 1000000) as f32 / 1000000.0;
        let r = if pseudo_rand == 0.0 {
            0.000001
        } else {
            pseudo_rand
        };
        (-r.ln() * self.ml).floor() as usize
    }

    pub fn insert(&mut self, id: String, values: Vec<f32>, metadata: String) {
        let q = Vector::new(id.clone(), values, metadata);

        // Ensure not already inserted
        if self.vectors.contains_key(&id) {
            return;
        }

        self.vectors.insert(id.clone(), q.clone());

        let l = self.random_level();

        // Expand layers if needed
        while self.layers.len() <= l {
            self.layers.push(HashMap::new());
        }

        let ep = self.enter_point.clone();

        if let Some(ref current_ep) = ep {
            let mut w = vec![current_ep.clone()];
            let top_level = self.max_level;

            // Search down to level l+1
            for lc in (l + 1..=top_level).rev() {
                w = self.search_layer(&q, &w, 1, lc);
            }

            // From level min(L, top_level) down to 0
            let start_level = std::cmp::min(l, top_level);
            for lc in (0..=start_level).rev() {
                w = self.search_layer(&q, &w, self.ef_construction, lc);

                // Select M neighbors to connect
                let neighbors = self.select_neighbors(&q, &w, self.m);
                self.layers[lc].insert(id.clone(), neighbors.clone());

                // Add bidirectional connections
                for neighbor in neighbors {
                    let neighbor_links = self.layers[lc]
                        .entry(neighbor.clone())
                        .or_default();
                    neighbor_links.push(id.clone());
                    let links_len = neighbor_links.len();
                    let current_links = neighbor_links.clone();

                    let max_m = if lc == 0 { self.m_max0 } else { self.m_max };

                    // Shrink connections if exceeded
                    if links_len > max_m {
                        if let Some(e_node) = self.vectors.get(&neighbor) {
                            let shrunk = self.select_neighbors(e_node, &current_links, max_m);
                            self.layers[lc].insert(neighbor, shrunk);
                        }
                    }
                }
            }
        } else {
            // First element
            for lc in 0..=l {
                self.layers[lc].insert(id.clone(), Vec::new());
            }
        }

        // Update enter point if highest layer
        if ep.is_none() || l > self.max_level {
            self.max_level = l;
            self.enter_point = Some(id.clone());
        }
    }

    /// Search for candidate neighbors in a specific layer.
    fn search_layer(&self, q: &Vector, ep: &[String], ef: usize, lc: usize) -> Vec<String> {
        let mut v = HashSet::new();
        let mut c = BinaryHeap::new(); // Min-heap (closest to q)
        let mut w = BinaryHeap::new(); // Max-heap (farthest from q in candidate list)

        for p in ep {
            v.insert(p.clone());
            if let Some(node) = self.vectors.get(p) {
                let dist = q.distance(node);
                c.push(DistNode {
                    dist,
                    id: p.clone(),
                });
                w.push(MaxDistNode {
                    dist,
                    id: p.clone(),
                });
            }
        }

        while let Some(DistNode {
            dist: c_dist,
            id: c_id,
        }) = c.pop()
        {
            if let Some(farthest) = w.peek()
                && c_dist > farthest.dist {
                    break; // All remaining candidates in C are further than the furthest in W
                }

            if let Some(neighbors) = self.layers[lc].get(&c_id) {
                for e in neighbors {
                    if !v.contains(e) {
                        v.insert(e.clone());
                        if let Some(e_node) = self.vectors.get(e) {
                            let e_dist = q.distance(e_node);

                            let furthest_w_dist = w.peek().map(|n| n.dist).unwrap_or(f32::MAX);

                            if e_dist < furthest_w_dist || w.len() < ef {
                                c.push(DistNode {
                                    dist: e_dist,
                                    id: e.clone(),
                                });
                                w.push(MaxDistNode {
                                    dist: e_dist,
                                    id: e.clone(),
                                });

                                if w.len() > ef {
                                    w.pop(); // Remove the furthest
                                }
                            }
                        }
                    }
                }
            }
        }

        w.into_iter().map(|n| n.id).collect()
    }

    /// Selects M neighbors based on a simple distance heuristic
    fn select_neighbors(&self, q: &Vector, candidates: &[String], m: usize) -> Vec<String> {
        let mut heap = BinaryHeap::new();
        for c_id in candidates {
            if c_id == &q.id {
                continue;
            } // Don't connect to self
            if let Some(node) = self.vectors.get(c_id) {
                let dist = q.distance(node);
                heap.push(DistNode {
                    dist,
                    id: c_id.clone(),
                });
            }
        }

        let mut selected = Vec::new();
        while let Some(node) = heap.pop() {
            if selected.len() >= m {
                break;
            }
            selected.push(node.id);
        }
        selected
    }

    /// KNN search using the HNSW index.
    pub fn search(&self, query: &[f32], top_k: usize) -> Vec<Vector> {
        let enter_point = match &self.enter_point {
            Some(ep) => ep.clone(),
            None => return Vec::new(),
        };

        let q = Vector::new("query".to_string(), query.to_owned(), "".to_string());
        let mut ep = vec![enter_point];

        // Search down to level 1
        for lc in (1..=self.max_level).rev() {
            ep = self.search_layer(&q, &ep, 1, lc);
        }

        // Search in level 0
        let ef = std::cmp::max(top_k, self.ef_construction);
        let candidates = self.search_layer(&q, &ep, ef, 0);

        // Filter out candidates that don't exist in vectors
        let mut valid_candidates: Vec<String> = candidates
            .into_iter()
            .filter(|id| self.vectors.contains_key(id))
            .collect();

        // Sort by actual distance and take top_k
        valid_candidates.sort_by(|a, b| {
            let da = q.distance(self.vectors.get(a).expect("Verified existence above"));
            let db = q.distance(self.vectors.get(b).expect("Verified existence above"));
            if da < db { Ordering::Less } else if da > db { Ordering::Greater } else { Ordering::Equal }
        });

        valid_candidates
            .into_iter()
            .take(top_k)
            .filter_map(|id| self.vectors.get(&id).cloned())
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
    fn test_hnsw_hierarchy() {
        let mut db = AgentDB::with_params(2, 10);
        // Force deterministic layer generation behavior based on insertion order logic
        db.insert("A".to_string(), vec![1.0, 0.0, 0.0], "".to_string());
        db.insert("B".to_string(), vec![0.0, 1.0, 0.0], "".to_string());
        db.insert("C".to_string(), vec![0.0, 0.0, 1.0], "".to_string());
        db.insert("D".to_string(), vec![0.5, 0.5, 0.0], "".to_string());
        db.insert("E".to_string(), vec![0.0, 0.5, 0.5], "".to_string());

        let results = db.search(&vec![1.0, 0.0, 0.0], 2);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].id, "A");
        assert_eq!(results[1].id, "D");

        // Verify graph structure was created
        assert!(db.layers[0].contains_key("A"));
    }

    #[test]
    fn test_hnsw_persistence_serialization_deserialization() {
        let mut db = AgentDB::with_params(2, 10);
        db.insert(
            "persist_1".to_string(),
            vec![1.0, 0.5, 0.0],
            "data".to_string(),
        );

        // We can serialize and deserialize as JSON because it contains HashMaps, Vecs, and String.
        let json_str = serde_json::to_string(&db).expect("Failed to serialize AgentDB");
        let db_restored: AgentDB = serde_json::from_str(&json_str).expect("Failed to deserialize AgentDB");

        assert_eq!(db_restored.vectors.len(), 1);
        let results = db_restored.search(&vec![1.0, 0.5, 0.0], 1);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "persist_1");
    }
}
