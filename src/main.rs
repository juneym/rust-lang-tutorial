mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod pointers;
mod structs;
mod enums;
mod cli;

fn main() {
    println!("Hello, world from main.rs");
    print::run();

    //basic formatting
    println!("{} is from {}", "Brad", "Los Angeles");

    //positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Los Angeles", "code");

    //named arguments
    println!("{name} likes to play {activity}", name = "John", activity = "Soccer");

    //placeholder traits
    println!("Binary: {:b} Hexadecimal: {:x} Octal: {:o}", 10, 10, 10);

    //placeholder traits + positional
    println!("Binary: {0:b} Hexadecimal: {1:x} Octal: {0:o}", 11, 10);

    //placeholder traits + named args
    println!("Binary: {first:b} Hexadecimal: {second:x} Octal: {third:o}",  first=11, second=10, third=30);

    //Placeholder for debug trait  - sample tuple
    println!("{:?}", (12, true, "hello"));

    //Basic Math
    println!("10 + 10 equals {}", 10 + 10);


    //vars
    vars::run();


    //types
    types::run();

    //strings
    strings::run();

    //tuples
    tuples::run();

    //arrays
    arrays::run();

    //vectors
    vectors::run();

    //conditionals
    conditionals::run();

    //loops
    loops::run();

    //functions
    functions::run();

    //pointers
    pointers::run();

    //structs
    structs::run();

    //enums
    enums::run();

    //cli
    cli::run();
}
