

pub struct CacheStatus {
    hits: usize,
    misses: usize,
    evictions: usize
}

impl CacheStatus {
    pub fn new() -> CacheStatus {
        CacheStatus {
            hits: 0,
            misses: 0,
            evictions: 0
        }
    }

    pub fn display(&self) {
        println!("Hits: {}\nMisses: {}\nEvictions: {}", self.hits, self.misses, self. evictions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cache_status = CacheStatus::new();
        assert_eq!(cache_status.hits, 0);
        assert_eq!(cache_status.misses, 0);
        assert_eq!(cache_status.evictions, 0);
    }

    #[test]
    fn test_display() {
        let cache_status = CacheStatus::new();
        cache_status.display();
    }
}