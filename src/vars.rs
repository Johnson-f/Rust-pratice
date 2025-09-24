// Variables hold primitive data or reference to data 
// Variables are immutable by default
// Rust is a block-scoped language 


pub fn run() {
   let name = "Brad";
   let mut age = 37;
   println!("My name is {} and i am {}", name, age);
   age = 38;
   println!("My name is {} and i am {}", name, age);

   // Define const 
   const ID: i32 = 001;
   println!("ID: {}", ID);

   // Assign multiple variables 
   let ( my_name, my_age ) = ("Brad", 37);
   println!("{} is {}", my_name, my_age );
}