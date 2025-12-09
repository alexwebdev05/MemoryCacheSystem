use std::io;
use std::io::Write;
use rand::prelude::*;

use memory_cache_system::LruCache;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Variables
    let mut cache: Option<LruCache<String, String>> = None;

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
                cache = Some(LruCache::new(new_capacity.trim().parse().unwrap()));
                println!("Cache created with capacity {}", new_capacity);
            },
            "demo" => {
                // Check if cache already exists
                if let Some(c) = cache.as_mut() {
                    let mut rng = rand::rng();
                    // Preset capacity
                    let capacity = c.capacity;
                    // Fill 50% of capacity
                    for i in 0..capacity/2 {
                        // Insert
                        c.put(format!("Key: {}", i), format!("Value: {}", rng.random_range(0..capacity)));
                    }
                    // Get all items that are creating
                    for _ in 0..capacity {
                        c.get(&format!("Key: {}", rng.random_range(0..capacity)));
                    }
                    // Fill 50% of capacity
                    for i in capacity/2..capacity {
                        // Insert
                        c.put(format!("Key: {}", i), format!("Value: {}", rng.random_range(0..capacity)));
                    }
                    // Stats
                    c.stats();
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
                    c.put(key, value);
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
                    let value = c.get(&key);
                    match value {
                        Some(v) => println!("Response: {}", v),
                        None => println!("Error: key not found")
                    }
                }
            },
            "stats" => {
                if let Some(c) = cache.as_mut() {
                    c.stats();
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
