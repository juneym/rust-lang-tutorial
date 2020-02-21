//reference pointers - point to a resource in memory
pub fn run() {

    //primitive array
    let arr1 = [1,2,3,4,5];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));


    //Vectors -  non primitives
    let vec1 = vec![1,2,3,4,5];
    let vec2 = &vec1;   
    println!("Values: {:?}", (&vec1, vec2));
 
}