use std::io;
use std::io::Write;

// Simple Calculator(only integers pls)
pub fn run(){
    let ( mut in1, mut in2, mut in3 ) = {
        ( String::new(), String::new(), String::new() )
    };
    println!("==========================");
    println!("--------CALCULATOR--------");
    println!("==========================");
    println!("CHOICES: +, -, *, /");
    println!("Exemple: Your choice: +");
    println!("==========================");

    print!("Your choice: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut in1).unwrap();
    

    print!("First num: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut in2).unwrap();

    print!("Second num: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut in3).unwrap();

    let n1: i32 = in2.trim().parse().unwrap();
    let n2: i32 = in3.trim().parse().unwrap();

    cho(in1, n1, n2);
}

fn cho(x: String, y: i32, z: i32) {

    let trimm = x.trim(); 

    if trimm == "+"{
        println!("{} + {} = {}", y, z, sum(y, z))
    } else if trimm == "-"{
        println!("{} - {} = {}", y, z, sub(y, z))
    } else if trimm == "*"{
        println!("{} * {} = {}", y, z, mult(y, z))
    } else if trimm == "/"{
        println!("{} / {} = {}", y, z, div(y, z))
    }
}
// SUM FUNCTION
fn sum(x: i32, y: i32) -> i32 {
    x + y
}
// SUBTRACTION FUNCTION
fn sub(x: i32, y: i32) -> i32 {
    x - y
}
// MULTIPLICATION FUNCTION
fn mult(x: i32, y: i32) -> i32 {
    x * y
}
// DIVISION FUNCTION
fn div(x: i32, y: i32) -> i32 {
    x / y
}