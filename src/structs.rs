
//traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

//tuple structs
struct ColorTuple(u8, u8, u8);


//persons structs - structs with methods
struct Person {
    first_name: String,
    last_name: String
}

//implement a Person struct by adding methods
impl Person {
    //construct a person
    fn new(first: &str, last: &str) -> Person {
        Person{
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    //get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)  //no semi-colon for returning data
    }

    //set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    //return a tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }

}


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


    let mut person = Person::new("John", "Doe");
    person.set_last_name("Doeeer");
    println!("person: {} {} - {}", person.first_name, person.last_name, person.full_name());

    println!("person: {:?}", person.to_tuple());
}