// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own
// string data 

pub fn run(){

    let hello1 = "Hello"; // First type
    let mut hello2 = String::from("Hello"); // Second type

    // Get length

    println!("\nLength of hello1: {}\n", hello1.len());

    // Push char
    hello2.push(' '); // .push just works with second type(hello2)

    // Push string
    hello2.push_str("World"); // This as well

    println!("hello1: {}\nhello2: {}\nLegth of hello2: {}\n", hello1, hello2, hello2.len());

    // Capacity in bytes
    println!("Capacity: {}\n", hello2.capacity());

    // Check if empty
    println!("Is Empty: {}\n", hello2.is_empty());


    // Contains 
    println!("Contains 'World' {}\n", hello2.contains("World"));
 
    // Replace
    println!("Repalce: {}\n", hello2.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello2.split_whitespace(){
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("\n{}\n", s);

    // Assertion testing
    assert_eq!(2, s.len()); // Tests if something is equal to something else
    assert_eq!(10, s.capacity());



}