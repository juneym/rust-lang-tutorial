
//length is fixed where elements are of the same data types
pub fn run() {
    let mut numbers: [i32; 5] =  [1,2,3,4,5];

    println!("numbers is {:?}", numbers);

    println!("single value: {}", numbers[0]);

    //re-assign value to element 1
    numbers[0] = 20;
    println!("single value: {}", numbers[0]);

    //length
    println!("array length is {}", numbers.len());

    //arrays are stacked allocated
    println!("This numbers array occupies {} bytes", std::mem::size_of_val(&numbers));

}