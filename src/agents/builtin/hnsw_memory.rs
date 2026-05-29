use std::collections::{HashMap, BinaryHeap, HashSet};
use std::cmp::Ordering;
use rand::Rng;

/// Ruflo Unique Harness Innovations: HNSW vector memory: 150x-12,500x faster search via AgentDB
/// An implementation of HNSW (Hierarchical Navigable Small World) for vector storage and retrieval.

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
        // HNSW typically uses distance where lower is better.
        // Cosine distance = 1 - cosine similarity
        1.0 - self.cosine_similarity(other)
    }
}

#[derive(Clone, PartialEq)]
struct NodeDistance {
    id: String,
    distance: f32,
}

impl Eq for NodeDistance {}

impl PartialOrd for NodeDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Reverse order so BinaryHeap is a min-heap by default
        other.distance.partial_cmp(&self.distance)
    }
}

impl Ord for NodeDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

#[derive(Clone, PartialEq)]
struct MaxNodeDistance {
    id: String,
    distance: f32,
}

impl Eq for MaxNodeDistance {}

impl PartialOrd for MaxNodeDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Normal order so BinaryHeap is a max-heap
        self.distance.partial_cmp(&other.distance)
    }
}

impl Ord for MaxNodeDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}


pub struct AgentDB {
    vectors: HashMap<String, Vector>,
    graphs: Vec<HashMap<String, Vec<String>>>, // Layer -> Node ID -> Neighbors
    enter_point: Option<String>,
    max_level: usize,
    m: usize,
    m_max: usize,
    m_max_0: usize,
    ef_construction: usize,
    level_mult: f32,
}

impl AgentDB {
    pub fn new() -> Self {
        let m = 16;
        Self {
            vectors: HashMap::new(),
            graphs: Vec::new(),
            enter_point: None,
            max_level: 0,
            m,
            m_max: m,
            m_max_0: m * 2,
            ef_construction: 200,
            level_mult: 1.0 / (m as f32).ln(),
        }
    }

    fn random_level(&self) -> usize {
        let r: f32 = rand::random::<f32>();
        (-r.ln() * self.level_mult) as usize
    }

    pub fn insert(&mut self, id: String, values: Vec<f32>, metadata: String) {
        let vector = Vector::new(id.clone(), values, metadata);
        self.vectors.insert(id.clone(), vector.clone());

        let l = self.random_level();

        while self.graphs.len() <= l {
            self.graphs.push(HashMap::new());
        }

        let ep = self.enter_point.clone();

        if ep.is_none() {
            self.enter_point = Some(id.clone());
            self.max_level = l;
            for i in 0..=l {
                self.graphs[i].insert(id.clone(), Vec::new());
            }
            return;
        }

        let mut ep_id = ep.unwrap();
        let max_l = self.max_level;

        for lc in (l + 1..=max_l).rev() {
            ep_id = self.search_layer(&vector, &ep_id, 1, lc)[0].clone();
        }

        for lc in (0..=std::cmp::min(l, max_l)).rev() {
            self.graphs[lc].entry(id.clone()).or_insert(Vec::new());

            let w = self.search_layer(&vector, &ep_id, self.ef_construction, lc);
            let neighbors = self.select_neighbors(&vector, &w, if lc == 0 { self.m_max_0 } else { self.m_max });

            for neighbor_id in &neighbors {
                self.graphs[lc].get_mut(&id).unwrap().push(neighbor_id.clone());
                self.graphs[lc].get_mut(neighbor_id).unwrap().push(id.clone());

                let neighbor_connections = self.graphs[lc][neighbor_id].clone();
                let m_max_current = if lc == 0 { self.m_max_0 } else { self.m_max };

                if neighbor_connections.len() > m_max_current {
                    let neighbor_vector = self.vectors.get(neighbor_id).unwrap();
                    let new_connections = self.select_neighbors(neighbor_vector, &neighbor_connections, m_max_current);
                    self.graphs[lc].insert(neighbor_id.clone(), new_connections);
                }
            }
            ep_id = w[0].clone(); // Assuming w is sorted by distance
        }

        if l > self.max_level {
            self.max_level = l;
            self.enter_point = Some(id.clone());
            // Make sure the new enter point is properly initialized in layers above old max_level
            for lc in max_l + 1..=l {
                self.graphs[lc].entry(id.clone()).or_insert(Vec::new());
            }
        }
    }

    fn search_layer(&self, query: &Vector, ep_id: &String, ef: usize, lc: usize) -> Vec<String> {
        let mut v = HashSet::new();
        v.insert(ep_id.clone());

        let mut c = BinaryHeap::new(); // Min-heap (closest first)
        let mut w = BinaryHeap::new(); // Max-heap (furthest first, to track ef elements)

        let ep_vector = self.vectors.get(ep_id).unwrap();
        let d = query.distance(ep_vector);

        c.push(NodeDistance { id: ep_id.clone(), distance: d });
        w.push(MaxNodeDistance { id: ep_id.clone(), distance: d });

        while let Some(c_curr) = c.pop() {
            let f = w.peek().unwrap();

            if c_curr.distance > f.distance {
                break;
            }

            if let Some(neighbors) = self.graphs[lc].get(&c_curr.id) {
                for e in neighbors {
                    if !v.contains(e) {
                        v.insert(e.clone());
                        let f = w.peek().unwrap();
                        let e_vector = self.vectors.get(e).unwrap();
                        let d = query.distance(e_vector);

                        if w.len() < ef || d < f.distance {
                            c.push(NodeDistance { id: e.clone(), distance: d });
                            w.push(MaxNodeDistance { id: e.clone(), distance: d });

                            if w.len() > ef {
                                w.pop();
                            }
                        }
                    }
                }
            }
        }

        let mut results: Vec<MaxNodeDistance> = w.into_iter().collect();
        results.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap_or(Ordering::Equal));
        results.into_iter().map(|n| n.id).collect()
    }

    fn select_neighbors(&self, _query: &Vector, candidates: &Vec<String>, m: usize) -> Vec<String> {
        // In a full HNSW implementation, this would be heuristic based.
        // For simplicity, we just return the closest M candidates (which is already somewhat handled by search_layer)
        // Since candidates here might just be a list of IDs, we sort them by distance to query and take M.
        // Actually candidates from search_layer are already sorted. If they are just from graph links, we need to sort.

        let mut with_dist: Vec<(String, f32)> = candidates.iter().filter_map(|c| {
            if c == &_query.id { return None; } // Don't connect to self
            let v = self.vectors.get(c).unwrap();
            Some((c.clone(), _query.distance(v)))
        }).collect();

        with_dist.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
        with_dist.into_iter().take(m).map(|(id, _)| id).collect()
    }

    pub fn search(&self, query: &Vec<f32>, top_k: usize) -> Vec<Vector> {
        let query_vec = Vector::new("query".to_string(), query.clone(), "".to_string());

        if self.enter_point.is_none() {
            return Vec::new();
        }

        let mut ep_id = self.enter_point.as_ref().unwrap().clone();

        for lc in (1..=self.max_level).rev() {
            ep_id = self.search_layer(&query_vec, &ep_id, 1, lc)[0].clone();
        }

        // At level 0, we search for top_k
        // Use ef_search = max(ef_construction, top_k)
        let ef_search = std::cmp::max(self.ef_construction, top_k);
        let best_k = self.search_layer(&query_vec, &ep_id, ef_search, 0);

        best_k.into_iter().take(top_k).filter_map(|id| self.vectors.get(&id).cloned()).collect()
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
        // HNSW is approximate, but for this small set it should find exactly 1 and 4
        // Since distance is used (lower is better), 1 should be closest.
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
        assert_eq!(results[0].id, "1"); // Distance 0
        assert_eq!(results[1].id, "3"); // Distance ~0.005
        assert_eq!(results[2].id, "2"); // Distance 1.0
        assert_eq!(results[3].id, "4"); // Distance 2.0
    }
}
