use std::collections::btree_map::Values;
use std::{cell::RefCell, collections::HashMap};
use std::rc::Rc;

use super::CacheStatus;

pub struct LruCache<K, V> {
    map: HashMap<K, Rc<RefCell<Node<K, V>>>>,
    head: Option<Rc<RefCell<Node<K, V>>>>,
    tail: Option<Rc<RefCell<Node<K, V>>>>,
    pub capacity: usize,
    stats: CacheStatus
}

pub struct Node<K, V> {
    key: K,
    value: V,
    prev: Option<Rc<RefCell<Node<K, V>>>>,
    next: Option<Rc<RefCell<Node<K, V>>>>
}

impl<K, V> LruCache<K, V> where K: Eq + std::hash::Hash {

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

    }

    pub fn get() {

    }

    pub fn stats() {

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

    fn remove_node(&mut self, node: Rc<RefCell<Node<K,V>>>) {
        self.map.remove(&node.borrow().key);
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
        if let Some(old_tail) = self.tail.take() {
            if let Some(prev) = old_tail.borrow().prev.clone() {
                prev.borrow_mut().next = None;
                self.tail = Some(prev);
            } else {
                self.head = None;
            }
            let key = &old_tail.borrow().key;
            self.map.remove(&key);

            Some(old_tail.clone())
        } else {
            None
        }
    }
}