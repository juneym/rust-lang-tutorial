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
}
