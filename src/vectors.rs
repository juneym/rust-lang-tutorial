
use std::mem;

//vectors are resizeable arrays
pub fn run() {
    let mut numbers: Vec<i32> =  vec![1,2,3,4];

    println!("-------------Vectors----------");
    println!("numbers is {:?}", numbers);

    println!("single value: {}", numbers[0]);

    //re-assign value to element 1
    numbers[0] = 20;

    //add or remove to/from to the vector
    numbers.push(5);
    numbers.push(6);
    numbers.pop();

    println!("single value: {}", numbers[0]);

    //length
    println!("array length is {}", numbers.len());

    //arrays are stacked allocated
    println!("This numbers array occupies {} bytes", mem::size_of_val(&numbers));

    //get a slice
    let slice: &[i32] = &numbers;

    println!("slice: {:?}", slice);

    let slice2: &[i32] = &numbers[1..2];

    println!("slice2: {:?}", slice2);


    //loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    //loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2; 
    }


    println!("Numbers Vec: {:?}", numbers);

}