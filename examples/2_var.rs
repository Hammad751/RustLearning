#![allow(unused)]

// Constants
// - constants will live inside the compile code
// - while the other variables live inside the main function
// - ecause the memory will be allocaed to the variables inside the main function
const NUM: u32 = 432;

fn main(){
    // Variables
    //  - immutable by default
    //  - use mut keywork to make the variale mutable

    let mut x = 1; 
    x +=1;

    println!("x = {x}");

    // the default data type of rust variables is i32
    // type inference

    let y: i32 = -2;
    let z = -1;

    // shadowing
    //  - declaring the same variable multiple times with diff types and/or values

    let x: i32 = 1;
    let x: i32 = 2;
    let x: bool = true;

    println!("x = {}", x);

    // type placeholder
    let x: _ = 123; // this will the compiler to find out the type of the variable

    // Debug

    println!("Debug = {:?}", x);
    println!("Debug = {:#?}", x);
}