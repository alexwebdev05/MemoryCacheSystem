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

    pub fn new() {

    }

    pub fn put() {

    }

    pub fn get() {

    }

    pub fn stats() {

    }
}