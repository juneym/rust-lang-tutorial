
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

    //define constants
    const ID: i32 = 001;

    println!("ID is {}", ID);

    //assign multiple variables at once
    let (my_name, my_age) = ("Raul", 40);

    println!("{} is {}", my_name, my_age);
}