use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

/// Ruflo Unique Harness Innovations: HNSW vector memory: 150x-12,500x faster search via AgentDB
/// A basic implementation of AgentDB using Hierarchical Navigable Small World (HNSW)
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
    pub connections: Vec<Vec<usize>>,
}

// Min-heap for keeping track of the closest elements (closest = highest cosine similarity)
#[derive(PartialEq)]
struct MinDist(usize, f32);
impl Eq for MinDist {}
impl PartialOrd for MinDist {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // We want the smallest distance (lowest similarity) to be popped first, so we reverse the comparison.
        other.1.partial_cmp(&self.1)
    }
}
impl Ord for MinDist {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

// Max-heap for traversing (furthest to pop first?) Actually for traversal we usually keep closest at the top.
// Let's keep it simple.

pub struct AgentDB {
    nodes: Vec<HnswNode>,
    entry_point: Option<usize>,
    m: usize,
    m_max: usize,
    m_max0: usize,
    level_mult: f64,
    ef_construction: usize,
}

// Pseudo-random generator to avoid external dependencies if possible, or we can just use a simple LCG
struct Lcg {
    state: u64,
}
impl Lcg {
    fn new(seed: u64) -> Self { Self { state: seed } }
    fn next_f64(&mut self) -> f64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let val = (self.state >> 11) as f64;
        val / (1u64 << 53) as f64
    }
}

impl AgentDB {
    pub fn new() -> Self {
        let m = 16;
        Self {
            nodes: Vec::new(),
            entry_point: None,
            m,
            m_max: m,
            m_max0: m * 2,
            level_mult: 1.0 / (m as f64).ln(),
            ef_construction: 64,
        }
    }

    fn random_level(&self, mut unif: f64) -> usize {
        if unif < 1e-6 { unif = 1e-6; }
        (-unif.ln() * self.level_mult) as usize
    }

    // Search layer: returns `ef` nearest neighbors
    fn search_layer(&self, query: &Vector, entry_point: usize, ef: usize, layer: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        visited.insert(entry_point);

        let mut candidates = BinaryHeap::new(); // Max-heap: holds nearest candidates (so we pop furthest of the nearest?)
        // Wait, for finding nearest, candidates should pop the CLOSEST to explore. So MinDist where we reverse.
        // Actually, rust BinaryHeap is a max-heap. If we want to pop the highest similarity first, we just store similarity.
        #[derive(PartialEq)]
        struct MaxSim(usize, f32);
        impl Eq for MaxSim {}
        impl PartialOrd for MaxSim {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.1.partial_cmp(&other.1)
            }
        }
        impl Ord for MaxSim {
            fn cmp(&self, other: &Self) -> Ordering {
                self.partial_cmp(other).unwrap_or(Ordering::Equal)
            }
        }

        // To keep the best `ef` results, we can keep a min-heap of the best found so far.
        // If the new element is worse than the worst in `ef` best, we might still explore it, but we don't add it to best.
        let dist = self.nodes[entry_point].vector.cosine_similarity(query);
        candidates.push(MaxSim(entry_point, dist));

        let mut results = BinaryHeap::new(); // Min-heap to keep top `ef` results
        results.push(MinDist(entry_point, dist));

        while let Some(MaxSim(c, c_dist)) = candidates.pop() {
            // results.peek() gives the WORST (lowest similarity) among the best `ef`
            if let Some(MinDist(_, worst_dist)) = results.peek() {
                if c_dist < *worst_dist && results.len() >= ef {
                    break;
                }
            }

            for &neighbor in &self.nodes[c].connections[layer] {
                if visited.insert(neighbor) {
                    let n_dist = self.nodes[neighbor].vector.cosine_similarity(query);

                    if results.len() < ef || n_dist > results.peek().unwrap().1 {
                        candidates.push(MaxSim(neighbor, n_dist));
                        results.push(MinDist(neighbor, n_dist));
                        if results.len() > ef {
                            results.pop(); // remove the worst
                        }
                    }
                }
            }
        }

        // Return results sorted by similarity descending
        let mut final_results: Vec<_> = results.into_iter().collect();
        final_results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
        final_results.into_iter().map(|m| m.0).collect()
    }

    pub fn insert(&mut self, id: String, values: Vec<f32>, metadata: String) {
        let vec = Vector::new(id, values, metadata);
        let mut lcg = Lcg::new((self.nodes.len() as u64).wrapping_add(12345));
        let level = self.random_level(lcg.next_f64());

        let new_idx = self.nodes.len();
        let mut connections = vec![Vec::new(); level + 1];

        if self.nodes.is_empty() {
            self.nodes.push(HnswNode { vector: vec, connections });
            self.entry_point = Some(new_idx);
            return;
        }

        let mut ep = self.entry_point.unwrap();
        let max_level = self.nodes[ep].connections.len() - 1;

        // Traverse down from top layer to `level + 1`
        for l in (level + 1..=max_level).rev() {
            let mut curr = ep;
            let mut curr_dist = self.nodes[curr].vector.cosine_similarity(&vec);
            loop {
                let mut best_neighbor = curr;
                let mut best_dist = curr_dist;
                for &neighbor in &self.nodes[curr].connections[l] {
                    let d = self.nodes[neighbor].vector.cosine_similarity(&vec);
                    if d > best_dist {
                        best_dist = d;
                        best_neighbor = neighbor;
                    }
                }
                if best_neighbor == curr {
                    break;
                }
                curr = best_neighbor;
                curr_dist = best_dist;
            }
            ep = curr;
        }

        // Insert at layers `min(level, max_level)` down to 0
        let start_level = level.min(max_level);
        for l in (0..=start_level).rev() {
            let neighbors = self.search_layer(&vec, ep, self.ef_construction, l);
            let m_conn = if l == 0 { self.m_max0 } else { self.m };

            // Connect to top `m`
            let to_connect = neighbors.into_iter().take(self.m).collect::<Vec<_>>();
            for &n in &to_connect {
                connections[l].push(n);
            }

            self.nodes.push(HnswNode { vector: vec.clone(), connections: connections.clone() });
            let _ = self.nodes.pop(); // remove temporary, we'll push at the end

            // Back-connections
            for &n in &to_connect {
                let node_n = &mut self.nodes[n];
                node_n.connections[l].push(new_idx);

                // Prune if exceeds max connections
                if node_n.connections[l].len() > m_conn {
                    let mut node_n_conns = node_n.connections[l].clone();
                    let node_n_vec = &node_n.vector;
                    node_n_conns.sort_by(|&a, &b| {
                        let da = self.nodes[a].vector.cosine_similarity(node_n_vec);
                        let db = self.nodes[b].vector.cosine_similarity(node_n_vec);
                        db.partial_cmp(&da).unwrap_or(Ordering::Equal)
                    });
                    node_n_conns.truncate(m_conn);
                    self.nodes[n].connections[l] = node_n_conns;
                }
            }

            // Update ep to the closest for the next level down
            if !to_connect.is_empty() {
                ep = to_connect[0];
            }
        }

        self.nodes.push(HnswNode { vector: vec, connections });

        if level > max_level {
            self.entry_point = Some(new_idx);
        }
    }

    pub fn search(&self, query: &Vec<f32>, top_k: usize) -> Vec<Vector> {
        if self.nodes.is_empty() {
            return Vec::new();
        }

        let query_vec = Vector::new("query".to_string(), query.clone(), "".to_string());
        let mut ep = self.entry_point.unwrap();
        let max_level = self.nodes[ep].connections.len() - 1;

        for l in (1..=max_level).rev() {
            let mut curr = ep;
            let mut curr_dist = self.nodes[curr].vector.cosine_similarity(&query_vec);
            loop {
                let mut best_neighbor = curr;
                let mut best_dist = curr_dist;
                for &neighbor in &self.nodes[curr].connections[l] {
                    let d = self.nodes[neighbor].vector.cosine_similarity(&query_vec);
                    if d > best_dist {
                        best_dist = d;
                        best_neighbor = neighbor;
                    }
                }
                if best_neighbor == curr {
                    break;
                }
                curr = best_neighbor;
                curr_dist = best_dist;
            }
            ep = curr;
        }

        let nearest = self.search_layer(&query_vec, ep, top_k.max(10), 0);
        nearest.into_iter().take(top_k).map(|idx| self.nodes[idx].vector.clone()).collect()
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
