use std::io;
use std::io::Write;

// Calculator(only integers yet)

pub fn run(){
    let ( mut in1, mut in2 ) = ( String::new(), String::new() );
    


    print!("First num: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut in1).unwrap();

    print!("Second num: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut in2).unwrap();

    println!("{}", sum(in1, in2));
}

fn esc(){

    
}

// SUM FUNCTION
fn sum(x: String, y: String) -> i32 {
    // PARSING: STRING -> I32
    let n1: i32 = x.trim().parse().unwrap();
    let n2: i32 = y.trim().parse().unwrap();

    n1 + n2
}
// SUBTRACTION FUNCTION
fn sub(x: String, y: String) -> i32 {
    // PARSING: STRING -> I32
    let n1: i32 = x.trim().parse().unwrap();
    let n2: i32 = y.trim().parse().unwrap();

    n1 - n2
}

// MULTIPLICATION FUNCTION
fn mult(x: String, y: String) -> i32 {
    // PARSING: STRING -> I32
    let n1: i32 = x.trim().parse().unwrap();
    let n2: i32 = y.trim().parse().unwrap();

    n1 * n2
}

// DIVISION FUNCTION
fn div(x: String, y: String) -> i32 {
    // PARSING: STRING -> I32
    let n1: i32 = x.trim().parse().unwrap();
    let n2: i32 = y.trim().parse().unwrap();

    n1 / n2
}