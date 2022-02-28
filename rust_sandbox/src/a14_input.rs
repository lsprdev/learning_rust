use std::io; 
use std::io::Write;

pub fn run() {

    let mut input = String::new();
    // instead of String::new() you have an option of using String::with_capacity(…) 
    // to pre-allocate the expected size of the buffer, for efficiency. 
    // println!("Type your name: ");
    // Ex- Type your name: 
    //     Gabriel

    print!("Type your name: ");
    io::stdout() // use std::io::Write;
        .flush()
        .unwrap();
    // Ex- Type your name: Gabriel 


    io::stdin() // The rough equivalent of 'std::cin'
        .read_line(&mut input)  // It actually reads the line
        .unwrap(); // In case read_line fails
        //instead of .unwrap could be .expect("Error reading input") 
        // or "?" if in a func that returns something 
    
    println!("Hello {}", input); 
}