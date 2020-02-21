pub fn run() {
    greeting("Hello", "Rachelle");

    //bind values to variables
    let get_sum = add(5,3);
    println!("get_sum {}", get_sum);

    let n3: i32 =  17;
    //closure
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C sum: {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(n1: i32, n2: i32)  -> i32 {

    //when we don't use a semi-colon, 
    //you're telling the compiler that it's the 
    //value you want to return
    n1 + n2  
}