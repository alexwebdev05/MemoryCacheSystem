use std::io;
use std::io::Write;
use rand::prelude::Rng;
use std::sync::RwLock;

use memory_cache_system::LruCache;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Variables
    let mut cache: Option<RwLock<LruCache<String, String>>> = None;

    // Start log
    println!("### MEMORY CACHE SYSTEM ###\n");

    loop {

        // Option
        let mut option = String::new();
        print!("Choose an option (new, demo, put, get, stats, quit): ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut option)?;

        match option.trim() {
            "new" => {
                // Data
                let mut new_capacity = String::new();
                print!("Enter the cache capacity: ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut new_capacity)?;

                // Cache creation
                cache = Some(RwLock::new(LruCache::new(new_capacity.trim().parse().unwrap())));
                println!("Cache created with capacity {}", new_capacity);
            },
            "demo" => {
                // Check if cache already exists
                if let Some(c) = cache.as_mut() {
                    let mut rng = rand::rng();
                    // Preset capacity
                    let capacity = c.read().unwrap().capacity;
                    // Fill 50% of capacity
                    for i in 0..capacity/2 {
                        // Insert
                        c.write().unwrap().put(format!("Key: {}", i), format!("Value: {}", rng.random_range(0..capacity)));
                    }
                    // Get all items that are creating
                    for _ in 0..capacity {
                        c.write().unwrap().get(&format!("Key: {}", rng.random_range(0..capacity)));
                    }
                    // Fill 50% of capacity
                    for i in capacity/2..capacity {
                        // Insert
                        c.write().unwrap().put(format!("Key: {}", i), format!("Value: {}", rng.random_range(0..capacity)));
                    }
                    // Stats
                    c.read().unwrap().stats();
                } else {
                    println!("Create a new cache to use this function");
                }
            },
            "put" => {
                // Data
                let mut key = String::new();
                let mut value = String::new();
                print!("Enter the value key: ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut key)?;
                print!("Enter the value: ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut value)?;

                let key = key.trim().to_string();
                let value = value.trim().to_string();

                // Insert data
                if let Some(c) = cache.as_mut() {
                    c.write().unwrap().put(key, value);
                }
            },
            "get" => {
                // Data
                let mut key = String::new();
                print!("Enter value key: ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut key)?;

                let key = key.trim().to_string();

                // Get node
                if let Some(c) = cache.as_mut() {
                    let value = c.write().unwrap().get(&key);
                    match value {
                        Some(v) => println!("Response: {}", v),
                        None => println!("Error: key not found")
                    }
                }
            },
            "stats" => {
                if let Some(c) = cache.as_mut() {
                    c.read().unwrap().stats();
                } else {
                    println!("Cache does not exist");
                }  
            },
            "quit" => {
                println!("Programe closed");
                return Ok(());
            },
            // Invalid or empty option
            _ => println!("Invalid option. Available: new, demo, put, get, stats, quit")
        }
    }
}
