// Primitives types
// Floats: f32, f64
// Boolean: (bool)
// Characters (char)
// Tuples - list
// Arrays -> Fixed length 


pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;
    
    // Add explict type 
    // let z = 45454454545
    
    // Find max size 
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean 
    let is_active = true;

    // Get Boolean from expressions
    let is_greater: bool = 10 < 5;

    let a1 = 'a';
    let face = '\u{1f600}';

    println!("{:?}", (x, y, is_active, is_greater, a1, face));
}