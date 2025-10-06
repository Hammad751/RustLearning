#![allow(unused)]

// There are two main string types in Rust:
// 1. String Slices (&str) - are immutable references to a sequence of UTF-8 bytes
//    - they are usually borrowed from String or string literals
//    - they have a fixed size and cannot be modified
// 2. String (String) - are growable, heap-allocated data structures
//    - they are owned and can be modified
//    - they can be created from string literals or other String instances

fn main(){
    let s: &str = "Hello, World!"; // string slice
    println!("s = {}", s);

    let s: String = String::from("Hello, World!"); // String
    println!("s = {}", s);

    let mut s: String = String::from("Hello"); // appending the string
    s.push_str(", World!");
    println!("s = {}", s);

    let mut s: String = String::from("Hello"); // appending the string in another way
    s.push(' ');
    s.push_str("World!");
    println!("s = {}", s);

    let s1: String = String::from("Hello, ");
    let s2: String = String::from("World!");
    let s3: String = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3 = {}", s3);

    let s1: String = String::from("Hello, ");
    let s2: String = String::from("World!");
    let s3: String = format!("{}{}", s1, s2); // note s1 and s2 are not moved
    println!("s3 = {}", s3);

    let msg: String = "Hello Rust".to_string();
    println!("msg = {}", msg);

    // length in bytes. usize is the type for indexing
    // - and measuring size. it depends on the architecture (32 or 64 bit)
    let len: usize = msg.len(); 
    println!("Length of msg = {}", len);

    // By default, String is UTF-8 encoded
    let hello = "Здравствуйте"; // "Hello" in Russian
    println!("Length of hello = {}", hello.len()); // each Cyrillic character takes 2 bytes

    let s = "Hello, World!"; // it' a string slice (&str)
    let h = &s[0..5]; // slicing the first 5 bytes ("Hello")
    println!("h = {}", h);

    // to convert the string slice to a String
    let s = "Hello, World!";
    let h: String = s[0..5].to_string(); // slicing and converting to String
    println!("h = {}", h);

    // Rust automatially converts the &String to &str when needed
    let s: String = String::from("Hello, World!");
    let h: &str = &s[0..5]; // slicing the first 5 bytes ("Hello")

    // Appending a &str to a String
    let mut s: String = String::from("Hello");
    // let world: &str = " World!";
    // s.push_str(world); // appending the &str to the String
    s += " World!"; // using the += operator to append a &str
    println!("s = {}", s);

    // String interpolation
    // - using the format! macro to create a formatted string
    let name: &str = "Alice";
    let age: u32 = 30;
    let info: String = format!("My name is {} and I am {} years old.", name, age);
    println!("{}", info);

}