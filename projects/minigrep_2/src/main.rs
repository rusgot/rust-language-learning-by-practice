use std::env;
use std::process;

use minigrep_2::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Create config object
    let config = Config::new(&args).unwrap_or_else(|e| {
        eprintln!("Error creating minigrep Config: {}", e);
        process::exit(1);
    });

    // Run the search
    // Print matching lines if they exist
    // If error occurs, exit with code 1
    if let Err(e) = minigrep_2::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
