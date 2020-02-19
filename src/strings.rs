
//two types 
//  1. primitive - immutable fixed-length
//  2. growable  - heap-allocated data structure
pub fn run()
{
    
    //primitive

    let mut hello = String::from("Hello");

    println!("hello is {} and then length is {}", hello, hello.len());

    //push a char
    hello.push('w'); //need to declare the variable as mutable

    println!("hello is {} and then length is {}", hello, hello.len());

    //push a string
    hello.push_str("orld");

    println!("hello is {} and then length is {}", hello, hello.len());
}