// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Gabriel";
    let mut age = 17;
    println!("\nMy name is {} and I am {}\n", name, age);
    age = 18;
    println!("My name is {} and I am {}\n", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}\n", ID);

    //Assign multiple vars
    let ( my_name, my_age ) = ("Gabriel", 17);
    println!("{} is {}\n", my_name, my_age);
}