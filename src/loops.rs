pub fn run() {

    let mut count = 0;

    //infinite loop!
    loop {
        count += 1;
        println!("Number: {} ", count);
        if count == 20 {
            break;
        }
    }


    //while loop (FizzBuzz)
    //if divisible by 3 print Fizz
    //if divisible by 5 print Buzz
    //if divisble by both print FizzBuzz
    while count <= 100 {
        if count % 15 == 0 {
            println!("{} fizzbuzz", count);
        } else if count % 3 == 0 {
            println!("{} fizz", count);
        } else if count % 5 == 0 {
            println!("{} buzz", count);
        }

        count += 1;
    }

    //for-range loop
    for y in 0..100 {
        if y % 15 == 0 {
            println!("x is {}", y);
        }
    }

}