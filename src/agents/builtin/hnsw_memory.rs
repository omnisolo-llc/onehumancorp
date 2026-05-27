use std::collections::HashMap;
use std::cmp::Ordering;

/// Ruflo Unique Harness Innovations: HNSW vector memory: 150x-12,500x faster search via AgentDB
/// An actual HNSW implementation for vector storage and retrieval.

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
struct Node {
    vector: Vector,
    /// Edges (neighbors) per layer: layer -> vec of neighbor ids
    neighbors: Vec<Vec<String>>,
}

// Wrapper to help with Max-Heap / Min-Heap comparisons
#[derive(Debug, Clone)]
struct OrderedDistance {
    id: String,
    similarity: f32,
}

impl PartialEq for OrderedDistance {
    fn eq(&self, other: &Self) -> bool {
        self.similarity.to_bits() == other.similarity.to_bits() && self.id == other.id
    }
}

impl Eq for OrderedDistance {}

impl PartialOrd for OrderedDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Min-Heap style: `other` compares to `self`
        other.similarity.partial_cmp(&self.similarity)
            .map(|cmp| cmp.then_with(|| self.id.cmp(&other.id)))
    }
}

impl Ord for OrderedDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

pub struct AgentDB {
    nodes: HashMap<String, Node>,
    entry_point: Option<String>,
    max_level: usize,
    m_max: usize,
    m_max_0: usize,
    ef_construction: usize,
    level_mult: f64,
}

impl AgentDB {
    pub fn new() -> Self {
        let m = 16;
        Self {
            nodes: HashMap::new(),
            entry_point: None,
            max_level: 0,
            m_max: m,
            m_max_0: m * 2,
            ef_construction: 200,
            level_mult: 1.0 / (m as f64).ln(),
        }
    }

    fn random_level(&self) -> usize {
        let seed = self.nodes.len();
        let r: f64 = ((seed * 1103515245 + 12345) % 2147483648) as f64 / 2147483648.0;
        let mut level = (-r.ln() * self.level_mult).floor() as usize;
        if level > 10 {
            level = 10;
        }
        level
    }

    pub fn insert(&mut self, id: String, values: Vec<f32>, metadata: String) {
        let q_vec = Vector::new(id.clone(), values, metadata);
        let level = self.random_level();

        let mut new_node = Node {
            vector: q_vec.clone(),
            neighbors: vec![Vec::new(); level + 1],
        };

        let ep_opt = self.entry_point.clone();

        if ep_opt.is_none() {
            // First element
            self.nodes.insert(id.clone(), new_node);
            self.entry_point = Some(id);
            self.max_level = level;
            return;
        }

        let mut curr_ep = ep_opt.unwrap();
        let top_level = self.max_level;

        // Phase 1: greedy search from top layer down to the insertion level
        if level < top_level {
            for lc in (level + 1..=top_level).rev() {
                curr_ep = self.search_layer_greedy(&curr_ep, &q_vec, lc);
            }
        }

        // Phase 2: search neighbors from insertion level to layer 0
        let mut ep_candidates = vec![curr_ep.clone()];
        for lc in (0..=std::cmp::min(top_level, level)).rev() {
            let (w, curr_ep_new) = self.search_layer(&ep_candidates, &q_vec, lc, self.ef_construction);
            ep_candidates = vec![curr_ep_new];

            // Select neighbors
            let neighbors = self.select_neighbors(w, if lc == 0 { self.m_max_0 } else { self.m_max });
            new_node.neighbors[lc] = neighbors.clone();

            // Connect neighbors
            for n_id in &neighbors {
                let need_shrink;
                let m_max = if lc == 0 { self.m_max_0 } else { self.m_max };
                let mut current_neighbors = Vec::new();
                let neighbor_vec = self.nodes.get(n_id).unwrap().vector.clone();

                {
                    let neighbor_node = self.nodes.get_mut(n_id).unwrap();
                    neighbor_node.neighbors[lc].push(id.clone());
                    need_shrink = neighbor_node.neighbors[lc].len() > m_max;
                    if need_shrink {
                        current_neighbors = neighbor_node.neighbors[lc].clone();
                    }
                }

                // Shrink if too many connections
                if need_shrink {
                    let mut nn_candidates = Vec::new();
                    for nn_id in current_neighbors {
                        let nn_vec = if nn_id == id {
                            &q_vec // Use q_vec because it's not in self.nodes yet
                        } else {
                            &self.nodes.get(&nn_id).unwrap().vector
                        };
                        let sim = neighbor_vec.cosine_similarity(nn_vec);
                        nn_candidates.push(OrderedDistance { id: nn_id.clone(), similarity: sim });
                    }
                    nn_candidates.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap()); // descending
                    nn_candidates.truncate(m_max);
                    let new_neighbors: Vec<String> = nn_candidates.into_iter().map(|od| od.id).collect();
                    self.nodes.get_mut(n_id).unwrap().neighbors[lc] = new_neighbors;
                }
            }
        }

        if level > self.max_level {
            self.max_level = level;
            self.entry_point = Some(id.clone());
        }

        self.nodes.insert(id, new_node);
    }

    fn search_layer_greedy(&self, ep: &String, q: &Vector, lc: usize) -> String {
        let mut curr_node = ep;
        let mut curr_sim = self.nodes.get(curr_node).unwrap().vector.cosine_similarity(q);

        loop {
            let mut next_node = curr_node;
            let mut next_sim = curr_sim;

            for neighbor_id in &self.nodes.get(curr_node).unwrap().neighbors[lc] {
                let neighbor_sim = self.nodes.get(neighbor_id).unwrap().vector.cosine_similarity(q);
                if neighbor_sim > next_sim {
                    next_node = neighbor_id;
                    next_sim = neighbor_sim;
                }
            }
            if next_node == curr_node {
                break;
            }
            curr_node = next_node;
            curr_sim = next_sim;
        }
        curr_node.clone()
    }

    fn search_layer(&self, eps: &Vec<String>, q: &Vector, lc: usize, ef: usize) -> (Vec<OrderedDistance>, String) {
        let mut visited = std::collections::HashSet::new();
        // `candidates` will store elements to explore, sorted descending by similarity (highest first)
        let mut candidates = Vec::new();
        // `results` will store the best elements found, sorted descending by similarity
        let mut results = Vec::new();

        for ep in eps {
            visited.insert(ep.clone());
            let sim = self.nodes.get(ep).unwrap().vector.cosine_similarity(q);
            candidates.push((ep.clone(), sim));
            results.push((ep.clone(), sim));
        }

        while !candidates.is_empty() {
            candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
            let (c, c_sim) = candidates.remove(0); // element with highest similarity

            results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
            let f_sim = results.last().unwrap().1; // element with lowest similarity in results

            if c_sim < f_sim && results.len() >= ef {
                break;
            }

            for e in &self.nodes.get(&c).unwrap().neighbors[lc] {
                if !visited.contains(e) {
                    visited.insert(e.clone());
                    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
                    let f_sim = results.last().unwrap().1;

                    let e_sim = self.nodes.get(e).unwrap().vector.cosine_similarity(q);

                    if e_sim > f_sim || results.len() < ef {
                        candidates.push((e.clone(), e_sim));
                        results.push((e.clone(), e_sim));
                        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
                        if results.len() > ef {
                            results.pop(); // remove lowest similarity
                        }
                    }
                }
            }
        }

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
        let best_node = results.first().unwrap().0.clone();

        let w = results.into_iter().map(|(id, sim)| OrderedDistance { id, similarity: sim }).collect();
        (w, best_node)
    }

    fn select_neighbors(&self, mut w: Vec<OrderedDistance>, m: usize) -> Vec<String> {
        w.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap_or(Ordering::Equal));
        w.truncate(m);
        w.into_iter().map(|od| od.id).collect()
    }

    pub fn search(&self, query: &Vec<f32>, top_k: usize) -> Vec<Vector> {
        if self.entry_point.is_none() {
            return vec![];
        }

        let q_vec = Vector::new("query".to_string(), query.clone(), "".to_string());
        let mut curr_ep = self.entry_point.as_ref().unwrap().clone();

        if self.max_level > 0 {
            for lc in (1..=self.max_level).rev() {
                curr_ep = self.search_layer_greedy(&curr_ep, &q_vec, lc);
            }
        }

        let ef_search = std::cmp::max(top_k, 50); // ef must be >= top_k
        let (w, _) = self.search_layer(&vec![curr_ep], &q_vec, 0, ef_search);

        let mut res = w;
        res.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap_or(Ordering::Equal));
        res.truncate(top_k);

        res.into_iter().map(|od| self.nodes.get(&od.id).unwrap().vector.clone()).collect()
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

#[cfg(test)]
mod hnsw_deep_tests {
    use super::*;

    #[test]
    fn test_agentdb_hnsw_large_insertion() {
        let mut db = AgentDB::new();
        // Insert 100 vectors
        for i in 0..100 {
            // Generate some deterministic vectors
            let vec = vec![(i as f32) / 100.0, 1.0 - (i as f32) / 100.0];
            db.insert(i.to_string(), vec, format!("meta {}", i));
        }

        // Query near the middle (0.5, 0.5)
        let query = vec![0.5, 0.5];
        let results = db.search(&query, 5);

        assert_eq!(results.len(), 5);
        // The closest one should be around 50
        let closest_id: i32 = results[0].id.parse().unwrap();
        assert!(closest_id >= 45 && closest_id <= 55, "Closest was {}", closest_id);
    }
}
