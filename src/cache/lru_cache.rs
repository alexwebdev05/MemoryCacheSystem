use std::{cell::RefCell, collections::HashMap};
use std::rc::Rc;

use super::CacheStatus;

pub struct LruCache<K, V> {
    map: HashMap<K, Rc<RefCell<Node<K, V>>>>,
    pub capacity: usize,
    head: Option<Rc<RefCell<Node<K, V>>>>,
    tail: Option<Rc<RefCell<Node<K, V>>>>,
    stats: CacheStatus
}

#[derive(PartialEq, Debug)]
pub struct Node<K, V> {
    key: K,
    value: V,
    prev: Option<Rc<RefCell<Node<K, V>>>>,
    next: Option<Rc<RefCell<Node<K, V>>>>
}

impl<K, V> LruCache<K, V> where K: Eq + std::hash::Hash + Clone {

    pub fn new(capacity: usize) -> LruCache<K, V> {
        LruCache {
            map: HashMap::new(),
            head: None,
            tail: None,
            capacity,
            stats: CacheStatus::new()
        }
    }

    pub fn put(&mut self, key: K, value: V) {

        // If already exists
        if let Some(node_rc) = self.map.get(&key).cloned() {
            {
                let mut node = node_rc.borrow_mut();
                node.value = value;
            }
            self.move_to_head(node_rc);
            return;
        }
        // Check capacity
        if self.map.len() >= self.capacity {
            self.remove_tail();
        }

        // Create new node
        let node = Rc::new(RefCell::new(Node{key: key.clone(), value: value, prev: None, next: self.head.clone()}));

        self.map.insert(key, node.clone());
        self.add_to_head(&node);
        // self.move_to_head(Rc::new(RefCell::new(Node{key, value, prev: None, next: None})));
        // self.map.insert(key, Rc::new(RefCell::new(Node{key, value, prev: None, next: None})));
    }

    pub fn get(&mut self, key: &K) -> Option<V> where V: Clone {
        if let Some(node_rc) = self.map.get(key).cloned() {
            let value = node_rc.borrow().value.clone();
            self.stats.hit();
            self.move_to_head(node_rc);
            Some(value)
        } else {
            self.stats.miss();
            None
        }
    }

    pub fn remove_lru(&mut self) {
        self.remove_tail();
    }

    pub fn stats(&self) {
        self.stats.display();
    }

    fn add_to_head(&mut self, node: &Rc<RefCell<Node<K,V>>>) {
        {
            let mut n = node.borrow_mut();
            n.prev = None;
            n.next = self.head.clone();
        }

        if let Some(old_head) = &self.head {
            old_head.borrow_mut().prev = Some(node.clone());
        } else {
            self.tail = Some(node.clone());
        }
        self.head = Some(node.clone());
    }
    
    fn move_to_head(&mut self, node: Rc<RefCell<Node<K,V>>>) {
        {
            let mut n = node.borrow_mut();

            // Update prev
            if let Some(prev_node) = n.prev.clone() {
                prev_node.borrow_mut().next = n.next.clone();
            } else {
                // If it is the head
                self.head = n.next.clone();
            }

            // Update next
            if let Some(next_node) = n.next.clone() {
                next_node.borrow_mut().prev = n.prev.clone();
            } else {
                // It it is the tail
                self.tail = n.prev.clone();
            }

            // Clear pointers before send
            n.prev = None;
            n.next = None;
        }

        // Ahora engancharlo como nuevo head
        self.add_to_head(&node);

    }

    fn remove_tail(&mut self) -> Option<Rc<RefCell<Node<K,V>>>> {
        if let Some(old_tail) = self.tail.clone() {
            if let Some(prev) = old_tail.borrow().prev.clone() {
                prev.borrow_mut().next = None;
                self.tail = Some(prev);
            } else {
                self.head = None;
            }
            let key = old_tail.borrow().key.clone();
            self.map.remove(&key);

            Some(old_tail.clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::cache::CacheStatus;

    use super::LruCache;

    #[test]
    fn test_new() {
        let cache: LruCache<String, String> = LruCache::new(1000);

        assert_eq!(cache.capacity, 1000);
        assert!(cache.map.is_empty());
        assert!(cache.head.is_none());
        assert!(cache.tail.is_none());
        assert_eq!(cache.stats, CacheStatus::new());
    }

    #[test]
    fn test_put() {
        let mut cache: LruCache<String, String> = LruCache::new(2000);
        cache.put("1".to_string(), "Hello".to_string());
        let head = cache.head.as_ref().unwrap().borrow();

        assert_eq!(head.key, "1".to_string());
        assert_eq!(head.value, "Hello".to_string());
    }

    #[test]
    fn test_get() {
        let mut cache: LruCache<String, String> = LruCache::new(3000);
        let key = "1".to_string();
        cache.put(key.clone(), "Hello".to_string());
        let value = cache.get(&key);

        assert_eq!(value, Some("Hello".to_string()));
    }
}