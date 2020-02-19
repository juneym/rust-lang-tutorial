
//types: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
//       f32, f64
//       bool
//       char
//       tuples
//       Arrays (fixed length)
//       Strings
//       Vector
pub fn run() {

    //implicit & inferred
    let x = 1;  //by default i32
    let y = 2.5; //by default f64
    
     //explicit types
     let z: i64 = 4389489343;
    
     println!("x is {}, y is {}, z is {}", x, y, z);

     println!("max of i32 is {}", std::i32::MAX);
     println!("max of i64 is {}", std::i64::MAX);

     let is_active:bool = true;
     println!("{:?}", is_active);
}