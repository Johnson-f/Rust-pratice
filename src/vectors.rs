// Vectors are resizable arrays 
use std::mem;


pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Re-assign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off the last value 
    numbers.pop();

    println!("{:?}", numbers);

    // Get single val 
    println!("Single value: {}", numbers[0]);

    // Get array length 
    println!("Vector length: {}", numbers.len());

    // Arrays are stack allocated 
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slices 
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Numbers: {}", x);
    }

    // Loop & mutate values - just like maps in javascript 
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers vec: {:?}", numbers);
}