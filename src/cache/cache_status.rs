#[derive(PartialEq, Debug)]
pub struct CacheStatus {
    hits: usize,
    misses: usize
}

impl CacheStatus {
    pub fn new() -> CacheStatus {
        CacheStatus {
            hits: 0,
            misses: 0
        }
    }

    pub fn display(&self) {
        println!("Hits: {}\nMisses: {}", self.hits, self.misses)
    }

    pub fn hit(&mut self) {
        self.hits += 1;
    }

    pub fn miss(&mut self) {
        self.misses += 1;
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
    }

    #[test]
    fn test_display() {
        let cache_status = CacheStatus::new();
        cache_status.display();
    }

    #[test]
    fn test_hit() {
        let mut cache_status = CacheStatus::new();
        cache_status.hit();
        assert_eq!(cache_status.hits, 1);
    }

    #[test]
    fn test_miss() {
        let mut cache_status = CacheStatus::new();
        cache_status.miss();
        assert_eq!(cache_status.misses, 1);
    }
}