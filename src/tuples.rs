
//tuples are grouped of values, can be of different types
pub fn run()
{
    let person: (&str, &str, i8) = ("Brad", "Los Angeles", 37);
    

    println!("{} is from {} and is {} years old", person.0, person.1, person.2);

}