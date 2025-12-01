use std::io;
use std::io::Write;

use memory_cache_system::Cache;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Variables
    let mut cache: Option<Cache<String, String>> = None;

    println!("### MEMORY CACHE SYSTEM ###");
    println!("Options: new, get, put, quit");

    loop {

        // Option
        let mut option = String::new();
        print!("Choose an option: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut option)?;

        match option.trim() {
            "new" => {
                // Data
                let mut new_capacity = String::new();
                print!("Enter the cache capacity: ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut new_capacity)?;

                cache = Some(Cache::<String, String>::new(new_capacity.trim().parse()?));
                println!("Cache created with capacity {}", new_capacity);
            },
            "get" => {
                
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

                if let Some(ref mut c) = cache {
                    c.put(key.trim().parse()?, value.trim().parse()?);
                }

            }
            _ => println!("Invalid option. Available: new, get, put")
        }
    }
}
