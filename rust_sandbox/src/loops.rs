// Loops are used to iterate until a condition is met.
// Rust has two types of loops: while and for.

pub fn run(){
    let mut _count = 0;

    // Infinite loop
    // loop {
    //     count += 1;
    //     println!("Number: {}", count);

    //     if count == 20 {
    //         break;
    //     }
    // }

    // While Loop (FizzBuzz) Div 15 == "FizzBuzz", Div 3 == "Fizz", Div 5 == "Buzz"
    // while count != 101 {

    //     if count % 15 == 0 {
    //         println!("{}: FizzBuzz", count);
    //     } else if count % 3 == 0 {
    //         println!("{}: Fizz", count);
    //     } else if count % 5 == 0 {
    //         println!("{}: Buzz", count);
    //     } else {
    //         println!("{}:", count);

    //     }
    //     count += 1;
    // }

    // For Loop Range
    for x in 0..101 {
        if x % 15 == 0 {
            println!("{}: FizzBuzz", x);
        } else if x % 3 == 0 {
            println!("{}: Fizz", x);
        } else if x % 5 == 0 {
            println!("{}: Buzz", x);
        } else {
            println!("{}:", x);

        }
    }

}