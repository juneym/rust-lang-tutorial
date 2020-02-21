//enums are types with have a few definite values


enum Movement {
    //variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {
    //perform actions based on 'm' (movement)
    //use 'match' which is like a 'switch'

    match m {
        Movement::Up   => println!("Avatar is moving up"),
        Movement::Down => println!("Avatar is moving down"),
        Movement::Left => println!("Avatar is moving left"),
        Movement::Right => println!("Avatar is moving right"),
    }
}


pub fn run() {

    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Down;
    let avatar4 = Movement::Right;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);    
    move_avatar(avatar4);    


}