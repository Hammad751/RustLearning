#![allow(unused)]

// If we deploy the enum command without the debug trait
// we will not be able to print it using the println! macro
// because the enum does not implement the display trait
// To fix this we can use the debug trait
// which will allow us to print the enum using the {:?} format specifier
// or we can use the {:#?} format specifier for pretty printing
// The debug trait is implemented using the derive attribute
// which is a way to automatically implement traits for our types
// We can also implement the display trait for our enum
// but that is more complex and requires more code
// For now we will use the debug trait

// #[derive(Debug)]
// #[derive(PartialEq)] // This will allow us to compare the enum values

// Ther way we can use the macro in single format
#[derive(Debug, PartialEq)]

enum Command{
    Play,
    Stop,
    Skip(u32),
    Back(u32),
    Resize { width: u32, height: u32}
}
// Enum
fn main(){
    // The simplest way to write the enum command
    let cmd_1: Command = Command::Play;
    let cmd_1: Command = Command::Skip(10);
    let cmd_1: Command = Command::Resize{width: 100, height: 50};
    // let cmd_1: Commad = Command.Skip(12);
    println!("{:#?}", cmd_1); // The output will throw an error;
    // - which is why we will use the formate method

    // Now we compare the enum values 
    let cmd_2: Command = Command::Play;
    let cmd_3: Command = Command::Skip(10);

    println!("cmd_1 == cmd_2: {}", cmd_1 == cmd_2); // false
    println!("cmd_1 == cmd_3: {}", cmd_1 == cmd_3); // true

    // // Now we compare the enum command with a match expression
    // match cmd_1 {
    //     Command::Play => println!("Playing"),
    //     Command::Stop => println!("Stopping"),
    //     Command::Skip(x) => println!("Skipping {} seconds", x),
    //     Command::Back(x) => println!("Going back {} seconds", x),
    //     Command::Resize{width, height} => println!("Resizing to {}x{}", width, height),
    // }

    // There are some built-in enums in Rust
   
    // * Option
    // - It often used to represent a value that can be either present or absent
    // - It is defined as follows:
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // these values become handy when we have to access the array values
    // - we don't need to access the values directely from the array
    // - instead we use the Option using the get() as if the index is inside the array boundry,
    // - we will get the Some with the index value
    // - and if the queried index is out of bound, this will return the None and the code will not panic
    let x: Option<i32> = Some(1); // This means there is some value
    let x: Option<i32> = None; // This means there is no value

    // * Result
    // - It is used to represent the result of an operation that can either succeed or fail
    // - It is defined as follows:
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    let x: Result<i32, String> = Ok(100);
    let x: Result<i32, String> = Err(
        "Failed to parse the number into String".to_string()
    );
}