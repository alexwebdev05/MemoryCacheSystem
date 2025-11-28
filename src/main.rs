use std::io;

use memory_cache_system::Cache;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("### MEMORY CACHE SYSTEM ###");
    println!("Options: new, get, put, quit");

    loop {

        // Option
        let mut option = String::new();
        print!("Choose an option: ");
        io::stdin().read_line(&mut option)?;

        match option.trim() {
            "new" => {

                // Data
                let mut new_capacity = String::new();
                println!("Enter the cache capacity: ");
                io::stdin().read_line(&mut new_capacity)?;

                Cache::<String, String>::new(new_capacity.trim().parse()?);
                println!("Cache created with capacity {}", new_capacity);
            },
            "get" => {
                
            },
            _ => println!("Invalid option. Available: new, get, put")
        }
    }
}
