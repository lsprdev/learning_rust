pub fn run(){

    // Print to console
    println!("\nHello from the print.rs file!\n");

    // Basic Formatting
    println!("{} is from {}", "Brad", "Mass\n");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code\n");

    // Named Arguments
    println!("{name} likes to play {activity}\n", name = "John", activity = "Baseball");

    // Placeholder traits
    println!("Binary: {:b}\nHex: {:x}\nOctal: {:o}\n", 10, 10, 10);

    // Placeholder for debug traits
    println!("{:?}\n", (12, true, "Hello"));

    //Basic Math
    println!("10 + 10 = {}\n", 10+10);
}