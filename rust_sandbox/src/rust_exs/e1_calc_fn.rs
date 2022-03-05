#![allow(dead_code)]

use std::io;
use std::io::Write;

pub fn run(){
    let ( mut in1, mut in2 ) = ( String::new(), String::new());
    let mut cho1 = String::new();

    println!("===================================");
    print!("Your choice(1=+, 2=-, 3=*, 4=/): ", );
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut cho1).unwrap();
    let cho2: u8 = cho1.trim().parse().unwrap();

    print!("Num 1: ", );
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut in1).unwrap();
    let n1: f32 = in1.trim().parse().unwrap();

    print!("Num 2: ", );
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut in2).unwrap();
    let n2: f32 = in2.trim().parse().unwrap();

    match cho2 {

        1 => println!("{} + {} = {}", n1, n2, sum(n1, n2)),
        2 => println!("{} - {} = {}", n1, n2, sub(n1, n2)),
        3 => println!("{} * {} = {}", n1, n2, mult(n1, n2)),
        4 => println!("{} / {} = {}", n1, n2, div(n1, n2)),
        _ => println!("erro")
    }
}

fn sum(x: f32, y: f32) -> f32{
    x + y
}
fn sub(x: f32, y: f32) -> f32{
    x - y
}
fn mult(x: f32, y: f32) -> f32{
    x * y
}
fn div(x: f32, y: f32) -> f32{
    x / y
}