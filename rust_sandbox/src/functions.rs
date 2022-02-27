// Functions are used to store blocks of code for re-use

pub fn run(){
    greeting("Hello", "Jane");  

    //Bind function values to variables
    let get_sum = add(8, 8);
    println!("Sum: {}", get_sum);

    // Closure
    let n3: i32 = 10; // We can use outsid vars, we can't do that in fn's
    let add_nums = |n1: i32, n2: i32| n1+n2+n3;
    println!("Closure Sum: {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str){

    println!("{} {}, nice to meet you!", greet, name );

}

fn add(n1: i32, n2: i32) -> i32 {
    // "-> i32" is saying that the func should return i32 values
    n1 + n2 // To return something I shouldn't use ";"
}