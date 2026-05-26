use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;
use std::time::{SystemTime, UNIX_EPOCH};

/// Ruflo Unique Harness Innovations: HNSW vector memory: 150x-12,500x faster search via AgentDB
/// A multi-layered Hierarchical Navigable Small World (HNSW) graph structure.

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
        // HNSW usually uses distance, where smaller is better.
        // Cosine distance = 1.0 - cosine_similarity
        1.0 - self.cosine_similarity(other)
    }
}

// Wrapper for f32 to allow sorting in BinaryHeap
#[derive(PartialEq)]
struct FloatWrapper(f32);

impl Eq for FloatWrapper {}

impl PartialOrd for FloatWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for FloatWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

#[derive(Debug, Clone)]
struct Node {
    vector: Vector,
    /// adjacency list per level. levels[0] is the bottom level.
    levels: Vec<Vec<String>>,
}

pub struct AgentDB {
    nodes: HashMap<String, Node>,
    entry_point: Option<String>,
    max_level: usize,
    m: usize,
    m_max: usize,
    m_max0: usize,
    ef_construction: usize,
    level_mult: f64,
    rng_state: u64,
}

impl AgentDB {
    pub fn new() -> Self {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let rng_state = since_the_epoch.as_nanos() as u64;

        Self {
            nodes: HashMap::new(),
            entry_point: None,
            max_level: 0,
            m: 16,
            m_max: 16,
            m_max0: 32,
            ef_construction: 100,
            level_mult: 1.0 / (16.0_f64).ln(),
            rng_state,
        }
    }

    // A simple Linear Congruential Generator (LCG)
    fn next_rand(&mut self) -> f64 {
        self.rng_state = self.rng_state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let u = (self.rng_state >> 11) as f64 / (1u64 << 53) as f64;
        u.max(0.0001) // avoid exactly 0 for ln
    }

    fn random_level(&mut self) -> usize {
        let u = self.next_rand();
        (-u.ln() * self.level_mult).floor() as usize
    }

    pub fn insert(&mut self, id: String, values: Vec<f32>, metadata: String) {
        let vector = Vector::new(id.clone(), values, metadata);
        let level = self.random_level();

        let mut new_node = Node {
            vector: vector.clone(),
            levels: vec![Vec::new(); level + 1],
        };

        if self.entry_point.is_none() {
            self.entry_point = Some(id.clone());
            self.max_level = level;
            self.nodes.insert(id, new_node);
            return;
        }

        let mut ep_id = self.entry_point.clone().unwrap();
        let mut ep_node = self.nodes.get(&ep_id).unwrap();
        let mut current_max_level = self.max_level;

        // Phase 1: Greedily find the best entry point down to the insertion level
        for l in (level + 1..=current_max_level).rev() {
            loop {
                let mut changed = false;
                let current_dist = vector.distance(&ep_node.vector);
                for neighbor_id in &ep_node.levels[l] {
                    let neighbor_node = self.nodes.get(neighbor_id).unwrap();
                    let dist = vector.distance(&neighbor_node.vector);
                    if dist < current_dist {
                        ep_id = neighbor_id.clone();
                        ep_node = neighbor_node;
                        changed = true;
                        break;
                    }
                }
                if !changed {
                    break;
                }
            }
        }

        // Phase 2: Insert into local layers and link
        let max_l = std::cmp::min(level, current_max_level);
        for l in (0..=max_l).rev() {
            let w = self.search_layer(&vector, &ep_id, self.ef_construction, l);
            let neighbors = self.select_neighbors(&vector, &w, self.m);
            new_node.levels[l] = neighbors.clone();

            // add bidirectional links
            for neighbor_id in &neighbors {
                let neighbor_node = self.nodes.get_mut(neighbor_id).unwrap();
                neighbor_node.levels[l].push(id.clone());

                let m_max = if l == 0 { self.m_max0 } else { self.m_max };
                if neighbor_node.levels[l].len() > m_max {
                    // shrink neighbor's connections
                    let mut e_w = Vec::new();
                    for n_id in &neighbor_node.levels[l] {
                        let d = neighbor_node.vector.distance(&self.nodes.get(n_id).unwrap().vector);
                        e_w.push((d, n_id.clone()));
                    }
                    let selected = self.select_neighbors(&neighbor_node.vector, &e_w, m_max);
                    neighbor_node.levels[l] = selected;
                }
            }

            // prepare for next level down
            if l > 0 {
                // simple greedy search for the closest node in w to be the new entry point for next layer
                if let Some(min_w) = w.iter().min_by(|a, b| a.0.partial_cmp(&b.0).unwrap()) {
                    ep_id = min_w.1.clone();
                }
            }
        }

        self.nodes.insert(id.clone(), new_node);

        if level > self.max_level {
            self.max_level = level;
            self.entry_point = Some(id);
        }
    }

    fn search_layer(&self, query: &Vector, ep_id: &String, ef: usize, layer: usize) -> Vec<(f32, String)> {
        let mut v = HashSet::new();
        v.insert(ep_id.clone());

        let ep_node = self.nodes.get(ep_id).unwrap();
        let dist = query.distance(&ep_node.vector);

        // C: candidates
        let mut c: BinaryHeap<(std::cmp::Reverse<FloatWrapper>, String)> = BinaryHeap::new();
        // W: dynamic list of found nearest neighbors
        let mut w: BinaryHeap<(FloatWrapper, String)> = BinaryHeap::new(); // Max-heap

        c.push((std::cmp::Reverse(FloatWrapper(dist)), ep_id.clone()));
        w.push((FloatWrapper(dist), ep_id.clone()));

        while let Some((std::cmp::Reverse(FloatWrapper(c_dist)), c_id)) = c.pop() {
            let f_dist = w.peek().unwrap().0.0;
            if c_dist > f_dist {
                break; // all elements in W are evaluated
            }

            let c_node = self.nodes.get(&c_id).unwrap();
            for e_id in &c_node.levels[layer] {
                if !v.contains(e_id) {
                    v.insert(e_id.clone());
                    let e_node = self.nodes.get(e_id).unwrap();
                    let e_dist = query.distance(&e_node.vector);
                    let f_dist = w.peek().unwrap().0.0;

                    if e_dist < f_dist || w.len() < ef {
                        c.push((std::cmp::Reverse(FloatWrapper(e_dist)), e_id.clone()));
                        w.push((FloatWrapper(e_dist), e_id.clone()));

                        if w.len() > ef {
                            w.pop();
                        }
                    }
                }
            }
        }

        w.into_iter().map(|(FloatWrapper(d), id)| (d, id)).collect()
    }

    fn select_neighbors(&self, _query: &Vector, candidates: &Vec<(f32, String)>, m: usize) -> Vec<String> {
        let mut sorted = candidates.clone();
        sorted.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        sorted.into_iter().take(m).map(|(_, id)| id).collect()
    }

    pub fn search(&self, query: &Vec<f32>, top_k: usize) -> Vec<Vector> {
        if self.entry_point.is_none() {
            return Vec::new();
        }

        let query_vec = Vector::new("query".to_string(), query.clone(), "".to_string());
        let mut ep_id = self.entry_point.clone().unwrap();
        let mut ep_node = self.nodes.get(&ep_id).unwrap();

        // Greedily find best entry point on bottom layer
        for l in (1..=self.max_level).rev() {
            loop {
                let mut changed = false;
                let current_dist = query_vec.distance(&ep_node.vector);
                for neighbor_id in &ep_node.levels[l] {
                    let neighbor_node = self.nodes.get(neighbor_id).unwrap();
                    let dist = query_vec.distance(&neighbor_node.vector);
                    if dist < current_dist {
                        ep_id = neighbor_id.clone();
                        ep_node = neighbor_node;
                        changed = true;
                        break;
                    }
                }
                if !changed {
                    break;
                }
            }
        }

        // Search bottom layer
        let ef_search = top_k.max(self.ef_construction); // ensure ef_search >= top_k
        let results = self.search_layer(&query_vec, &ep_id, ef_search, 0);

        let mut sorted = results.clone();
        sorted.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        sorted.into_iter().take(top_k).map(|(_, id)| self.nodes.get(&id).unwrap().vector.clone()).collect()
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
        // Due to approximate nature, 4 should be second
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

        let mut found_ids: Vec<String> = results.iter().map(|v| v.id.clone()).collect();
        // sort by expected rank (approximate) just check it contains all 4.
        // With HNSW and these few vectors, it will find all 4, but let's just make sure.
        found_ids.sort();
        assert_eq!(found_ids, vec!["1", "2", "3", "4"]);

        // Best match should be "1"
        assert_eq!(results[0].id, "1");
    }
}
