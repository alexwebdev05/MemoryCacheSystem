# Concurrent LRU Cache System in Rust

## Overview

Thread-safe in-memory cache system with LRU (Least Recently Used) eviction policy. Implements advanced Rust concepts: smart pointers, ownership, concurrency, and synchronization.

## Key Features

- **Thread-safe**: Concurrent access via `Arc<RwLock<T>>`
- **LRU eviction**: Automatic removal of least recently used items
- **Capacity management**: Fixed-size cache with configurable limits
- **Real-time statistics**: Tracks hits, misses, and evictions
- **High performance**: `RwLock` enables concurrent reads

## Project Structure

```
rust_cache/
src/
    main.rs          # Multi-threaded demonstration
    lib.rs           # Module exports
    cache.rs         # Main cache implementation
    lru.rs           # LRU eviction logic
    stats.rs         # Metrics tracking
        tests/
        integration_tests.rs
```

## Implementation Roadmap

### Phase 1: Basic Cache (No Concurrency, No LRU)
**Goal**: Implement simple cache with HashMap

- [x] Create `Cache<K, V>` struct with internal `HashMap`
- [x] Implement `new(capacity: usize)`
- [x] Implement `get(&self, key: &K) -> Option<&V>`
- [x] Implement `put(&mut self, key: K, value: V)`
- [x] Add capacity validation (reject when full)
- [x] Write basic unit tests

**Concepts**: Generics, HashMap, borrowing, lifetimes

---

### Phase 2: Statistics System
**Goal**: Add metrics tracking

- [x] Create `CacheStats` struct (hits, misses, evictions)
- [x] Add `stats: CacheStats` to `Cache`
- [x] Increment `hits` on successful `get()`
- [x] Increment `misses` on failed `get()`
- [x] Implement `stats(&self) -> &CacheStats`
- [x] Implement `Display` trait for `CacheStats`
- [x] Write tests for statistics accuracy

**Concepts**: Traits, immutable vs mutable methods

---

### Phase 3: Basic Thread Safety
**Goal**: Make cache accessible from multiple threads

- [x] Wrap `Cache` in `Arc<Mutex<Cache<K, V>>>`
- [x] Create helper: `create_shared_cache(capacity)`
- [x] Create 3 threads accessing same cache:
  - Thread 1: Write 50% elements
  - Thread 2: Read random elements
  - Thread 3: Write 50% more elements
- [x] Use `.lock().unwrap()` for access
- [x] Wait for threads with `join()`
- [x] Print final statistics

**Concepts**: `Arc<T>`, `Mutex<T>`, `thread::spawn`, `JoinHandle`

**Watch out for**: Deadlocks, lifetime errors

---

### Phase 4: Implement LRU (Least Recently Used)
**Goal**: Add intelligent eviction policy

**Data Structure**:
- `HashMap<K, Box<Node<K, V>>>` for O(1) access
- Doubly-linked list for usage order
- Each node: `key`, `value`, `prev`, `next`

**Tasks**:
- [x] Create `src/cache/lru.rs` with `LRUCache<K, V>`
- [x] Create `Node<K, V>` struct with:
  - `key: K`, `value: V`
  - `prev: Option<Rc<RefCell<Node<K, V>>>>`
  - `next: Option<Rc<RefCell<Node<K, V>>>>`
- [x] Implement doubly-linked list with `Rc<RefCell<>>`
- [x] Implement `get()`: move node to front (most recent)
- [x] Implement `put()`: add to front, evict from end if full
- [x] Method `remove_lru()` removes last node
- [x] Write comprehensive LRU tests

**Concepts**: `Box<T>`, `Rc<T>`, `RefCell<T>`, interior mutability, doubly-linked list

---

### Phase 5: Integrate LRU into Cache
**Goal**: Replace simple HashMap with LRUCache

- [x] Change `Cache` to use `LRUCache` internally
- [x] Update `put()` to auto-evict when full
- [x] Increment `stats.evictions` on eviction
- [x] Update all tests
- [ ] Verify statistics include correct eviction counts

**Concepts**: Composition, refactoring

---

### Phase 6: Optimize with RwLock
**Goal**: Allow concurrent reads for better performance

**Why RwLock?**
- `Mutex`: Only ONE thread at a time (reader or writer)
- `RwLock`: Multiple simultaneous readers OR one writer

**Tasks**:
- [ ] Change `Arc<Mutex<Cache>>` to `Arc<RwLock<Cache>>`
- [ ] Use `.read().unwrap()` for reads (`get`)
- [ ] Use `.write().unwrap()` for writes (`put`)
- [ ] Create 10 reader threads and 2 writer threads
- [ ] Benchmark: compare `Mutex` vs `RwLock` performance

**Concepts**: `RwLock<T>`, performance trade-offs, benchmarking

---

### Phase 7: Monitor Thread with Channels
**Goal**: Inter-thread communication using channels

- [ ] Create channel: `let (tx, rx) = mpsc::channel()`
- [ ] Make `CacheStats` `Clone` and `Send`
- [ ] Create monitor thread that:
  - Receives stats via channel
  - Prints every X seconds
  - Exits when sender closes
- [ ] Send periodic stat snapshots from workers
- [ ] Graceful shutdown handling

**Concepts**: `mpsc::channel()`, `Sender<T>`, `Receiver<T>`, `Clone`, `Send`, `Sync`

---

### Phase 8: Advanced Testing
**Goal**: Robust tests for concurrent code

- [ ] Race condition tests: many threads reading/writing
- [ ] Capacity test: verify never exceeds limit
- [ ] LRU test: verify correct eviction order
- [ ] Statistics test: verify exact counts
- [ ] Deadlock test: ensure no deadlocks
- [ ] Use `cargo test --test-threads=1` for deterministic tests

**Concepts**: Concurrent testing, assertions in multi-threading

---

### Phase 9: Enhancements (Optional)
**Goal**: Advanced features

- [ ] TTL (Time To Live): elements expire after X time
- [ ] Persistence: save/load from disk
- [ ] Advanced metrics: latency, percentiles
- [ ] Thread pool instead of individual threads
- [ ] Lock-free structures with `AtomicPtr`
- [ ] Benchmarks with `criterion`

**Concepts**: `Duration`, `Instant`, serialization (`serde`), atomics

---

## Rust Concepts Covered

- [x] `Arc<T>` - Share ownership between threads
- [x] `Mutex<T>` - Mutual exclusion
- [x] `RwLock<T>` - Concurrent reads
- [x] `Box<T>` - Heap allocation
- [x] `Rc<T>` - Reference counting (single-thread)
- [x] `RefCell<T>` - Interior mutability
- [x] `thread::spawn` - Create threads
- [x] `JoinHandle` - Wait for threads
- [x] `mpsc::channel` - Inter-thread communication
- [x] `Send` and `Sync` traits
- [x] Doubly-linked list
- [x] Concurrent testing

## Useful Commands

```bash
cargo build              # Compile
cargo run                # Run
cargo test               # Run tests
cargo test -- --nocapture  # Tests with output
cargo check              # Fast validation
cargo fmt                # Format code
cargo clippy             # Linter
cargo build --release    # Optimized build
```

---

**Built with Rust 🦀** - Demonstrating systems programming, memory safety, and fearless concurrency.
