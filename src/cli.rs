// sample cli's use of arguments

use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();  //why do you have to use .clone()??

    println!("\nargs: {:?}", args);
    println!("\ncommand: {}", command);    

    if command ==  "hello" {
        println!("Hey there. Hello!");
    }
}