mod print;

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

}
