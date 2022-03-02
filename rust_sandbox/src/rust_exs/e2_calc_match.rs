use std::io;
use std::io::Write;

pub fn run() {

    let ( mut in1, mut in2 ) = ( String::new(), String::new() );
    let mut cho = String::new();

    println!("======================================");
    print!("Choices: || 1 = +, 2 = -, 3 = *, 4 = / ||: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut cho).unwrap();  
    let cho2: u8 = cho.trim().parse().unwrap();

    print!("Num 1: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut in1).unwrap();    
    let n1: f64 = in1.trim().parse().unwrap();

    print!("Num 2: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut in2).unwrap();
    let n2: f64 = in2.trim().parse().unwrap();

    match cho2 {

        1 => println!("{} + {} = {}", n1, n2, n1+n2),
        2 => println!("{} - {} = {}", n1, n2, n1-n2),
        3 => println!("{} * {} = {}", n1, n2, n1*n2),
        4 => println!("{} / {} = {}", n1, n2, n1/n2),
        _ => println!("erro"),
        
    }
}
