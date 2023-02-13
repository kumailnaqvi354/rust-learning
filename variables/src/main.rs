fn main() {

    // MUTABILITY
// let mut x = 5;
// println!("the value of x is {}", x);
// x = 6;
// println!("the value of x is {}", x);

// CONSTANT 
//CANNOT BE MUTABLE AT ANY STAGE AND ARE TYPE FIXED
// RUST HELPS IN READIBLITY OF THE INTEGERS BY ADDING _ IN BETWEEN THE NUMBERS
const SUBSCRIBER_COUNT: u32 = 100_000; println!("the value of x is {}", SUBSCRIBER_COUNT); 

// SHADOWING
// redeclaring same variable to shadow previous one
// helps maintaining immutability and in changing data types of variables
let x = 5;
println!("the value of x is {}", x); 
let x = "six";
println!("the value of x is {}", x);

}
