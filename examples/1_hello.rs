 /*
  * Attribute - metadata for the compiler
  * it tells the compiler to ignor the vriables or functions that are not in use to run the example file write the example command as:
  *  cargo run --example hello (hello is the file name without .rs extention) 
  * compile time : "target(s) in 0.82s" 
  */ 
#![allow(unused)]
use std::println; // import the println macro from the std library
fn main() {
    // functions that end with ! mark are called Macros : that generates the rust code at compile time
    // and are invoked with exclaimatery mark (!)
    // println! is the macro that consoles the output 
    println!("Hello, world!");
}
