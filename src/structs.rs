
//traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

//tuple structs
struct ColorTuple(u8, u8, u8);


//structs are used to create custom data types
pub fn run() {


    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    println!("color: {} {} {}", c.red, c.green, c.blue);

    let mut c2 = ColorTuple(255, 0, 0);
    c2.1 = 200;
    
    println!("color2: {} {} {}", c2.0, c2.1, c2.2);


}