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

    pub fn put(&mut self, key: K, value: V) {
        if self.map.len() >= self.capacity {
            println!("Capacity is full");
            return;
        }
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

}

#[cfg(test)]
mod rests {
    use super::Cache;

    #[test]
    fn test_new() {
        let cache: Cache::<String, String> = Cache::new(1000);
        assert_eq!(cache.capacity, 1000);
    }

    #[test]
    fn test_put() {
        let mut cache: Cache::<String, String> = Cache::new(2000);
        cache.put("1".to_string(), "Hello".to_string());
        assert_eq!(cache.map["1"], "Hello".to_string());
    }

    #[test]
    fn test_get() {
        let mut cache: Cache::<String, String> = Cache::new(3000);
        cache.put("1".to_string(), "Hello".to_string()); 
        assert_eq!(cache.get(&"1".to_string()), Some(&"Hello".to_string()));
    }
}