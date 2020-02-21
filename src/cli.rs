// sample cli's use of arguments

use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    println!("\nargs: {:?}", args);
}