use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    value: i32,
}

pub struct Map {
    root: TrieNode,
    keys: HashMap<String, i32>,
}

impl Map {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Map {
            root: TrieNode::default(),
            keys: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, key: String, val: i32) {
        let delta = val - self.keys.get(&key).unwrap_or(&0);
        self.keys.insert(key.clone(), val);

        let mut node = &mut self.root;
        for ch in key.chars() {
            node = node.children.entry(ch).or_default();
            node.value += delta;
        }
    }

    #[allow(dead_code)]
    pub fn sum(&self, prefix: String) -> i32 {
        let mut node = &self.root;
        for ch in prefix.chars() {
            if let Some(child) = node.children.get(&ch) {
                node = child;
            } else {
                return 0;
            }
        }
        node.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_sum() {
        let mut map = Map::new();
        map.insert("apple".to_string(), 3);
        assert_eq!(map.sum("ap".to_string()), 3);
        map.insert("app".to_string(), 2);
        assert_eq!(map.sum("ap".to_string()), 5);
        map.insert("apple".to_string(), 2);
        assert_eq!(map.sum("ap".to_string()), 4);
    }
}
