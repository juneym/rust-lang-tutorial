
//variables hold primitive data or references to data
//immutable by default
//Rust is blocked-scoped language
pub fn run()
{
    let name = "Brand";
    let mut age  = 37;   //use 'mut' to make a variable mutable

    println!("My name is {} and I am {}", name, age);

    age = 38;
    println!("My name is {} and I am {}", name, age);
}