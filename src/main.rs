use std::io;
use std::io::Write;
use std::sync::{Arc, Mutex};

use memory_cache_system::Cache;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Variables
    let cache: Arc<Mutex<Option<Cache<String, String>>>> = Arc::new(Mutex::new(None));

    // Start log
    println!("### MEMORY CACHE SYSTEM ###\n");

    loop {

        // Option
        let mut option = String::new();
        print!("Choose an option (new, put, get, quit, stats): ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut option)?;

        match option.trim() {
            "new" => {
                // Data
                let mut guard = cache.lock().unwrap();
                let mut new_capacity = String::new();
                print!("Enter the cache capacity: ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut new_capacity)?;

                // Cache creation
                *guard = Some(Cache::<String, String>::new(new_capacity.trim().parse()?));
                println!("Cache created with capacity {}", new_capacity);
            },
            "put" => {
                // Data
                let mut guard = cache.lock().unwrap();
                let mut key = String::new();
                let mut value = String::new();
                print!("Enter the value key: ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut key)?;
                print!("Enter the value: ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut value)?;

                // Insert data
                if let Some(ref mut c) = *guard {
                    c.put(key.trim().parse()?, value.trim().to_string());
                }

            },
            "get" => {
                // Data
                let mut guard = cache.lock().unwrap();
                let mut key = String::new();
                print!("Enter value key: ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut key)?;

                if let Some(ref mut c) = *guard {
                    match c.get(&key.trim().to_string()) {
                        Some(value) => {
                            println!("Value: {}", value)
                    },
                        _ => println!("Key not found")
                    }
                    
                }
            },
            "stats" => {
                let mut guard = cache.lock().unwrap();
                if let Some(ref mut c) = *guard {
                    c.stats();
                }
                println!("Cache does not exist")
            }
            // Invalid or empty option
            _ => println!("Invalid option. Available: new, get, put")
        }
    }
}
