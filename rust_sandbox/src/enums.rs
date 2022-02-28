// Enums are types which have a few definite values

enum Moviment {
    // Varients
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Moviment){
    // Perform action depending on info
    match m {
        Moviment::Up => println!("Avatar moving up"),
        Moviment::Down => println!("Avatar moving down"),
        Moviment::Left => println!("Avatar moving left"),
        Moviment::Right => println!("Avatar moving right")
    }
}

pub fn run(){
    let avatar1 = Moviment::Left;
    let avatar2 = Moviment::Up;
    let avatar3 = Moviment::Right;
    let avatar4 = Moviment::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}