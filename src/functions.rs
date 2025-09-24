//Functions - used to store block of code for reuse 


pub fn run() {
    greetings("Hello", "Jane");

    // Bind function values to variables 
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // Closure 
    let add_nums 
}

fn greetings(greet: &str, name: &str) {
    println!("{} {}, nice to meet to you", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}