use std::io;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

use memory_cache_system::Cache;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Variables
    let cache: Arc<Mutex<Option<Cache<String, String>>>> = Arc::new(Mutex::new(None));

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
                let mut guard = cache.lock().unwrap();
                let mut new_capacity = String::new();
                print!("Enter the cache capacity: ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut new_capacity)?;

                // Cache creation
                *guard = Some(Cache::new(new_capacity.trim().parse()?));
                println!("Cache created with capacity {}", new_capacity);
            },
            "demo" => {
                // Check if cache already exists
                let guard = cache.lock().unwrap();
                if guard.is_none() {
                    println!("Error: Create a cache first with 'new'");
                    continue;
                }
                drop(guard);

                // Get capacity before spawn threads
                let capacity = {
                    let guard = cache.lock().unwrap();
                    match *guard {
                        Some(ref c) => c.capacity,
                        None => {
                            println!("Error: Create a cache first with 'new'");
                            continue;
                        }
                    }
                };

                // Thread 1
                let cache_clone1 = Arc::clone(&cache);
                let handler1 = thread::spawn(move || {
                    for i in 0..capacity/2 {
                        let mut guard = cache_clone1.lock().unwrap();
                        if let Some(ref mut c) = *guard {
                            c.put(format!("key {}", i), format!("value {}", i));
                        }
                    }
                    println!("Thread 1 finished");
                });

                // Thread 2
                let cache_clone2 = Arc::clone(&cache);
                let handler2 = thread::spawn(move || {
                    for _ in 0..capacity {
                        let random_key = rand::random_range(0..capacity);
                        let mut guard = cache_clone2.lock().unwrap();
                        if let Some(ref mut c) = *guard {
                            c.get(&format!("key {}", random_key));
                        }
                    }
                    println!("Thread 2 finished");
                });
                
                // Thread 3
                let cache_clone3 = Arc::clone(&cache);
                let handler3 = thread::spawn(move || {
                    for i in capacity/2..capacity {
                        let mut guard = cache_clone3.lock().unwrap();
                        if let Some(ref mut c) = *guard {
                            c.put(format!("key {}", i), format!("value {}", i));
                        }
                    }
                    println!("Thread 3 finished");
                });

                handler1.join().unwrap();
                handler2.join().unwrap();
                handler3.join().unwrap();

                println!("Demo comleted!");

                let guard = cache.lock().unwrap();
                if let Some(ref c) = *guard {
                    c.stats();
                } else {
                    println!("Error: cache removed during demo");
                }

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
