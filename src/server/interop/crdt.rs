
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Clone, Debug, PartialEq)]
pub struct LWWElementSet<T: Eq + Hash + Clone> {
    pub add_set: HashMap<T, u64>,
    pub remove_set: HashMap<T, u64>,
}

impl<T: Eq + Hash + Clone> LWWElementSet<T> {
    pub fn new() -> Self {
        Self {
            add_set: HashMap::new(),
            remove_set: HashMap::new(),
        }
    }

    pub fn add(&mut self, element: T, timestamp: u64) {
        let current_add = self.add_set.entry(element.clone()).or_insert(0);
        if timestamp > *current_add {
            *current_add = timestamp;
        }
    }

    pub fn remove(&mut self, element: T, timestamp: u64) {
        let current_remove = self.remove_set.entry(element.clone()).or_insert(0);
        if timestamp > *current_remove {
            *current_remove = timestamp;
        }
    }

    pub fn lookup(&self, element: &T) -> bool {
        let add_ts = self.add_set.get(element);
        let remove_ts = self.remove_set.get(element);

        match (add_ts, remove_ts) {
            (Some(a), Some(r)) => a >= r,
            (Some(_), None) => true,
            _ => false,
        }
    }

    pub fn elements(&self) -> HashSet<T> {
        let mut result = HashSet::new();
        for k in self.add_set.keys() {
            if self.lookup(k) {
                result.insert(k.clone());
            }
        }
        result
    }

    pub fn merge(&mut self, other: &Self) {
        for (k, v) in &other.add_set {
            self.add(k.clone(), *v);
        }
        for (k, v) in &other.remove_set {
            self.remove(k.clone(), *v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lww_element_set_basic() {
        let mut set = LWWElementSet::new();
        set.add("item1", 100);
        assert!(set.lookup(&"item1"));

        set.remove("item1", 50); // Older remove, should still exist
        assert!(set.lookup(&"item1"));

        set.remove("item1", 150); // Newer remove, should be gone
        assert!(!set.lookup(&"item1"));
    }

    #[test]
    fn test_lww_element_set_merge() {
        let mut set1 = LWWElementSet::new();
        set1.add("a", 10);
        set1.add("b", 20);

        let mut set2 = LWWElementSet::new();
        set2.add("b", 15); // Older add, ignored in merge
        set2.remove("a", 15); // Newer remove

        set1.merge(&set2);

        assert!(!set1.lookup(&"a"));
        assert!(set1.lookup(&"b"));
    }

    #[test]
    fn test_lww_element_set_concurrent_same_ts() {
        let mut set = LWWElementSet::new();
        set.add("a", 10);
        set.remove("a", 10);
        // By definition, if add and remove timestamps are equal, add wins (bias to addition)
        assert!(set.lookup(&"a"));
    }
}
