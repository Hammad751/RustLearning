#![allow(unused)]

use core::num;
use std::{io, println};

fn main(){
    let mut celc: u8 = 25;

    println!("\nTemperature Converter\n");

    println!("1: Temperature from fahrenheit to Celsius: {}", celc);
    println!("2: Temperature from Celsius to Fahrenheit: {}", (celc as f32 * 1.8) + 32.0);
    println!("3: Temperature from Celsius to Kelvin: {}", celc as f32 + 273.15);
    println!("4: Temperature from Kelvin to Celsius: {}", celc as f32 - 273.15);

    println!("\nEnter the Choice (1-4): ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    // let input: u8 = input.trim().parse().expect("Please type a number!");
    let input: &str = input.trim();
    let input: u8 = match input.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please type a number!");
            return;
        }
    };

    if input == 1{
        fahrenheit_to_celsius();
        // println!("Temperature in Celsius: {}", (celc as f32 - 32.0) * 5.0 / 9.0);
    }

    else if input == 2{

        celsius_to_fahrenheit();
        // println!("Temperature in Fahrenheit: {}", (celc as f32 * 1.8) + 32.0);
    }

    else if input == 3{
        celsius_to_kelvin();
        // println!("Temperature in Kelvin: {}", celc as f32 + 273.15);
    }
    else if input == 4{
        kelvin_to_celsius();
        // println!("Temperature in Celsius: {}", celc as f32 - 273.15);
    }

    else{
        println!("Invalid Choice");
    }

    // io::stdin().read_line(&mut input).expect("Failed to read line");
}

fn celsius_to_fahrenheit() {

    println!("Enter the temperature in Celsius: ");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read line");

    let temp: f32 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please type a number!");
            return;
        }
    };

    let fahrenheit = (temp * 1.8) + 32.0; // 9.0 / 5.0 is 1.8
    println!("Temperature in Fahrenheit: {}", fahrenheit);
    println!("{:.2}°C is equal to {:.2}°F", temp, fahrenheit);
}

fn fahrenheit_to_celsius() {
    println!("Enter the temperature in Fahrenheit: ");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read line");

    let temp: f32 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please type a number!");
            return;
        }
    };

    let celsius = (temp - 32.0) * 5.0 / 9.0;
    println!("Temperature in Celsius: {}", celsius);
    println!("{:.2}°F is equal to {:.2}°C", temp, celsius);
}

fn kelvin_to_celsius() {
    println!("Enter the temperature in Kelvin: ");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read line");

    let temp: f32 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please type a number!");
            return;
        }
    };

    let celsius = temp - 273.15;
    println!("Temperature in Celsius: {}", celsius);
    println!("{:.2}K is equal to {:.2}°C", temp, celsius);
}

fn celsius_to_kelvin() {
    println!("Enter the temperature in Celsius: ");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read line");

    let temp: f32 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please type a number!");
            return;
        }
    };

    let kelvin = temp + 273.15;
    println!("Temperature in Kelvin: {}", kelvin);
    println!("{:.2}°C is equal to {:.2}K", temp, kelvin);
}