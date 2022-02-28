// Vectors are resizeble arrays

use std::mem;

pub fn run(){

    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    // Re-assign value
    numbers[2] = 20;
    println!("Re-assigning{:?}", numbers);

    // Add on to vector
    numbers.push(5);
    numbers.push(6); 
    numbers.push(3243);   
    println!("Using push: {:?}", numbers);

    // Pop off last value
    numbers.pop();
    println!("Using pop: {:?}", numbers);

    // Get single val
    println!("Single value: {}", numbers[0]);

    // Get vector length
    println!("Vector Length: {}", numbers.len());
    
    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];

    println!("Slice: {:?}", slice);

    // Loop through vector values

    for x in numbers.iter() {
        println!("Numbers: {}", x);

    }

    // Loop and mutate values
    for x in numbers.iter_mut(){
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
    
}