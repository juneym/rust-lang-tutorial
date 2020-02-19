
//two types 
//  1. primitive - immutable fixed-length
//  2. growable  - heap-allocated data structure
pub fn run()
{
    
    //primitive

    let mut hello = String::from("Hello ");

    println!("hello is {} and then length is {}", hello, hello.len());

    //push a char
    hello.push('w'); //need to declare the variable as mutable

    println!("hello is {} and then length is {}", hello, hello.len());

    //push a string
    hello.push_str("orld");

    println!("hello is {} and then length is {}", hello, hello.len());

    println!("Capacity is {}", hello.capacity());

    println!("is empty: {:?} ", hello.is_empty());

    println!("Contains world: {}", hello.contains("world"));

    println!("Replace world with there: {}", hello.replace("world", "there"));

    //loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }


    //Strings with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //assertion testing
    assert_eq!(2, s.len());
    
    println!("s is {}", s);
}