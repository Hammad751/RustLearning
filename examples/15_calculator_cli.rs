#![allow(unused)]

use std::{io, print, println};

fn main(){

    println!("\nCalculator\n");
    
    println!("Enter the first number: ");
    let mut first = String::new();

    io::stdin().read_line(&mut first).expect("Failed to read line");

    let first: i32 = match first.trim().parse(){
        Ok(num) => num, 
        Err(_) =>{
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    println!("Enter the second number: ");
    let mut second = String::new();
    io::stdin().read_line(&mut second).expect("Failed to read line");


    let second: i32 = match second.trim().parse(){
        Ok(num) => num, 
        Err(_) =>{
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    println!("Enter the choice (1 for addition, 2 for subtraction, 3 for multiplication, 4 for division, 5 for modulo): ");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");

    let choice: f32 = match choice.trim().parse(){
        Ok(op) => op, 
        Err(_) =>{
            println!("Invalid input. Please enter a valid operation.");
            return;
        }
    };

    if choice == 1.0{
        let sum = add(first, second);
        println!("Result: {}", sum);
    }

    else if choice == 2.0{
        let difference = sub(first, second);
        println!("Result: {}", difference);
    }

    else if choice == 3.0{
        let product = mul(first, second);
        println!("Result: {}", product);
    }

    else if choice == 4.0{
        let quotient = div(first, second);
        match quotient {
            Some(result) => println!("Result: {}", result),
            None => println!("Error: Division by zero is not allowed."),
        }
    }

    else if choice == 5.0{
        let remainder = modulo(first, second);
        match remainder {
            Some(result) => println!("Result: {}", result),
            None => println!("Error: Division by zero is not allowed."),
        }
    }

    else {
        print!("Invalid choice. Please enter a valid operation.");
    }

    // match operation.trim() {
    //     "+" => println!("Result: {}", add(first, second)),
    //     "-" => println!("Result: {}", sub(first, second)),
    //     "*" => println!("Result: {}", mul(first, second)),
    //     "/" => match div(first, second) {
    //         Some(result) => println!("Result: {}", result),
    //         None => println!("Error: Division by zero is not allowed."),
    //     },
    //     "%" => match mod(first, second) {
    //         Some(result) => println!("Result: {}", result),
    //         None => println!("Error: Division by zero is not allowed."),
    //     },
    //     _ => println!("Invalid operation. Please enter a valid operation."),
    // }

    for number in (1..4){ // .rev to reverse the loop
        println!("Number: {number}");
    }

    println!("LIFTOFF...");
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn sub(x: i32, y: i32) -> i32 {
    x - y
}

fn mul(x: i32, y: i32) -> i32 {
    x * y
}

fn div(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

fn modulo(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x % y)
    }
}