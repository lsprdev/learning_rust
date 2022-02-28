use std::io;
use std::io::Write;

pub fn run(){

    // DECLARING INPUT VARIABLES
    let mut in1 = String::new();
    let mut in2 = String::new();

    // FIRST INPUT
    print!("Type a number: ");
    io::stdout() // This part is just for aesthetics
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut in1) // Using in1
        .unwrap();

    // SECOND INPUT
    print!("Type another number: ");
    io::stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut in2) // Using in2
        .unwrap();

    // STRING TO INTEGER - FIRST INPUT
    let n1: i32 = in1.trim()  // ".trim" ignore whitespace around input
        .parse() // ".parse" convert to integers
        .unwrap();

    // STRING TO INTEGER - SECOND INPUT
    let n2: i32 = in2.trim()
        .parse()
        .unwrap();

    // SUM OF VALUES AND FINAL PRINT
    let sum = n1+n2;
    println!("{} + {} = {}", n1, n2, sum);

}