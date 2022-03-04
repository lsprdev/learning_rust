#![allow(dead_code)]
use std::io;
use std::io::Write;

struct Nums {
    x: f32,
    y: f32
}

impl Nums {
    fn sum(&self) -> f32{
        self.x + self.y
    }
    fn sub(&self) -> f32{
        self.x - self.y
    }
    fn mult(&self) -> f32{
        self.x * self.y
    }
    fn div(&self) -> f32{
        self.x / self.y
    }
}

pub fn run(){
    let ( mut in1, mut in2 ) = ( String::new(), String::new() );
    let mut cho = String::new();

    print!("Your choice(1=+, 2=-, 3=*, 4=/): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut cho).unwrap();
    let cho2: i32 = cho.trim().parse().unwrap();

    print!("Num 1: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut in1).unwrap();
    let n1: f32 = in1.trim().parse().unwrap();

    print!("Num 2: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut in2).unwrap();
    let n2: f32 = in2.trim().parse().unwrap();

    let num = Nums {
        x: n1,
        y: n2
    };

    match cho2 {

        1 => println!("{} + {} = {}", n1, n2, num.sum()),
        2 => println!("{} + {} = {}", n1, n2, num.sub()),
        3 => println!("{} + {} = {}", n1, n2, num.mult()),
        4 => println!("{} + {} = {}", n1, n2, num.div()),
        _ => println!("Erro"),

    }
}