pub fn run()
{
    let age: u8 = 22;
    let check_id: bool = true;
    let knows_person_of_age = true;

    //if else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender says: What do you want to drink?");
    } else if age <21 && check_id {
        println!("Bartender says: Sorry you have to leave..");
    } else {
        println!("Bartender says: I need to see your ID");       
    }

    //no ternary operator in Rust
    //here's a shorthand
    let is_of_age = if age >= 21 { true } else { false };

    println!("is_of_age {:?} ", is_of_age);
}
