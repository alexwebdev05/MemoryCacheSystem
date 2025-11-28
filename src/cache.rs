use std::collections::HashMap;

pub struct Cache<K, V> {
    map: HashMap<K, V>,
    capacity: usize
}

impl<K, V> Cache<K, V> where K: Eq + std::hash::Hash {

    pub fn new(capacity: usize) -> Cache<K, V> {
        Cache {
            map: HashMap::new(),
            capacity
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.map.len() >= self.capacity {
            return;
        }
        self.map.insert(key, value);
    }
}